use evm::backend::{Backend, Basic};
use primitive_types::{H160, H256, U256};
use std::collections::BTreeMap;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct CustomMemoryAccount {
    /// Account nonce.
    pub nonce: U256,
    /// Account balance.
    pub balance: U256,
    /// Full account storage.
    pub storage: BTreeMap<H256, H256>,
    /// Account code.
    pub code: Vec<u8>,
}

pub struct EvmIoInterface {
    // Backend refers to storage, not execution platform
    state: BTreeMap<H160, CustomMemoryAccount>,
}

impl EvmIoInterface {
    pub fn new(state: BTreeMap<H160, CustomMemoryAccount>) -> Self {
        Self { state }
    }
}

impl Backend for EvmIoInterface {
    fn gas_price(&self) -> U256 {
        unimplemented!()
    }

    fn origin(&self) -> H160 {
        unimplemented!()
    }

    fn block_hash(&self, _: U256) -> H256 {
        unimplemented!()
    }

    fn block_number(&self) -> U256 {
        unimplemented!()
    }

    fn block_coinbase(&self) -> H160 {
        unimplemented!()
    }

    fn block_timestamp(&self) -> U256 {
        unimplemented!()
    }

    fn block_difficulty(&self) -> U256 {
        unimplemented!()
    }

    //fn block_randomness(&self) -> Option<H256> { // Put note for PR
    //    None
    //}

    fn block_gas_limit(&self) -> U256 {
        unimplemented!()
    }

    fn block_base_fee_per_gas(&self) -> U256 {
        unimplemented!()
    }

    fn chain_id(&self) -> U256 {
        unimplemented!()
    }

    fn exists(&self, address: H160) -> bool {
        println!("Checking if address '{:?}' exists!", address);
        false
        //        unimplemented!()
    }

    fn basic(&self, address: H160) -> Basic {
        println!("Getting basic info for '{:?}'", address);
        Basic {
            balance: 0.into(),
            nonce: 0.into(),
        }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        println!("Requesting code for '{:?}'", address);
        self.state
            .get(&address)
            .map(|v| v.code.clone())
            .unwrap_or_default()
    }

    fn storage(&self, _address: H160, _index: H256) -> H256 {
        unimplemented!()
    }

    fn original_storage(&self, _address: H160, _index: H256) -> Option<H256> {
        unimplemented!()
    }

    // todo: this.
    fn code_as_json(&self, _address: H160) -> Vec<u8> {
        unimplemented!()
    }

    fn init_data_as_json(&self, _address: H160) -> Vec<u8> {
        unimplemented!()
    }

    // todo: this.
    fn substate_as_json(&self, _address: H160, _vname: &str, _indices: &[String]) -> Vec<u8> {
        unimplemented!()
    }
}
