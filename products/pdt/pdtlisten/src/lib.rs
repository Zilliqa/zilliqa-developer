mod importer;
mod listener;
mod types;
use anyhow::{anyhow, bail, Context, Error, Result};
use ethers::{prelude::*, providers::StreamExt, utils::hex};
use itertools::Itertools;
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use pdtbq::{bq::ZilliqaBQProject, bq_utils::BigQueryDatasetLocation};
use pdtdb::{
    utils::ProcessCoordinates,
    values::{BQMicroblock, BQTransaction, PSQLMicroblock, PSQLTransaction, ZILTransactionBody},
    zqproj::{Inserter, ZilliqaDBProject},
};
use pdtpsql::psql::ZilliqaPSQLProject;
use serde::Serialize;
use serde_json::{from_value, to_value, Value};
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, marker::PhantomData, time::Duration};
use tokio::pin;
use tokio::task::JoinSet;
use tokio_stream::StreamExt as TokioStreamExt;

use crate::listener::listen_blocks;

const MAX_TASKS: usize = 50;

#[allow(dead_code)]
async fn get_block_info(number: U64, client: &HttpClient) -> Result<types::GetTxBlockResponse> {
    let params = rpc_params![number.to_string()];
    let response: Value = client.request("GetTxBlock", params).await?;
    let tx_block: types::GetTxBlockResponse = from_value(response)?;
    Ok(tx_block)
}

fn convert_block_and_txns(
    block: &Block<Transaction>,
    zil_txn_bodies: &Vec<ZILTransactionBody>,
) -> Result<(BQMicroblock, Vec<BQTransaction>)> {
    let my_block = block.clone();
    let bq_block = BQMicroblock::from_eth(&my_block)?;
    let version = bq_block.header_version;
    let zil_transactions: HashMap<&str, &ZILTransactionBody> =
        zil_txn_bodies.iter().map(|x| (x.id.as_str(), x)).collect();

    let (bq_transactions, txn_errs): (Vec<BQTransaction>, Vec<Error>) = my_block
        .transactions
        .into_iter()
        .map(|txn| {
            zil_transactions
                .get(hex::encode(txn.hash.as_bytes()).as_str())
                .map_or(
                    Err(anyhow!("zil transaction body not found")),
                    |&txn_body| {
                        BQTransaction::from_eth_with_zil_txn_bodies(&txn, txn_body, version)
                    },
                )
        })
        .partition_result();

    if !txn_errs.is_empty() {
        bail!(
            "some transactions could not be converted, skipping this block: {:#?}",
            txn_errs
        );
    }
    Ok((bq_block, bq_transactions))
}

async fn insert_block_and_txns<P: ZilliqaDBProject>(
    block_req: Inserter<impl Into<BQMicroblock> + Serialize + std::marker::Send>,
    txn_req: Inserter<impl Into<BQTransaction> + Serialize + std::marker::Send>,
    block_num: i64,
    proj: &P,
) -> Result<()> {
    proj.insert_microblocks(block_req, &(block_num..(block_num + 1)))
        .await
        .map_err(|e| anyhow!("could not insert block to err: {:}", e))?;
    proj.insert_transactions(txn_req, &(block_num..(block_num + 1)))
        .await
        .map_err(|e| anyhow!("could not insert transactions due to error: {:}", e))
}

async fn postgres_insert_block_and_txns(
    block: &Block<Transaction>,
    proj: &ZilliqaPSQLProject,
) -> Result<()> {
    let my_block = block.clone();
    let psql_block: PSQLMicroblock = BQMicroblock::from_eth(&my_block)?.into();
    let version = psql_block.header_version;
    let block_num = psql_block.block;

    let (txns, txn_errs): (Vec<BQTransaction>, Vec<Error>) = my_block
        .transactions
        .into_iter()
        .map(|txn| BQTransaction::from_eth(&txn, version))
        .partition_result();
    let psql_txns = txns
        .into_iter()
        .map(|txn| Into::<PSQLTransaction>::into(txn))
        .collect_vec();

    if !txn_errs.is_empty() {
        bail!(
            "some transactions could not be converted, skipping this block: {:#?}",
            txn_errs
        );
    }
    // let err_psql_block = psql_block.clone();
    let psql_block_req = Inserter {
        _marker: PhantomData,
        req: vec![psql_block],
    };
    let psql_txn_req = Inserter {
        _marker: PhantomData,
        req: psql_txns,
    };

    insert_block_and_txns(psql_block_req, psql_txn_req, block_num, proj)
        .await
        .with_context(|| format!("caused by block {:?}", block))
}

async fn get_block_by_hash(x: H256, provider: &Provider<Http>) -> Result<Block<Transaction>> {
    println!("found block with hash {:?}, getting block info", x);
    fn serialize(v: impl serde::Serialize) -> Value {
        to_value(v).unwrap()
    }
    let mut raw_block: Value = provider
        .request("eth_getBlockByHash", [serialize(x), serialize(true)])
        .await?;
    // ZIL-5328 means our nonce is only one byte instead of 8, which ethers
    // is not happy about.
    raw_block["nonce"] = serde_json::to_value("0x0000000000000000")?;

    let mut block: Block<Transaction> = serde_json::from_value(raw_block)?;
    while block.number.is_none() {
        println!("{:?} is pending, looping", x);

        // loop until the block is no longer pending
        // the sleep duration is set arbitrarily.
        tokio::time::sleep(Duration::from_millis(1000)).await;
        raw_block = provider
            .request("eth_getBlockByHash", [serialize(x), serialize(true)])
            .await?;
        raw_block["nonce"] = serde_json::to_value("0x0000000000000000")?;

        block = serde_json::from_value(raw_block)?;
    }
    println!("found block number {:?}", block.number);
    // println!("{:#?}", block);
    Ok(block)
}

