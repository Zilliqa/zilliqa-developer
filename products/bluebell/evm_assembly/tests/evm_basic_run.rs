use evm::backend::Backend;
use evm::executor::stack::PrecompileFn;
use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType};
use evm::{Context, ExitError, ExitSucceed};
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::types::EvmTypeValue;
use primitive_types::H160;
use std::collections::BTreeMap;
use std::str::FromStr;

// See
// https://odra.dev/blog/evm-at-risc0/
// https://github.com/Zilliqa/zq2/blob/main/zilliqa/src/exec.rs#L152

// Transaction spec:
// https://docs.soliditylang.org/en/latest/abi-spec.html#formal-specification-of-the-encoding

fn test_precompile(
    input: &[u8],
    gas_limit: Option<u64>,
    _contex: &Context,
    _backend: &dyn Backend,
    _is_static: bool,
) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
    println!("Running precompile!");
    let gas_needed = match required_gas(input) {
        Ok(i) => i,
        Err(err) => return Err(PrecompileFailure::Error { exit_status: err }),
    };

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

fn required_gas(_input: &[u8]) -> Result<u64, ExitError> {
    Ok(20)
}

pub fn get_precompiles() -> BTreeMap<H160, PrecompileFn> {
    BTreeMap::from([(
        H160::from_str("0000000000000000000000000000000000000005").unwrap(),
        test_precompile as PrecompileFn,
    )])
}

#[cfg(test)]
mod tests {
    use evm_assembly::executor::EvmExecutor;

    use crate::test_precompile;
    use crate::{EvmCompilerContext, EvmTypeValue};
    use evm_assembly::block::EvmBlock;
    use evm_assembly::EvmAssemblyGenerator;
    use evm_assembly::EvmByteCodeBuilder;

    #[test]
    fn blah() {
        let mut specification = EvmCompilerContext::new();
        specification.declare_integer("Int8", 8);
        specification.declare_integer("Int16", 16);
        specification.declare_integer("Int32", 32);
        specification.declare_integer("Int64", 64);
        specification.declare_unsigned_integer("Uint8", 8);
        specification.declare_unsigned_integer("Uint16", 16);
        specification.declare_unsigned_integer("Uint32", 32);
        specification.declare_unsigned_integer("Uint64", 64);
        specification.declare_unsigned_integer("Uint256", 256);

        let _ = specification
            .declare_function("fibonacci", ["Uint256"].to_vec(), "Uint256")
            .attach_runtime(|| test_precompile);

        ///////
        // Executable
        let mut builder = EvmByteCodeBuilder::new(&mut specification);

        builder
            .define_function("hello", ["Uint256"].to_vec(), "Uint256")
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

        let executable = builder.build();
        //        println!("{}", builder.generate_evm_assembly());
        //        println!("Code: {}", hex::encode(executable.clone()));

        let executor = EvmExecutor::new(&specification, executable);
        executor.execute("hello", [EvmTypeValue::Uint32(10)].to_vec());

        assert!(false);
    }
}
