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
          IsContractOwner;
        end
"#
    );
}
