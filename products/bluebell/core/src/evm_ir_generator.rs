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
    //context: &'ctx mut EvmCompilerContext,
    builder: EvmByteCodeBuilder<'ctx>,
    ir: Box<HighlevelIr>,
}

impl<'ctx> EvmIrGenerator<'ctx> {
    pub fn new(context: &'ctx mut EvmCompilerContext, ir: Box<HighlevelIr>) -> Self {
        let builder = context.create_builder();
        Self { builder, ir }
    }

    pub fn write_function_definitions_to_module(&mut self) -> Result<u32, String> {
        for func in &self.ir.function_definitions {
            /*
                        let arg_types: Vec<_> = func
                            .arguments
                            .iter()
                            .map(|arg| &arg.typename.unresolved)
                            .collect();
            */

            let arg_types: Vec<&str> = func
                .arguments
                .iter()
                .map(|arg| arg.typename.unresolved.as_str())
                .collect();

            let function_name = func
                .name
                .qualified_name()
                .unwrap_or(func.name.unresolved.clone());

            let return_type = match func.return_type.as_ref() {
                Some(return_type) => return_type.as_str(),
                None => "Uint256", // TODO: panic!("Void type not implemented for EVM")
            };

            self.builder
                .define_function(&function_name, arg_types, return_type)
                .build(|code_builder| {
                    let mut ret: Vec<EvmBlock> = Vec::new();

                    // TODO: deal with arguments
                    for block in &func.body.blocks {
                        let block_name = match block.name.qualified_name() {
                            Ok(b) => b,
                            Err(_) => panic!("Failed to get qualified name."),
                        };
                        let mut blk = EvmBlock::new(None, &block_name);

                        for instr in &block.instructions {
                            match instr.operation {
                                Operation::CallExternalFunction {
                                    ref name,
                                    ref arguments,
                                } => {
                                    let _ = name;
                                    let _ = arguments;
                                    println!("\n");
                                    println!("Argumnets: {:?}", arguments);
                                    // Copying arguments

                                    // Invoking
                                    let qualified_name = match &name.resolved {
                                        Some(n) => n,
                                        None => {
                                            // TODO: Fix error propagation
                                            panic!(
                                                "{}",
                                                format!(
                                                    "Encountered unresolved function name {}",
                                                    name.unresolved
                                                )
                                            )
                                        }
                                    };

                                    let ctx = &code_builder.context;
                                    if ctx.function_declarations.contains_key(qualified_name) {
                                        // Ordinary function
                                        unimplemented!()
                                    } else if ctx.inline_generics.contains_key(&name.unresolved) {
                                        // TODO: This ought to be the resovled name, but it should be resovled without instance parameters - make a or update pass
                                        // Builtin assembly generator
                                        let f = ctx.inline_generics.get(&name.unresolved).unwrap();
                                        let args: Vec<String> = arguments
                                            .iter()
                                            .map(|arg| arg.resolved.clone().unwrap())
                                            .collect();
                                        match f(&mut blk, args) {
                                            Ok(v) => v,
                                            Err(e) => {
                                                panic!("Error in external call: {}", e);
                                            }
                                        }
                                    } else {
                                        panic!("{}", format!("{} not found.", qualified_name));
                                    }
                                }

                                Operation::Literal {
                                    ref data,
                                    ref typename,
                                } => {
                                    let qualified_name = match typename.qualified_name() {
                                        Ok(v) => v,
                                        _ => panic!("Qualified name could not be resolved"),
                                    };
                                    println!("\nCreating literal: {:?}", instr);
                                    let ssa_name = match &instr.ssa_name {
                                        Some(v) => match &v.resolved {
                                            Some(x) => x,
                                            _ => panic!("Literal symbol name was unresolved."),
                                        },
                                        _ => panic!("Literals with no SSA name are not supported"),
                                    };

                                    match qualified_name.as_str() {
                                        "String" => {
                                            let ssa_name = match instr
                                                .ssa_name
                                                .clone()
                                                .unwrap()
                                                .qualified_name()
                                            {
                                                Ok(v) => v,
                                                _ => panic!("Could not resolve SSA qualified name"),
                                            };
                                            let payload = data.clone().into_bytes();
                                            println!(
                                                "{}: {}",
                                                ssa_name,
                                                hex::encode(payload.clone())
                                            );
                                            code_builder.data.push((ssa_name, payload));
                                        }
                                        "Uint64" => {
                                            let value = EvmTypeValue::Uint64(data.parse().unwrap());
                                            blk.push(value.to_bytes_unpadded());
                                        }
                                        // TODO: add cases for other types of literals here if needed
                                        _ => {
                                            panic!(
                                                "{}",
                                                format!(
                                                    "Unhandled literal type: {:?}",
                                                    typename.qualified_name()
                                                )
                                            );
                                        }
                                    }
                                }

                                _ => {
                                    println!("Unhandled instruction: {:#?}", instr);
                                    unimplemented!() // Add handling for other operations here
                                }
                            }
                            println!("{:?}", instr);
                        }

                        ret.push(blk);
                    }

                    /*
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
                    */
                    ret // [entry].to_vec()
                });
        }

        Ok(0)
    }

    pub fn build_executable(&mut self) -> Result<Vec<u8>, String> {
        // self.write_type_definitions_to_module()?;
        self.write_function_definitions_to_module()?;

        self.builder.finalize_blocks();
        println!("{}", self.builder.generate_evm_assembly());
        Ok(self.builder.build())
    }
}

impl<'ctx> IrLowering for EvmIrGenerator<'ctx> {
    fn lower_concrete_type(&mut self, _con_type: &ConcreteType) {
        unimplemented!()
    }

    fn lower_concrete_function(&mut self, _con_function: &ConcreteFunction) {
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
