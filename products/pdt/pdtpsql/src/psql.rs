use crate::tracked::TrackedPartitionedTable;
use crate::utils::find_table;
use anyhow::Result;
use async_trait::async_trait;
use pdtdb::{
    tracked::Trackable,
    utils::ProcessCoordinates,
    values,
    zqproj::{
        Inserter, InsertionErrors, ZilliqaDBProject, MICROBLOCKS_TABLE_NAME, TRANSACTION_TABLE_NAME,
    },
};
use serde::Serialize;
use sqlx::{query_file, PgPool};
use std::{marker::PhantomData, ops::Range};

#[derive(Clone)]
pub struct ZilliqaPSQLProject {
    pub url: String,
    pub client: PgPool,
    pub transactions: TrackedPartitionedTable,
    pub microblocks: TrackedPartitionedTable,
    pub client_id: String,
}

impl ZilliqaPSQLProject {
    pub async fn ensure_schema(client: &PgPool) -> Result<()> {
        if let Some(schema) = find_table(client, "transactions").await? {
            println!("table tranasactions found, schema: {:#?}", schema);
        } else {
            println!("creating transactions table..");
            Self::create_transaction_table(client).await?;
        }
        TrackedPartitionedTable::ensure_schema(client, TRANSACTION_TABLE_NAME).await?;

        if let Some(schema) = find_table(client, "microblocks").await? {
            println!("table microblocks exists, schema: {:#?}", schema);
        } else {
            println!("creating microblocks table..");
            Self::create_microblocks_table(client).await?;
        }

        TrackedPartitionedTable::ensure_schema(client, MICROBLOCKS_TABLE_NAME).await?;
        Ok(())
    }
    pub(crate) async fn create_transaction_table(client: &PgPool) -> Result<()> {
        query_file!("queries/make_transactions.sql")
            .execute(client)
            .await?;
        Ok(())
    }
    pub(crate) async fn create_microblocks_table(client: &PgPool) -> Result<()> {
        query_file!("queries/make_microblocks.sql")
            .execute(client)
            .await?;
        Ok(())
    }
    pub fn new(
        url: &str,
        client: PgPool,
        coords: &ProcessCoordinates,
        nr_blks: i64,
        partition_size: i64,
    ) -> Result<ZilliqaPSQLProject> {
        let txns = TrackedPartitionedTable::new(
            &TRANSACTION_TABLE_NAME.to_string(),
            coords,
            nr_blks,
            partition_size,
        )?;

        let mblocks = TrackedPartitionedTable::new(
            &MICROBLOCKS_TABLE_NAME.to_string(),
            coords,
            nr_blks,
            partition_size,
        )?;

        Ok(ZilliqaPSQLProject {
            url: url.to_string(),
            transactions: txns,
            microblocks: mblocks,
            client_id: coords.client_id.to_string(),
            client,
        })
    }
}

#[async_trait]
impl ZilliqaDBProject for ZilliqaPSQLProject {
    async fn insert_transactions(
        &self,
        req: Inserter<impl Into<values::BQTransaction> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        let mapped_inserter = Inserter::<values::PSQLTransaction> {
            req: req
                .req
                .into_iter()
                .map(|x| Into::<values::BQTransaction>::into(x))
                .map(|x| Into::<values::PSQLTransaction>::into(x))
                .collect(),
            _marker: PhantomData,
        };
        core::result::Result::Ok(
            self.transactions
                .insert(&self.client, mapped_inserter, blks)
                .await?,
        )
    }
    async fn insert_microblocks(
        &self,
        req: Inserter<impl Into<values::BQMicroblock> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        let mapped_inserter = Inserter::<values::PSQLMicroblock> {
            req: req
                .req
                .into_iter()
                .map(|x| Into::<values::BQMicroblock>::into(x))
                .map(|x| Into::<values::PSQLMicroblock>::into(x))
                .collect(),
            _marker: PhantomData,
        };
        core::result::Result::Ok(
            self.microblocks
                .insert(&self.client, mapped_inserter, blks)
                .await?,
        )
    }

    async fn get_txn_range(&self, start_at: i64) -> Result<Option<Range<i64>>> {
        Ok(self
            .transactions
            .find_next_range_to_do(&self.client, start_at)
            .await?)
    }

    async fn is_txn_range_covered_by_entry(
        &self,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>> {
        self.transactions
            .is_range_covered_by_entry(&self.client, start, blks)
            .await
    }
}
