use crate::utils::{self, decode_u8, encode_u8};
use crate::zqproj::PSQLInsertable;
use anyhow::{anyhow, Result};
use ethers::types::Transaction;
use hex;
use pdtlib::proto::ProtoTransactionWithReceipt;
use primitive_types::{H160, H256};
use psql_derive::PSQLInsertable;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::FromRow;
#[derive(Serialize, Deserialize, Clone, FromRow, PSQLInsertable, Debug, PartialEq)]
pub struct BQTransaction {
    #[psql_type = "text"]
    pub id: String,
    #[psql_type = "BIGINT"]
    pub block: i64,
    #[psql_type = "BIGINT"]
    pub offset_in_block: i64,
    #[psql_type = "BIGINT"]
    pub zqversion: i64,
    #[psql_type = "numeric(76, 38)"]
    #[sqlx(default)]
    pub amount: Option<String>,
    // application/x-scilla-contract or application/x-evm-contract
    #[psql_type = "text"]
    #[sqlx(default)]
    pub api_type: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub code: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub data: Option<String>,
    #[psql_type = "BIGINT"]
    pub gas_limit: i64,
    #[psql_type = "numeric(76, 38)"]
    #[sqlx(default)]
    pub gas_price: Option<String>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub nonce: Option<i64>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub raw_receipt: Option<String>,
    #[psql_type = "jsonb"]
    #[sqlx(default)]
    pub receipt: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub sender_public_key: Option<String>,
    #[psql_type = "varchar(40)"]
    #[sqlx(default)]
    pub from_addr_zil: Option<String>,
    #[psql_type = "varchar(40)"]
    #[sqlx(default)]
    pub from_addr_eth: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub signature: Option<String>,
    #[psql_type = "varchar(40)"]
    pub to_addr: Option<String>,
    #[psql_type = "BIGINT"]
    pub version: i64,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub cum_gas: Option<i64>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub shard_id: Option<i64>,
    #[psql_type = "text"]
    pub imported_from: String,
    #[psql_type = "bigint"]
    #[sqlx(default)]
    pub eth_transaction_index: Option<i64>, //index of txn within block
    #[psql_type = "numeric"]
    #[sqlx(default)]
    pub eth_value: Option<String>, //same as zq amount, probably
    // eth_v, eth_r and eth_s are signature information
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub eth_v: Option<i64>,
    #[psql_type = "text"]
    #[sqlx(default)]
    pub eth_r: Option<String>,
    #[psql_type = "text"]
    #[sqlx(default)]
    pub eth_s: Option<String>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub eth_transaction_type: Option<i64>,
}

impl BQTransaction {
    pub fn from_proto(
        in_val: &ProtoTransactionWithReceipt,
        blk: i64,
        offset_in_block: i64,
        shard_id: i64,
    ) -> Result<Self> {
        let val = in_val.clone();
        let txn = val
            .transaction
            .ok_or(anyhow!("Transaction object does not contain transaction"))?;
        let core = txn.info.ok_or(anyhow!("Transaction has no core info"))?;
        let gas_limit: i64 = <u64>::try_into(core.gaslimit)?;
        let id = H256::from_slice(&txn.tranid);
        let to_addr = H160::from_slice(&core.toaddr);
        let sender_public_key = core
            .senderpubkey
            .as_ref()
            .map_or(None, |x| Some(encode_u8(x.data.as_slice())));
        let from_addr_zil = core
            .senderpubkey
            .as_ref()
            .and_then(|x| utils::maybe_hex_address_from_public_key(&x.data, utils::API::Zilliqa));
        let from_addr_eth = core
            .senderpubkey
            .as_ref()
            .and_then(|x| utils::maybe_hex_address_from_public_key(&x.data, utils::API::Ethereum));
        let nonce: Option<i64> = if let Some(nonce_val) = core.oneof2 {
            let pdtlib::proto::proto_transaction_core_info::Oneof2::Nonce(actual) = nonce_val;
            Some(<i64>::try_from(actual)?)
        } else {
            None
        };
        let api_type = Some("unknown".to_string());

        let code = core.oneof8.map_or(None, |x| {
            let pdtlib::proto::proto_transaction_core_info::Oneof8::Code(y) = x;
            Some(encode_u8(y.as_slice()))
        });
        let data = core.oneof9.map_or(None, |x| {
            let pdtlib::proto::proto_transaction_core_info::Oneof9::Data(y) = x;
            Some(encode_u8(y.as_slice()))
        });
        let signature = txn
            .signature
            .map_or(None, |x| Some(encode_u8(x.data.as_slice())));

        let raw_receipt = val.receipt.as_ref().and_then(|x| {
            std::str::from_utf8(&x.receipt)
                .ok()
                .and_then(|x| Some(x.to_string()))
        });
        let receipt = raw_receipt
            .as_ref()
            .and_then(|x| Some(x.replace("\\u0000", "\\ufffd")));
        let raw_receipt = raw_receipt.and_then(|x| Some(encode_u8(x.as_bytes())));
        let cum_gas = val.receipt.as_ref().and_then(|x| {
            x.oneof2.as_ref().and_then(|y| {
                let pdtlib::proto::proto_transaction_receipt::Oneof2::Cumgas(z) = y;
                <u64>::try_into(*z).ok()
            })
        });
        let amount = core
            .amount
            .clone()
            .and_then(|x| utils::u128_string_from_storage(&x));
        let gas_price = core
            .gasprice
            .clone()
            .and_then(|x| utils::u128_string_from_storage(&x));
        Ok(BQTransaction {
            id: hex::encode(id.as_bytes()),
            block: blk,
            offset_in_block,
            zqversion: 100,
            amount: amount.clone(),
            api_type,
            code,
            data,
            gas_limit,
            gas_price,
            nonce,
            raw_receipt,
            receipt,
            sender_public_key,
            from_addr_zil,
            from_addr_eth,
            signature,
            to_addr: Some(hex::encode(to_addr.as_bytes())),
            version: i64::from(core.version),
            cum_gas,
            shard_id: Some(shard_id),
            imported_from: "zq".to_string(),
            eth_transaction_index: Some(offset_in_block),
            eth_value: amount.clone(),
            eth_v: None,
            eth_r: None,
            eth_s: None,
            eth_transaction_type: None,
        })
    }

