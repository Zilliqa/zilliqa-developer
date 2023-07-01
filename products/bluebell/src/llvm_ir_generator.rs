use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, FunctionKind, HighlevelIr, IrLowering, Variant,
};
use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::{builder::Builder, context::Context};

pub struct LlvmIrGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    ir: Box<HighlevelIr>, // TODO: Add any members needed for the generation here
}

impl<'ctx> LlvmIrGenerator<'ctx> {
    pub fn get_type_definition(&self, name: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match name {
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
                                 /*{
                                     println!("Failed to convert {:?}", size);
                                     println!("- Type: {:?}", typevalue);
                                     unimplemented!()
                                 }*/
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
                ConcreteType::Tuple { name, data_layout } => {
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
                ConcreteType::Variant { name, data_layout } => {
                    self.build_variant(data_layout, &name.qualified_name()?);
                }
            }
        }

        Ok(0)
    }

    pub fn write_function_definitions_to_module(&mut self) -> Result<u32, String> {
        println!("Functions: {:#?}", self.ir.function_definitions);

        Ok(0)
    }

    pub fn new(context: &'ctx Context, ir: Box<HighlevelIr>) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");

        LlvmIrGenerator {
            context,
            builder,
            module,
            ir,
        }
    }
}

impl<'ctx> IrLowering for LlvmIrGenerator<'ctx> {
    // Lower a single concrete type from HighlevelIr to LLVM IR.
    fn lower_concrete_type(&mut self, con_type: &ConcreteType) {
        match con_type {
            ConcreteType::Tuple { name, data_layout } => {
                // provide functionality to handle tuple type
                unimplemented!()
            }
            ConcreteType::Variant { name, data_layout } => {
                // provide functionality to handle variant type
                unimplemented!()
            }
        }
    }
    // Lower a single concrete function from HighlevelIr to LLVM IR.
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction) {
        let func_name = &con_function
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
    fn lower(&mut self, highlevel_ir: &HighlevelIr) {
        for con_type in &highlevel_ir.type_definitions {
            self.lower_concrete_type(con_type);
        }
        for con_function in &highlevel_ir.function_definitions {
            self.lower_concrete_function(con_function);
        }
        // After lowering all elements, perform a final step.
    }
}
