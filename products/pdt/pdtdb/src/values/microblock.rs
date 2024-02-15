use std::ops::Mul;

use crate::utils::{self, decode_u8, encode_u8};
use anyhow::{anyhow, Result};
use ethers::types::{Block, Transaction, Withdrawal, U256};
// use hex;
use crate::zqproj::PSQLInsertable;
use pdtlib::proto::ProtoMicroBlock;
use psql_derive::PSQLInsertable;

//use pdtlib::proto::ProtoTransactionWithReceipt;
// use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Serialize, Deserialize, Clone, FromRow, PSQLInsertable, Debug, PartialEq)]
pub struct BQMicroblock {
    #[psql_type = "BIGINT"]
    pub block: i64,
    #[psql_type = "BIGINT"]
    pub offset_in_block: i64,
    // The shard id from the index.
    #[psql_type = "BIGINT"]
    pub shard_id: Option<i64>,
    #[psql_type = "BIGINT"]
    pub header_version: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub header_committee_hash: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub header_prev_hash: Option<String>,
    #[psql_type = "BIGINT"]
    pub gas_limit: i64,
    // Rewards.
    #[sqlx(default)]
    #[psql_type = "numeric(76, 38)"]
    pub rewards: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub prev_hash: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub tx_root_hash: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub miner_pubkey: Option<String>,
    #[sqlx(default)]
    #[psql_type = "varchar(40)"]
    pub miner_addr_zil: Option<String>,
    #[sqlx(default)]
    #[psql_type = "varchar(40)"]
    pub miner_addr_eth: Option<String>,
    #[psql_type = "BIGINT"]
    pub ds_block_num: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub state_delta_hash: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub tran_receipt_hash: Option<String>,
    #[psql_type = "BIGINT"]
    pub block_shard_id: i64,
    #[psql_type = "BIGINT"]
    pub gas_used: i64,
    #[psql_type = "BIGINT"]
    pub epoch_num: i64,
    #[psql_type = "BIGINT"]
    pub num_txs: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub blockhash: Option<String>,
    #[psql_type = "BIGINT"]
    pub timestamp: i64,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub cs1: Option<String>,
    // represented as a string of '0' or '1' for easier querying.
    #[sqlx(default)]
    #[psql_type = "varchar(10)"]
    pub b1: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub cs2: Option<String>,
    // represented as a string of '0' or '1' for easier querying.
    #[sqlx(default)]
    #[psql_type = "varchar(10)"]
    pub b2: Option<String>,
    #[psql_type = "text"]
    pub imported_from: String,
    #[psql_type = "bytea"]
    pub eth_parent_hash: Option<String>,
    #[psql_type = "bytea"]
    pub eth_uncles_hash: Option<String>,
    #[psql_type = "bytea"]
    pub eth_state_root: Option<String>, //root of the state tree
    #[psql_type = "bytea"]
    pub eth_extra_data: Option<String>,
    #[psql_type = "bytea"]
    pub eth_logs_bloom: Option<String>, //bloom hash (https://en.wikipedia.org/wiki/Bloom_filter) of logs
    #[psql_type = "BIGINT"]
    pub eth_difficulty: Option<i64>,
    #[psql_type = "BIGINT"]
    pub eth_total_difficulty: Option<i64>,
    #[psql_type = "bytea"]
    pub eth_nonce: Option<String>,
    #[psql_type = "BIGINT"]
    pub eth_base_fee_per_gas: Option<i64>,
    #[psql_type = "bytea"]
    pub eth_withdrawals_root: Option<String>, //usually None, but keeping them around anyway
    #[psql_type = "jsonb"]
    pub eth_withdrawals: Option<String>,
}

