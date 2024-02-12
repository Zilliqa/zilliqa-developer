use crate::meta;
use anyhow::Result;
use async_trait::async_trait;
use pdtdb::{
    meta::MetaTable,
    tracked::Trackable,
    utils::ProcessCoordinates,
    zqproj::{BlockInsertable, Inserter, InsertionErrors, PSQLInsertable},
};
use serde::Serialize;
use sqlx::{query, query_as, PgPool};
use std::ops::Range;

#[async_trait]
pub trait Partitioned {
    // default implementations exist

    /// get the appropriately formattted name for a partition
    fn get_partition_name(table: &str, block: i64, partition_size: i64) -> String {
        format!("{}_{}", table, block - block % partition_size)
    }

    /// create the partition that includes `block` if it doesn't already exist.
    /// this acquires an advisory lock with key `partition_start` to avoid
    /// concurrent table creation errors.
    /// this is a no-op rather than a failure if already created.
    async fn make_partition(
        &self,
        client: &PgPool,
        block: i64,
        partition_size: i64,
        name: &str,
    ) -> Result<()>;

    /// attach the partition that includes `block` if isn't already attached.
    /// this acquires an advisory lock with key `partition_start` to avoid
    /// concurrency errors.
    /// this is a no-op rather than a failure if already attached.
    async fn attach_partition(
        &self,
        client: &PgPool,
        block: i64,
        partition_size: i64,
        name: &str,
    ) -> Result<()>;

