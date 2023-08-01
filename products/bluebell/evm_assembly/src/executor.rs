use crate::compiler_context::EvmCompilerContext;
use crate::io_interface::CustomMemoryAccount;
use crate::io_interface::EvmIoInterface;
use crate::types::EvmTypeValue;
use evm::executor::stack::MemoryStackState;
use evm::executor::stack::StackExecutor;
use evm::executor::stack::StackSubstateMetadata;
use evm::Config;
use primitive_types::H160;
use primitive_types::U256;
use std::collections::BTreeMap;
use std::str::FromStr;

pub struct EvmExecutor<'a> {
    context: &'a EvmCompilerContext,
    code: Vec<u8>, // state: MemoryStackState,
}

impl<'a> EvmExecutor<'a> {
    pub fn new(context: &'a EvmCompilerContext, code: Vec<u8>) -> Self {
        Self { context, code }
    }

    pub fn execute(&self, name: &str, args: Vec<EvmTypeValue>) {
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
                code: self.code.clone(),
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
        let state = MemoryStackState::new(metadata, &backend);
        let precompiles = self.context.get_precompiles();
        let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);
        println!("Execute input: {}", hex::encode(input.clone()));
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

        println!("Exit reason: {:#?}", exit_reason);
        println!("Result: {:#?}", result);
    }
}
