use evm_assembly::EvmByteCodeBuilder;
use std::collections::HashMap;

use evm::backend::{Backend, Basic};
use evm::executor::stack::PrecompileFn;
use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType};
use evm::{Context, ExitError, ExitSucceed};
use primitive_types::{H160, H256, U256};
use std::collections::BTreeMap;
use std::str::FromStr;

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

pub struct EvmIoInterface {
    // Backend refers to storage, not execution platform
    state: BTreeMap<H160, CustomMemoryAccount>,
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
pub enum EvmTypeValue {
    Uint32(u32),
    Uint256(U256),
    // Address(Address),
    // Add more types as needed
}

impl EvmTypeValue {
    fn pad_byte_array(mut bytes: Vec<u8>) -> Vec<u8> {
        let padding_size = 32 - bytes.len();
        let mut ret = vec![0; padding_size];
        ret.extend(bytes);
        ret
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            EvmTypeValue::Uint32(value) => Self::pad_byte_array(value.to_be_bytes().to_vec()),
            // EvmTypeValue::Uint256(value) => pad_byte_array(value.to_big_endian(/* &mut [u8] */).to_vec()),
            // TODO EvmTypeValue::Address(value) => pad_byte_array(value.as_bytes().to_vec()),
            // Handle other types here
            _ => panic!("Type conversion not implemented."),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EvmType {
    Uint(usize),
    Int(usize),
    Bytes(usize),
    Address,
    Bool,
    String,
}

impl EvmType {
    fn signature(&self) -> String {
        match self {
            EvmType::Uint(size) => format!("uint{}", size).to_string(),
            EvmType::Int(size) => format!("int{}", size).to_string(),
            EvmType::Bytes(size) => format!("bytes{}", size).to_string(),
            EvmType::Address => "address".to_string(),
            EvmType::Bool => "bool".to_string(),
            EvmType::String => "string".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvmFunctionSignature {
    name: String,
    arguments: Vec<EvmType>,
    return_type: EvmType,
}

impl EvmFunctionSignature {
    pub fn new(name: String, arguments: Vec<EvmType>, return_type: &EvmType) -> Self {
        Self {
            name,
            arguments,
            return_type: return_type.clone(),
        }
    }

    pub fn signature(&self) -> String {
        let mut argnames = Vec::new();
        for arg in &self.arguments {
            argnames.push(arg.signature());
        }

        format!("{}({})", self.name, argnames.join(",")).to_string()
    }

    pub fn full_signature(&self) -> String {
        let mut argnames = Vec::new();
        for arg in &self.arguments {
            argnames.push(arg.signature());
        }

        format!(
            "{}({})->{}",
            self.name,
            argnames.join(","),
            self.return_type.signature()
        )
        .to_string()
    }

    pub fn selector(&self) -> Vec<u8> {
        let signature = self.signature();
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(signature);

        let mut selector = Vec::new();
        selector.extend_from_slice(&hash[..4]);
        selector
    }

    pub fn generate_transaction_data(&self, args: Vec<EvmTypeValue>) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.selector());

        // Encode the arguments
        for arg in args {
            data.extend(arg.to_bytes());
        }

        data
    }
}

pub struct EvmExecutable {
    builder: EvmByteCodeBuilder,
}

struct EvmBackendSpecification {
    type_declarations: HashMap<String, EvmType>,
    function_declarations: HashMap<String, EvmFunctionSignature>,
    /// Scilla types -> EVM types
    precompiles: BTreeMap<H160, PrecompileFn>,
    contract_offset: usize,
}

impl EvmBackendSpecification {
    pub fn new() -> Self {
        Self {
            type_declarations: HashMap::new(),
            function_declarations: HashMap::new(),
            precompiles: BTreeMap::new(),
            contract_offset: 15,
        }
    }

    pub fn declare_integer(&mut self, name: &str, bits: usize) {
        assert!(bits <= 256);
        self.type_declarations
            .insert(name.to_string(), EvmType::Int(bits));
    }

    pub fn declare_unsigned_integer(&mut self, name: &str, bits: usize) {
        assert!(bits <= 256);
        self.type_declarations
            .insert(name.to_string(), EvmType::Uint(bits));
    }

    pub fn declare_address(&mut self, name: &str) {
        self.type_declarations
            .insert(name.to_string(), EvmType::String);
    }

    pub fn declare_function(&mut self, name: &str, arg_types: Vec<&str>, return_type: &str) {
        let return_type = self
            .type_declarations
            .get(return_type)
            .expect("Return type not found.");

        // Resolve argument types
        let arg_types: Vec<_> = arg_types
            .iter()
            .map(|&type_name| {
                self.type_declarations
                    .get(type_name)
                    .expect("Arg type not found.")
                    .clone()
            })
            .collect();

        let function_signature =
            EvmFunctionSignature::new(name.to_string(), arg_types, return_type);

        self.function_declarations
            .insert(name.to_string(), function_signature.clone());
    }

    pub fn get_function(&self, name: &str) -> Option<&EvmFunctionSignature> {
        self.function_declarations.get(name).clone()
    }
}

struct EvmExecutor<'a> {
    spec: &'a mut EvmBackendSpecification,
    // state: MemoryStackState,
}

impl<'a> EvmExecutor<'a> {}

pub(crate) fn test_precompile(
    input: &[u8],
    gas_limit: Option<u64>,
    _contex: &Context,
    _backend: &dyn Backend,
    _is_static: bool,
) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
    println!("Running precompile!");
    let gas_needed = match required_gas(input) {
        Ok(i) => i,
        Err(err) => return Err(PrecompileFailure::Error { exit_status: err }),
    };

    if let Some(gas_limit) = gas_limit {
        if gas_limit < gas_needed {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::OutOfGas,
            });
        }
    }