impl BQMicroblock {
    pub fn from_proto(in_val: &ProtoMicroBlock, blk: i64, shard_id: i64) -> Result<Self> {
        let val = in_val.clone();
        let header = val
            .header
            .as_ref()
            .ok_or(anyhow!("Couldn't find block header"))?;
        let header_base = header
            .blockheaderbase
            .as_ref()
            .ok_or(anyhow!("Block header base"))?;
        let block_base = val.blockbase.as_ref().ok_or(anyhow!("Block base"))?;
        let cosigs = block_base.cosigs.as_ref().ok_or(anyhow!("No cosigs"))?;

        let header_committee_hash = Some(encode_u8(header_base.committeehash.as_slice()));
        let header_prev_hash = Some(encode_u8(header_base.prevhash.as_slice()));
        let rewards = header
            .rewards
            .clone()
            .and_then(|x| utils::u128_string_from_storage(&x));
        let prev_hash = Some(encode_u8(header.prevhash.as_slice()));
        let tx_root_hash = Some(encode_u8(header.txroothash.as_slice()));
        let miner_pubkey = header
            .minerpubkey
            .clone()
            .and_then(|x| Some(encode_u8(x.data.as_slice())));
        let miner_addr_zil = header
            .minerpubkey
            .as_ref()
            .and_then(|x| utils::maybe_hex_address_from_public_key(&x.data, utils::API::Zilliqa));
        let miner_addr_eth = header
            .minerpubkey
            .as_ref()
            .and_then(|x| utils::maybe_hex_address_from_public_key(&x.data, utils::API::Ethereum));
        let state_delta_hash = Some(encode_u8(header.statedeltahash.as_slice()));
        let tran_receipt_hash = Some(encode_u8(header.tranreceipthash.as_slice()));
        let block_shard_id: i64 = header.oneof2.as_ref().map_or(-1, |x| {
            let pdtlib::proto::proto_micro_block::micro_block_header::Oneof2::Shardid(val) = x;
            i64::try_from(*val).unwrap_or(-1)
        });
        let gas_used = header.oneof4.as_ref().map_or(-1, |x| {
            let pdtlib::proto::proto_micro_block::micro_block_header::Oneof4::Gasused(val) = x;
            i64::try_from(*val).unwrap_or(-1)
        });
        let epoch_num = header.oneof7.as_ref().map_or(-1, |x| {
            let pdtlib::proto::proto_micro_block::micro_block_header::Oneof7::Epochnum(val) = x;
            i64::try_from(*val).unwrap_or(-1)
        });
        let num_txs = header.oneof9.as_ref().map_or(-1, |x| {
            let pdtlib::proto::proto_micro_block::micro_block_header::Oneof9::Numtxs(val) = x;
            i64::try_from(*val).unwrap_or(-1)
        });

        let blockhash = Some(encode_u8(block_base.blockhash.as_slice()));
        let cs1 = cosigs
            .cs1
            .clone()
            .and_then(|x| Some(encode_u8(x.data.as_slice())));
        let cs2 = cosigs
            .cs2
            .clone()
            .and_then(|x| Some(encode_u8(x.data.as_slice())));
        let b1: String = cosigs
            .b1
            .iter()
            .map(|x| if *x { '1' } else { '0' })
            .collect();
        let b2: String = cosigs
            .b2
            .iter()
            .map(|x| if *x { '1' } else { '0' })
            .collect();

        Ok(Self {
            block: blk,
            offset_in_block: 0,
            shard_id: Some(shard_id),
            header_version: header_base.version.into(),
            header_committee_hash,
            header_prev_hash,
            gas_limit: header.gaslimit.try_into()?,
            rewards,
            prev_hash,
            tx_root_hash,
            miner_pubkey,
            miner_addr_zil,
            miner_addr_eth,
            ds_block_num: header.dsblocknum.try_into()?,
            state_delta_hash,
            tran_receipt_hash,
            block_shard_id,
            gas_used,
            epoch_num,
            num_txs,
            blockhash,
            timestamp: block_base.timestamp.try_into()?,
            cs1,
            b1: Some(b1),
            cs2,
            b2: Some(b2),
            imported_from: "zq".to_string(),
            eth_parent_hash: None,
            eth_uncles_hash: None,
            eth_state_root: None,
            eth_extra_data: None,
            eth_logs_bloom: None,
            eth_difficulty: None,
            eth_total_difficulty: None,
            eth_nonce: None,
            eth_base_fee_per_gas: None,
            eth_withdrawals_root: None,
            eth_withdrawals: None,
        })
    }

