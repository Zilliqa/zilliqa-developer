use crate::intermediate_representation::primitives::Operation;
use crate::intermediate_representation::primitives::{
    ConcreteFunction, ConcreteType, FunctionKind, IntermediateRepresentation, IrLowering, Variant,
};
use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::{builder::Builder, context::Context};
use std::collections::HashMap;

type Scope<'a> = HashMap<String, inkwell::values::BasicValueEnum<'a>>;

/// `LlvmIrGenerator` is a structure encapsulating the generation of LLVM Intermediate Representation (IR)
/// from a higher-level IR. It holds references to the LLVM context and module, a builder for generating
/// instructions, the high-level IR to be lowered, and a scope stack for managing variables.
pub struct LlvmIrGenerator<'ctx, 'module> {
    /// `context` is the LLVM context, which holds any LLVM-specific configuration and state.
    context: &'ctx Context,

    /// `builder` is an LLVM builder, which is a helper object that makes it easy to generate LLVM instructions.
    builder: Builder<'ctx>,

    /// `module` is the LLVM module, which is the top level container of all LLVM Intermediate Representation
    /// objects. Each module contains a list of global variables, functions, libraries for dynamic linking, etc.
    module: &'module mut Module<'ctx>,

    /// `ir` is the high-level intermediate representation of the program, stored as a boxed value.
    ir: Box<IntermediateRepresentation>,

    /// `scopes` is a stack of scopes, with each scope containing information about variable bindings in that
    /// scope. This allows us to keep track of variables and their values.
    scopes: Vec<Scope<'ctx>>,
}

impl<'ctx, 'module> LlvmIrGenerator<'ctx, 'module> {
    /// This function builds an LLVM module from the stored high-level intermediate representation.
    /// It translates the type and function definitions in the high-level IR to corresponding constructs
    /// in the LLVM IR.
    pub fn build_module(&mut self) -> Result<u32, String> {
        self.write_type_definitions_to_module()?;
        self.write_function_definitions_to_module()?;

        Ok(0)
    }

    /// This constructs a new `LlvmIrGenerator` with an LLVM context, a high-level intermediate representation,
    /// and an LLVM module. It also creates a new LLVM builder, initializes a scope stack with one empty scope.
    pub fn new(
        context: &'ctx Context,
        ir: Box<IntermediateRepresentation>,
        module: &'module mut Module<'ctx>,
    ) -> Self {
        let builder = context.create_builder();
        let mut scopes = Vec::new();
        scopes.push(Scope::new());
        LlvmIrGenerator {
            context,
            builder,
            module,
            ir,
            scopes,
        }
    }

    pub fn get_type_definition(&self, name: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match name {
            // TODO: Collect this into a single module
            "Uint8" => Ok(self.context.i8_type().into()),
            "Uint16" => Ok(self.context.i16_type().into()),
            "Uint32" => Ok(self.context.i32_type().into()),
            "Uint64" => Ok(self.context.i64_type().into()),
            "Int8" => Ok(self.context.i8_type().into()),
            "Int16" => Ok(self.context.i16_type().into()),
            "Int32" => Ok(self.context.i32_type().into()),
            "Int64" => Ok(self.context.i64_type().into()),
            // "Unit" => Ok(self.context.void_type().into()),
            _ => {
                // Get the struct type from the module if it exists
                let val = self.module.get_struct_type(name);
                match val {
                    Some(val) => Ok(BasicTypeEnum::StructType(val)),
                    None => Err(format!("Type '{}' not defined.", name)),
                }
            }
        }
    }

    pub fn get_constructor_name(&self, name: &String) -> String {
        format!("_construct_{}", name).to_string()
    }

    pub fn get_enum_constructor_name(&self, enum_name: &String, name: &String) -> String {
        format!("{}_construct_{}", enum_name, name).to_string()
    }

    fn build_variant(
        &self,
        data_layout: &Variant,
        name: &str,
    ) -> Result<AnyTypeEnum<'ctx>, String> {
        let mut variant_fields = Vec::new();
        let mut data_size = 0;
        for field in &data_layout.fields {
            if let Some(typename) = field.data.as_ref() {
                let typename = typename.qualified_name()?;
                let typevalue = self.get_type_definition(&typename).unwrap();
                let size = match typevalue.size_of() {
                    Some(s) => s,
                    _ => unimplemented!(), // Add the remaining enums as per your implementation.
                };
                let size = match size.get_sign_extended_constant() {
                    Some(s) => s,
                    None => 100, // TODO: This needs fixing for structs - get the size
                };
                if size > data_size {
                    data_size = size;
                }
                variant_fields.push(typevalue);
            }
        }

        // Creating Tag Type
        let data_size = data_size as u32;
        let max_tag_value = data_layout.fields.len() as u32;
        let required_bits = 32 - max_tag_value.leading_zeros();
        let tag_type = self.context.custom_width_int_type(required_bits);
        // tag_type.set_name("TagType");

        // Type erased container
        let i8_type = self.context.i8_type();
        let variant_struct_type = self.context.opaque_struct_type(name);