    Ok((
        PrecompileOutput {
            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
            output: input.to_vec(),
        },
        gas_needed,
    ))
}

fn required_gas(_input: &[u8]) -> Result<u64, ExitError> {
    Ok(20)
}

pub fn get_precompiles() -> BTreeMap<H160, PrecompileFn> {
    BTreeMap::from([(
        H160::from_str("0000000000000000000000000000000000000005").unwrap(),
        test_precompile as PrecompileFn,
    )])
}

#[cfg(test)]
mod tests {
    use crate::{
        get_precompiles, CustomMemoryAccount, EvmBackendSpecification, EvmExecutor, EvmIoInterface,
        EvmTypeValue,
    };
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
        let mut specification = EvmBackendSpecification::new();
        specification.declare_integer("Int8", 8);
        specification.declare_integer("Int16", 16);
        specification.declare_integer("Int32", 32);
        specification.declare_integer("Int64", 64);
        specification.declare_unsigned_integer("Uint8", 8);
        specification.declare_unsigned_integer("Uint16", 16);
        specification.declare_unsigned_integer("Uint32", 32);
        specification.declare_unsigned_integer("Uint64", 64);
        specification.declare_unsigned_integer("Uint256", 256);

        specification.declare_function("add", ["Int32", "Int32"].to_vec(), "Int32");

        specification.declare_function("fibonacci", ["Uint256"].to_vec(), "Uint256");
        specification.declare_function(
            "expmod",
            ["Uint256", "Uint256", "Uint256"].to_vec(),
            "Uint256",
        );
        ///////
        // Executable
        let bytes = hex::decode("608060405234801561001057600080fd5b50600436106100415760003560e01c806361047ff414610046578063771602f714610076578063783ce458146100a6575b600080fd5b610060600480360381019061005b91906101be565b6100d6565b60405161006d91906101fa565b60405180910390f35b610090600480360381019061008b9190610215565b610124565b60405161009d91906101fa565b60405180910390f35b6100c060048036038101906100bb9190610255565b61013a565b6040516100cd91906101fa565b60405180910390f35b6000600182116100e85781905061011f565b6100fd6002836100f891906102d7565b6100d6565b61011260018461010d91906102d7565b6100d6565b61011c919061030b565b90505b919050565b60008183610132919061030b565b905092915050565b600060405160208152602080820152602060408201528460608201528360808201528260a082015260208160c0836005615208fa61017757600080fd5b80519150509392505050565b600080fd5b6000819050919050565b61019b81610188565b81146101a657600080fd5b50565b6000813590506101b881610192565b92915050565b6000602082840312156101d4576101d3610183565b5b60006101e2848285016101a9565b91505092915050565b6101f481610188565b82525050565b600060208201905061020f60008301846101eb565b92915050565b6000806040838503121561022c5761022b610183565b5b600061023a858286016101a9565b925050602061024b858286016101a9565b9150509250929050565b60008060006060848603121561026e5761026d610183565b5b600061027c868287016101a9565b935050602061028d868287016101a9565b925050604061029e868287016101a9565b9150509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006102e282610188565b91506102ed83610188565b9250828203905081811115610305576103046102a8565b5b92915050565b600061031682610188565b915061032183610188565b9250828201905080821115610339576103386102a8565b5b9291505056fea26469706673582212208f6844dbbc3c1d9c61e27101822285ef53e3a6bd6cf0c6586fd944c00ad486cf64736f6c63430008140033").unwrap();
        let builder = EvmByteCodeBuilder::from_bytes(bytes);

        let code = builder.build();

        // Small test
        let input = specification
            .get_function("fibonacci")
            .expect("REASON")
            .generate_transaction_data([EvmTypeValue::Uint32(10)].to_vec());
        println!("data: {:?}", hex::encode(input.clone()));
        assert!(
            input
                == hex::decode(
                    "61047ff4000000000000000000000000000000000000000000000000000000000000000a"
                )
                .unwrap()
        );

        // Define EVM configuration.
        // Call
        let input = specification
            .get_function("expmod")
            .expect("REASON")
            .generate_transaction_data(
                [
                    EvmTypeValue::Uint32(1),
                    EvmTypeValue::Uint32(2),
                    EvmTypeValue::Uint32(3),
                ]
                .to_vec(),
            );
        println!("calldata: {:?}", hex::encode(input.clone()));

        ////

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
        let backend = EvmIoInterface { state }; //MemoryBackend::new(&vicinity, state);
        let metadata = StackSubstateMetadata::new(u64::MAX, &config);
        let state = MemoryStackState::new(metadata, &backend);
        let precompiles = get_precompiles();
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
