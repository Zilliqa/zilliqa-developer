use std::{marker::PhantomData, time::Duration};

use async_stream::try_stream;
use async_trait::async_trait;

use anyhow::{anyhow, Result};
use ethers::{
    providers::Middleware,
    types::{BlockNumber, Filter, Log, U64},
};
use ethers_contract::{parse_log, EthEvent};
use futures::{Stream, StreamExt, TryStreamExt};
use tokio::time::interval;
use tracing::{debug, info, warn};

use crate::client::ChainClient;

#[async_trait]
pub trait BlockPolling {
    async fn stream_finalized_blocks(&mut self) -> Result<()>;
    async fn get_historic_blocks(&self, from: u64, to: u64) -> Result<()>;

    async fn get_events<D>(
        &self,
        event: Filter,
        from_block: BlockNumber,
        to_block: BlockNumber,
    ) -> Result<Vec<D>>
    where
        D: EthEvent;
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

        let _res = concurrent_requests.await;

        Ok(())
    }

    async fn get_events<D>(
        &self,
        event: Filter,
        from_block: BlockNumber,
        to_block: BlockNumber,
    ) -> Result<Vec<D>>
    where
        D: EthEvent,
    {
        let event = event.from_block(from_block).to_block(to_block);

        let logs: Vec<serde_json::Value> = self
            .client
            .provider()
            .request("eth_getLogs", [event])
            .await?;

        let logs = logs
            .into_iter()
            .map(|log| {
                // Parse log values
                let mut log = log;
                match log["removed"].as_str() {
                    Some("true") => log["removed"] = serde_json::Value::Bool(true),
                    Some("false") => log["removed"] = serde_json::Value::Bool(false),
                    Some(&_) => warn!("invalid parsing"),
                    None => (),
                };
                let log: Log = serde_json::from_value(log)?;
                Ok(log)
            })
            .collect::<Result<Vec<Log>>>()?;

        let events: Vec<D> = logs
            .into_iter()
            .map(|log| Ok(parse_log::<D>(log)?))
            .collect::<Result<Vec<D>>>()?;

        return Ok(events);
    }
}

pub struct EventListener<D: EthEvent> {
    chain_client: ChainClient,
    current_block: U64,
    event: Filter,
    phantom: PhantomData<D>,
}

impl<D: EthEvent> EventListener<D> {
    pub fn new(chain_client: ChainClient, event: Filter) -> Self {
        EventListener {
            current_block: 0.into(),
            chain_client,
            event,
            phantom: PhantomData,
        }
    }

    async fn get_block_number(&self) -> Result<U64> {
        if self.chain_client.block_instant_finality {
            self.chain_client.client.get_block_number().await
        } else {
            self.chain_client
                .client
                .get_block(BlockNumber::Finalized)
                .await
                .map(|block| block.unwrap().number.unwrap())
        }
        .map_err(|_| anyhow!("Unable to get block number"))
    }

    async fn poll_next_events(&mut self) -> Result<Vec<D>>
    where
        D: EthEvent,
    {
        let new_block: U64 = match self.get_block_number().await {
            Err(_) => return Ok(vec![]),
            // Return early if smaller block
            Ok(block) if block <= self.current_block => return Ok(vec![]),
            Ok(block) => block,
        };

        // `eth_getLogs`'s block_number is inclusive, so `current_block` is already retrieved
        let events = self
            .chain_client
            .get_events(
                self.event.clone(),
                (self.current_block + 1).into(),
                new_block.into(),
            )
            .await?;

        debug!(
            "Getting from {} to {}, events gathered {:?}",
            (self.current_block + 1),
            new_block,
            events.len(),
        );
        if events.len() > 0 {
            info!(
                "Getting from {} to {}, events gathered {:?}",
                (self.current_block + 1),
                new_block,
                events.len(),
            )
        }

        self.current_block = new_block;

        Ok(events)
    }

    pub fn listen(mut self) -> impl Stream<Item = Result<Vec<D>>> {
        let stream = try_stream! {
            // TODO: update block interval on config
            let mut interval = interval(Duration::from_secs(3));
            self.current_block = self.chain_client.client.get_block_number().await?;

            loop {
                interval.tick().await;

                let new_events =  self.poll_next_events().await?;
                yield new_events
            }
        };
        Box::pin(stream)
    }
}
