use crate::bq_client::client_from_default_credentials;
use crate::bq_utils;
use crate::tracked::TrackedTable;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gcp_bigquery_client::model::{
    dataset::Dataset, range_partitioning::RangePartitioning,
    range_partitioning_range::RangePartitioningRange, table::Table,
    table_field_schema::TableFieldSchema, table_schema::TableSchema,
};
use gcp_bigquery_client::Client;
use pdtdb::tracked::Trackable;
use pdtdb::utils::ProcessCoordinates;
use pdtdb::{
    values,
    zqproj::{BlockInsertable, Inserter, InsertionErrors, ZilliqaDBProject},
};
use serde::Serialize;

use std::marker::PhantomData;
use std::ops::Range;
use tokio::time::{sleep, Duration};

pub const TRANSACTION_TABLE_ID: &str = "transactions";
pub const MICROBLOCK_TABLE_ID: &str = "microblocks";

// BigQuery imposes a limit of 10MiB per HTTP request.
pub const MAX_QUERY_BYTES: usize = 9 << 20;

// BigQuery imposes a 50k row limit and recommends 500
pub const MAX_QUERY_ROWS: usize = 500;

#[derive(Clone)]
pub struct ZilliqaBQProject {
    pub bq: bq_utils::BigQueryDatasetLocation,
    pub bq_client: Client,
    pub ds: Dataset,
    pub transactions: TrackedTable,
    pub microblocks: TrackedTable,
    pub client_id: String,
}

impl ZilliqaBQProject {
    /// Creates tables so that we can do this in a single thread.
    pub async fn ensure_schema(bq: &bq_utils::BigQueryDatasetLocation) -> Result<()> {
        let client = client_from_default_credentials().await?.client;
        let txn_location = bq_utils::BigQueryTableLocation::new(&bq, TRANSACTION_TABLE_ID);
        if txn_location.find_table(&client).await.is_none() {
            Self::create_transaction_table(&client, &txn_location).await?;
        }

        TrackedTable::ensure_schema(&client, &txn_location).await?;
        let microblock_location = bq.with_table_id(MICROBLOCK_TABLE_ID);
        if microblock_location.find_table(&client).await.is_none() {
            Self::create_microblock_table(&client, &microblock_location).await?;
        }
        TrackedTable::ensure_schema(&client, &microblock_location).await?;
        Ok(())
    }

    pub async fn new(
        bq: &bq_utils::BigQueryDatasetLocation,
        coords: &ProcessCoordinates,
        nr_blks: i64,
    ) -> Result<Self> {
        // Application default creds don't work here because the auth library looks for a service
        // account key in the file you give it and, of course, it's not there ..
        let my_client = client_from_default_credentials().await?.client;

        let zq_ds = if let Ok(ds) = my_client
            .dataset()
            .get(&bq.project_id, &bq.dataset_id)
            .await
        {
            ds
        } else {
            my_client
                .dataset()
                .create(Dataset::new(&bq.project_id, &bq.dataset_id))
                .await?
        };

        let txn_location = bq.with_table_id(TRANSACTION_TABLE_ID);
        let transaction_table = txn_location.find_table(&my_client).await.ok_or(anyhow!(
            "No transaction table - have you created the schema?"
        ))?;
        let txns = TrackedTable::new(&txn_location, coords, transaction_table, nr_blks)?;

        let microblock_location = bq.with_table_id(MICROBLOCK_TABLE_ID);
        let microblock_table = microblock_location
            .find_table(&my_client)
            .await
            .ok_or(anyhow!(
                "No microblock table - have you created the schema?"
            ))?;
        let micros = TrackedTable::new(&microblock_location, coords, microblock_table, nr_blks)?;

        Ok(ZilliqaBQProject {
            bq: bq.clone(),
            bq_client: my_client,
            ds: zq_ds,
            transactions: txns,
            microblocks: micros,
            client_id: coords.client_id.to_string(),
        })
    }

