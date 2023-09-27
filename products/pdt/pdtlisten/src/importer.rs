use anyhow::{bail, Result};
use pdtdb::{
    values::{BQMicroblock, BQTransaction},
    zqproj::{Inserter, ZilliqaDBProject},
};
use std::{self, ops::Range};

pub(crate) struct Buffers {
    pub(crate) txn_inserter: Inserter<BQTransaction>,
    pub(crate) mb_inserter: Inserter<BQMicroblock>,
}

pub(crate) struct BatchedImporter {
    pub(crate) buffers: Option<Buffers>,
    pub(crate) range: Option<Range<i64>>,
}

impl BatchedImporter {
    pub fn new() -> Self {
        Self {
            buffers: None,
            range: None,
        }
    }
    pub(crate) async fn reset_buffer<P: ZilliqaDBProject + std::marker::Sync>(
        &mut self,
        project: &P,
    ) -> Result<()> {
        self.buffers = Some(Buffers {
            txn_inserter: project.make_inserter::<BQTransaction>().await?,
            mb_inserter: project.make_inserter::<BQMicroblock>().await?,
        });
        self.range = None;
        Ok(())
    }

    pub(crate) fn take_buffers(&mut self) -> Result<Buffers> {
        if self.buffers.is_none() {
            bail!("attempted to take buffers when buffers were not set.")
        }
        Ok(self.buffers.take().unwrap())
    }

    pub(crate) fn insert_into_buffer(
        &mut self,
        block: BQMicroblock,
        txns: Vec<BQTransaction>,
    ) -> Result<()> {
        match self.buffers.as_mut() {
            None => bail!("buffers have not been reset!"),
            Some(buffers) => {
                if let Some(range) = self.range.as_ref() {
                    if block.block < range.start {
                        self.range = Some(block.block..range.end)
                    } else if block.block > range.end {
                        self.range = Some(range.start..block.block)
                    }
                } else {
                    self.range = Some(block.block..(block.block + 1))
                }
                buffers.mb_inserter.insert_row(block)?;
                for txn in txns {
                    buffers.txn_inserter.insert_row(txn)?;
                }
                Ok(())
            }
        }
    }

    pub(crate) fn n_blocks(&self) -> usize {
        match self.buffers.as_ref() {
            None => 0,
            Some(buffers) => buffers.mb_inserter.req.len(),
        }
    }
}