    pub fn from_eth(in_val: &Block<Transaction>) -> Result<Self> {
        // let u64_to_i64 = |x| i64::try_from(x).expect("U64 transaction index should fit in i64");
        let val = in_val.clone();
        let block: i64 = val
            .number
            .ok_or(anyhow!("this block has no number, is it still pending?"))?
            .try_into()
            .expect("block number should fit in an i64!");
        let header_version = i64::from_str_radix(
            val.other
                .get("version")
                .ok_or(anyhow!("this block has no version specified!"))?
                .as_str()
                .ok_or(anyhow!("the version field for this block isn't a string!"))?
                .strip_prefix("0x")
                .unwrap(),
            16,
        )?;
        let gas_limit: i64 = val
            .gas_limit
            .try_into()
            .expect("gas_limit should fit in an i64!");
        let tx_root_hash = Some(encode_u8(val.transactions_root.as_bytes()));
        let author_bytes = val
            .author
            .as_ref()
            .ok_or(anyhow!("this block has no author, is it still pending?"))?
            .as_bytes();
        let miner_pubkey = None;
        let miner_addr_zil = None; //TODO: work out how to get the zil address from the eth address

        // let miner_addr_zil = maybe_hex_address_from_public_key(author_bytes, utils::API::Zilliqa);
        let miner_addr_eth = Some(hex::encode(author_bytes));
        let tran_receipt_hash = Some(encode_u8(val.receipts_root.as_bytes()));
        let gas_used: i64 = val
            .gas_used
            .try_into()
            .expect("gas_used should fit in an i64!");
        let num_txs = val.transactions.len().try_into()?;
        let blockhash = Some(encode_u8(
            val.hash
                .ok_or(anyhow!("this block is missing a hash!"))?
                .as_bytes(),
        ));
        let timestamp: i64 = val
            .timestamp
            .mul(U256::from(1000)) // pad to convert: milliseconds -> microseconds
            .try_into()
            .expect("timestamp should fit in an i64!");
        let eth_parent_hash = Some(encode_u8(val.parent_hash.as_bytes()));
        let eth_uncles_hash = Some(encode_u8(val.uncles_hash.as_bytes()));
        let eth_state_root = Some(encode_u8(val.state_root.as_bytes()));
        let eth_extra_data = Some(encode_u8(val.extra_data.as_ref()));
        let eth_logs_bloom = val.logs_bloom.map(|x| encode_u8(x.as_bytes()));
        let eth_difficulty: Option<i64> = Some(
            val.difficulty
                .try_into()
                .expect("difficulty should fit in an i64!"),
        );
        let eth_total_difficulty: Option<i64> = val.total_difficulty.map(|x| {
            x.try_into()
                .expect("total_difficulty should fit in an i64!")
        });

        let eth_nonce = val.nonce.map(|x| encode_u8(x.as_bytes()));
        let eth_base_fee_per_gas: Option<i64> = val.base_fee_per_gas.map(|x| {
            x.try_into()
                .expect("total_difficulty should fit in an i64!")
        });

        let eth_withdrawals_root = val.withdrawals_root.map(|x| encode_u8(x.as_bytes()));
        let eth_withdrawals = val
            .withdrawals
            .map(|x| serde_json::to_string::<Vec<Withdrawal>>(&x))
            .transpose()?;
        Ok(BQMicroblock {
            block,
            offset_in_block: 0,
            shard_id: None,
            header_version,
            header_committee_hash: None,
            header_prev_hash: None,
            gas_limit,
            rewards: None, //TODO: eth apis don't give rewards, so either compute or query persistence later
            prev_hash: None,
            tx_root_hash,
            miner_pubkey,
            miner_addr_zil,
            miner_addr_eth,
            ds_block_num: block / 100, //this is probably cheating?
            state_delta_hash: None,
            tran_receipt_hash,
            block_shard_id: 0, //TODO: figure out what this field is
            gas_used,
            epoch_num: block,
            num_txs,
            blockhash,
            timestamp,
            cs1: None,
            b1: None,
            cs2: None,
            b2: None,
            imported_from: "eth".to_string(),
            eth_parent_hash,
            eth_uncles_hash,
            eth_state_root,
            eth_extra_data,
            eth_logs_bloom,
            eth_difficulty,
            eth_total_difficulty,
            eth_nonce,
            eth_base_fee_per_gas,
            eth_withdrawals_root,
            eth_withdrawals,
        })
    }
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Serialize, Deserialize, Clone, FromRow, PSQLInsertable, Debug, PartialEq)]
pub struct PSQLMicroblock {
    #[psql_type = "BIGINT"]
    pub block: i64,
    #[psql_type = "BIGINT"]
    pub offset_in_block: i64,
    // The shard id from the index.
    #[psql_type = "BIGINT"]
    pub shard_id: Option<i64>,
    #[psql_type = "BIGINT"]
    pub header_version: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub header_committee_hash: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub header_prev_hash: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub gas_limit: i64,
    // Rewards.
    #[sqlx(default)]
    #[psql_type = "numeric(76, 38)"]
    pub rewards: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub prev_hash: Option<Vec<u8>>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub tx_root_hash: Option<Vec<u8>>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub miner_pubkey: Option<Vec<u8>>,
    #[sqlx(default)]
    #[psql_type = "varchar(40)"]
    pub miner_addr_zil: Option<String>,
    #[sqlx(default)]
    #[psql_type = "varchar(40)"]
    pub miner_addr_eth: Option<String>,
    #[psql_type = "BIGINT"]
    pub ds_block_num: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub state_delta_hash: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub tran_receipt_hash: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub block_shard_id: i64,
    #[psql_type = "BIGINT"]
    pub gas_used: i64,
    #[psql_type = "BIGINT"]
    pub epoch_num: i64,
    #[psql_type = "BIGINT"]
    pub num_txs: i64,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub blockhash: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub timestamp: i64,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub cs1: Option<Vec<u8>>,
    // represented as a string of '0' or '1' for easier querying.
    #[sqlx(default)]
    #[psql_type = "varchar(10)"]
    pub b1: Option<String>,
    #[sqlx(default)]
    #[psql_type = "bytea"]
    pub cs2: Option<Vec<u8>>,
    // represented as a string of '0' or '1' for easier querying.
    #[sqlx(default)]
    #[psql_type = "varchar(10)"]
    pub b2: Option<String>,
    #[psql_type = "text"]
    pub imported_from: String,
    #[psql_type = "bytea"]
    pub eth_parent_hash: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    pub eth_uncles_hash: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    pub eth_state_root: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    pub eth_extra_data: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    pub eth_logs_bloom: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub eth_difficulty: Option<i64>,
    #[psql_type = "BIGINT"]
    pub eth_total_difficulty: Option<i64>,
    #[psql_type = "bytea"]
    pub eth_nonce: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub eth_base_fee_per_gas: Option<i64>,
    #[psql_type = "bytea"]
    pub eth_withdrawals_root: Option<Vec<u8>>,
    #[psql_type = "jsonb"]
    pub eth_withdrawals: Option<String>,
}