        let type_erased_container = i8_type.array_type(data_size);
        if data_size == 0 {
            variant_struct_type.set_body(&[tag_type.into()], false);
        } else {
            variant_struct_type.set_body(&[tag_type.into(), type_erased_container.into()], false);
        }

        for (i, field) in data_layout.fields.iter().enumerate() {
            let constructor_name =
                self.get_enum_constructor_name(&name.to_string(), &field.name.qualified_name()?);

            let (func_val, concrete_field_type) = match &field.data {
                None => {
                    // Creating constructor
                    let func_type = variant_struct_type.fn_type(&[], false);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);

                    (func_val, variant_struct_type)
                }
                Some(typename) => {
                    // Enum value with associated data
                    let inner_value_type = self
                        .get_type_definition(&typename.qualified_name()?)
                        .unwrap();

                    // Defining the concrete type
                    let concrete_field_type = self.context.opaque_struct_type(&format!(
                        "{}_{}",
                        name,
                        field.name.qualified_name()?
                    ));
                    concrete_field_type
                        .set_body(&[tag_type.into(), inner_value_type.into()], false);

                    // TODO: Add padding if the type is less then max value

                    // Creating constructor
                    let func_type = variant_struct_type.fn_type(&[inner_value_type.into()], false);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);

                    (func_val, concrete_field_type)
                }
            };

            // Allocating the contrete type and populating its fields
            let ret_value = self.builder.build_alloca(
                concrete_field_type,
                &format!("{}_value", field.name.qualified_name()?.to_lowercase()),
            );

            if data_size != 0 {
                if let Some(data_value) = func_val.get_param_iter().last() {
                    let data_ptr = self
                        .builder
                        .build_struct_gep(concrete_field_type, ret_value, 1, "data_ptr")
                        .unwrap();
                    self.builder.build_store(data_ptr, data_value);
                } else {
                    // TODO: Store zeros in concrete field type to avoid uninitalised memory?
                }
            }

            // Getting the pointers to storage
            let id_ptr = self
                .builder
                .build_struct_gep(concrete_field_type, ret_value, 0, "id")
                .unwrap();
            self.builder
                .build_store(id_ptr, self.context.i32_type().const_int(i as u64, false));

            // Converting the concrete value to a type-erased version
            let erased_ret =
                self.builder
                    .build_bitcast(ret_value, variant_struct_type, "erased_ret");
            self.builder.build_return(Some(&erased_ret));
        }
        Ok(variant_struct_type.into())
    }

    pub fn write_type_definitions_to_module(&mut self) -> Result<u32, String> {
        for concrete_type in self.ir.type_definitions.iter() {
            match concrete_type {
                ConcreteType::Tuple {
                    name,
                    data_layout,
                    namespace: _,
                } => {
                    let mut field_types = Vec::new();
                    for field in &data_layout.fields {
                        let field_type = self.get_type_definition(&field.qualified_name()?)?;
                        field_types.push(field_type);
                    }

                    let tuple_struct_type =
                        self.context.opaque_struct_type(&name.qualified_name()?);
                    tuple_struct_type.set_body(&field_types[..], false);
                    // let tuple_struct_type = self.context.struct_type(&field_types[..], false);

                    let func_type = tuple_struct_type.fn_type(
                        &field_types
                            .into_iter()
                            .map(|t| t.into())
                            .collect::<Vec<_>>(),
                        false,
                    );

                    // TODO: Clean up: Streamline internal naming here
                    let constructor_name = self.get_constructor_name(&name.qualified_name()?);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);
                    let struct_val = self
                        .builder
                        .build_alloca(tuple_struct_type, "struct_alloca");
                    for (index, param) in func_val.get_param_iter().enumerate() {
                        let ptr = self
                            .builder
                            .build_struct_gep(
                                tuple_struct_type,
                                struct_val,
                                index as u32,
                                &format!("field{}", index),
                            )
                            .unwrap();
                        self.builder.build_store(ptr, param);
                    }

                    self.builder.build_return(Some(&struct_val));
                }
                ConcreteType::Variant {
                    name,
                    data_layout,
                    namespace: _,
                } => {
                    let _ = self.build_variant(data_layout, &name.qualified_name()?);
                }
            }
        }

        Ok(0)
    }

    pub fn write_function_definitions_to_module(&mut self) -> Result<u32, String> {
        for func in &self.ir.function_definitions {
            let _scope = Scope::new();

            let arg_types: Vec<_> = func
                .arguments
                .iter()
                .map(|arg| self.get_type_definition(&arg.typename.unresolved).unwrap())
                .collect();
            let func_type = match func.return_type.as_ref() {
                Some(return_type) => {
                    let return_type = self.get_type_definition(&return_type).unwrap();
                    return_type.fn_type(
                        &arg_types
                            .clone()
                            .into_iter()
                            .map(|t| t.into())
                            .collect::<Vec<_>>(),
                        false,
                    )
                }
                None => self.context.void_type().fn_type(
                    &arg_types
                        .clone()
                        .into_iter()
                        .map(|t| t.into())
                        .collect::<Vec<_>>(),
                    false,
                ),
            };
            let function = self.module.add_function(
                &func
                    .name
                    .qualified_name()
                    .unwrap_or(func.name.unresolved.clone()),
                func_type,
                None,
            );
            for (i, param) in function.get_param_iter().enumerate() {
                param.set_name(&func.arguments[i].name.unresolved);
            }
            let basic_block = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(basic_block);
            for (i, arg) in func.arguments.iter().enumerate() {
                let alloca = self
                    .builder
                    .build_alloca(arg_types[i], &arg.name.unresolved);
                self.builder
                    .build_store(alloca, function.get_nth_param(i as u32).unwrap());
            }
            let last_instruction_was_return = false;
            for instr in func.body.blocks.iter().flat_map(|x| x.instructions.iter()) {
                match instr.operation {
                    // Add handling for all operations defined in the Operation enum here.
                    // For now, we only implement the CallExternalFunction Operation.
                    Operation::CallExternalFunction {
                        ref name,
                        ref arguments,
                    } => {
                        // Get function value from module
                        let function_name = match &name.resolved {
                            Some(n) => n,
                            None => {
                                // return Err(format!("Encountered unresolved function name {}", name.unresolved))
                                // TODO: Fix this
                                &name.unresolved
                            }
                        };
                        let called_function = self.module.get_function(&function_name);
                        let called_function = match called_function {
                            Some(called_function) => called_function,
                            None => {
                                return Err(format!("Unable to find function {}", function_name));
                            }
                        };

                        let argument_values: Vec<_> = arguments
                            .iter()
                            .map(|arg| {
                                // Assuming all arguments are defined in the current function
                                let name = match &arg.resolved {
                                    Some(n) => n,
                                    None => {
                                        // TODO: Properly propagate error message
                                        panic!("Encountered unresolved symbol {}", arg.unresolved);
                                    }
                                };
                                self.scopes
                                    .iter()
                                    .rev()
                                    .filter_map(|s| s.get(name))
                                    .next()
                                    .expect(&format!("Failed to find {}", name))
                            })
                            .collect();
                        let _ = self.builder.build_call(
                            called_function,
                            &argument_values
                                .into_iter()
                                .map(|t| (*t).into())
                                .collect::<Vec<_>>(),
                            "calltmp",
                        );
                    }
                    Operation::Literal {
                        ref data,
                        ref typename,
                    } => {
                        match typename.qualified_name()?.as_str() {
                            "String" => {
                                let ssa_name = instr.ssa_name.clone().unwrap().qualified_name()?;
                                let string_type =
                                    self.context.i8_type().array_type(data.len() as u32);
                                let global_string =
                                    self.module
                                        .add_global(string_type, None, &ssa_name.as_str());
                                global_string.set_initializer(
                                    &self.context.const_string(data.as_bytes(), false),
                                );
                                if let Some(scope) = self.scopes.last_mut() {
                                    let pointer_val = global_string.as_pointer_value();
                                    scope.insert(ssa_name, pointer_val.into());
                                }
                            }
                            // TODO: add cases for other types of literals here if needed
                            _ => {
                                return Err(format!(
                                    "Unhandled literal type: {:?}",
                                    typename.qualified_name()?
                                ));
                            }
                        }
                    }
                    _ => {
                        println!("Unhandled instruction: {:#?}", instr);
                        unimplemented!() // Add handling for other operations here
                    }
                }
            }
            if !last_instruction_was_return {
                self.builder.build_return(None);
            };
        }
        Ok(0)
    }
}

