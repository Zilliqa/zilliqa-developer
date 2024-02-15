use anyhow::{bail, Result};
use pdtdb::{
    values::{BQMicroblock, BQTransaction},
    zqproj::{Inserter, ZilliqaDBProject},
};
use std::{
    self,
    cmp::{max, min},
    ops::Range,
};

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
                let block_range = block.block..block.block + 1;

                self.range = self
                    .range
                    .as_ref()
                    .map_or(Some(block_range.clone()), |range| {
                        let range_start = min(range.start, block_range.start);
                        let range_end = max(range.end, block_range.end);
                        Some(range_start..range_end)
                    });

                buffers.mb_inserter.insert_row(block)?;
                for txn in txns {
                    buffers.txn_inserter.insert_row(txn)?;
                }
                Ok(())
            }
        }
    }

    pub(crate) fn n_blocks(&self) -> usize {
        self.buffers
            .as_ref()
            .map(|buffers| buffers.mb_inserter.req.len())
            .unwrap_or_default()
    }
}