impl From<PSQLMicroblock> for BQMicroblock {
    fn from(mb: PSQLMicroblock) -> Self {
        let conv = move |y: Vec<u8>| encode_u8(y.as_slice());
        BQMicroblock {
            header_committee_hash: mb.header_committee_hash.map(conv),
            header_prev_hash: mb.header_prev_hash.map(conv),
            prev_hash: mb.prev_hash.map(conv),
            tx_root_hash: mb.tx_root_hash.map(conv),
            miner_pubkey: mb.miner_pubkey.map(conv),
            state_delta_hash: mb.state_delta_hash.map(conv),
            tran_receipt_hash: mb.tran_receipt_hash.map(conv),
            blockhash: mb.blockhash.map(conv),
            cs1: mb.cs1.map(conv),
            cs2: mb.cs2.map(conv),
            block: mb.block,
            offset_in_block: mb.offset_in_block,
            shard_id: mb.shard_id,
            header_version: mb.header_version,
            gas_limit: mb.gas_limit,
            rewards: mb.rewards,
            miner_addr_zil: mb.miner_addr_zil,
            miner_addr_eth: mb.miner_addr_eth,
            ds_block_num: mb.ds_block_num,
            block_shard_id: mb.block_shard_id,
            gas_used: mb.gas_used,
            epoch_num: mb.epoch_num,
            num_txs: mb.num_txs,
            timestamp: mb.timestamp,
            b1: mb.b1,
            b2: mb.b2,
            imported_from: mb.imported_from,
            eth_parent_hash: mb.eth_parent_hash.map(conv),
            eth_uncles_hash: mb.eth_uncles_hash.map(conv),
            eth_state_root: mb.eth_state_root.map(conv),
            eth_extra_data: mb.eth_extra_data.map(conv),
            eth_logs_bloom: mb.eth_logs_bloom.map(conv),
            eth_difficulty: mb.eth_difficulty,
            eth_total_difficulty: mb.eth_total_difficulty,
            eth_nonce: mb.eth_nonce.map(conv),
            eth_base_fee_per_gas: mb.eth_base_fee_per_gas,
            eth_withdrawals_root: mb.eth_withdrawals_root.map(conv),
            eth_withdrawals: mb.eth_withdrawals,
        }
    }
}

