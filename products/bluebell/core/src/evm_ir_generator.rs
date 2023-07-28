use crate::intermediate_representation::highlevel_ir::Operation;
use crate::intermediate_representation::highlevel_ir::{
    ConcreteFunction, ConcreteType, HighlevelIr, IrLowering,
};
use evm_assembly::block::EvmBlock;
use evm_assembly::compiler_context::EvmCompilerContext;

use evm_assembly::types::EvmTypeValue;
use evm_assembly::EvmAssemblyGenerator;
use evm_assembly::EvmByteCodeBuilder;

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

                    // Return PC + Arguments are expected to be on the stack

                    for block in &func.body.blocks {
                        let block_name = match block.name.qualified_name() {
                            Ok(b) => b,
                            Err(_) => panic!("Failed to get qualified name."),
                        };
                        let mut blk = EvmBlock::new(None, &block_name);

                        println!("Block: {:#?}", block);
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
                                    // We have three types of calls:
                                    // - Precompiles / external function
                                    // - Inline assembler generics
                                    // - Internal calls

                                    if ctx.function_declarations.contains_key(qualified_name) {
                                        let signature = match ctx.get_function(qualified_name) {
                                            Some(s) => s,
                                            None => panic!(
                                                "Internal error: Unable to retrieve function"
                                            ),
                                        };
                                        let mut args: Vec<String> = Vec::new();
                                        println!("Resolving arguments {:?}", arguments);
                                        for arg in arguments {
                                            match &arg.resolved {
                                                Some(n) => args.push(n.to_string()),
                                                None => panic!("Argument name was not resolved"),
                                            }
                                        }
                                        // Precompiled or external function
                                        blk.call(signature, args);
                                    } else if ctx.inline_generics.contains_key(&name.unresolved) {
                                        // Copying arguments
                                        for arg in arguments {
                                            match &arg.resolved {
                                                Some(n) => match blk.duplicate_stack_name(n) {
                                                    Err(e) => panic!("{}", e),
                                                    _ => (),
                                                },
                                                None => panic!("Argument name was not resolved"),
                                            }
                                        }

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
                                            code_builder.data.push((ssa_name, payload));
                                            todo!()
                                        }
                                        "Uint64" => {
                                            let value = EvmTypeValue::Uint64(data.parse().unwrap());
                                            blk.push(value.to_bytes_unpadded());
                                            match blk.register_stack_name(ssa_name) {
                                                Err(_) => {
                                                    panic!("Failed to register SSA stack name.")
                                                }
                                                _ => (),
                                            }
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
                                Operation::Return(ref value) => {
                                    match value {
                                        Some(_value) => {
                                            todo!();
                                            // TODO: write return to the stack
                                            // blk.r#return();
                                        }
                                        None => {
                                            blk.push([0x00].to_vec());
                                            blk.dup1();
                                            blk.r#return();
                                        }
                                    }
                                }
                                _ => {
                                    println!("Unhandled instruction: {:#?}", instr);
                                    unimplemented!() // Add handling for other operations here
                                }
                            }
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
