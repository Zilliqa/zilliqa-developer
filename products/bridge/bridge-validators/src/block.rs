use std::{collections::btree_map::Range, future::IntoFuture};

use async_trait::async_trait;

use anyhow::Result;
use ethers::{
    abi::{ethereum_types::BloomInput, Address},
    providers::Middleware,
    types::{BlockNumber, Bloom, H256},
};
use ethers_contract::EthEvent;
use futures::{StreamExt, TryStreamExt};
use tracing::warn;

use crate::{client::ChainClient, DispatchedFilter, RelayedFilter};

#[async_trait]
pub trait BlockPolling {
    async fn stream_finalized_blocks(&mut self) -> Result<()>;
    async fn check_filter(&self) -> Result<()>;
    async fn get_historic_blocks(&self, from: u64, to: u64) -> Result<()>;
}

#[async_trait]
impl BlockPolling for ChainClient {
    async fn stream_finalized_blocks(&mut self) -> Result<()> {
        Ok(())
    }

    async fn get_historic_blocks(&self, from: u64, to: u64) -> Result<()> {
        let concurrent_requests = futures::stream::iter(
            (from..to)
                .into_iter()
                .map(|block_number| self.client.get_block(block_number)),
        )
        .buffer_unordered(3)
        .map(|r| {
            println!("finished request: {:?}", r);
            r
        })
        .try_collect::<Vec<_>>();

        let res = concurrent_requests.await;
        dbg!(res);

        Ok(())
    }

    async fn check_filter(&self) -> Result<()> {
        // Try an example
        let block = if let Some(block) = self.client.get_block(6542681).await? {
            block
        } else {
            warn!("Latest block not found");
            return Ok(());
        };

        dbg!(block.logs_bloom);
        if let Some(logs_bloom) = block.logs_bloom {
            let address: Address = "0x517bBe8f8ca40B71BB88979b132138894801200a".parse()?;
            let addr = check_bloom_address(logs_bloom, address);
            let event = check_bloom_event(logs_bloom, RelayedFilter::signature());
            let event_dispatch = check_bloom_event(logs_bloom, DispatchedFilter::signature());

            println!("Filter contains info {} {} {}", addr, event, event_dispatch);
        }

        Ok(())
    }
}

fn check_bloom_address(bloom_filter: Bloom, address: Address) -> bool {
    bloom_filter.contains_input(BloomInput::Raw(address.as_bytes()))
}

fn check_bloom_event(bloom_filter: Bloom, signature: H256) -> bool {
    bloom_filter.contains_input(BloomInput::Raw(signature.as_bytes()))
}