impl From<BQMicroblock> for PSQLMicroblock {
    fn from(mb: BQMicroblock) -> Self {
        PSQLMicroblock {
            header_committee_hash: mb.header_committee_hash.map(decode_u8),
            header_prev_hash: mb.header_prev_hash.map(decode_u8),
            prev_hash: mb.prev_hash.map(decode_u8),
            tx_root_hash: mb.tx_root_hash.map(decode_u8),
            miner_pubkey: mb.miner_pubkey.map(decode_u8),
            state_delta_hash: mb.state_delta_hash.map(decode_u8),
            tran_receipt_hash: mb.tran_receipt_hash.map(decode_u8),
            blockhash: mb.blockhash.map(decode_u8),
            cs1: mb.cs1.map(decode_u8),
            cs2: mb.cs2.map(decode_u8),
            block: mb.block,
            offset_in_block: mb.offset_in_block,
            shard_id: mb.shard_id,
            header_version: mb.header_version,
            gas_limit: mb.gas_limit,
            rewards: mb.rewards,
            miner_addr_zil: mb.miner_addr_zil,
            miner_addr_eth: mb.miner_addr_eth,
            ds_block_num: mb.ds_block_num,
            block_shard_id: mb.block_shard_id,
            gas_used: mb.gas_used,
            epoch_num: mb.epoch_num,
            num_txs: mb.num_txs,
            timestamp: mb.timestamp,
            b1: mb.b1,
            b2: mb.b2,
            imported_from: mb.imported_from,
            eth_parent_hash: mb.eth_parent_hash.map(decode_u8),
            eth_uncles_hash: mb.eth_uncles_hash.map(decode_u8),
            eth_state_root: mb.eth_state_root.map(decode_u8),
            eth_extra_data: mb.eth_extra_data.map(decode_u8),
            eth_logs_bloom: mb.eth_logs_bloom.map(decode_u8),
            eth_difficulty: mb.eth_difficulty,
            eth_total_difficulty: mb.eth_total_difficulty,
            eth_nonce: mb.eth_nonce.map(decode_u8),
            eth_base_fee_per_gas: mb.eth_base_fee_per_gas,
            eth_withdrawals_root: mb.eth_withdrawals_root.map(decode_u8),
            eth_withdrawals: mb.eth_withdrawals,
        }
    }
}

