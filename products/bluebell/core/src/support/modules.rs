use std::{
    collections::{BTreeSet, HashMap},
    mem,
    str::FromStr,
};

use evm::{
    backend::Backend,
    executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType},
    Context as EvmContext, ExitError, ExitSucceed,
};
use evm_assembly::{block::EvmBlock, compiler_context::EvmCompilerContext, types::EvmType};
use log::info;

use crate::intermediate_representation::{
    name_generator::NameGenerator,
    symbol_table::{SymbolTable, SymbolTableConstructor},
};

// TODO: Generalize to support both EVM and LLVM

pub trait BluebellModule {
    fn attach(&self, context: &mut EvmCompilerContext);
}

impl SymbolTableConstructor for EvmCompilerContext {
    fn new_symbol_table(&self) -> SymbolTable {
        let type_of_table = HashMap::new();

        let mut ret = SymbolTable {
            aliases: HashMap::new(),
            type_of_table,
            name_generator: NameGenerator::new(),
            state_layout: HashMap::new(),
        };

        // TODO: Get types from self
        let _ = ret.declare_type("Int8");
        let _ = ret.declare_type("Int16");
        let _ = ret.declare_type("Int32");
        let _ = ret.declare_type("Int64");
        let _ = ret.declare_type("Uint8");
        let _ = ret.declare_type("Uint16");
        let _ = ret.declare_type("Uint32");
        let _ = ret.declare_type("Uint64");
        let _ = ret.declare_type("String");
        let _ = ret.declare_type("ByStr20");

        let _ = ret.declare_special_variable("_sender", "ByStr20");

        ret.aliases
            .insert("True".to_string(), "Bool::True".to_string());
        ret.aliases
            .insert("False".to_string(), "Bool::False".to_string());
        let _ = ret.declare_constructor("Bool::True", &[].to_vec(), "Bool");
        let _ = ret.declare_constructor("Bool::False", &[].to_vec(), "Bool");

        // Adding function types
        for (name, (args, return_type)) in self.raw_function_declarations.iter() {
            // info!("Declaring {:#?}",f);
            let _ = ret.declare_function_type(&name, args, &return_type);
        }

        ret
    }
}

pub struct ScillaDefaultTypes;
impl BluebellModule for ScillaDefaultTypes {
    // TODO: Generalise to support both LLVM and EVM
    fn attach(&self, context: &mut EvmCompilerContext) {
        context.declare_integer("Bool", 1);
        context.declare_integer("Int8", 8);
        context.declare_integer("Int16", 16);
        context.declare_integer("Int32", 32);
        context.declare_integer("Int64", 64);
        context.declare_integer("Int128", 128);
        context.declare_integer("Int256", 256);
        context.declare_unsigned_integer("Uint8", 8);
        context.declare_unsigned_integer("Uint16", 16);
        context.declare_unsigned_integer("Uint32", 32);
        context.declare_unsigned_integer("Uint64", 64);
        context.declare_unsigned_integer("Uint128", 128);
        context.declare_unsigned_integer("Uint256", 256);

        for i in 0..=32 {
            context.declare_unsigned_integer(&format!("ByStr{}", i), i * 8);
        }

        context.declare_dynamic_string("String");

        context.declare_default_constructor("Bool::False", |block| {
            block.push([0].to_vec());
        });
        context.declare_default_constructor("Bool::True", |block| {
            block.push([1].to_vec());
        });

        // TODO: Functions to be moved out to another
    }
}

pub struct ScillaDebugBuiltins;
impl BluebellModule for ScillaDebugBuiltins {
    fn attach(&self, specification: &mut EvmCompilerContext) {
        let _ = specification
            .declare_function("builtin__print::<>", Vec::new(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    info!("\n");
                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });

        let _ = specification
            .declare_function("print::<Uint64>", ["Uint64"].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    if input.len() >= 8 {
                        let last_8 = &input[input.len() - 8..];
                        let v = u64::from_be_bytes([
                            last_8[0], last_8[1], last_8[2], last_8[3], last_8[4], last_8[5],
                            last_8[6], last_8[7],
                        ]);
                        info!("{}", format!("{}", v));
                    }
                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });

        // TODO: Make runtime module that does this for the real chain
        let _ = specification
            .declare_function("__intrinsic_accept_transfer::<>", Vec::new(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    info!("--- ACCEPTING FUNDS [DEBUG FUNCTION] ---");
                    println!("--- ACCEPTING FUNDS [DEBUG FUNCTION] ---");
                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });

