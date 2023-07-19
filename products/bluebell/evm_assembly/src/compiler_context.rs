use crate::function_signature::EvmFunctionSignature;
use crate::types::EvmType;
use evm::executor::stack::PrecompileFn;
use primitive_types::H160;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct EvmCompilerContext {
    type_declarations: HashMap<String, EvmType>,
    function_declarations: HashMap<String, EvmFunctionSignature>,
    /// Scilla types -> EVM types
    precompiles: BTreeMap<H160, PrecompileFn>,
    contract_offset: usize,
}

impl EvmCompilerContext {
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
