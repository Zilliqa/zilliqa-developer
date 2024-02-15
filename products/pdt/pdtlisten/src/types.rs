use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MicroblockInfo {
    pub(crate) micro_block_hash: String,
    pub(crate) micro_block_shard_id: i64,
    pub(crate) micro_block_txn_root_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TxBlockBody {
    pub(crate) block_hash: String,
    pub(crate) header_sign: String,
    pub(crate) micro_block_infos: Vec<MicroblockInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TxBlockHeader {
    pub(crate) block_num: String,
    #[serde(rename = "DSBlockNum")]
    pub(crate) ds_block_num: String,
    pub(crate) gas_limit: String,
    pub(crate) gas_used: String,
    pub(crate) mb_info_hash: String,
    pub(crate) miner_pub_key: String,
    pub(crate) num_micro_blocks: i64,
    pub(crate) num_pages: i64,
    pub(crate) num_txns: i64,
    pub(crate) prev_block_hash: String,
    pub(crate) rewards: String,
    pub(crate) state_delta_hash: String,
    pub(crate) state_root_hash: String,
    pub(crate) timestamp: String,
    pub(crate) txn_fees: String,
    pub(crate) version: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GetTxBlockResponse {
    pub(crate) body: TxBlockBody,
    pub(crate) header: TxBlockHeader,
}
