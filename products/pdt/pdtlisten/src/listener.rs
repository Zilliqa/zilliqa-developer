use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Block, Transaction, U64},
};

use anyhow::Result;
use async_stream::try_stream;
use pdtdb::values::ZILTransactionBody;
use serde_json::{to_value, Value};
use tokio::time::{interval, Duration};
use tokio_stream::Stream;

async fn get_zil_transaction_bodies_from_block(
    block_number: U64,
    provider: &Provider<Http>,
) -> Result<Vec<ZILTransactionBody>> {
    let mut raw_zil_txn_bodies: Value = provider
        .request("GetTxnBodiesForTxBlock", [block_number.to_string()])
        .await?;

    // Serialise all receipts again
    if let Some(txn_bodies) = raw_zil_txn_bodies.as_array_mut() {
        txn_bodies.into_iter().for_each(|value| {
            if let Some(v) = value.get_mut("receipt") {
                *v = Value::String(v.to_string());
            }
        })
    }

    let zil_txn_bodies: Vec<ZILTransactionBody> =
        serde_json::from_value::<Vec<ZILTransactionBody>>(raw_zil_txn_bodies)?;

    Ok(zil_txn_bodies)
}

async fn get_block_by_number(x: U64, provider: &Provider<Http>) -> Result<Block<Transaction>> {
    println!("found block with number {:?}, getting block info", x);
    fn serialize(v: impl serde::Serialize) -> Value {
        to_value(v).unwrap()
    }
    let mut raw_block: Value = provider
        .request("eth_getBlockByNumber", [serialize(x), serialize(true)])
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
            .request("eth_getBlockByNumber", [serialize(x), serialize(true)])
            .await?;
        raw_block["nonce"] = serde_json::to_value("0x0000000000000000")?;

        block = serde_json::from_value(raw_block)?;
    }
    println!("found block number {:?}", block.number);
    // println!("{:#?}", block);
    Ok(block)
}

/// Fetches the most recent block number and compares against `last_seen_block_number` and retrieves all blocks in between
async fn get_block(
    provider: &Provider<Http>,
    last_seen_block_number: &mut Option<U64>,
) -> Result<Vec<(Block<Transaction>, Vec<ZILTransactionBody>)>> {
    let block_number = provider.get_block_number().await?;

    let last_seen_block_number_unwrap = if let Some(_block_number) = last_seen_block_number {
        _block_number.clone()
    } else {
        // if does not know last_seen_block, assumes it was just the one before
        block_number - 1
    };

    if block_number <= last_seen_block_number_unwrap {
        // Already seen this block
        return Ok(Vec::new());
    }

    let mut blocks: Vec<(Block<Transaction>, Vec<ZILTransactionBody>)> = Vec::new();
    for _block_number in last_seen_block_number_unwrap.as_u64() + 1..=block_number.as_u64() {
        let block = get_block_by_number(_block_number.into(), provider).await?;
        let txn_bodies = if block.transactions.is_empty() {
            Vec::default()
        } else {
            get_zil_transaction_bodies_from_block(block_number, provider).await?
        };
        blocks.push((block, txn_bodies));
    }

    *last_seen_block_number = Some(block_number);

    Ok(blocks)
}

/// Polls in an interval for new blocks, tracking using `last_seen_block_number`
pub fn listen_blocks(
    provider: &Provider<Http>,
    from_block: Option<i64>,
) -> impl Stream<Item = Result<Vec<(Block<Transaction>, Vec<ZILTransactionBody>)>>> + '_ {
    try_stream! {
        let mut interval = interval(Duration::from_secs(15));
        let mut last_seen_block_number: Option<U64> = from_block.map(U64::from);
        loop {
            interval.tick().await;
            let blocks = get_block(provider, &mut last_seen_block_number).await?;
            yield blocks
        }
    }
}
