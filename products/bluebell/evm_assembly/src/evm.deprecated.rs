use crate::support::evm_bytecode_builder::EvmByteCodeBuilder;
use evm::backend::MemoryAccount;
use evm::backend::MemoryBackend;
use evm::executor::stack::MemoryStackState;
use evm::executor::stack::StackSubstateMetadata;
use std::str::FromStr;

use evm::{backend::MemoryVicinity, executor::stack::StackExecutor, Config};
use primitive_types::{H160, U256};
use std::collections::BTreeMap;
// See https://odra.dev/blog/evm-at-risc0/
pub fn evm_test() {
    // let program = "60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056";
    // let code = hex::decode(program).unwrap();
    let mut builder = EvmByteCodeBuilder::new();
    builder
        .push_u8(0x60)
        .push_u8(0x2A)
        .push_u8(0x60)
        .push_u8(0x00)
        //	    .external_call()
        .push_u8(0x55)
        .stop();
    let code = builder.build();

    let input = "61047ff4000000000000000000000000000000000000000000000000000000000000000a";
    // Define EVM configuration.
    let config = Config::istanbul();
    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
    };

    // Initialized the state of EVM's memory.
    let mut state = BTreeMap::new();

    // Add our contract under the 0x10 address.
    state.insert(
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code,
        },
    );

    // Add new user 0xf0 that will be used as the contract caller.
    state.insert(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    // Prepare the executor.
    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    // Call the 0x10 contract using the 0xf0 user.
    // Use the input variable.
    let (exit_reason, result) = executor.transact_call(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        U256::zero(),
        hex::decode(input).unwrap(),
        u64::MAX,
        Vec::new(),
    );

    // Make sure the execution succeeded.
    // assert!(exit_reason == ExitReason::Succeed(ExitSucceed::Returned));

    // Return hex encoded string.
    // hex::encode(result)
}
