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
    types::{EvmType, UserType},
};

type InlineGenericsFn =
    fn(&mut EvmCompilerContext, &mut EvmBlock, Vec<String>) -> Result<Vec<EvmBlock>, String>;
type SpecialVariableFn =
    fn(&mut EvmCompilerContext, &mut EvmBlock) -> Result<Vec<EvmBlock>, String>;

pub struct GenericDeclaration {
    pub name: String,
    pub parameters: Vec<String>,
    pub layout: Vec<(String, String)>,
}

pub struct EvmCompilerContext {
    pub raw_function_declarations: HashMap<String, (Vec<String>, String)>,

    pub type_declarations: HashMap<String, EvmType>,
    pub default_constructors: HashMap<String, AssemblyBuilderFn>,
    pub function_declarations: HashMap<String, EvmFunctionSignature>,
    pub inline_generics: HashMap<String, InlineGenericsFn>,
    pub special_variables: HashMap<String, SpecialVariableFn>,

    pub user_types: HashMap<String, Box<UserType>>,
    pub generic_types: HashMap<String, GenericDeclaration>,

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

            user_types: HashMap::new(),
            generic_types: HashMap::new(),

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

    pub fn declare_user_struct(&mut self, name: &str, properties: Vec<(String, String)>) {
        let mut layout: Vec<(String, EvmType)> = Vec::new();
        for (field_name, type_name) in properties.iter() {
            let evm_type = self
                .type_declarations
                .get(type_name)
                .expect("Type not found")
                .clone();
            layout.push((field_name.clone(), evm_type));
        }

        // TODO: Add support for namespace
        let type_id = format!("user_struct::{}", name);

        let strct = UserType::Struct {
            type_id: type_id.clone(),
            layout,
        };

        self.user_types.insert(type_id, Box::new(strct));
    }

    pub fn declare_generic_type(
        &mut self,
        name: &str,
        parameters: Vec<String>,
        fields: Vec<(String, String)>,
    ) {
        let decl = GenericDeclaration {
            name: name.to_string(),
            parameters,
            layout: fields,
        };

        self.generic_types.insert(name.to_string(), decl);
    }

    pub fn instantiate_generic_type(&mut self, name: &str, actual_parameters: Vec<String>) {
        let generic_declaration = self
            .generic_types
            .get(name)
            .expect("Generic type not found");

        let mut parameter_map: HashMap<String, String> = HashMap::new();
        for (i, parameter) in generic_declaration.parameters.iter().enumerate() {
            parameter_map.insert(parameter.clone(), actual_parameters[i].clone());
        }

        let actual_layout: Vec<(String, String)> = generic_declaration
            .layout
            .iter()
            .map(|(field_name, type_id)| {
                (
                    field_name.clone(),
                    parameter_map.get(type_id).unwrap_or(type_id).clone(),
                )
            })
            .collect();

        let name = format!("{}::<{}>", name, actual_parameters.join(","));
        self.declare_user_struct(&name, actual_layout);
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