        let _ = specification
            .declare_function("print::<ByStr20>", ["ByStr20"].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    info!("{}", hex::encode(input));

                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });

        let _ = specification
            .declare_function("print::<String>", ["String"].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    assert!(input.len() > 32);
                    let (head, tail) = input.split_at(32);
                    let location_bytes = head[28..].try_into().expect("");
                    let location = u32::from_be_bytes(location_bytes) as usize;
                    assert_eq!(location, 0x20);

                    let length_bytes = tail[0..4]
                        .try_into()
                        .expect("Failed to extract string length");
                    let length = u32::from_be_bytes(length_bytes) as usize;

                    assert!(length <= (tail.len() - 4).try_into().unwrap());
                    let s = &tail[4..];

                    assert_eq!(length, s.len());
                    match std::str::from_utf8(&s) {
                        Ok(v) => info!("{}", v),
                        Err(_) => panic!(
                            "While panicking: Failed to decode '{}'",
                            format!("{}\n", hex::encode(input))
                        ),
                    };

                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });
        let _ = specification
            .declare_function("panic::<String>", ["String"].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    assert!(input.len() > 32);
                    let (head, tail) = input.split_at(32);
                    let location_bytes = head[28..].try_into().expect("");
                    let location = u32::from_be_bytes(location_bytes) as usize;
                    assert_eq!(location, 0x20);

                    let length_bytes = tail[0..4]
                        .try_into()
                        .expect("Failed to extract string length");
                    let length = u32::from_be_bytes(length_bytes) as usize;

                    assert!(length <= (tail.len() - 4).try_into().unwrap());
                    let s = &tail[4..];

                    assert_eq!(length, s.len());
                    match std::str::from_utf8(&s) {
                        Ok(v) => panic!("{}", v),
                        Err(_) => panic!(
                            "While panicking: Failed to decode '{}'",
                            format!("{}\n", hex::encode(input))
                        ),
                    };
                }

                custom_runtime
            });

        let _ = specification
            .declare_function("print::<Bool>", ["Bool"].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    info!("Was here??");

                    if input.iter().all(|&byte| byte == 0) {
                        info!("false");
                    } else {
                        info!("true");
                    }

                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        0,
                    ))
                }

                custom_runtime
            });

        let _ = specification.declare_inline_generics("builtin__print", |ctx, block, arg_types| {
            let mut ret: Vec<EvmBlock> = Vec::new();
            for arg in arg_types {
                let signature = match ctx.get_function(&format!("builtin__print__impl::<{}>", arg))
                {
                    Some(s) => s,
                    None => panic!("Internal error: Unable to retrieve function"),
                };
                let subcall_arg_types: Vec<String> = [arg.clone()].to_vec();
                // TODO: There is some issue with this function
                /*
                                MVP error:
                scilla_version 0

                library HelloWorld
                type Bool =
                  | True
                  | False

                contract HelloWorld()

                transition setHello (msg : Uint64)
                  is_owner = True;
                  match is_owner with
                  | True =>
                    x = builtin print msg
                  end
                end
                                */
                // TODO: In the event of string, this is not one to one
                if arg == "String" {
                    // Putting string onto stack so it is accessible to our precompile function
                    block.dup1(); // Duplicate to preserve base address for loop

                    block.mload();
                    block.push_u32(256 - 32);
                    block.shr(); // Stack now contains size of string

                    block.push([0x04].to_vec()); // Offset / counter

                    block.jump_to("loop_start");

                    let mut loop_start = EvmBlock::new(None, BTreeSet::new(), "loop_start");
                    let mut loop_body = EvmBlock::new(None, BTreeSet::new(), "loop_body");
                    let mut loop_end = EvmBlock::new(None, BTreeSet::new(), "loop_end");

                    loop_start.dup2();
                    loop_start.dup2();
                    loop_start.gt();
                    loop_start.jump_if_to("loop_end");
                    loop_start.jump_to("loop_body");

                    loop_body.dup3(); // Duplicating base address
                    loop_body.dup2(); // Counter / offset
                    loop_body.add();
                    loop_body.mload();
                    loop_body.call(
                        signature,
                        subcall_arg_types
                            .iter()
                            .map(|s| EvmType::from_str(s).unwrap())
                            .collect(),
                    );
                    loop_body.pop(); // Removing result

                    loop_body.push([0x20].to_vec()); // Incrementing counter
                    loop_body.add();
                    loop_body.jump_to("loop_start");

                    loop_end.pop(); // Remove counter
                    loop_end.pop(); // Removing size
                    loop_end.pop(); // Removing base address

                    // End block becomes the new main block
                    mem::swap(block, &mut loop_end);

                    ret.push(loop_end);
                    ret.push(loop_start);
                    ret.push(loop_body);
                } else {
                    block.call(
                        signature,
                        subcall_arg_types
                            .iter()
                            .map(|s| EvmType::from_str(s).unwrap())
                            .collect(),
                    );
                    // block.swap1(); // Moving the result so it does not get popped
                    block.pop(); // Removing result
                }

                block.pop(); // Removing the argument that was to be printed
            }

            Ok(ret)
        });
    }
}

pub struct ScillaDefaultBuiltins;
impl BluebellModule for ScillaDefaultBuiltins {
    // TODO: Generalise to support both LLVM and EVM

