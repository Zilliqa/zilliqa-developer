use serde::Serialize;
use std::ops::Range;

use anyhow::Result;
use async_trait::async_trait;

use crate::zqproj::{BlockInsertable, Inserter, InsertionErrors, PSQLInsertable};

#[async_trait]
pub trait Trackable {
    type Client;

    /// Retrieve the last (blk,txnid) pair for the blocks in the range, so we can avoid inserting duplicates.
    /// Since these blocks are assigned to only one thread at a time, we know another thread can't try to insert
    /// them concurrently - but we might have crashed half-way through a block of insert requests earlier.
    async fn get_last_txn_for_blocks(
        &self,
        client: &Self::Client,
        blks: &Range<i64>,
    ) -> Result<(i64, i64)>;

    async fn insert<
        T: Serialize + BlockInsertable + PSQLInsertable + std::marker::Send + std::fmt::Debug,
    >(
        &self,
        client: &Self::Client,
        req: Inserter<T>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors>;

    async fn is_range_covered_by_entry(
        &self,
        client: &Self::Client,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>>;

    async fn find_next_range_to_do(
        &self,
        client: &Self::Client,
        start_at: i64,
    ) -> Result<Option<Range<i64>>>;
}