    /// checks if a partition exists. doesn't need a lock.
    async fn does_partition_exist(
        &self,
        client: &PgPool,
        table: &str,
        partition_size: i64,
        partition_start: i64,
    ) -> Result<bool> {
        // ideally this would be memoized, but given that we query the meta
        // table for every range anyways it probably wouldn't help performance
        // that much.
        let name = Self::get_partition_name(table, partition_start, partition_size);
        let x = query!("SELECT tablename FROM pg_tables WHERE tablename=$1", name)
            .fetch_optional(client)
            .await?;
        match x {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

#[derive(Clone)]
pub struct TrackedPartitionedTable {
    pub table: String,
    pub meta: meta::Meta,
    pub partition_size: i64,
}

impl TrackedPartitionedTable {
    pub(crate) fn get_partition_name(table: &str, block: i64, partition_size: i64) -> String {
        format!("{}_{}", table, block - block % partition_size)
    }

    pub async fn ensure_schema(client: &PgPool, table: &str) -> Result<()> {
        Ok(meta::Meta::ensure_schema(client, &format!("{table}_meta")).await?)
    }

    pub fn new(
        table: &str,
        coords: &ProcessCoordinates,
        nr_blks: i64,
        partition_size: i64,
    ) -> Result<Self> {
        let meta = meta::Meta::new(&format!("{table}_meta"), coords, nr_blks)?;
        Ok(TrackedPartitionedTable {
            table: table.to_string(),
            meta,
            partition_size,
        })
    }
}

#[async_trait]
impl Partitioned for TrackedPartitionedTable {
    async fn make_partition(
        &self,
        client: &PgPool,
        block: i64,
        partition_size: i64,
        table_name: &str,
    ) -> Result<()> {
        let partition_start = block - block % partition_size;
        let mut db_txn = client.begin().await?;
        // acquire advisory lock to avoid concurrency issues with table creation
        let _ = query(&format!("SELECT pg_advisory_xact_lock({partition_start})"))
            .execute(&mut *db_txn)
            .await?;

        println!(
            "{}: acquired lock {partition_start}",
            self.meta.coords.client_id
        );

        let part_name = Self::get_partition_name(table_name, partition_start, partition_size);
        let x = query!(
            "SELECT tablename FROM pg_tables WHERE tablename=$1",
            part_name
        )
        .fetch_optional(&mut *db_txn)
        .await?;
        let partition_exists = match x {
            Some(_) => true,
            None => false,
        };
        if partition_exists {
            println!(
                "{}: partition {part_name} already exists, skipping and releasing lock",
                self.meta.coords.client_id
            );
            db_txn.commit().await?;
            return Ok(());
        }
        println!("creating partition {part_name}...");
        let _ = query(&format!(
            "CREATE TABLE IF NOT EXISTS {part_name}
        (LIKE {table_name} INCLUDING DEFAULTS INCLUDING CONSTRAINTS)"
        ))
        .execute(&mut *db_txn)
        .await?;
        let _ = query(&format!(
            "ALTER TABLE {part_name} ADD CONSTRAINT within_{partition_start}
            CHECK (block >= {partition_start} AND block < {})",
            partition_start + partition_size
        ))
        .execute(&mut *db_txn)
        .await?;
        db_txn.commit().await?;

        println!(
            "{}: created partition {part_name}... released lock",
            self.meta.coords.client_id
        );

        Ok(())
    }
    async fn attach_partition(
        &self,
        client: &PgPool,
        block: i64,
        partition_size: i64,
        table_name: &str,
    ) -> Result<()> {
        let part_name = Self::get_partition_name(table_name, block, partition_size);
        println!(
            "{}: attaching partition {part_name}",
            self.meta.coords.client_id
        );

        let partition_start = block - block % partition_size;
        let mut db_txn = client.begin().await?;
        // acquire advisory lock to avoid concurrency issues with table creation
        let _ = query(&format!("SELECT pg_advisory_xact_lock({partition_start})"))
            .execute(&mut *db_txn)
            .await?;
        println!(
            "{}: acquired lock {partition_start}",
            self.meta.coords.client_id
        );

        let part_attached = query!(
            "select relispartition from pg_class where relname=$1",
            &part_name
        )
        .fetch_one(&mut *db_txn)
        .await?
        .relispartition;

        if part_attached {
            println!(
                "{}: partition {part_name} already attached. skipping and releasing lock.",
                self.meta.coords.client_id
            );
            db_txn.commit().await?;
            return Ok(());
        }

        println!(
            "{}: attaching partition {part_name}...",
            self.meta.coords.client_id
        );
        let _ = query(&format!(
            "ALTER TABLE {table_name} ATTACH PARTITION {part_name}
            FOR VALUES FROM ({partition_start}) TO ({});",
            partition_start + partition_size
        ))
        .execute(&mut *db_txn)
        .await?;
        let _ = query(&format!(
            "ALTER TABLE {part_name} DROP CONSTRAINT within_{partition_start}"
        ))
        .execute(&mut *db_txn)
        .await?;

        println!(
            "{}: attached partition {part_name}...",
            self.meta.coords.client_id
        );
        db_txn.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Trackable for TrackedPartitionedTable {
    type Client = PgPool;

    async fn get_last_txn_for_blocks(
        &self,
        client: &PgPool,
        blks: &Range<i64>,
    ) -> Result<(i64, i64)> {
        // if both bounds lie in the same partition, all is well

        // try to find the txn in the upper bound's partition first.
        let upper_part_start = blks.end - blks.end % self.partition_size;
        if self
            .does_partition_exist(client, &self.table, self.partition_size, upper_part_start)
            .await?
        {
            let query_str = format!(
                "SELECT block, offset_in_block FROM {} 
                WHERE block >= $1 and block < $2
                ORDER BY block DESC, offset_in_block DESC
                LIMIT 1",
                Self::get_partition_name(&self.table, blks.end, self.partition_size)
            );

            let result: Option<(i64, i64)> = query_as(&query_str)
                .bind(blks.start)
                .bind(blks.end)
                .fetch_optional(client)
                .await?;
            if let Some(pair) = result {
                // if we find it, great
                Ok(pair)
            } else {
                // otherwise query the rest of the table
                let query_str = format!(
                    "SELECT block, offset_in_block FROM {} 
                    WHERE block >= $1 and block < $2
                    ORDER BY block DESC, offset_in_block DESC
                    LIMIT 1",
                    self.table
                );
                let result: Option<(i64, i64)> = query_as(&query_str)
                    .bind(blks.start)
                    .bind(blks.end)
                    .fetch_optional(client)
                    .await?;

                if let Some(pair) = result {
                    Ok(pair)
                } else {
                    Ok((-1, -1))
                }
            }
        } else {
            let query_str = format!(
                "SELECT block, offset_in_block FROM {} 
                WHERE block >= $1 and block < $2
                ORDER BY block DESC, offset_in_block DESC
                LIMIT 1",
                self.table
            );
            let result: Option<(i64, i64)> = query_as(&query_str)
                .bind(blks.start)
                .bind(blks.end)
                .fetch_optional(client)
                .await?;

            if let Some(pair) = result {
                Ok(pair)
            } else {
                Ok((-1, -1))
            }
        }
    }

    async fn is_range_covered_by_entry(
        &self,
        client: &PgPool,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>> {
        self.meta
            .is_range_covered_by_entry(client, start, blks)
            .await
    }

    async fn find_next_range_to_do(
        &self,
        client: &PgPool,
        start_at: i64,
    ) -> Result<Option<Range<i64>>> {
        Ok(self.meta.find_next_range_to_do(client, start_at).await?)
    }
    async fn insert<
        T: Serialize + BlockInsertable + PSQLInsertable + std::marker::Send + std::fmt::Debug,
    >(
        &self,
        client: &PgPool,
        req: Inserter<T>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        // This is a bit horrid. If there is any action at all, we need to check what the highest txn
        // we successfully inserted was.
        let (last_blk, last_txn) = if req.req.is_empty() {
            (-1, -1)
        } else {
            self.get_last_txn_for_blocks(client, blks)
                .await
                .map_err(|err| {
                    InsertionErrors::from_msg(&format!("Cannot find inserted txn ids - {}", err))
                })?
        };

        let part1_start = blks.start - blks.start % self.partition_size;
        let part2_start = blks.end - blks.end % self.partition_size;
        println!(
            "{}: request is in partition {} to {}",
            self.meta.coords.client_id,
            part1_start,
            part1_start + self.partition_size
        );

        // check if the partition exists first. if it does, we don't want to
        // lock up the entire partition.
        if !self
            .does_partition_exist(client, &self.table, self.partition_size, part1_start)
            .await?
        {
            // this fn waits for a lock on the partition.
            self.make_partition(client, part1_start, self.partition_size, &self.table)
                .await?;
        }

        if part2_start != part1_start {
            println!(
                "{}: insertion range spans partition boundary",
                self.meta.coords.client_id
            );

            // range spans partition boundary
            // create second partition if it doesn't already exist
            if !self
                .does_partition_exist(client, &self.table, self.partition_size, part2_start)
                .await?
            {
                self.make_partition(client, part2_start, self.partition_size, &self.table)
                    .await?;
            }
        }

        // split the request across partition boundaries
        let (part1, part2): (Vec<_>, Vec<_>) = req
            .req
            .into_iter()
            .filter(|txn| {
                let (blk, offset) = txn.get_coords();
                blk > last_blk || (blk == last_blk && offset > last_txn)
            })
            .partition(|txn| {
                let (blk, _) = txn.get_coords();
                (blk - blk % self.partition_size) == part1_start
            });
        // moving here is necessary because otherwise rust yells at us that the
        // future isn't thread safe, because we held a value (the inserter?)
        // across an await boundary. moving here is fine because the inserter
        // goes out of scope afterwards anyways.

        async fn commit_request<
            T: Serialize + BlockInsertable + PSQLInsertable + std::marker::Send + std::fmt::Debug,
        >(
            client: &PgPool,
            table: &str,
            request: Vec<T>,
        ) -> Result<(), InsertionErrors> {
            // println!("inserting: {:#?}", request);
            <T as PSQLInsertable>::bulk_insert(request, table, client).await?;
            Ok(())
        }
        if !part1.is_empty() {
            println!(
                "{}: inserting {} rows into first partition...",
                self.meta.coords.client_id,
                part1.len()
            );
            commit_request(
                client,
                &Self::get_partition_name(&self.table, blks.start, self.partition_size),
                part1,
            )
            .await?;
        } else {
            println!(
                "{}: no rows to insert! moving on..",
                self.meta.coords.client_id,
            );
        }
        if !part2.is_empty() {
            println!(
                "{}: inserting {} rows into second partition",
                self.meta.coords.client_id,
                part2.len()
            );
            commit_request(
                client,
                &Self::get_partition_name(&self.table, blks.end, self.partition_size),
                part2,
            )
            .await?;
        }
        println!("{}: committing run {:?}", self.meta.coords.client_id, blks);
        self.meta.commit_run(client, &blks).await?;
        if let Some(_) = self
            .is_range_covered_by_entry(client, part1_start, self.partition_size)
            .await?
        {
            let _ = self
                .attach_partition(client, part1_start, self.partition_size, &self.table)
                .await;
        }
        Ok(())
    }
}