    async fn create_microblock_table(
        client: &Client,
        bq: &bq_utils::BigQueryTableLocation,
    ) -> Result<Table> {
        let microblock_table = Table::new(
            &bq.dataset.project_id,
            &bq.dataset.dataset_id,
            &bq.table_id,
            TableSchema::new(vec![
                TableFieldSchema::integer("block"),
                TableFieldSchema::integer("offset_in_block"),
                TableFieldSchema::integer("shard_id"),
                TableFieldSchema::integer("header_version"),
                TableFieldSchema::bytes("header_committee_hash"),
                TableFieldSchema::bytes("header_prev_hash"),
                TableFieldSchema::integer("gas_limit"),
                TableFieldSchema::big_numeric("rewards"),
                TableFieldSchema::bytes("prev_hash"),
                TableFieldSchema::bytes("tx_root_hash"),
                TableFieldSchema::bytes("miner_pubkey"),
                TableFieldSchema::bytes("miner_addr_zil"),
                TableFieldSchema::bytes("miner_addr_eth"),
                TableFieldSchema::integer("ds_block_num"),
                TableFieldSchema::bytes("state_delta_hash"),
                TableFieldSchema::bytes("tran_receipt_hash"),
                TableFieldSchema::integer("block_shard_id"),
                TableFieldSchema::integer("gas_used"),
                TableFieldSchema::integer("epoch_num"),
                TableFieldSchema::integer("num_txs"),
                TableFieldSchema::bytes("blockhash"),
                TableFieldSchema::integer("timestamp"),
                TableFieldSchema::bytes("cs1"),
                TableFieldSchema::string("b1"),
                TableFieldSchema::bytes("cs2"),
                TableFieldSchema::string("b2"),
                TableFieldSchema::string("imported_from"),
                TableFieldSchema::bytes("eth_parent_hash"),
                TableFieldSchema::bytes("eth_uncles_hash"),
                TableFieldSchema::bytes("eth_state_root"),
                TableFieldSchema::bytes("eth_extra_data"),
                TableFieldSchema::bytes("eth_logs_bloom"),
                TableFieldSchema::integer("eth_difficulty"),
                TableFieldSchema::integer("eth_total_difficulty"),
                TableFieldSchema::bytes("eth_nonce"),
                TableFieldSchema::integer("eth_base_fee_per_gas"),
                TableFieldSchema::bytes("eth_withdrawals_root"),
                TableFieldSchema::string("eth_withdrawals"),
            ]),
        )
        .range_partitioning(RangePartitioning {
            field: Some("block".to_string()),
            range: Some(RangePartitioningRange {
                start: "0".to_string(),
                // There can only be 10k partitions.
                // Make it 10k less than that.
                end: "100000000".to_string(),
                // About once every two weeks?
                interval: "100000".to_string(),
            }),
        });
        let the_table = client.table().create(microblock_table).await;
        match the_table {
            Ok(tbl) => Ok(tbl),
            Err(_) => {
                // Wait a bit and then fetch the table.
                sleep(Duration::from_millis(5_000)).await;
                Ok(client
                    .table()
                    .get(
                        &bq.dataset.project_id,
                        &bq.dataset.dataset_id,
                        &bq.table_id,
                        Option::None,
                    )
                    .await?)
            }
        }
    }