    pub fn from_eth(in_val: &Transaction, zqversion: i64) -> Result<Self> {
        let u64_to_i64 = |x| i64::try_from(x).expect("U64 transaction index should fit in i64");
        let val = in_val.clone();
        let id = hex::encode(val.hash.as_bytes());
        let block: i64 = val
            .block_number
            .ok_or(anyhow!(
                "this transaction has no block number, is it still pending?"
            ))?
            .try_into()
            .expect("block number should fit in an i64!");
        let offset_in_block: i64 = val
            .transaction_index
            .ok_or(anyhow!(
                "this transaction has no index within block, is it still pending?"
            ))?
            .try_into()
            .expect("offset_in_block should fit in an i64!");
        let data = Some(encode_u8(val.input.as_ref()));
        let gas_limit: i64 = i64::from_str_radix(
            val.other
                .get("gasLimit")
                .ok_or(anyhow!("transaction has no gasLimit"))?
                .as_str()
                .ok_or(anyhow!(
                    "gasLimit is not a string: {:#?}",
                    in_val.other.get("gasLimit")
                ))?
                .strip_prefix("0x")
                .expect("gasLimit should start with 0x"),
            16,
        )?;
        let nonce: Option<i64> = Some(val.nonce.as_u64().try_into()?);
        let gas_price = val.gas_price.map(|x| x.to_string());
        let from_addr_eth = Some(hex::encode(val.from.as_bytes()));
        let to_addr = val.to.map(|x| hex::encode(x.as_bytes()));
        let cum_gas: Option<i64> = Some(val.gas.as_u64().try_into()?);
        let eth_transaction_index = val.transaction_index.map(u64_to_i64);
        let eth_value = Some(val.value.to_string());
        let eth_v = Some(u64_to_i64(val.v));
        let eth_s = Some(val.s.to_string());
        let eth_r = Some(val.r.to_string());
        let eth_transaction_type = val.transaction_type.map(u64_to_i64);
        Ok(BQTransaction {
            id,
            block,
            offset_in_block,
            zqversion,
            amount: Some(val.value.to_string()),
            api_type: None,
            code: None,
            data,
            gas_limit,
            gas_price,
            nonce,
            raw_receipt: None,
            receipt: None,
            sender_public_key: None,
            from_addr_zil: None,
            from_addr_eth,
            signature: None,
            to_addr,
            version: zqversion,
            cum_gas,
            shard_id: None,
            imported_from: "eth".to_string(),
            eth_transaction_index,
            eth_value,
            eth_v,
            eth_r,
            eth_s,
            eth_transaction_type,
        })
    }

