use crate::constants::TreeTraversalMode;
use crate::intermediate_representation::pass::IrPass;
use crate::intermediate_representation::primitives::Operation;
use crate::intermediate_representation::primitives::{
    ConcreteFunction, ConcreteType, IntermediateRepresentation, IrLowering,
};
use crate::intermediate_representation::symbol_table::StateLayoutEntry;
use crate::passes::debug_printer::DebugPrinter;
use evm_assembly::block::EvmBlock;
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executable::EvmExecutable;
use evm_assembly::instruction::EvmSourcePosition;
use primitive_types::U256;
use std::collections::BTreeSet;

use evm_assembly::types::EvmTypeValue;
use evm_assembly::EvmAssemblyGenerator;
use evm_assembly::EvmByteCodeBuilder;
use log::info;
use sha3::{Digest, Keccak256};
use std::mem;

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
}

impl<'ctx> EvmBytecodeGenerator<'ctx> {
    /// This constructs a new `EvmBytecodeGenerator`. It takes an existing EVM compiler context
    /// and a boxed intermediate representation (IR) of the program.  
    pub fn new(
        context: &'ctx mut EvmCompilerContext,
        ir: Box<IntermediateRepresentation>,
        abi_support: bool,
    ) -> Self {
        let builder = if abi_support {
            context.create_builder()
        } else {
            context.create_builder_no_abi_support()
        };
        Self { builder, ir }
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

            self.ir
                .symbol_table
                .state_layout
                .insert(name.to_string(), state);
            address_offset += 1;
        }

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

            let arg_names: BTreeSet<String> = func
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

