use evm::backend::Backend;
use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType};
use evm::{Context as EvmContext, ExitError, ExitSucceed};
use evm_assembly::compiler_context::EvmCompilerContext;

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
    }
}

pub struct ScillaDebugBuiltins;
impl BluebellModule for ScillaDebugBuiltins {
    fn attach(&self, specification: &mut EvmCompilerContext) {
        let _ = specification
            .declare_function("builtin__print::<>", [].to_vec(), "Uint256")
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    _gas_limit: Option<u64>,
                    _context: &EvmContext,
                    _backend: &dyn Backend,
                    _is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    println!("");
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
            .declare_function("builtin__print::<Uint64>", ["Uint256"].to_vec(), "Uint256")
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
                        print!("{}", v);
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
                    println!("Running precompile {:?}!", input);
                    println!("Len: {} / {}", input.len() / 32, input.len());
                    println!("Context: {:#?}", context);
                    println!("Static: {}", is_static);
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

        // Assuming you have a 'specification' object available...

        // Implementing `add`:
        let _ = specification.declare_inline_generics("builtin__add", |block, _arg_types| {
            // TODO: Check that the number of arguments is two and otherwise return an error
            block.add();
            Ok(())
        });

        // Implementing `sub`:
        let _ = specification.declare_inline_generics("builtin__sub", |block, _arg_types| {
            block.sub();
            Ok(())
        });

        // Implementing `mul`:
        let _ = specification.declare_inline_generics("builtin__mul", |block, _arg_types| {
            block.mul();
            Ok(())
        });

        // Implementing `div`:
        let _ = specification.declare_inline_generics("builtin__div", |block, _arg_types| {
            block.div();
            Ok(())
        });

        // Implementing `rem`:
        let _ = specification.declare_inline_generics("builtin__rem", |block, _arg_types| {
            block.smod(); // smod might be the appropriate EVM instruction for remainder, but verify with EVM docs.
            Ok(())
        });

        // Implementing comparison builtins:
        let _ = specification.declare_inline_generics("builtin__lt", |block, _arg_types| {
            block.lt();
            Ok(())
        });

        let _ = specification.declare_inline_generics("builtin__lte", |_block, _arg_types| {
            // block.lte();  // This might need additional logic as EVM directly doesn't have lte
            // Ok(())

            panic!("LTE not supported directly by EVM");
        });

        let _ = specification.declare_inline_generics("builtin__eq", |block, _arg_types| {
            block.eq();
            Ok(())
        });

        let _ = specification.declare_inline_generics("builtin__gt", |block, _arg_types| {
            block.gt();
            Ok(())
        });

        let _ = specification.declare_inline_generics("builtin__gte", |_block, _arg_types| {
            // block.gte();  // This might need additional logic as EVM directly doesn't have gte
            // Ok(())

            panic!("GTE not supported directly by EVM");
        });

        // Implementing boolean builtins:
        let _ = specification.declare_inline_generics("builtin__andb", |block, _arg_types| {
            block.and();
            Ok(())
        });

        let _ = specification.declare_inline_generics("builtin__orb", |block, _arg_types| {
            block.or();
            Ok(())
        });

        let _ = specification.declare_inline_generics("builtin__notb", |block, _arg_types| {
            block.not();
            Ok(())
        });

        // Implementing cryptographic builtins:
        let _ =
            specification.declare_inline_generics("builtin__sha256hash", |_block, _arg_types| {
                // block.external_sha256();
                // Ok(())
                unimplemented!()
            });

        let _ =
            specification.declare_inline_generics("builtin__keccak256hash", |block, _arg_types| {
                block.external_sha3();
                Ok(())
            });

        let _ = specification.declare_inline_generics(
            "builtin__ripemd160hash",
            |_block, _arg_types| {
                // block.external_ripemd160();
                // Ok(())
                unimplemented!()
            },
        );

        let _ =
            specification.declare_inline_generics("builtin__schnorr_sign", |_block, _arg_types| {
                // EVM doesn't natively support Schnorr; you'd need to call a precompiled contract or use an external lib.
                // For now, just placing a placeholder.
                // block.schnorr_sign();
                unimplemented!()
            });

        let _ = specification.declare_inline_generics(
            "builtin__schnorr_verify",
            |_block, _arg_types| {
                // Same as schnorr_sign, EVM doesn't natively support Schnorr.
                // block.schnorr_verify();
                unimplemented!()
            },
        );
    }
}
