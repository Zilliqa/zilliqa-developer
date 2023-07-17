use evm::backend::{Backend, Basic};
use primitive_types::{H160, H256, U256};
use std::collections::BTreeMap;

// See
// https://odra.dev/blog/evm-at-risc0/
// https://github.com/Zilliqa/zq2/blob/main/zilliqa/src/exec.rs#L152

// Transaction spec:
// https://docs.soliditylang.org/en/latest/abi-spec.html#formal-specification-of-the-encoding

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

pub struct EvmBackend {
    // Backend refers to storage, not execution platform
    state: BTreeMap<H160, CustomMemoryAccount>,
}

impl Backend for EvmBackend {
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

    fn storage(&self, address: H160, index: H256) -> H256 {
        unimplemented!()
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
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

#[derive(Clone, Debug)]
enum EvmArgument {
    Uint32(u32),
    Uint256(U256),
    // Address(Address),
    // Add more types as needed
}

fn pad_byte_array(mut bytes: Vec<u8>) -> Vec<u8> {
    let padding_size = 32 - bytes.len();
    let mut ret = vec![0; padding_size];
    ret.extend(bytes);
    ret
}

fn evm_argument_to_byte_array(arg: EvmArgument) -> Vec<u8> {
    match arg {
        EvmArgument::Uint32(value) => pad_byte_array(value.to_be_bytes().to_vec()),
        // EvmArgument::Uint256(value) => pad_byte_array(value.to_big_endian(/* &mut [u8] */).to_vec()),
        // TODO EvmArgument::Address(value) => pad_byte_array(value.as_bytes().to_vec()),
        // Handle other types here
        _ => panic!("Type conversion not implemented."),
    }
}

fn generate_transaction_data(function_signature: &str, args: Vec<EvmArgument>) -> Vec<u8> {
    let mut data = Vec::new();

    // Encode the function selector
    let function_selector = {
        use sha3::{Digest, Keccak256};
        // TODO: Filter out whitespaces from function signature
        let hash = Keccak256::digest(function_signature.as_bytes());

        let mut selector = Vec::new();
        selector.extend_from_slice(&hash[..4]);
        selector
    };
    data.extend(function_selector);

    // Encode the arguments
    // data.extend(args.len() as u64); // Encode the length of the arguments as a 256-bit integer
    for arg in args {
        data.extend(evm_argument_to_byte_array(arg));
    }

    data
}

#[cfg(test)]
mod tests {
    use crate::{generate_transaction_data, CustomMemoryAccount, EvmArgument, EvmBackend};
    // use evm::backend::MemoryAccount;
    // use evm::backend::MemoryBackend;
    use evm::executor::stack::MemoryStackState;
    use evm::executor::stack::StackSubstateMetadata;
    use std::str::FromStr;

    use evm::{backend::MemoryVicinity, executor::stack::StackExecutor, Config};
    use evm_assembly::EvmByteCodeBuilder;
    use primitive_types::{H160, U256};
    use std::collections::BTreeMap;
    #[test]
    fn blah() {
        let bytes = hex::decode("608060405234801561001057600080fd5b506004361061002b5760003560e01c8063783ce45814610030575b600080fd5b61004a600480360381019061004591906100e4565b610060565b6040516100579190610146565b60405180910390f35b600060405160208152602080820152602060408201528460608201528360808201528260a082015260208160c0836005615208fa61009d57600080fd5b80519150509392505050565b600080fd5b6000819050919050565b6100c1816100ae565b81146100cc57600080fd5b50565b6000813590506100de816100b8565b92915050565b6000806000606084860312156100fd576100fc6100a9565b5b600061010b868287016100cf565b935050602061011c868287016100cf565b925050604061012d868287016100cf565b9150509250925092565b610140816100ae565b82525050565b600060208201905061015b6000830184610137565b9291505056fea26469706673582212206008bacab5b8086d29fb94b94fb75a13869173c803877a325b2295e47d96358664736f6c63430008140033").unwrap();
        let builder = EvmByteCodeBuilder::from_bytes(bytes);

        let code = builder.build();

        // Small test
        let input =
            generate_transaction_data("fibonacci(uint256)", [EvmArgument::Uint32(10)].to_vec());
        println!("data: {:?}", hex::encode(input.clone()));
        assert!(
            input
                == hex::decode(
                    "61047ff4000000000000000000000000000000000000000000000000000000000000000a"
                )
                .unwrap()
        );

        // Define EVM configuration.
        let input = generate_transaction_data(
            "expmod(uint256,uint256,uint256)",
            [
                EvmArgument::Uint32(1),
                EvmArgument::Uint32(2),
                EvmArgument::Uint32(3),
            ]
            .to_vec(),
        );

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
            CustomMemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10000000),
                storage: BTreeMap::new(),
                code,
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
        let backend = EvmBackend { state }; //MemoryBackend::new(&vicinity, state);
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
            input,
            u64::MAX,
            Vec::new(),
        );

        println!("exit_reason: {:?}", exit_reason);
        println!("Finished: {:?}", result);
        // Make sure the execution succeeded.
        // assert!(exit_reason == ExitReason::Succeed(ExitSucceed::Returned));

        // Return hex encoded string.
        // hex::encode(result)
        assert!(false);
    }
}
