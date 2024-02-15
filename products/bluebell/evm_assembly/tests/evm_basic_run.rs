use std::{collections::BTreeMap, str::FromStr};

use evm::{
    backend::Backend,
    executor::stack::{PrecompileFailure, PrecompileFn, PrecompileOutput, PrecompileOutputType},
    Context, ExitError, ExitSucceed,
};
use evm_assembly::{compiler_context::EvmCompilerContext, types::EvmTypeValue};
use primitive_types::H160;

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
    use std::str::FromStr;

    use evm_assembly::{executor::EvmExecutor, types::EvmType, EvmByteCodeBuilder};

    use crate::{test_precompile, EvmCompilerContext, EvmTypeValue};

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
        let mut builder = EvmByteCodeBuilder::new(&mut specification, true);

        builder
            .define_function("hello", ["Uint256"].to_vec(), "Uint256")
            .build(|code_builder| {
                let mut entry = code_builder.new_evm_block("entry");
                let mut success = code_builder.new_evm_block("success");
                let mut finally = code_builder.new_evm_block("finally");

                // EvmBlock::new(None, [].to_vec().into_iter().collect(), "entry");

                let fnc = code_builder.context.get_function("fibonacci").unwrap();
                entry.call(
                    fnc,
                    ["Uint256".to_string()]
                        .to_vec()
                        .iter()
                        .map(|s| EvmType::from_str(s).unwrap())
                        .collect(),
                );

                entry.push1([1].to_vec());
                entry.jump_if_to(&success.name);
                entry.jump_to(&finally.name);

                success.jump_to(&finally.name);

                finally.r#return();
                [entry, success, finally].to_vec()
            });

        let executable = builder.build();

        let executor = EvmExecutor::new(&specification, executable);
        executor.execute("hello", [EvmTypeValue::Uint32(10)].to_vec());

        assert!(false);
    }
}
