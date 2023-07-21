use crate::highlevel_ir::Operation;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, FunctionKind, HighlevelIr, IrLowering, Variant,
};
use evm_assembly::block::EvmBlock;
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executor::EvmExecutor;
use evm_assembly::types::EvmTypeValue;
use evm_assembly::EvmAssemblyGenerator;
use evm_assembly::EvmByteCodeBuilder;
use std::collections::HashMap;

type Scope<'a> = HashMap<String, inkwell::values::BasicValueEnum<'a>>;

pub struct EvmIrGenerator<'ctx> {
    context: &'ctx mut EvmCompilerContext,
    ir: Box<HighlevelIr>,
}

impl<'ctx> EvmIrGenerator<'ctx> {
    pub fn new(context: &'ctx mut EvmCompilerContext, ir: Box<HighlevelIr>) -> Self {
        Self { context, ir }
    }

    pub fn build_executable(&mut self) -> Result<Vec<u8>, String> {
        let mut builder = EvmByteCodeBuilder::new(&mut self.context);

        // TODO: From AST
        builder
            .define_function("TheContractPart::hello", ["Uint256"].to_vec(), "Uint256")
            .build(|code_builder| {
                let mut entry = EvmBlock::new(None, "entry");

                let fnc = code_builder.context.get_function("fibonacci").unwrap();
                entry.call(fnc, [EvmTypeValue::Uint32(10)].to_vec());

                entry.push1([1].to_vec());
                entry.jump_if_to("success");
                entry.jump_to("finally");

                let mut success = EvmBlock::new(None, "success");
                success.jump_to("finally");

                let mut finally = EvmBlock::new(None, "finally");

                finally.r#return();
                [entry, success, finally].to_vec()
            });

        println!("{}", builder.generate_evm_assembly());
        Ok(builder.build())
    }
}

impl<'ctx> IrLowering for EvmIrGenerator<'ctx> {
    fn lower_concrete_type(&mut self, con_type: &ConcreteType) {
        unimplemented!()
    }

    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction) {
        unimplemented!()
    }

    fn lower(&mut self, highlevel_ir: &HighlevelIr) {
        for con_type in &highlevel_ir.type_definitions {
            self.lower_concrete_type(con_type);
        }

        for con_function in &highlevel_ir.function_definitions {
            self.lower_concrete_function(con_function);
        }
    }
}
