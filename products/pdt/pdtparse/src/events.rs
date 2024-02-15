use crate::{
    ledger::{LedgerInsertable, LedgerRow},
    Event,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{types::Decimal, PgPool};

#[derive(Debug)]
pub enum ZRC2Event {
    Mint(MintedEvent),
    Burn(BurntEvent),
    Transfer(TransferSuccessEvent),
    TransferFrom(TransferFromSuccess),
    OperatorSend(OperatorSendSuccess),
}

#[derive(Debug)]
pub struct MintedEvent {
    txn_id: String,
    block: i64,
    #[allow(dead_code)]
    minter: String,
    recipient: String,
    amount: Decimal,
}

#[async_trait]
impl LedgerInsertable for MintedEvent {
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()> {
        LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.recipient.clone(),
            amount: self.amount,
        }
        .insert_single(client)
        .await
    }
    fn from_event(e: Event) -> Result<MintedEvent> {
        let minter = e.find_param("minter")?;
        let recipient = e.find_param("recipient")?;
        let amount = Decimal::from_str_exact(&e.find_param("amount")?)?;
        Ok(MintedEvent {
            txn_id: e.txn_id,
            block: e.block,
            amount,
            minter,
            recipient,
        })
    }
}
#[derive(Debug)]
pub struct BurntEvent {
    txn_id: String,
    block: i64,
    #[allow(dead_code)]
    burner: String,
    burn_account: String,
    amount: Decimal,
}
#[async_trait]
impl LedgerInsertable for BurntEvent {
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()> {
        LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.burn_account.clone(),
            amount: -self.amount,
        }
        .insert_single(client)
        .await
    }
    fn from_event(e: Event) -> Result<BurntEvent> {
        let burner = e.find_param("burner")?;
        let burn_account = e.find_param("burn_account")?;
        let amount = Decimal::from_str_exact(&e.find_param("amount")?)?;
        Ok(BurntEvent {
            txn_id: e.txn_id,
            block: e.block,
            amount,
            burner,
            burn_account,
        })
    }
}
#[derive(Debug)]
pub struct TransferSuccessEvent {
    txn_id: String,
    block: i64,
    sender: String,
    recipient: String,
    amount: Decimal,
}
#[async_trait]
impl LedgerInsertable for TransferSuccessEvent {
    fn from_event(e: Event) -> Result<Self> {
        let sender = e.find_param("sender")?;
        let recipient = e.find_param("recipient")?;
        let amount = Decimal::from_str_exact(&e.find_param("amount")?)?;
        Ok(TransferSuccessEvent {
            txn_id: e.txn_id,
            block: e.block,
            sender,
            recipient,
            amount,
        })
    }
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()> {
        let reduce_sender = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.sender.clone(),
            amount: -self.amount,
        };
        let increase_recipient = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.recipient.clone(),
            amount: self.amount,
        };
        LedgerRow::insert_multiple(vec![reduce_sender, increase_recipient], client).await
    }
}
#[derive(Debug)]
pub struct TransferFromSuccess {
    txn_id: String,
    block: i64,
    #[allow(dead_code)]
    initiator: String,
    sender: String,
    recipient: String,
    amount: Decimal,
}
#[async_trait]
impl LedgerInsertable for TransferFromSuccess {
    fn from_event(e: Event) -> Result<Self> {
        let sender = e.find_param("sender")?;
        let recipient = e.find_param("recipient")?;
        let initiator = e.find_param("initiator")?;
        let amount = Decimal::from_str_exact(&e.find_param("amount")?)?;
        Ok(TransferFromSuccess {
            txn_id: e.txn_id,
            block: e.block,
            initiator,
            sender,
            recipient,
            amount,
        })
    }
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()> {
        let reduce_sender = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.sender.clone(),
            amount: -self.amount,
        };
        let increase_recipient = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.recipient.clone(),
            amount: self.amount,
        };
        LedgerRow::insert_multiple(vec![reduce_sender, increase_recipient], client).await
    }
}
#[derive(Debug)]
pub struct OperatorSendSuccess {
    txn_id: String,
    block: i64,
    #[allow(dead_code)]
    initiator: String,
    sender: String,
    recipient: String,
    amount: Decimal,
}
#[async_trait]
impl LedgerInsertable for OperatorSendSuccess {
    fn from_event(e: Event) -> Result<Self> {
        let sender = e.find_param("sender")?;
        let recipient = e.find_param("recipient")?;
        let initiator = e.find_param("initiator")?;
        let amount = Decimal::from_str_exact(&e.find_param("amount")?)?;
        Ok(OperatorSendSuccess {
            txn_id: e.txn_id,
            block: e.block,
            initiator,
            sender,
            recipient,
            amount,
        })
    }
    async fn insert_into_ledger(&self, client: &PgPool) -> Result<()> {
        let reduce_sender = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.sender.clone(),
            amount: -self.amount,
        };
        let increase_recipient = LedgerRow {
            txn_id: self.txn_id.clone(),
            block: self.block,
            account: self.recipient.clone(),
            amount: self.amount,
        };
        LedgerRow::insert_multiple(vec![reduce_sender, increase_recipient], client).await
    }
}

pub async fn insert_into_ledger(event: &ZRC2Event, client: &PgPool) -> Result<()> {
    match event {
        ZRC2Event::Mint(e) => e.insert_into_ledger(client).await,
        ZRC2Event::Burn(e) => e.insert_into_ledger(client).await,
        ZRC2Event::Transfer(e) => e.insert_into_ledger(client).await,
        ZRC2Event::TransferFrom(e) => e.insert_into_ledger(client).await,
        ZRC2Event::OperatorSend(e) => e.insert_into_ledger(client).await,
    }
}
