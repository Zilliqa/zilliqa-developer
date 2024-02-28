use crate::bq;
use crate::bq_utils;
use crate::meta::Meta;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gcp_bigquery_client::model::{
    query_request::QueryRequest, table::Table,
    table_data_insert_all_request::TableDataInsertAllRequest,
};
use gcp_bigquery_client::Client;
use pdtdb::meta::MetaTable;
use pdtdb::tracked::Trackable;
use pdtdb::utils::ProcessCoordinates;
use pdtdb::zqproj::{BlockInsertable, Inserter, InsertionErrors};
use serde::Serialize;
use std::ops::Range;

#[derive(Clone)]
pub struct TrackedTable {
    pub location: bq_utils::BigQueryTableLocation,
    pub table: Table,
    pub meta: Meta,
}

impl TrackedTable {
    pub async fn ensure_schema(
        client: &Client,
        location: &bq_utils::BigQueryTableLocation,
    ) -> Result<()> {
        Ok(Meta::ensure_schema(client, &location.to_meta()).await?)
    }

    pub fn new(
        location: &bq_utils::BigQueryTableLocation,
        coords: &ProcessCoordinates,
        table: Table,
        nr_blks: i64,
    ) -> Result<Self> {
        let meta = Meta::new(&location.to_meta(), coords, nr_blks)?;
        Ok(TrackedTable {
            location: location.clone(),
            table,
            meta,
        })
    }

    pub async fn get_max_meta_block(&self, client: &Client) -> Result<Option<i64>> {
        return self.meta.find_max_block(client).await;
    }
}

#[async_trait]
impl Trackable for TrackedTable {
    type Client = Client;

    async fn get_last_txn_for_blocks(
        &self,
        client: &Client,
        blks: &Range<i64>,
    ) -> Result<(i64, i64)> {
        let mut result = client.job()
            .query(&self.location.dataset.project_id,
                   QueryRequest::new(format!("SELECT block,offset_in_block FROM `{}` WHERE block >= {} AND block < {} ORDER BY block DESC, offset_in_block DESC LIMIT 1",
                                             self.location.get_table_desc(), blks.start, blks.end))).await?;
        if result.next_row() {
            // There was one!1
            let blk = result.get_i64(0)?.ok_or(anyhow!("Cannot decode blk"))?;
            let offset = result.get_i64(1)?.ok_or(anyhow!("Cannot decode offset"))?;
            Ok((blk, offset))
        } else {
            Ok((-1, -1))
        }
    }

    async fn insert<T: Serialize + BlockInsertable + std::marker::Send>(
        &self,
        client: &Client,
        req: Inserter<T>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors> {
        let _txn_table_name = self.location.get_table_desc();

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

        async fn commit_request(
            client: &Client,
            loc: &bq_utils::BigQueryTableLocation,
            req: TableDataInsertAllRequest,
        ) -> Result<(), InsertionErrors> {
            let mut err_rows = Vec::<String>::new();
            let resp = client
                .tabledata()
                .insert_all(
                    &loc.dataset.project_id,
                    &loc.dataset.dataset_id,
                    &loc.table_id,
                    req,
                )
                .await
                .or_else(|e| Err(InsertionErrors::from_msg(&format!("Cannot insert - {}", e))))?;

            if let Some(row_errors) = resp.insert_errors {
                err_rows.extend(
                    row_errors
                        .into_iter()
                        .map(|e| format!("{:?}: {:?}", e.index, e.errors)),
                );
            }

            if !err_rows.is_empty() {
                return Err(InsertionErrors {
                    errors: err_rows,
                    msg: "Insertion failed".to_string(),
                });
            }
            Ok(())
        }
        let mut current_request = TableDataInsertAllRequest::new();
        let mut current_request_bytes: usize = 0;

        for txn in req.req {
            let (txn_block, txn_offset_in_block) = txn.get_coords();
            // Check if txn already seen
            if txn_block > last_blk || (txn_block == last_blk && txn_offset_in_block > last_txn) {
                let nr_bytes = txn.estimate_bytes().map_err(|x| {
                    InsertionErrors::from_msg(&format!("Cannot get size of transaction - {}", x))
                })?;

                // Check exceeded batch limit
                if current_request_bytes + nr_bytes >= bq::MAX_QUERY_BYTES
                    || current_request.len() >= bq::MAX_QUERY_ROWS - 1
                {
                    println!(
                        "{}: Inserting {} rows with {} bytes ending at {}/{}",
                        self.meta.coords.client_id,
                        current_request.len(),
                        current_request_bytes,
                        txn_block,
                        txn_offset_in_block
                    );

                    commit_request(client, &self.location, current_request).await?;
                    // Reset request parameters
                    current_request = TableDataInsertAllRequest::new();
                    current_request_bytes = 0;
                }

                current_request.add_row(None, &txn).map_err(|err| {
                    InsertionErrors::from_msg(&format!("Cannot add row to request - {}", err))
                })?;
                current_request_bytes += nr_bytes;
            }
        }

        if !current_request.is_empty() {
            println!(
                "{}: [F] Inserting {} rows at end of block",
                self.meta.coords.client_id,
                current_request.len(),
            );
            commit_request(&client, &self.location, current_request).await?;
        }

        // Mark that these blocks were done.
        self.meta.commit_run(&client, &blks).await.map_err(|err| {
            InsertionErrors::from_msg(&format!("Could not commit run result - {:?}", err))
        })?;

        Ok(())
    }

    async fn is_range_covered_by_entry(
        &self,
        client: &Client,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>> {
        self.meta
            .is_range_covered_by_entry(client, start, blks)
            .await
    }

    async fn find_next_range_to_do(
        &self,
        client: &Client,
        start_at: i64,
    ) -> Result<Option<Range<i64>>> {
        Ok(self.meta.find_next_range_to_do(client, start_at).await?)
    }
}
