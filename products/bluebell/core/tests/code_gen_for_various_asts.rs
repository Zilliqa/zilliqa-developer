#[cfg(test)]
mod tests {
    use bluebell::support::{
        evm::EvmCompiler,
        modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
    };
    use evm_assembly::{executor::ExecutorResult, types::EvmTypeValue};
    use serde_json;

    fn result_to_string(ret: ExecutorResult) -> String {
        let mut result = "".to_string();
        let mut sorted_changeset: Vec<(String, Option<String>)> =
            ret.changeset.into_iter().collect();
        sorted_changeset.sort_by_key(|(key, _)| key.clone());
        for (k, v) in sorted_changeset {
            match v {
                Some(v) => {
                    result.push_str("+");
                    result.push_str(&k);
                    result.push_str("=");
                    result.push_str(&v);
                }
                None => {
                    result.push_str("-");
                    result.push_str(&k);
                }
            }
            result.push_str("\n");
        }

        result.trim().to_string()
    }

    fn compile_scilla_to_evm(script: &str) -> Result<(), String> {
        let mut compiler = EvmCompiler::new();
        let default_types = ScillaDefaultTypes {};
        let default_builtins = ScillaDefaultBuiltins {};
        let debug = ScillaDebugBuiltins {};

        compiler.attach(&default_types);
        compiler.attach(&default_builtins);
        compiler.attach(&debug);
        let executable = compiler.executable_from_script(script.to_string())?;

        Ok(())
    }

    macro_rules! test_compilation_and_evm_code_generation {
        ( $source:expr) => {
            match compile_scilla_to_evm($source) {
                Err(err) => panic!("{}", err),
                _ => (),
            }
        };
    }
    // This test is intended to verify the capability of the Scilla to Rust compiler
    // to handle import statements, library declarations and contract declaration with
    // functions. It minimizes the Scilla code used, focusing mainly on the components that could
    // trigger the `emit_import_declarations` error, including "import" statements.
    #[test]
    fn test_import_handling() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
            import ListUtils
            library TestLib
            contract TestContract()
            "#
        );
    }

    /*
    TODO: Not handled yet
    #[test]
    fn test_alias_import_handling() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
            import ListUtils as HelloWorld
            library TestLib
            contract TestContract()
            "#
        );
    }
    */

    /*
    #[test]
    // Testing the failure when handling NodeTypeNameIdentifier::EventType in the emit_type_name_identifier() function.
    fn test_event_type_not_implemented() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library DummyRefinery

            contract DummyRefinery()

            transition Register(claimer : ByStr20)
            end

            transition Refine(to: ByStr20, amount: Uint128)
            end
    "#);
    }
    */
    /*
    #[test]
    // This test runs the Scilla compilation and evm code generation with an `Empty` transition function
    // It's useful for testing how the compiler handles empty blocks
    fn test_empty_function_body() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library Dummy

            contract Dummy()

            transition Dummy()
            end
    "#);
    }
    */
    /*
    #[test]
    // This test case is used to generate an unimplemented error for the contract_type_arguments of
    // the ConstructorCall enum in the NodeFullExpression. A contract of this nature forces the program
    // to enter the unimplemented!() block.
    fn test_unimplemented_constructor_call() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library ProductLibrary

            contract ProductContract ()
            field products: Map String (ProductType) = Emp String (ProductType)
            "#);
    }
    */

    /*
    #[test]
    fn test_type_arg_error() {
        // This test is meant to reproduce the `TemplateFunction` error
        // by involving a function that uses `Option` Enum and pattern-matching in Scilla
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library TestTypeArgument

            let option_value =
            tfun 'A =>
            fun (default: 'A) =>
            fun (opt_val: Option 'A) =>
                match opt_val with
                | Some v => v
                | None => default
                end

            contract TestTypeArgument()
            "#
        );
    }
    */
    /*
    #[test]
    fn test_emit_library_single_definition_unimplemented() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library TestLibrary

            let zero = Uint32 0

            contract TestContract()
            "#);

        // This test is attempting to trigger the unimplemented!() call in emit_library_single_definition
        // It does this by defining a library with a single let definition.
        // The let definition will cause the emit_library_single_definition method to be called during contract compilation.
        // Since the current implementation of this function cannot handle let definitions, it should trigger the unimplemented error.
    }
    */
    /*
    #[test]
    // This test is meant to reproduce the error caused by the unimplemented match case in the
    // emit_full_expression function.
    fn test_unimplemented_match_case() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library Test

            type Error =
            | CodeNotAuthorized

            let make_error =
            fun (result : Error) =>
                let result_code =
                match result with
                | CodeNotAuthorized => Int32 -2
                end
                in
                { _exception : "Error"; code : result_code }

            contract Test(contract_owner: ByStr20)
            procedure ThrowError(err : Error)
            e = make_error err;
            throw e
            end
            procedure IsContractOwner()
            is_contract_owner = builtin eq _sender contract_owner;
            match is_contract_owner with
            | True =>
            | False =>
                err = CodeNotAuthorized;
                ThrowError err
            end
            end
            transition BlockAddress (wallet: ByStr20)
                IsContractOwner
            end
    "#);
    }
    */
}