impl PSQLMicroblock {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[test]
fn check_involution() {
    let bq_mb = BQMicroblock {
        block: 276976,
        offset_in_block: 0,
        shard_id: Some(
            1,
        ),
        header_version: 1,
        header_committee_hash: Some(
            "Rt5CiTmpcaeHtAMXjFs6/u/aEp7QGXiL3XbaVFOJUHo=".to_string(),
        ),
        header_prev_hash: Some(
            "dAUYU52Z5TZtv7BypWXfV0YFgVsucKw/UakSFzDKmMg=".to_string(),
        ),
        gas_limit: 50000,
        rewards: Some(
            "1000000000".to_string(),
        ),
        prev_hash: Some(
            "".to_string(),
        ),
        tx_root_hash: Some(
            "TsSO1quhBndOKirfCPCFLrUXhuW7us3xyZYfQPGbnFY=".to_string(),
        ),
        miner_pubkey: Some(
            "AlpfdgAhkMlWCu+aujEtFuo335RNYTURrYii5eCNRnkn".to_string(),
        ),
        miner_addr_zil: Some(
            "a897a14382905de85d200f26b9b4f8a4d733df89".to_string(),
        ),
        miner_addr_eth: Some(
            "d0b2f8831e21eb99618094fd4955002fc36850a6".to_string(),
        ),
        ds_block_num: 2769,
        state_delta_hash: Some(
            "NKeUGoT4iXcIQCPRRSLu6/A2WWr55VDMUxNj4Xwa68Q=".to_string(),
        ),
        tran_receipt_hash: Some(
            "VLb83hlHkEqQp6YC55i9Gyr8vGjxgIj1DMbDo9QcOQc=".to_string(),
        ),
        block_shard_id: 1,
        gas_used: 1,
        epoch_num: 276976,
        num_txs: 1,
        blockhash: Some(
            "sw0fIWtUH5uaau6YU4yIqlbkUdZS5KA4m1A//1K1vK0=".to_string(),
        ),
        timestamp: 1553154354973091,
        cs1: Some(
            "tyqe18/vSbcMnYs5osxEl24A13FKEI1+lDqUd2fqfye2vDzkvJCa8xOf8NvtQag5n2N3LLDFkQbSHuEseHBaoQ==".to_string(),
        ),
        b1: Some(
            "0110111110".to_string(),
        ),
        cs2: Some(
            "mVN9Vu0wFta+/87lb+A14LG1k62M1Ee0mdeXa5JJt4/vNRZl+ge8ypBlMOdIgJ5qQhEragQBdLxztf2dQv2lyA==".to_string(),
        ),
        b2: Some(
            "1111111000".to_string(),
        ),
        imported_from: "zq".to_string(),
        eth_parent_hash: None,
        eth_uncles_hash: None,
        eth_state_root: None,
        eth_extra_data: None,
        eth_logs_bloom: None,
        eth_difficulty: None,
        eth_total_difficulty: None,
        eth_nonce: None,
        eth_base_fee_per_gas: None,
        eth_withdrawals_root: None,
        eth_withdrawals: None,
    };
    let psql_mb = Into::<PSQLMicroblock>::into(bq_mb.clone());
    let bq_mb_2 = Into::<BQMicroblock>::into(psql_mb);
    assert_eq!(bq_mb, bq_mb_2)
}