    pub fn from_eth_with_zil_txn_bodies(
        in_val: &Transaction,
        zil_txn_body: &ZILTransactionBody,
        zqversion: i64,
    ) -> Result<Self> {
        let txn_body =
            Self::from_eth(in_val, zqversion).expect("should be compatible with eth transactions");

        let from_addr_zil = utils::maybe_hex_address_from_public_key(
            zil_txn_body.sender_pub_key.as_bytes(),
            utils::API::Zilliqa,
        );
        let raw_receipt = encode_u8(zil_txn_body.receipt.as_bytes());
        let code = zil_txn_body
            .code
            .as_ref()
            .map(|code| encode_u8(code.as_bytes()));
        Ok(BQTransaction {
            code,
            receipt: Some(zil_txn_body.receipt.clone()),
            raw_receipt: Some(raw_receipt),
            sender_public_key: Some(zil_txn_body.sender_pub_key.clone()),
            from_addr_zil,
            signature: Some(zil_txn_body.signature.clone()),
            ..txn_body
        })
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Serialize, Deserialize, Clone, FromRow, PSQLInsertable, Debug, PartialEq)]
pub struct PSQLTransaction {
    #[psql_type = "text"]
    pub id: String,
    #[psql_type = "BIGINT"]
    pub block: i64,
    #[psql_type = "BIGINT"]
    pub offset_in_block: i64,
    #[psql_type = "BIGINT"]
    pub zqversion: i64,
    #[psql_type = "numeric(76, 38)"]
    #[sqlx(default)]
    pub amount: Option<String>,
    // application/x-scilla-contract or application/x-evm-contract
    #[psql_type = "text"]
    #[sqlx(default)]
    pub api_type: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub code: Option<Vec<u8>>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub data: Option<Vec<u8>>,
    #[psql_type = "BIGINT"]
    pub gas_limit: i64,
    #[psql_type = "numeric(76, 38)"]
    #[sqlx(default)]
    pub gas_price: Option<String>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub nonce: Option<i64>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub raw_receipt: Option<Vec<u8>>,
    #[psql_type = "jsonb"]
    #[sqlx(default)]
    pub receipt: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub sender_public_key: Option<Vec<u8>>,
    #[psql_type = "varchar(40)"]
    #[sqlx(default)]
    pub from_addr_zil: Option<String>,
    #[psql_type = "varchar(40)"]
    #[sqlx(default)]
    pub from_addr_eth: Option<String>,
    #[psql_type = "bytea"]
    #[sqlx(default)]
    pub signature: Option<Vec<u8>>,
    #[psql_type = "varchar(40)"]
    pub to_addr: Option<String>,
    #[psql_type = "BIGINT"]
    pub version: i64,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub cum_gas: Option<i64>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub shard_id: Option<i64>,
    #[psql_type = "text"]
    pub imported_from: String,
    #[psql_type = "bigint"]
    #[sqlx(default)]
    pub eth_transaction_index: Option<i64>,
    #[psql_type = "numeric"]
    #[sqlx(default)]
    pub eth_value: Option<String>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub eth_v: Option<i64>,
    #[psql_type = "text"]
    #[sqlx(default)]
    pub eth_r: Option<String>,
    #[psql_type = "text"]
    #[sqlx(default)]
    pub eth_s: Option<String>,
    #[psql_type = "BIGINT"]
    #[sqlx(default)]
    pub eth_transaction_type: Option<i64>,
}

