use anyhow::{anyhow, Result};
use base64::Engine;
use hex;
use pdtbq::utils;
use pdtlib::proto::ProtoTransactionWithReceipt;
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub block: i64,
}

impl Block {}
