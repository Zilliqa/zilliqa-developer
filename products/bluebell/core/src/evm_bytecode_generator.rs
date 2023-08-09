use crate::intermediate_representation::primitives::Operation;
use crate::intermediate_representation::primitives::{
    ConcreteFunction, ConcreteType, IntermediateRepresentation, IrLowering,
};
use evm_assembly::block::EvmBlock;
use evm_assembly::compiler_context::EvmCompilerContext;
use primitive_types::U256;
use std::collections::HashMap;

use evm_assembly::types::EvmTypeValue;
use evm_assembly::EvmAssemblyGenerator;
use evm_assembly::EvmByteCodeBuilder;
use std::mem;

#[derive(Debug)]
pub struct StateLayoutEntry {
    pub address_offset: U256,
    pub size: u64,
    pub initializer: U256,
}

/// `EvmBytecodeGenerator` is a structure responsible for generating Ethereum Virtual Machine (EVM) bytecode.
/// It stores an EVM bytecode builder and an intermediate representation (IR) of the program to be compiled.
///
/// 'ctx lifetime marker is tied with `EvmByteCodeBuilder`'s lifetime which represents the lifetime of the EvmCompilerContext.
pub struct EvmBytecodeGenerator<'ctx> {
    /// `builder` is an instance of `EvmByteCodeBuilder` that provides methods to construct
    /// EVM bytecode sequentially. The lifetime of this builder should not outlive the context 'ctx.
    builder: EvmByteCodeBuilder<'ctx>,

    /// `ir` is an intermediate representation (IR) of the smart contract code. It's a
    /// high-level, platform-independent representation used for code optimization before
    /// it's translated into the target bytecode.
    ir: Box<IntermediateRepresentation>,

    // TODO: State allocation - TODO: move to IR and setup using pass
    state_layout: HashMap<String, StateLayoutEntry>,
}

impl<'ctx> EvmBytecodeGenerator<'ctx> {
    /// This constructs a new `EvmBytecodeGenerator`. It takes an existing EVM compiler context
    /// and a boxed intermediate representation (IR) of the program.  
    pub fn new(context: &'ctx mut EvmCompilerContext, ir: Box<IntermediateRepresentation>) -> Self {
        let builder = context.create_builder();
        Self {
            builder,
            ir,
            state_layout: HashMap::new(),
        }
    }

    /// TODO:
    pub fn build_state_layout(&mut self) -> Result<(), String> {
        // TODO: Add support for immutables
        let mut address_offset: u64 = 4919;

        for field in &self.ir.fields_definitions {
            let name = &field.variable.name.unresolved;
            let address = U256::from(address_offset);
            let initializer = U256::from(0);

            let state = StateLayoutEntry {
                address_offset: address,
                size: 1, // TODO:
                initializer,
            };

            self.state_layout.insert(name.to_string(), state);
            address_offset += 1;
        }
        println!("State: {:#?}", self.state_layout);
        Ok(())
        //        unimplemented!()
    }