impl From<PSQLTransaction> for BQTransaction {
    fn from(txn: PSQLTransaction) -> BQTransaction {
        let conv = move |y: Vec<u8>| encode_u8(y.as_slice());
        BQTransaction {
            code: txn.code.map(conv),
            data: txn.data.map(conv),
            raw_receipt: txn.raw_receipt.map(conv),
            sender_public_key: txn.sender_public_key.map(conv),
            signature: txn.signature.map(conv),
            id: txn.id,
            block: txn.block,
            offset_in_block: txn.offset_in_block,
            zqversion: txn.zqversion,
            amount: txn.amount,
            api_type: txn.api_type,
            gas_limit: txn.gas_limit,
            gas_price: txn.gas_price,
            nonce: txn.nonce,
            receipt: txn.receipt,
            from_addr_zil: txn.from_addr_zil,
            from_addr_eth: txn.from_addr_eth,
            to_addr: txn.to_addr,
            version: txn.version,
            cum_gas: txn.cum_gas,
            shard_id: txn.shard_id,
            imported_from: txn.imported_from,
            eth_transaction_index: txn.eth_transaction_index,
            eth_value: txn.eth_value,
            eth_v: txn.eth_v,
            eth_r: txn.eth_r,
            eth_s: txn.eth_s,
            eth_transaction_type: txn.eth_transaction_type,
        }
    }
}
impl From<BQTransaction> for PSQLTransaction {
    fn from(txn: BQTransaction) -> Self {
        PSQLTransaction {
            code: txn.code.map(decode_u8),
            data: txn.data.map(decode_u8),
            raw_receipt: txn.raw_receipt.map(decode_u8),
            sender_public_key: txn.sender_public_key.map(decode_u8),
            signature: txn.signature.map(decode_u8),
            id: txn.id,
            block: txn.block,
            offset_in_block: txn.offset_in_block,
            zqversion: txn.zqversion,
            amount: txn.amount,
            api_type: txn.api_type,
            gas_limit: txn.gas_limit,
            gas_price: txn.gas_price,
            nonce: txn.nonce,
            receipt: txn.receipt,
            from_addr_zil: txn.from_addr_zil,
            from_addr_eth: txn.from_addr_eth,
            to_addr: txn.to_addr,
            version: txn.version,
            cum_gas: txn.cum_gas,
            shard_id: txn.shard_id,
            imported_from: txn.imported_from,
            eth_transaction_index: txn.eth_transaction_index,
            eth_value: txn.eth_value,
            eth_v: txn.eth_v,
            eth_r: txn.eth_r,
            eth_s: txn.eth_s,
            eth_transaction_type: txn.eth_transaction_type,
        }
    }
}

impl PSQLTransaction {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZILTransactionBody {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: String,
    pub code: Option<String>, // sometimes has
    pub data: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_limit: i64,
    pub gas_price: String,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: i64,
    pub receipt: String,
    pub sender_pub_key: String,
    pub signature: String,
    pub to_addr: String,
    #[serde_as(as = "DisplayFromStr")]
    pub version: i64,
}

#[test]
fn check_involution() {
    let bq_txn = BQTransaction {
        id: "674f088f8c4cdb2f19855fa6103f318433b4205fa78a45fb39cab2f919b92dcc".to_string(),
        block: 258686,
        offset_in_block: 8,
        zqversion: 100,
        amount: Some(
            "1000000000".to_string(),
        ),
        api_type: Some(
            "unknown".to_string(),
        ),
        code: None,
        data: None,
        gas_limit: 1,
        gas_price: Some(
            "1000000000".to_string(),
        ),
        nonce: Some(
            3664,
        ),
        raw_receipt: Some(
            "ewoJImN1bXVsYXRpdmVfZ2FzIiA6ICIxIiwKCSJlcG9jaF9udW0iIDogIjI1ODY4NiIsCgkic3VjY2VzcyIgOiB0cnVlCn0=".to_string(),
        ),
        receipt: Some(
            "{\n\t\"cumulative_gas\" : \"1\",\n\t\"epoch_num\" : \"258686\",\n\t\"success\" : true\n}".to_string(),
        ),
        sender_public_key: Some(
            "A80JT0iCJ1Z/LLtRwyavXedZZamR60KQEuSYl7yz9vVl".to_string(),
        ),
        from_addr_zil: Some(
            "cc02a3c906612cc5bdb087a30e6093c9f0aa04fc".to_string(),
        ),
        from_addr_eth: Some(
            "89e4f43065ce750369ff9ac56774f489df253a33".to_string(),
        ),
        signature: Some(
            "pL/Qq2kTzfv5eDr4P/jX6aE4eWO/RxW2+j3tJb8oBKh7OYRQNlG4WOs2jNp/kwwgUNOgBebgT8QIXPxfn8Hgkg==".to_string(),
        ),
        to_addr: Some(
            "c3780fd75afefb595f7a592f291054aef0984c64".to_string(),
        ),
        version: 21823489,
        cum_gas: Some(
            1,
        ),
        shard_id: Some(
            0,
        ),
        imported_from: "zq".to_string(),
        eth_transaction_index: Some(
            8,
        ),
        eth_value: Some(
            "1000000000".to_string(),
        ),
        eth_v: None,
        eth_r: None,
        eth_s: None,
        eth_transaction_type: None,
    };
    let psql_txn = Into::<PSQLTransaction>::into(bq_txn.clone());
    let bq_txn_2 = Into::<BQTransaction>::into(psql_txn);
    assert_eq!(bq_txn, bq_txn_2);
}