pub async fn listen_psql(postgres_url: &str, api_url: &str) -> Result<()> {
    let mut jobs = JoinSet::new();
    let coords = ProcessCoordinates {
        nr_machines: 1,
        batch_blks: 1,
        machine_id: 0,
        client_id: "listen".to_string(),
    };

    let p_client = PgPoolOptions::new()
        .max_connections(100)
        .connect(postgres_url)
        .await?;

    println!("checking schemas..");
    ZilliqaPSQLProject::ensure_schema(&p_client).await?;
    println!("all good.");

    let zilliqa_psql_proj = ZilliqaPSQLProject::new(
        postgres_url,
        p_client.clone(),
        &coords.with_client_id("listen_psql"),
        i64::MAX,
        10000,
    )?;

    let provider = Provider::<Http>::try_from(api_url)?;

    let mut stream = StreamExt::map(provider.watch_blocks().await?, |hash: H256| {
        get_block_by_hash(hash, &provider)
    })
    .buffered(MAX_TASKS);

    while let Some(block) = StreamExt::next(&mut stream).await {
        match block {
            Ok(block) => {
                // let postgres go off and do its thing

                // the projects themselves contain no state and should be cheap
                // to clone.
                let my_psql_proj = zilliqa_psql_proj.clone();
                jobs.spawn(async move {
                    match postgres_insert_block_and_txns(&block, &my_psql_proj).await {
                        Ok(_) => {
                            println!("inserted into postgres");
                        }
                        Err(err) => {
                            eprintln!(
                                "unable to insert block and transactions into postgres with error {:#?}",
                                err
                            );
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("could not get block from hash due to error {:#?}", e);
            }
        }
        println!("waiting for next block...");
    }
    println!("main: waiting for jobs to complete..");
    while let Some(_) = jobs.join_next().await {
        continue;
    }
    Ok(())
}

/// Have implemented a listening system that queries the latest found block in the meta table.
/// This allows continuity from last listen or import was carried out
/// The listen also keeps track blocks that it has encountered before, discarding any seen blocks
/// If encounters a gap of block received with last seen, tries to patch it
pub async fn listen_bq(
    bq_project_id: &str,
    bq_dataset_id: &str,
    api_url: &str,
    block_buffer_size: usize,
) -> Result<()> {
    // let mut jobs = JoinSet::new();
    let coords = ProcessCoordinates {
        nr_machines: 1,
        batch_blks: 1,
        machine_id: 0,
        client_id: "listen".to_string(),
    };
    let loc = BigQueryDatasetLocation {
        project_id: bq_project_id.to_string(),
        dataset_id: bq_dataset_id.to_string(),
    };

    println!("checking schemas..");
    ZilliqaBQProject::ensure_schema(&loc).await?;
    println!("all good.");

    let zilliqa_bq_proj = ZilliqaBQProject::new(
        &loc,
        &coords.with_client_id("listen_bq"),
        i64::MAX, //we don't need nr_blks, but max it out just in case
    )
    .await?;

    let mut bq_importer = importer::BatchedImporter::new();
    bq_importer.reset_buffer(&zilliqa_bq_proj).await?;

    let provider = Provider::<Http>::try_from(api_url)?;

    let stream = listen_blocks(&provider, zilliqa_bq_proj.get_latest_block().await?);
    pin!(stream);

    while let Some(blocks) = TokioStreamExt::next(&mut stream).await {
        match blocks {
            Ok(blocks) => {
                if bq_importer.buffers.is_none() {
                    bq_importer.reset_buffer(&zilliqa_bq_proj).await?;
                }

                for (block, zil_txn_bodies) in blocks {
                    // convert our blocks and insert it into our buffer
                    convert_block_and_txns(&block, &zil_txn_bodies)
                        .and_then(|(bq_block, bq_txns)| {
                            bq_importer.insert_into_buffer(bq_block, bq_txns)
                        })
                        .unwrap_or_else(|err| {
                            eprintln!("conversion to bq failed due to {:?}", err);
                        })
                }

                // if we've got enough blocks in hand
                if bq_importer.n_blocks() >= block_buffer_size {
                    let my_bq_proj = zilliqa_bq_proj.clone();
                    let buffers = bq_importer.take_buffers()?;
                    let range = bq_importer
                        .range
                        .take()
                        .expect("range should be set if buffers were taken.");
                    // let bigquery go off and do its thing, we can start
                    // collecting the next batch.
                    tokio::spawn(async move {
                        match async {
                            my_bq_proj
                                .insert_microblocks(buffers.mb_inserter, &range)
                                .await?;
                            my_bq_proj
                                .insert_transactions(buffers.txn_inserter, &range)
                                .await
                        }
                        .await
                        {
                            Ok(_) => {
                                println!("inserted into bq");
                            }
                            Err(err) => {
                                eprintln!("unable to insert block and transactions into bq due to error {:}",err);
                            }
                        }
                    });
                    bq_importer.reset_buffer(&zilliqa_bq_proj).await?;
                }
            }
            Err(e) => {
                eprintln!("could not get block from hash due to error {:#?}", e);
            }
        }
        println!("waiting for next block...");
    }
    println!("main: waiting for jobs to complete..");

    Ok(())
}
