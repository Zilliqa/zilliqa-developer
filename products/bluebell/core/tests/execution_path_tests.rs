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

    fn compile_and_execute_full_evm(
        entry_point: &str,
        args: &str,
        script: &str,
    ) -> Result<ExecutorResult, String> {
        let mut compiler = EvmCompiler::new();
        let default_types = ScillaDefaultTypes {};
        let default_builtins = ScillaDefaultBuiltins {};
        let debug = ScillaDebugBuiltins {};

        compiler.attach(&default_types);
        compiler.attach(&default_builtins);
        compiler.attach(&debug);
        let executable = compiler.executable_from_script(script.to_string())?;

        let arguments: Vec<EvmTypeValue> = if args == "" {
            [].to_vec()
        } else {
            serde_json::from_str(&args).expect("Failed to deserialize arguments")
        };

        let ret = executable.execute(&entry_point, arguments);

        Ok(ret)
    }

    macro_rules! test_compile_and_execute_full_evm {
        ($entry:expr, $args:expr, $source:expr, $expected:expr) => {
            match compile_and_execute_full_evm($entry, $args, $source) {
                Ok(result) => {
                    let result_str = result_to_string(result);
                    assert_eq!($expected.to_string(), result_str);
                }
                Err(err) => panic!("{}", err),
            }
        };
    }

    #[test]
    fn test_set_state_uint() {
        test_compile_and_execute_full_evm!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0
library HelloWorld
contract HelloWorld()

field welcome_msg : Uint64 = Uint64 0
transition setHello (msg : Uint64)
  welcome_msg := msg   
end
"#,
            "+0x1000000000000000000000000000000000000000.0x0000000000000000000000000000000000000000000000000000000000001337=0x000000000000000000000000000000000000000000000000000000000000002a"
        );
    }

    #[test]
    fn test_conditional_set_state_uint_negative() {
        test_compile_and_execute_full_evm!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()

field welcome_msg : Uint64 = Uint64 0
transition setHello (msg : Uint64)
  is_owner = False;
  match is_owner with
    | True => welcome_msg := msg  
  end
end
"#,
            ""
        );
    }

    #[test]
    fn test_conditional_set_state_uint_positive() {
        test_compile_and_execute_full_evm!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()

field welcome_msg : Uint64 = Uint64 0
transition setHello (msg : Uint64)
  is_owner = True;
  match is_owner with
    | True => welcome_msg := msg  
  end
end
"#,
            "+0x1000000000000000000000000000000000000000.0x0000000000000000000000000000000000000000000000000000000000001337=0x000000000000000000000000000000000000000000000000000000000000002a"
        );
    }
}
