use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::{fmt, marker::PhantomData, ops::Range};

use crate::values;

pub const TRANSACTION_TABLE_NAME: &str = "transactions";
pub const MICROBLOCKS_TABLE_NAME: &str = "microblocks";

pub struct Inserter<T: Serialize> {
    pub _marker: PhantomData<T>,
    pub req: Vec<T>,
}

impl<T: Serialize> Inserter<T> {
    pub fn insert_row(&mut self, row: T) -> Result<()> {
        self.req.push(row);
        Ok(())
    }
}

pub trait BlockInsertable {
    /// Return (block, offset_in_block)
    fn get_coords(&self) -> (i64, i64);

    /// return an number >= the number of bytes used by this object so that
    /// we can make sure our bigquery request isn't too big.
    fn estimate_bytes(&self) -> Result<usize>;
}

#[async_trait]
pub trait PSQLInsertable {
    async fn bulk_insert(req: Vec<Self>, table_name: &str, client: &sqlx::PgPool) -> Result<()>
    where
        Self: Sized;
}

impl BlockInsertable for values::BQTransaction {
    fn get_coords(&self) -> (i64, i64) {
        (self.block, self.offset_in_block)
    }

    /// Guess how many bytes this txn will take when encoded
    /// If we wanted to be more accurate, we could serialise and measure,
    /// but that would be quite expensive.
    fn estimate_bytes(&self) -> Result<usize> {
        // Annoyingly, because of Javascript escaping, this is the only way :-(
        Ok(self.to_json()?.len())
    }
}
impl BlockInsertable for values::PSQLTransaction {
    fn get_coords(&self) -> (i64, i64) {
        (self.block, self.offset_in_block)
    }

    /// Guess how many bytes this txn will take when encoded
    /// If we wanted to be more accurate, we could serialise and measure,
    /// but that would be quite expensive.
    fn estimate_bytes(&self) -> Result<usize> {
        // Annoyingly, because of Javascript escaping, this is the only way :-(
        Ok(self.to_json()?.len())
    }
}

impl BlockInsertable for values::BQMicroblock {
    fn get_coords(&self) -> (i64, i64) {
        (self.block, 0)
    }

    fn estimate_bytes(&self) -> Result<usize> {
        Ok(self.to_json()?.len())
    }
}
impl BlockInsertable for values::PSQLMicroblock {
    fn get_coords(&self) -> (i64, i64) {
        (self.block, 0)
    }

    fn estimate_bytes(&self) -> Result<usize> {
        Ok(self.to_json()?.len())
    }
}

pub struct InsertionErrors {
    pub errors: Vec<String>,
    pub msg: String,
}

impl fmt::Display for InsertionErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n {:?}", self.msg, self.errors)
    }
}

impl InsertionErrors {
    pub fn from_msg(msg: &str) -> Self {
        InsertionErrors {
            errors: Vec::new(),
            msg: msg.to_string(),
        }
    }
}

impl From<anyhow::Error> for InsertionErrors {
    fn from(value: anyhow::Error) -> Self {
        Self::from_msg(&value.to_string())
    }
}

#[async_trait]
pub trait ZilliqaDBProject {
    /// make an internal buffer to hold rows to be inserted
    async fn make_inserter<T: Serialize + BlockInsertable + std::marker::Send>(
        &self,
    ) -> Result<Inserter<T>> {
        Ok(Inserter {
            _marker: PhantomData,
            req: Vec::new(),
        })
    }

    /// insert the buffer of txns into the database, acts on an inserter
    async fn insert_transactions(
        &self,
        req: Inserter<impl Into<values::BQTransaction> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors>;

    /// insert the buffer of microblocks into the database, acts on an inserter
    async fn insert_microblocks(
        &self,
        req: Inserter<impl Into<values::BQMicroblock> + Serialize + std::marker::Send>,
        blks: &Range<i64>,
    ) -> Result<(), InsertionErrors>;

    /// If a single entry in the meta table contains start  .. start+blks , then return the client id
    /// that generated it, else return None.
    /// Returns a pair (nr_blks, client_id)
    async fn is_txn_range_covered_by_entry(
        &self,
        start: i64,
        blks: i64,
    ) -> Result<Option<(i64, String)>>;

    async fn get_txn_range(&self, start_at: i64) -> Result<Option<Range<i64>>>;
}
