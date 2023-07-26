use crate::block::EvmBlock;
use crate::evm_bytecode_builder::EvmByteCodeBuilder;
use crate::function_signature::EvmFunctionSignature;
use crate::types::EvmType;
use evm::executor::stack::PrecompileFn;
use primitive_types::H160;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;

type InlineGenericsFn = fn(&mut EvmBlock, Vec<String>) -> Result<(), String>;

pub struct EvmCompilerContext {
    type_declarations: HashMap<String, EvmType>,

    pub function_declarations: HashMap<String, EvmFunctionSignature>,
    pub inline_generics: HashMap<String, InlineGenericsFn>,
    /// Scilla types -> EVM types
    precompiles: BTreeMap<H160, PrecompileFn>,
    precompile_addresses: HashMap<String, u32>,
    contract_offset: u32,
}

pub struct EvmPrecompileBuilder<'a> {
    context: &'a mut EvmCompilerContext,
    pub signature: EvmFunctionSignature,
}

impl<'a> EvmPrecompileBuilder<'a> {
    pub fn attach_runtime<F>(&mut self, get_precompile: F) -> Result<(), String>
    where
        F: FnOnce() -> PrecompileFn,
    {
        let precompiled = get_precompile();
        let name = self.signature.name.clone();

        if self.context.precompile_addresses.contains_key(&name) {
            return Err(format!("Runtime function '{}' already exists.", name).to_string());
        }

        let index = self.context.contract_offset;
        self.context.contract_offset += 1;

        let address = {
            let value = index;
            let padded_string = format!("{:0>40}", value.to_string()); // Pad with leading zeros to 40 characters
            H160::from_str(&padded_string).unwrap()
        };

        self.signature.external_address = Some(index);
        self.context
            .function_declarations
            .insert(name.clone(), self.signature.clone());

        self.context.precompiles.insert(address, precompiled);

        Ok(())
    }
}

impl EvmCompilerContext {
    pub fn new() -> Self {
        Self {
            type_declarations: HashMap::new(),
            function_declarations: HashMap::new(),
            inline_generics: HashMap::new(),
            precompile_addresses: HashMap::new(),
            precompiles: BTreeMap::new(),
            contract_offset: 5,
        }
    }

    pub fn create_builder<'ctx>(&'ctx mut self) -> EvmByteCodeBuilder<'ctx> {
        EvmByteCodeBuilder::new(self)
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

    pub fn declare_inline_generics(
        &mut self,
        name: &str,
        builder: InlineGenericsFn,
    ) -> Result<(), String> {
        if self.inline_generics.contains_key(name) {
            return Err(format!("Geneic {} already exists", name).to_string());
        }
        self.inline_generics.insert(name.to_string(), builder);
        Ok(())
    }

    pub fn declare_function(
        &mut self,
        name: &str,
        arg_types: Vec<&str>,
        return_type: &str,
    ) -> EvmPrecompileBuilder {
        // TODO: check if the function already exists

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

        EvmPrecompileBuilder {
            context: self,
            signature: function_signature,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&EvmFunctionSignature> {
        self.function_declarations.get(name).clone()
    }

    pub fn get_precompiles(&self) -> BTreeMap<H160, PrecompileFn> {
        self.precompiles.clone()
    }
}
