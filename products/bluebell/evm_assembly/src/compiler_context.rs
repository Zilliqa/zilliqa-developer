use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use evm::executor::stack::PrecompileFn;
use primitive_types::H160;

use crate::{
    block::EvmBlock,
    evm_bytecode_builder::EvmByteCodeBuilder,
    function_signature::{AssemblyBuilderFn, EvmFunctionSignature},
    types::EvmType,
};

type InlineGenericsFn =
    fn(&mut EvmCompilerContext, &mut EvmBlock, Vec<String>) -> Result<Vec<EvmBlock>, String>;
type SpecialVariableFn =
    fn(&mut EvmCompilerContext, &mut EvmBlock) -> Result<Vec<EvmBlock>, String>;

pub struct EvmCompilerContext {
    pub raw_function_declarations: HashMap<String, (Vec<String>, String)>,

    pub type_declarations: HashMap<String, EvmType>,
    pub default_constructors: HashMap<String, AssemblyBuilderFn>,
    pub function_declarations: HashMap<String, EvmFunctionSignature>,
    pub inline_generics: HashMap<String, InlineGenericsFn>,
    pub special_variables: HashMap<String, SpecialVariableFn>,

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
            // Convert `value: u32` to a hexadecimal string, pad it with leading zeros to 40 characters, and then convert it to `H160`
            let padded_string = format!("{:0>40}", format!("{:x}", value)); // Pad with leading zeros to 40 characters
            H160::from_str(&padded_string).unwrap()
        };

        self.signature.external_address = Some(index);
        self.context
            .function_declarations
            .insert(name.clone(), self.signature.clone());

        self.context.precompiles.insert(address, precompiled);

        Ok(())
    }

    pub fn attach_assembly(&mut self, builder: AssemblyBuilderFn) -> Result<(), String> {
        let name = self.signature.name.clone();
        self.signature.inline_assembly_generator = Some(builder);
        self.context
            .function_declarations
            .insert(name.clone(), self.signature.clone());

        Ok(())
    }
}

impl EvmCompilerContext {
    pub fn new() -> Self {
        Self {
            raw_function_declarations: HashMap::new(),

            default_constructors: HashMap::new(),
            type_declarations: HashMap::new(),
            function_declarations: HashMap::new(),
            inline_generics: HashMap::new(),
            special_variables: HashMap::new(),
            precompile_addresses: HashMap::new(),
            precompiles: BTreeMap::new(),
            contract_offset: 5,
        }
    }

    pub fn create_builder<'ctx>(&'ctx mut self) -> EvmByteCodeBuilder<'ctx> {
        EvmByteCodeBuilder::new(self, true)
    }

    pub fn create_builder_no_abi_support<'ctx>(&'ctx mut self) -> EvmByteCodeBuilder<'ctx> {
        EvmByteCodeBuilder::new(self, false)
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

    pub fn declare_dynamic_string(&mut self, name: &str) {
        self.type_declarations
            .insert(name.to_string(), EvmType::String);
    }

    pub fn declare_opaque_type(&mut self, _name: &str) {
        unimplemented!()
        // self.type_declarations
        //     .insert(name.to_string(), EvmType::Opaque);
    }

    pub fn declare_special_variable(
        &mut self,
        name: &str,
        _typename: &str,
        builder: SpecialVariableFn,
    ) -> Result<(), String> {
        if self.special_variables.contains_key(name) {
            return Err(format!("Special variable {} already exists", name).to_string());
        }
        self.special_variables.insert(name.to_string(), builder);
        Ok(())
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

    pub fn declare_default_constructor(&mut self, name: &str, constructor: AssemblyBuilderFn) {
        self.default_constructors
            .insert(name.to_string(), constructor);
    }

    pub fn declare_function(
        &mut self,
        name: &str,
        arg_types: Vec<&str>,
        return_type: &str,
    ) -> EvmPrecompileBuilder {
        // TODO: check if the function already exists

        self.raw_function_declarations.insert(
            name.to_string(),
            (
                arg_types.iter().map(|x| x.to_string()).collect(),
                return_type.to_string(),
            ),
        );

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
