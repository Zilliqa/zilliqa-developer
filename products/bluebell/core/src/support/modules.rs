use evm::backend::Backend;
use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType};
use evm::{Context as EvmContext, ExitError, ExitSucceed};
use evm_assembly::block::EvmBlock;
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::types::EvmType;
use log::{error, info};
use std::collections::BTreeSet;
use std::mem;
use std::str::FromStr;
// TODO: Generalize to support both EVM and LLVM

pub trait BluebellModule {
    fn attach(&self, context: &mut EvmCompilerContext);
}

pub struct ScillaDefaultTypes;
impl BluebellModule for ScillaDefaultTypes {
    // TODO: Generalise to support both LLVM and EVM
    fn attach(&self, context: &mut EvmCompilerContext) {
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

        context.declare_dynamic_string("String");
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
            .declare_function(
                "builtin__print__impl::<Uint64>",
                ["Uint64"].to_vec(),
                "Uint256",
            )
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

        let _ = specification
            .declare_function(
                "builtin__print__impl::<String>",
                ["String"].to_vec(),
                "Uint256",
            )
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    match std::str::from_utf8(input) {
                        Ok(v) => info!("{}", format!("{}\n", v)),
                        Err(_) => error!("{}", format!("{}\n", hex::encode(input))),
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
                    println!("WAS Here??");
                    match std::str::from_utf8(input) {
                        Ok(v) => panic!("{}", v),
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
                    context: &EvmContext,
                    _backend: &dyn Backend,
                    is_static: bool,
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

        // Memory management
        /*
        let _ = specification.declare_inline_generics("alloca", |_ctx, block, arg_types| {
            // size

            block.push
            block.add();
            Ok(())
        });
        */

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

        let _ = specification.declare_inline_generics("builtin__eq", |_ctx, block, _arg_types| {
            block.eq();
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
        let _ =
            specification.declare_inline_generics("builtin__andb", |_ctx, block, _arg_types| {
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