            self.builder
                .define_function(&function_name, arg_types, return_type)
                .build(|code_builder| {
                    let mut ret: Vec<EvmBlock> = Vec::new();
                    let mut symbol_table = self.ir.symbol_table.clone();

                    // TODO: Check that arg_names matches length of the arguments in the first block
                    if let Some(entry) = func.body.blocks.first() {
                        if arg_names.len() != entry.block_arguments.len() {
                            panic!("Internal error: Function argument names differ from block names in length: {:?} vs {:?}", arg_names, entry.block_arguments);
                        }
                        if arg_names != entry.block_arguments {
                            panic!("Internal error: Function argument names differ from block names in order");
                        }
                    }

                    // Return PC + Arguments are expected to be on the stack
                    for block in &func.body.blocks {
                        let block_name = match block.name.qualified_name() {
                            Ok(b) => b,
                            Err(_) => panic!("Failed to get qualified name."),
                        };


                        // Creating entry function
                        let block_args : BTreeSet<String> = block.block_arguments.clone();

                        let mut evm_block =
                            EvmBlock::new(None, block_args, &block_name);

                        for instr in &block.instructions {
                            let mut instruction_printer = DebugPrinter::new();
                            let mut instr_copy = instr.clone();
                            let _ = instruction_printer.visit_instruction(TreeTraversalMode::Enter, &mut instr_copy, &mut symbol_table);
                            evm_block.set_next_instruction_comment(instruction_printer.value());

                            let (l_pos, r_pos) = &instr.source_location;
                            if l_pos.is_valid() && r_pos.is_valid() {
                                let pos = EvmSourcePosition {
                                    start: l_pos.position,
                                    end: r_pos.position,
                                    line: l_pos.line,
                                    column: l_pos.column,
                                };
                                evm_block.set_next_instruction_location(pos);
                            }


                            match &instr.operation {
                                Operation::CallFunction {
                                    ref name,
                                    ref arguments,
                                } => {
                                    let mut exit_block = EvmBlock::new(
                                        Some(0),
                                        ["result".to_string()].to_vec().into_iter().collect(),
                                        "exit_block",
                                    );

                                    // Adding return point
                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.push_label("exit_block");

                                    for arg in arguments {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        let _ = match &arg.resolved {
                                            Some(a) => evm_block.duplicate_stack_name(&a),
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

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump_to(&label);
                                    mem::swap(&mut evm_block, &mut exit_block);
                                    ret.push(exit_block);
                                }
                                Operation::CallExternalFunction {
                                    ref name,
                                    ref arguments,
                                } => {

                                    // Invoking
                                    let qualified_name = match &name.resolved {
                                        Some(n) => n,
                                        None => {
                                            // TODO: Fix error propagation
                                            panic!(
                                                "Encountered unresolved function name {}",
                                                name.unresolved
                                            )
                                        }
                                    };

                                    let mut ctx = &mut code_builder.context;
                                    // We have three types of calls:
                                    // - Precompiles / external function
                                    // - Inline assembler generics
                                    // - Internal calls

                                    // Copying arguments to stack
                                    for arg in arguments {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        match &arg.resolved {
                                            Some(n) => match evm_block.duplicate_stack_name(n) {
                                                Err(e) => panic!("{}", e),
                                                _ => (),
                                            },
                                            None => panic!("Argument name was not resolved"),
                                        }
                                    }

                                    let args_types: Vec<String> = arguments
                                        .iter()
                                        .map(|arg| arg.type_reference.clone().unwrap())
                                        .collect();

                                    if ctx.function_declarations.contains_key(qualified_name) {
                                        let signature = match ctx.get_function(qualified_name) {
                                            Some(s) => s,
                                            None => panic!(
                                                "Internal error: Unable to retrieve function"
                                            ),
                                        };

                                        // Precompiled or external function
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.call(signature, args_types);
                                    } else if ctx.inline_generics.contains_key(&name.unresolved) {
                                        // TODO: This ought to be the resovled name, but it should be resovled without instance parameters - make a or update pass
                                        // Builtin assembly generator

                                        let block_generator =
                                            ctx.inline_generics.get(&name.unresolved).unwrap();
                                        let new_blocks =
                                            block_generator(&mut ctx, &mut evm_block, args_types);
                                        match new_blocks {
                                            Ok(new_blocks) => {
                                                for block in new_blocks {
                                                    ret.push(block);
                                                }
                                            }
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
                                            code_builder.ir.data.push((ssa_name, payload));

                                            // TODO: Load data from code into memory
                                            // TODO: We need a way to reference the data section
                                            todo!()
                                        }
                                        "Uint64" => {
                                            let value = EvmTypeValue::Uint64(data.parse().unwrap());
                                            evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                            evm_block.push(value.to_bytes_unpadded());
                                            match evm_block.register_stack_name(ssa_name) {
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

                                    if let Err(e) = evm_block.register_alias(source, dest) {
                                        panic!("Failed registering alias: {:?}", e);
                                    }
                                }
                                Operation::StateStore {
                                    ref address,
                                    ref value,
                                } => {
                                    // TODO: Ensure that we used resolved address name
                                    let binding = &self.ir.symbol_table.state_layout.get(&address.name.unresolved);
                                    let state = match binding {
                                        Some(v) => v,
                                        None => panic!(
                                            "{}",
                                            format!(
                                                "Unable to find state {} (storing {})",
                                                address.name.unresolved, value.unresolved
                                            )
                                        ),
                                    };

                                    let address = state.address_offset;

                                    let value_name = match &value.resolved {
                                        Some(v) => v,
                                        None => {
                                            panic!("{}", format!("Unable to resolve {:?}", value))
                                        }
                                    };

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    if let Err(e) = evm_block.duplicate_stack_name(value_name) {
                                        panic!("Unable to resolve value to be stored: {:?}", e);
                                    }

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.push_u256(address);
                                    evm_block.external_sstore();
                                }

                                Operation::StateLoad {
                                    ref address,
                                } => {
                                    // TODO: Ensure that we used resolved address name
                                    let binding = &self.ir.symbol_table.state_layout.get(&address.name.unresolved);
                                    let value = match &instr.ssa_name {
                                        Some(v) => v,
                                        None => panic!("Load does not assign value")

                                    };
                                    let state = match binding {
                                        Some(v) => v,
                                        None => panic!(
                                            "{}",
                                            format!(
                                                "Unable to find state {} (loading to {})",
                                                address.name.unresolved,
                                                value.unresolved
                                            )
                                        ),
                                    };

                                    let address = state.address_offset;

                                    let value_name = match &value.resolved {
                                        Some(v) => v,
                                        None => {
                                            panic!("{}", format!("Unable to resolve {:?}", value))
                                        }
                                    };

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.push_u256(address);
                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.external_sload();
                                    evm_block.register_stack_name(value_name);
                                }
                                Operation::Return(ref _value) => {
                                    // Assumes that the next element on the stack is return pointer
                                    // TODO: Pop all elements that were not used yet.
                                    // TODO: Push value if exists and swap1, then jump

                                    while evm_block.scope.stack_counter > 0 {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.pop();
                                    }
                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump();
                                }
                                Operation::CallStaticFunction {
                                    // TODO: Poor name
                                    ref name,
                                    owner: _,
                                    ref arguments,
                                } => {
                                    if arguments.len() > 0 {
                                        // TODO: Pack data
                                        unimplemented!();
                                    }
                                    let name = match &name.resolved {
                                        Some(n) => n,
                                        None => {
                                            panic!("Unable to resolve name {:?}", name.unresolved)
                                        }
                                    };

                                    // TODO: Assumes that a static call just produces the Keccak of the name
                                    // The "correct" to do would be to make this spec dependant
                                    let hash = Keccak256::digest(name);
                                    let mut selector = Vec::new();
                                    selector.extend_from_slice(&hash[..4]);
                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.push(selector);
                                }
                                Operation::IsEqual {
                                    ref left,
                                    ref right,
                                } => {

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    match &left.resolved {
                                        Some(l) => match evm_block.duplicate_stack_name(l) {
                                            Ok(()) => (),
                                            Err(e) => panic!("{:#?}", e),
                                        },
                                        None => panic!("Unresolved left hand side"),
                                    }
                                    match &right.resolved {
                                        Some(r) => match evm_block.duplicate_stack_name(r) {
                                            Ok(()) => (),
                                            Err(e) => panic!("{:#?}", e),
                                        },
                                        None => panic!("Unresolved left hand side"),
                                    }

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.eq();
                                }
                                Operation::Switch {
                                    // TODO: Deprecated?
                                    ref cases,
                                    ref on_default,
                                } => {
                                    for case in cases {
                                        let label = match &case.label.resolved {
                                            Some(l) => l,
                                            None => panic!("Could not resolve case label"),
                                        };
                                        // TODO: This assumes order in cases
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.jump_if_to(label);
                                    }

                                    let label = match &on_default.resolved {
                                        Some(l) => l,
                                        None => panic!("Could not resolve default label"),
                                    };

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump_to(label);
                                    // unimplemented!() // Add handling for other operations here
                                }
                                Operation::Jump(label) => {
                                    let label = match &label.resolved {
                                        Some(l) => l,
                                        None => panic!("Could not resolve default label"),
                                    };

                                    let mut pop_count = evm_block.scope.stack_counter;
                                    let jump_args = block
                                        .jump_required_arguments
                                        .get(label)
                                        .unwrap_or(&BTreeSet::new())
                                        .clone();

                                    // Preserving the args to the next block
                                    pop_count -= jump_args.len() as i32;

                                    // Moving arguments
                                    // Notice the reversing of the arguments, since positions are in relative stack
                                    // depth and consenquently the first argument becomes the deepest (highest number)
                                    for (i, arg) in jump_args.iter().rev().enumerate() {
                                        let pos = pop_count+i as i32;
                                        evm_block.set_next_instruction_comment(format!("Moving argument {} '{}' behind {}",pos,arg, pop_count).to_string()) ;
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        match evm_block.move_stack_name(&arg, pos) {
                                            Ok(()) => (),
                                            Err(e) => panic!("{:#?}", e),
                                        }

                                    }


                                    while pop_count > 0 {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.pop();
                                        pop_count -= 1;
                                    }

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump_to(label);
                                }
                                Operation::ConditionalJump {
                                    ref expression,
                                    ref on_success,
                                    ref on_failure,
                                } => {

                                    let _ = match &expression.resolved {
                                        Some(name) => evm_block.duplicate_stack_name(&name),
                                        None => panic!("Expression does not have a SSA name"),
                                    };

                                    let mut pop_count = evm_block.scope.stack_counter;

                                    let success_label = match &on_success.resolved {
                                        Some(l) => l,
                                        None => panic!("Could not resolve on_success label"),
                                    };

                                    let failure_label = match &on_failure.resolved {
                                        Some(l) => l,
                                        None => panic!("Could not resolve on_failure label"),
                                    };
                                    // TODO: Fix this such that it is done properly

                                    let success_jump_args = block
                                        .jump_required_arguments
                                        .get(success_label)
                                        .unwrap_or(&BTreeSet::new())
                                        .clone();
                                    let failure_jump_args = block
                                        .jump_required_arguments
                                        .get(failure_label)
                                        .unwrap_or(&BTreeSet::new())
                                        .clone();

                                    if !success_jump_args.eq(&failure_jump_args) {
                                        panic!("Block termination must require same number of subsequent variable dependencies.");
                                    }


                                    // Preserving the args to the next block and the condition                                    
                                    pop_count -= success_jump_args.len()  as i32;
                                    assert!(pop_count>=0);

                                    // Putting all arguments on the stack and preparing to pop before jumping
                                    // Notice the reversing of the arguments, since positions are in relative stack
                                    // depth and consenquently the first argument becomes the deepest (highest number)
                                    for (i, arg) in success_jump_args.iter().rev().enumerate() {
                                        let pos = pop_count+i as i32;
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.set_next_instruction_comment(format!("Moving argument {} '{}' to {}",i, arg, pos).to_string()) ;
                                        //assert_eq!(pos, evm_block.scope.stack_counter+1 - (success_jump_args.len() - i) as i32);

                                        match evm_block.move_stack_name(&arg, pos) {
                                            Ok(()) => (),
                                            Err(e) => panic!("{:#?}", e),
                                        }
                                    }

                                    // Making room for the condition
                                    assert!(pop_count>0);
                                    pop_count-= 1;

                                    if pop_count > 0 {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.set_next_instruction_comment(format!("Preserving jump condition and preparing stack deletion {}", pop_count).to_string());
                                        evm_block.swap(pop_count);
                                    }

                                    while pop_count > 0 {
                                        evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                        evm_block.pop();
                                        pop_count -= 1;
                                    }

                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump_if_to(success_label);

                                    // TODO: manage stack
                                    evm_block.set_next_rust_position(file!().to_string(), line!() as usize);
                                    evm_block.jump_to(failure_label);
                                }
                                Operation::TerminatingRef (_) => {
                                    // Ignore terminating ref as this will just be pop at the end of the block.
                                }
                                _ => {
                                    unimplemented!() // Add handling for other operations here
                                }
                            }

                            // Handling SSA
                            if let Some(ssa_name) = &instr.ssa_name {
                                let ssa_name = match &ssa_name.resolved {
                                    Some(x) => x,
                                    _ => panic!("SSA symbol name was unresolved."),
                                };

                                match instr.operation {
                                    Operation::ResolveSymbol { symbol: _ }
                                    | Operation::StateStore {
                                        address: _,
                                        value: _,
                                    }
                                    | Operation::StateLoad {
                                        address: _,
                                    }
                                    | Operation::Literal {
                                        data: _,
                                        typename: _,
                                    } => (), // Literals are handled in the first match statement
                                    _ => {
                                        match evm_block.register_stack_name(ssa_name) {
                                        Err(_) => {
                                            panic!(
                                                "Failed to register SSA stack name: {}.",
                                                ssa_name
                                            );
                                        }
                                        _ => (),
                                    }
                                    }
                                }
                            }
                        }

                        ret.push(evm_block);
                    }

                    ret
                });
        }

        Ok(0)
    }

    pub fn build_executable(&mut self) -> Result<EvmExecutable, String> {
        self.build_state_layout()?;

        self.write_function_definitions_to_module()?;

        self.builder.finalize_blocks();
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