    fn attach(&self, specification: &mut EvmCompilerContext) {
        let _ = specification
            .declare_function(
                "builtin__fibonacci::<Uint64,Uint64>",
                ["Uint256", "Uint256"].to_vec(),
                "Uint256",
            )
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    let gas_needed = 20;

                    if let Some(gas_limit) = gas_limit {
                        if gas_limit < gas_needed {
                            return Err(PrecompileFailure::Error {
                                exit_status: ExitError::OutOfGas,
                            });
                        }
                    }

                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        gas_needed,
                    ))
                }

                custom_runtime
            });

        let _ = specification
            .declare_function(
                "builtin__eq::<Uint64,Uint64>",
                ["Uint64", "Uint64"].to_vec(),
                "Bool",
            )
            .attach_assembly(|block| {
                block.eq();
            });

        let _ = specification
            .declare_function(
                "builtin__eq::<Bool,Bool>",
                ["Uint64", "Uint64"].to_vec(),
                "Bool",
            )
            .attach_assembly(|block| {
                block.eq();
            });

        // Memory management
        /*
        let _ = specification.declare_inline_generics("alloca", |_ctx, block, arg_types| {
            // size
            block.push
            block.add();
            Ok(())
        });
        */

        let _ = specification.declare_special_variable("_sender", "ByStr20", |_ctx, block| {
            block.external_caller();
            Ok([].to_vec())
        });

        // Assuming you have a 'specification' object available...
        // Implementing `add`:
        let _ = specification.declare_inline_generics("builtin__add", |_ctx, block, _arg_types| {
            // TODO: Check that the number of arguments is two and otherwise return an error
            block.add();
            Ok([].to_vec())
        });

        // Implementing `sub`:
        let _ = specification.declare_inline_generics("builtin__sub", |_ctx, block, _arg_types| {
            block.sub();
            Ok([].to_vec())
        });

        // Implementing `mul`:
        let _ = specification.declare_inline_generics("builtin__mul", |_ctx, block, _arg_types| {
            block.mul();
            Ok([].to_vec())
        });

        // Implementing `div`:
        let _ = specification.declare_inline_generics("builtin__div", |_ctx, block, _arg_types| {
            block.div();
            Ok([].to_vec())
        });

        // Implementing `rem`:
        let _ = specification.declare_inline_generics("builtin__rem", |_ctx, block, _arg_types| {
            block.smod(); // smod might be the appropriate EVM instruction for remainder, but verify with EVM docs.
            Ok([].to_vec())
        });

        // Implementing comparison builtins:
        let _ = specification.declare_inline_generics("builtin__lt", |_ctx, block, _arg_types| {
            block.lt();
            Ok([].to_vec())
        });

        let _ = specification.declare_inline_generics("builtin__lte", |_ctx, block, _arg_types| {
            block.dup2();
            block.dup2();
            block.lt();
            block.eq();
            block.or();

            Ok([].to_vec())
        });

        let _ = specification.declare_inline_generics("builtin__gt", |_ctx, block, _arg_types| {
            block.gt();
            Ok([].to_vec())
        });

        let _ = specification.declare_inline_generics("builtin__gte", |_ctx, block, _arg_types| {
            block.dup2();
            block.dup2();
            block.gt();
            block.eq();
            block.or();

            Ok([].to_vec())
        });

        // Implementing boolean builtins:
        let _ = specification.declare_inline_generics("builtin__and", |_ctx, block, _arg_types| {
            block.and();
            Ok([].to_vec())
        });

        let _ = specification.declare_inline_generics("builtin__orb", |_ctx, block, _arg_types| {
            block.or();
            Ok([].to_vec())
        });

        let _ =
            specification.declare_inline_generics("builtin__notb", |_ctx, block, _arg_types| {
                block.not();
                Ok([].to_vec())
            });

        // Implementing cryptographic builtins:
        let _ = specification.declare_inline_generics(
            "builtin__sha256hash",
            |_ctx, _block, _arg_types| {
                // block.external_sha256();
                // Ok(())
                unimplemented!()
            },
        );

        let _ = specification.declare_inline_generics(
            "builtin__keccak256hash",
            |_ctx, block, _arg_types| {
                block.external_sha3();
                Ok([].to_vec())
            },
        );

        let _ = specification.declare_inline_generics(
            "builtin__ripemd160hash",
            |_ctx, _block, _arg_types| {
                // block.external_ripemd160();
                // Ok(())
                unimplemented!()
            },
        );

        let _ = specification.declare_inline_generics(
            "builtin__schnorr_sign",
            |_ctx, _block, _arg_types| {
                // EVM doesn't natively support Schnorr; you'd need to call a precompiled contract or use an external lib.
                // For now, just placing a placeholder.
                // block.schnorr_sign();
                unimplemented!()
            },
        );

        let _ = specification.declare_inline_generics(
            "builtin__schnorr_verify",
            |_ctx, _block, _arg_types| {
                // Same as schnorr_sign, EVM doesn't natively support Schnorr.
                // block.schnorr_verify();
                unimplemented!()
            },
        );
    }
}
