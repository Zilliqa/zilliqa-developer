use crate::utils::{find_table, pg_to_std_range};
use anyhow::Result;
use async_trait::async_trait;
use pdtdb::{meta::MetaTable, utils::ProcessCoordinates};
use sqlx::{postgres::types::PgRange, query, PgPool, Row};
use std::ops::Range;

#[derive(Clone)]
pub struct Meta {
    pub(crate) table: String,
    pub coords: ProcessCoordinates,

    ///max block in persistence
    pub(crate) nr_blks: i64,
}

impl Meta {
    pub fn new(table: &String, coords: &ProcessCoordinates, nr_blks: i64) -> Result<Self> {
        Ok(Meta {
            table: table.clone(),
            coords: coords.clone(),
            nr_blks,
        })
    }

    pub async fn create_table(client: &PgPool, table: &str) -> Result<()> {
        let _ = query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (imported_ranges int8multirange NOT NULL)",
            table
        ))
        .execute(client)
        .await?;
        let _ = query(&format!(
            "INSERT INTO {} VALUES (multirange(int8range(1, 1)))",
            table
        ))
        .execute(client)
        .await?;
        Ok(())
    }

    pub async fn ensure_schema(client: &PgPool, table: &str) -> Result<()> {
        if let Some(schema) = find_table(client, table).await? {
            println!("table {} found, schema {:#?}", table, schema);
        } else {
            Self::create_table(client, table).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl MetaTable for Meta {
    type Client = PgPool;

    async fn is_range_covered_by_entry(
        &self,
        client: &PgPool,
        start: i64,
        nr_blks: i64,
    ) -> Result<Option<(i64, String)>> {
        let range = Range {
            start,
            end: start + nr_blks,
        };
        let pgrange = PgRange::from(range.clone());
        let query_str = format!(
            "SELECT * FROM {}
            WHERE imported_ranges @> $1
            LIMIT 1",
            self.table
        );
        let result = query(&query_str)
            .bind(pgrange)
            .fetch_optional(client)
            .await?;

        if let Some(_) = result {
            Ok(Some((nr_blks, "".to_string())))
        } else {
            Ok(None)
        }
    }

    fn get_nr_blocks(&self) -> i64 {
        self.nr_blks
    }

    async fn commit_run(&self, client: &PgPool, range: &Range<i64>) -> Result<()> {
        let pgrange = PgRange::from(range.clone());
        let _ = query(&format!(
            "UPDATE {} SET imported_ranges = imported_ranges + int8multirange($1)",
            self.table
        ))
        .bind(pgrange)
        .execute(client)
        .await?;
        Ok(())
    }

    async fn find_next_gap_above(
        &self,
        client: &PgPool,
        blk_to_find: i64,
    ) -> Result<Option<Range<i64>>> {
        let query_str = format!(
            "SELECT * FROM
            (SELECT unnest(imported_ranges) as range FROM {}) as r
            WHERE lower(range) > $1
            ORDER BY lower(range) ASC LIMIT 1",
            self.table
        );
        let result = query(&query_str)
            .bind(blk_to_find)
            .fetch_optional(client)
            .await?;

        if let Some(row) = result {
            let pgrange: PgRange<i64> = row.try_get("range")?;
            Ok(Some(pg_to_std_range(pgrange)))
        } else {
            Ok(None)
        }
    }

    async fn find_next_free_range(
        &self,
        client: &PgPool,
        start_at: i64,
        max_blk: i64,
    ) -> Result<Range<i64>> {
        println!(
            "{}, find_next_free_range() start at {} max {}",
            self.coords.client_id, start_at, max_blk
        );

        match self.find_next_gap_above(client, start_at).await? {
            None => {
                // no gaps between where we are and where we've imported up to
                let q_result = query(&format!(
                    "SELECT upper(imported_ranges) as upper_bound from {} WHERE not isempty(imported_ranges)",
                    self.table
                ))
                .fetch_optional(client)
                .await?;

                let result = match q_result {
                    Some(row) => Range {
                        start: row.try_get("upper_bound")?,
                        end: max_blk + 1,
                    },
                    None => Range {
                        start: start_at,
                        end: max_blk + 1,
                    },
                };
                println!(
                    "{}: no gap above, next free range is {:?}",
                    self.coords.client_id, result
                );
                Ok(result)
            }
            Some(next_done_blocks) => {
                let q_result = query(&format!(
                    "SELECT upper(range) as upper_bound FROM
                    (SELECT unnest(imported_ranges) as range FROM {}) as r
                    WHERE upper(range) < $1
                    ORDER BY upper(range) DESC LIMIT 1",
                    self.table
                ))
                .bind(next_done_blocks.start)
                .fetch_optional(client)
                .await?;

                let result = match q_result {
                    Some(row) => Range {
                        start: row.try_get("upper_bound")?,
                        end: next_done_blocks.start,
                    },
                    None => Range {
                        start: start_at,
                        end: max_blk + 1,
                    },
                };
                println!(
                    "{}: hole at the start, next free range is {:?}",
                    self.coords.client_id, result
                );
                return Ok(result);
            }
        }
    }
    async fn find_next_range_to_do(
        &self,
        client: &PgPool,
        start_at_in: i64,
    ) -> Result<Option<Range<i64>>> {
        //copied from bq implementation
        let mut start_at: i64 = start_at_in;
        loop {
            let next_range = self
                .find_next_free_range(client, start_at, self.nr_blks)
                .await?;

            println!(
                "{}: next_range {:?} start_at {} max_blk {}",
                self.coords.client_id, next_range, start_at, self.nr_blks
            );
            // The next range starts above the max_blk, so we don't really care.
            if next_range.start >= self.nr_blks {
                return Ok(None);
            }

            // OK. Does this range overlap one of my batches? The batch starts at (next_range.start/batch_blk*nr_machines)
            // Our next batch is at start + nr * batch_size.

            // skip_blocks is the gap between one of our batches and the next one.
            let skip_blocks = self.coords.batch_blks * self.coords.nr_machines;
            let batch_start = next_range.start - next_range.start % (skip_blocks);
            let mut our_next_batch_start =
                batch_start + (self.coords.machine_id * self.coords.batch_blks);
            let mut our_next_batch_end = our_next_batch_start + self.coords.batch_blks;
            println!(
                "{}: batch_start {} our_next_batch_start {} our_next_batch_end {}",
                self.coords.client_id, batch_start, our_next_batch_start, our_next_batch_end
            );
            // Make sure we address a block range above the start of the range of unprocessed blocks
            // we've found. Compare the end of the batch, because otherwise if we end up in the middle
            // of one of our batches, we'll never process the end of it.
            while our_next_batch_end <= next_range.start {
                our_next_batch_start += skip_blocks;
                our_next_batch_end += skip_blocks;
            }

            // Does it overlap?
            println!(
                "{}: ours {} .. {}",
                self.coords.client_id, our_next_batch_start, our_next_batch_end
            );
            let start_range = std::cmp::max(next_range.start, our_next_batch_start);
            let end_range = std::cmp::min(next_range.end, our_next_batch_end);
            if start_range < next_range.end && end_range > next_range.start {
                let result = Range {
                    start: start_range,
                    end: end_range,
                };
                println!("{}: OK. Fetching {:?}", self.coords.client_id, result);
                return Ok(Some(result));
            }
            start_at = next_range.end;
        }
    }
}
