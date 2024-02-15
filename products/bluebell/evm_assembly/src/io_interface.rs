use std::collections::BTreeMap;

use evm::backend::{Backend, Basic};
use primitive_types::{H160, H256, U256};

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

#[derive(Debug)]
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
        U256::zero()
    }

    fn origin(&self) -> H160 {
        unimplemented!()
    }

    fn block_hash(&self, _: U256) -> H256 {
        H256::zero()
    }

    fn block_number(&self) -> U256 {
        U256::zero()
    }

    fn block_coinbase(&self) -> H160 {
        unimplemented!()
    }

    fn block_timestamp(&self) -> U256 {
        U256::zero()
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
        self.state.contains_key(&address)
    }

    fn basic(&self, _address: H160) -> Basic {
        Basic {
            balance: 0.into(),
            nonce: 0.into(),
        }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        self.state
            .get(&address)
            .map(|v| v.code.clone())
            .unwrap_or_default()
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        let storage = self
            .state
            .get(&address)
            .map(|v| v.storage.clone())
            .unwrap_or_default();

        match storage.get(&index) {
            Some(v) => v.clone(),
            None => H256::zero(),
        }
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        let storage = self
            .state
            .get(&address)
            .map(|v| v.storage.clone())
            .unwrap_or_default();

        storage.get(&index).copied()
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