impl<'ctx, 'module> IrLowering for LlvmIrGenerator<'ctx, 'module> {
    // Lower a single concrete type from IntermediateRepresentation to LLVM IR.
    fn lower_concrete_type(&mut self, con_type: &ConcreteType) {
        match con_type {
            ConcreteType::Tuple {
                name: _,
                data_layout: _,
                namespace: _,
            } => {
                // provide functionality to handle tuple type
                unimplemented!()
            }
            ConcreteType::Variant {
                name: _,
                data_layout: _,
                namespace: _,
            } => {
                // provide functionality to handle variant type
                unimplemented!()
            }
        }
    }
    // Lower a single concrete function from IntermediateRepresentation to LLVM IR.
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction) {
        let _func_name = &con_function
            .name
            .resolved
            .as_ref()
            .unwrap_or(&con_function.name.unresolved);
        match con_function.function_kind {
            FunctionKind::Procedure => {
                // provide functionality for procedure kind
                unimplemented!()
            }
            FunctionKind::Transition => {
                // provide functionality for Transition kind
                unimplemented!()
            }
            FunctionKind::Function => {
                // provide functionality for function kind
                unimplemented!()
            }
        }
    }

    // Lower the entire HighLevelIr to LLVM IR.
    fn lower(&mut self, primitives: &IntermediateRepresentation) {
        for con_type in &primitives.type_definitions {
            self.lower_concrete_type(con_type);
        }
        for con_function in &primitives.function_definitions {
            self.lower_concrete_function(con_function);
        }
        // After lowering all elements, perform a final step.
    }
}
