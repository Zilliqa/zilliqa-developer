use std::ops::Range;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MetaTable {
    type Client;

    /// decide if the range start..nr_blks is covered by the metadata table already.
    async fn is_range_covered_by_entry(
        &self,
        client: &Self::Client,
        start: i64,
        nr_blks: i64,
    ) -> Result<Option<(i64, String)>>;

    /// Find the next range of blocks for this client to perform, starting at start_at_in.
    async fn find_next_range_to_do(
        &self,
        client: &Self::Client,
        start_at_in: i64,
    ) -> Result<Option<Range<i64>>>;

    /// Find the next set of done blocks above this one and return the range between them
    async fn find_next_gap_above(
        &self,
        client: &Self::Client,
        blk_to_find: i64,
    ) -> Result<Option<Range<i64>>>;

    async fn commit_run(&self, client: &Self::Client, range: &Range<i64>) -> Result<()>;

    async fn find_next_free_range(
        &self,
        client: &Self::Client,
        start_at: i64,
        max_blk: i64,
    ) -> Result<Range<i64>>;

    fn get_nr_blocks(&self) -> i64;
}