    async fn create_transaction_table(
        client: &Client,
        bq: &bq_utils::BigQueryTableLocation,
    ) -> Result<Table> {
        let transaction_table = Table::new(
            &bq.dataset.project_id,
            &bq.dataset.dataset_id,
            &bq.table_id,
            TableSchema::new(vec![
                TableFieldSchema::string("id"),
                TableFieldSchema::integer("block"),
                TableFieldSchema::integer("offset_in_block"),
                // Zilliqa version * 100
                TableFieldSchema::integer("zqversion"),
                TableFieldSchema::big_numeric("amount"),
                TableFieldSchema::string("api_type"),
                TableFieldSchema::bytes("code"),
                TableFieldSchema::bytes("data"),
                TableFieldSchema::integer("gas_limit"),
                TableFieldSchema::big_numeric("gas_price"),
                TableFieldSchema::integer("nonce"),
                TableFieldSchema::string("raw_receipt"),
                TableFieldSchema::string("receipt"),
                TableFieldSchema::bytes("sender_public_key"),
                TableFieldSchema::string("from_addr_zil"),
                TableFieldSchema::string("from_addr_eth"),
                TableFieldSchema::bytes("signature"),
                TableFieldSchema::string("to_addr"),
                TableFieldSchema::integer("version"),
                TableFieldSchema::integer("cum_gas"),
                TableFieldSchema::integer("shard_id"),
                TableFieldSchema::string("imported_from"),
                TableFieldSchema::integer("eth_transaction_index"),
                TableFieldSchema::big_numeric("eth_value"),
                TableFieldSchema::bytes("eth_input"),
                TableFieldSchema::integer("eth_v"),
                TableFieldSchema::string("eth_r"),
                TableFieldSchema::string("eth_s"),
                TableFieldSchema::integer("eth_transaction_type"),
            ]),
        )
        .range_partitioning(RangePartitioning {
            field: Some("block".to_string()),
            range: Some(RangePartitioningRange {
                start: "0".to_string(),
                // There can only be 10k partitions.
                // Make it 10k less than that.
                end: "100000000".to_string(),
                // About once every two weeks?
                interval: "100000".to_string(),
            }),
        });
        let the_table = client.table().create(transaction_table).await;
        match the_table {
            Ok(tbl) => Ok(tbl),
            Err(_) => {
                // Wait a bit and then fetch the table.
                sleep(Duration::from_millis(5_000)).await;
                Ok(client
                    .table()
                    .get(
                        &bq.dataset.project_id,
                        &bq.dataset.dataset_id,
                        &bq.table_id,
                        Option::None,
                    )
                    .await?)
            }
        }
    }

    pub fn get_max_insert_bytes(&self) -> usize {
        MAX_QUERY_BYTES
    }

    pub fn get_max_query_rows(&self) -> usize {
        MAX_QUERY_ROWS
    }

    pub async fn get_latest_block(&self) -> Result<Option<i64>> {
        return self.microblocks.get_max_meta_block(&self.bq_client).await;
    }
}

#[async_trait]
impl ZilliqaDBProject for ZilliqaBQProject {
    /// Create an insertion request
    async fn make_inserter<T: Serialize + BlockInsertable>(&self) -> Result<Inserter<T>> {
        Ok(Inserter {
            _marker: PhantomData,
            req: Vec::new(),
        })
    }

    /// Act on an inserter.
    async fn insert_transactions(
        &self,
        req: Inserter<impl Into<values::BQTransaction> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        let mapped_inserter = Inserter::<values::BQTransaction> {
            _marker: PhantomData,
            req: req.req.into_iter().map(|x| x.into()).collect(),
        };
        Ok(self
            .transactions
            .insert(&self.bq_client, mapped_inserter, blks)
            .await?)
    }

    /// Act on an inserter.
    async fn insert_microblocks(
        &self,
        req: Inserter<impl Into<values::BQMicroblock> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        let mapped_inserter = Inserter::<values::BQMicroblock> {
            _marker: PhantomData,
            req: req.req.into_iter().map(|x| x.into()).collect(),
        };
        Ok(self
            .microblocks
            .insert(&self.bq_client, mapped_inserter, blks)
            .await?)
    }

    async fn get_txn_range(&self, start_at: i64) -> Result<Option<Range<i64>>> {
        Ok(self
            .transactions
            .find_next_range_to_do(&self.bq_client, start_at)
            .await?)
    }

    async fn is_txn_range_covered_by_entry(
        &self,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>> {
        self.transactions
            .is_range_covered_by_entry(&self.bq_client, start, blks)
            .await
    }
}