    /// This function writes function definitions from the IR to the EVM module.
    /// It loops over all function definitions in the IR and creates corresponding function definitions
    /// in the EVM module using the byte code builder.
    pub fn write_function_definitions_to_module(&mut self) -> Result<u32, String> {
        for func in &self.ir.function_definitions {
            let arg_types: Vec<&str> = func
                .arguments
                .iter()
                .map(|arg| arg.typename.unresolved.as_str())
                .collect();

            let arg_names: Vec<String> = func
                .arguments
                .iter()
                .map(|arg| {
                    arg.name
                        .resolved
                        .clone()
                        .expect("Unresolved function argument name")
                })
                .collect();

            let function_name = func
                .name
                .qualified_name()
                .unwrap_or(func.name.unresolved.clone());

            let return_type = match func.return_type.as_ref() {
                Some(return_type) => return_type.as_str(),
                None => "Uint256", // TODO: panic!("Void type not implemented for EVM")
            };

            println!("Writing function {:#?}", function_name);

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

                        // Creating entry function
                        let mut blk = EvmBlock::new(None, arg_names.clone(), &block_name);

                        for instr in &block.instructions {
                            match instr.operation {
                                Operation::CallFunction {
                                    ref name,
                                    ref arguments,
                                } => {
                                    println!("Calling {:#?} -- {:#?}", name, arguments);
                                    let mut exit_block = EvmBlock::new(
                                        Some(0),
                                        ["result".to_string()].to_vec(),
                                        "exit_block",
                                    );

                                    // Adding return point
                                    blk.push_label("exit_block");

                                    for arg in arguments {
                                        match &arg.resolved {
                                            Some(a) => blk.duplicate_stack_name(&a),
                                            None => panic!("Unable to resolve {}", arg.unresolved),
                                        };
                                    }

                                    // Jumping to function
                                    let label = match &name.resolved {
                                        Some(v) => v,
                                        None => panic!(
                                            "Unresolved function name in function call {:?}",
                                            name
                                        ),
                                    };
                                    blk.jump_to(&label);
                                    mem::swap(&mut blk, &mut exit_block);
                                    ret.push(exit_block);
                                }
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
                                Operation::ResolveSymbol { ref symbol } => {
                                    let source = match &symbol.resolved {
                                        Some(v) => v,
                                        None => panic!("Unresolved symbol: {:?}", symbol),
                                    };
                                    let dest = match &instr.ssa_name {
                                        Some(v) => match &v.resolved {
                                            Some(x) => x,
                                            _ => panic!("Alias symbol name was unresolved."),
                                        },
                                        _ => panic!("Alias with no SSA name are not supported"),
                                    };

                                    if let Err(e) = blk.register_alias(source, dest) {
                                        panic!("Failed registering alias: {:?}", e);
                                    }
                                }
                                Operation::StateStore {
                                    ref address,
                                    ref value,
                                } => {
                                    // TODO: Ensure that we used resolved address name
                                    let binding = &self.state_layout.get(&address.name.unresolved);
                                    let state = match binding {
                                        Some(v) => v,
                                        None => panic!(
                                            "{}",
                                            format!(
                                                "Unable to find state {}",
                                                address.name.unresolved
                                            )
                                        ),
                                    };

                                    let address = state.address_offset;

                                    println!("Storing {:?} on {:?}", value, address);
                                    let value_name = match &value.resolved {
                                        Some(v) => v,
                                        None => {
                                            panic!("{}", format!("Unable to resolve {:?}", value))
                                        }
                                    };

                                    if let Err(e) = blk.duplicate_stack_name(value_name) {
                                        panic!("Unable to resolve value to be stored: {:?}", e);
                                    }

                                    blk.push_u256(address);
                                    blk.external_sstore();
                                }
                                Operation::Return(ref _value) => {
                                    // Assumes that the next element on the stack is return pointer
                                    // TODO: Pop all elements that were not used yet.
                                    // TODO: Push value if exists and swap1, then jump

                                    while blk.scope.stack_counter != 0 {
                                        blk.pop();
                                    }
                                    blk.jump();
                                }
                                _ => {
                                    println!("Unhandled instruction: {:#?}", instr);
                                    unimplemented!() // Add handling for other operations here
                                }
                            }
                        }

                        ret.push(blk);
                    }

                    ret
                });
            println!("DONE!");
        }

        Ok(0)
    }

    pub fn build_executable(&mut self) -> Result<Vec<u8>, String> {
        self.build_state_layout()?;

        self.write_function_definitions_to_module()?;

        self.builder.finalize_blocks();
        println!("{}", self.builder.generate_evm_assembly());
        Ok(self.builder.build())
    }
}

/// This impl block provides the lowering operations for our `EvmBytecodeGenerator`.
/// Here we translate high-level intermediate representation (IR) constructs into
/// lower-level constructs that are suitable for generating EVM bytecode.
impl<'ctx> IrLowering for EvmBytecodeGenerator<'ctx> {
    /// This function takes a `ConcreteType` and lowers it into a form suitable for generating
    /// EVM bytecode. How exactly this is done will depend on the concrete type in question.
    fn lower_concrete_type(&mut self, _con_type: &ConcreteType) {
        // TODO: Implement
        unimplemented!()
    }

    /// This function takes a `ConcreteFunction` and lowers it into a form suitable
    /// for generating EVM bytecode. This typically involves translating the function's
    /// high-level operations into equivalent sequences of low-level EVM operations.
    fn lower_concrete_function(&mut self, _con_function: &ConcreteFunction) {
        // TODO: Move write_function_definitions_to_module into this structure
        unimplemented!()
    }

    /// This is the main interface for lowering. It takes an intermediate representation (IR)
    /// and lowers all its types and function definitions.
    fn lower(&mut self, primitives: &IntermediateRepresentation) {
        for con_type in &primitives.type_definitions {
            self.lower_concrete_type(con_type);
        }

        for con_function in &primitives.function_definitions {
            self.lower_concrete_function(con_function);
        }
    }
}
