use crate::Event;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{query, types::Decimal, PgPool};

#[async_trait]
pub(crate) trait LedgerInsertable {
    /// parse this event out of the json receipt
    fn from_event(e: Event) -> Result<Self>
    where
        Self: Sized;
    /// insert the relevant items into the ledger
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct LedgerRow {
    pub(crate) txn_id: String,
    pub(crate) block: i64,
    pub(crate) account: String,
    pub(crate) amount: Decimal,
}

impl LedgerRow {
    pub async fn insert_single(&self, client: &PgPool) -> Result<()> {
        let _ = query!(
            "insert into ledger values ($1, $2, $3, $4)",
            self.txn_id,
            self.block,
            self.account.strip_prefix("0x").unwrap(),
            self.amount
        )
        .execute(client)
        .await?;
        Ok(())
    }
    pub async fn insert_multiple(rows: Vec<LedgerRow>, client: &PgPool) -> Result<()> {
        let (ids, blocks, accounts, amounts) = rows.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(mut ids, mut blocks, mut accounts, mut amounts), row| {
                ids.push(row.txn_id);
                blocks.push(row.block);
                accounts.push(row.account.strip_prefix("0x").unwrap().to_string());
                amounts.push(row.amount);
                (ids, blocks, accounts, amounts)
            },
        );
        let _ = query!(
            "
        insert into ledger(id, block, account, amount)
        select * from unnest($1::text[], $2::bigint[], $3::text[], $4::numeric[])
        ",
            &ids[..],
            &blocks[..],
            &accounts[..],
            &amounts[..]
        )
        .execute(client)
        .await?;
        Ok(())
    }
}
