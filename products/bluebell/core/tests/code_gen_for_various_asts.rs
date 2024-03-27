#[cfg(test)]
mod tests {
    use bluebell::support::{
        evm::EvmCompiler,
        modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
    };
    use evm_assembly::executor::ExecutorResult;

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
        let _executable = compiler.executable_from_script(script.to_string())?;

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
    "#
        );
    }

    // TODO: Fix ByteStr #[test]
    // Testing the failure when handling NodeTypeNameIdentifier::EventType in the emit_type_name_identifier() function.
    fn test_byte_str_not_implemented() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library DummyRefinery

            contract DummyRefinery()

            transition Register(claimer : ByStr20)
                x = claimer
            end

    "#
        );
    }

    #[test]
    // This test case is designed to reproduce a "not implemented" error about 'EnclosedTypeArguments' in Emitter.
    fn test_global_definition() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library TestLibrary

            let zero = Uint128 0
            contract TestLibrary()

            transition TestTransition()
                accept
            end
            "#
        );
    }

    // TODO: Emitter works, but code generator not working #[test]
    fn test_generic_constructor_call() {
        test_compilation_and_evm_code_generation!(
            r#"
            scilla_version 0
    
            library TestLibrary
            let zero_msg = Nil {Message}
            
            contract Test()
    "#
        );

        // Tests that the compiler can handle a constructor call that
        // uses generic type arguments.
        assert!(false)
    }

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

    // TODO: Fix #[test]
    // This test case is designed to reproduce a "not implemented" error about 'EnclosedTypeArguments' in Emitter.
    fn test_enclosed_type_argument_error() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library TestLibrary

            type ExampleType =
            | ExampleType of ByStr20 Uint128 

            let zero = Uint128 0
            contract TestLibrary()

            transition TestTransition(item: ExampleType)
              match item with
              | ExampleType account amount =>
                msg = {_tag : "TestTag"; _recipient : account; _amount : zero; 
                      account : account; amount: amount} 
              end
            end
            "#
        );
    }

    // TODO: Fix template instantiation #[test]
    // This test is trying to compile a scilla contract with an address argument type.
    // The Scilla code we are testing with has a piece instance `None {ByStr20}` which
    // is processed as an `AddressTypeArgument` in the Rust intermediary representation of Scilla.
    // Currently, in Rust our Scilla interpreter/compiler doesn't support `AddressTypeArgument`s
    // hence it should panic with a `not implemented` error.
    fn test_address_argument_type() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
            library Test
            contract Test()
            field owner : Option ByStr20 = None {ByStr20}
        "#
        );
        // Here we are expecting the test to panic hence we don't have any assertions
    }

    // TODO: #[test]
    fn test_map_key_type_not_implemented() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            contract TestContract(
                init_owner: ByStr20
            )
            field administrators : Map ByStr20 String = Emp ByStr20 String
    "#
        );
        // This test is validating the panic caused by unimplemented handling
        // of Map types declared in the field of a contract
    }

    // TODO: Fix this - it requires a full type deduction
    // #[test]
    fn test_unimplemented_message_error() {
        // This test checks an exception which is thrown
        // when a Message literal is encountered in AST
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
    
            library Example
        
            contract Example()
            field message : Uint64 = Uint64 0
    
            transition setMessage (msg: Uint64)
              zero = Uint64 0;
              test = Uint64 42;
              is_owner = builtin eq msg test;
              test2 = False;
              is_false = builtin eq test2 is_owner;
              match is_false with
              | True =>
                msg = {_recipient : zero; _tag: "Error1"; _amount: zero};
                message := zero
              | _ =>
                msg = {_recipient : zero; _tag: "Error2"; _amount: zero};
                message := msg 
              end
            end
        "#
        );
    }

    // TODO: #[test]
    // This test case is testing the handling of aliased imports (a feature not yet implemented),
    // which cause a panic in our Scilla compiler written in Rust.
    fn test_aliased_import() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
            import BoolUtils as BoolU
            library Test
            
            contract Test()
            field test_field : Uint64 = Uint64 0
    
            transition testTransition (msg: Uint64)
              zero = Uint64 0;
              test = Uint64 42;
              is_owner = BoolU.eq msg test;
              test2 = False;
              is_false = BoolU.eq test2 is_owner;
              match is_false with
              | True =>
                test_field := zero
              | _ =>
                test_field := msg 
              end
            end
            "#
        );

        // Expected output: a defined behaviour about how to handle aliased imports.
        // Current output: thread 'main' panicked at core/src/intermediate_representation/emitter.rs:400:17:
        // not implemented
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }

    // TODO: #[test]
    // This test is to check how Rust handles an unimplemented feature:
    // Generic types with arguments (GenericTypeWithArgs) using Scilla's "Option" and "Uint128" types
    fn test_generic_type_with_args() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0
               
             library Test
            
             let get_var_value =
                fun( var : Option Uint128) =>
                match var with
                | Some x => x
                | None => Uint128 0
                end

            contract Test()
            "#
        );
    }

    // TODO: #[test]
    // This test case is used to generate an unimplemented error for the contract_type_arguments of
    // the ConstructorCall enum in the NodeFullExpression. A contract of this nature forces the program
    // to enter the unimplemented!() block.
    fn test_unimplemented_constructor_call() {
        test_compilation_and_evm_code_generation!(
            r#"scilla_version 0

            library ProductLibrary

            contract ProductContract ()
            field products: Map String (ProductType) = Emp String (ProductType)
            "#
        );
    }

    // TODO: #[test]
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

    // TODO: #[test]
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
    "#
        );
    }
}
