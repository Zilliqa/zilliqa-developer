use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use evm::{
    backend::Apply,
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config,
};
use log::info;
use primitive_types::{H160, U256};

use crate::{
    compiler_context::EvmCompilerContext,
    executable::EvmExecutable,
    io_interface::{CustomMemoryAccount, EvmIoInterface},
    types::EvmTypeValue,
};

pub struct EvmExecutor<'a> {
    pub context: &'a EvmCompilerContext,
    pub executable: EvmExecutable,
}

#[derive(Debug, Clone)]
pub struct ExecutorResult {
    pub changeset: HashMap<String, Option<String>>,
    pub result: String,
}

impl<'a> EvmExecutor<'a> {
    pub fn new(context: &'a EvmCompilerContext, executable: EvmExecutable) -> Self {
        Self {
            context,
            executable,
        }
    }

    pub fn get_label_position(&self, label: &str) -> Option<u32> {
        self.executable.label_positions.get(label).copied()
    }

    pub fn execute(&self, name: &str, args: Vec<EvmTypeValue>) -> ExecutorResult {
        let input = self
            .context
            .get_function(name)
            .expect(&format!("Function name {} not found", name).to_string())
            .generate_transaction_data(args);

        // Initialized the state of EVM's memory.
        let config = Config::istanbul();
        let mut state = BTreeMap::new();

        // Add our contract under the 0x10 address.
        state.insert(
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            CustomMemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10000000),
                storage: BTreeMap::new(),
                code: self.executable.bytecode.clone(),
            },
        );

        // Add new user 0xf0 that will be used as the contract caller.
        state.insert(
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            CustomMemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10000000),
                storage: BTreeMap::new(),
                code: Vec::new(),
            },
        );

        // Prepare the executor.
        let backend = EvmIoInterface::new(state); //MemoryBackend::new(&vicinity, state);
        let metadata = StackSubstateMetadata::new(u64::MAX, &config);
        let mem_state = MemoryStackState::new(metadata, &backend);
        let precompiles = self.context.get_precompiles();
        let mut executor = StackExecutor::new_with_precompiles(mem_state, &config, &precompiles);

        // Call the 0x10 contract using the 0xf0 user.
        // Use the input variable.
        let (exit_reason, result) = executor.transact_call(
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            U256::zero(),
            input,
            u64::MAX,
            Vec::new(),
        );

        let (state_apply, _logs) = executor.into_state().deconstruct();
        info!("\n\n\nExit reason: {:#?}", exit_reason);
        info!("Result: {:#?}", result);

        let mut ret = ExecutorResult {
            changeset: HashMap::new(),
            result: format!("{:?}", result),
        };

        for update in state_apply {
            match update {
                Apply::Modify {
                    address,
                    basic: _,
                    code: _,
                    storage,
                    reset_storage: _,
                } => {
                    for (k, v) in storage {
                        let key = format!("{:?}.{:?}", address, k);
                        ret.changeset.insert(key, Some(format!("{:?}", v)));
                    }
                }
                Apply::Delete { address } => {
                    let key = format!("{:?}", address);
                    ret.changeset.insert(key, None);
                }
            }
        }

        ret
    }
}
