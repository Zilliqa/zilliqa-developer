#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use bluebell::support::{
        evm::EvmCompiler,
        modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
    };
    use evm_assembly::{
        executable::EvmExecutable, executor::ExecutorResult, observable_machine::ObservableMachine,
        types::EvmTypeValue,
    };
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

    fn compile_and_execute(
        entry_point: &str,
        args: &str,
        script: &str,
    ) -> Result<ExecutorResult, String> {
        let mut compiler = EvmCompiler::new_no_abi_support();
        compiler.pass_manager_mut().enable_debug_printer();

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

    macro_rules! test_compile_and_execute {
        ($entry:expr, $args:expr, $source:expr, $expected:expr) => {
            match compile_and_execute($entry, $args, $source) {
                Ok(result) => {
                    let result_str = result_to_string(result);
                    assert_eq!($expected.to_string(), result_str);
                }
                Err(err) => panic!("{}", err),
            }
        };
    }

    fn create_vm_and_run_code(source: String) -> (ObservableMachine, EvmExecutable) {
        let mut compiler = EvmCompiler::new_no_abi_support();
        compiler.pass_manager_mut().enable_debug_printer();

        let default_types = ScillaDefaultTypes {};
        let default_builtins = ScillaDefaultBuiltins {};
        let debug = ScillaDebugBuiltins {};

        compiler.attach(&default_types);
        compiler.attach(&default_builtins);
        compiler.attach(&debug);
        let executor = compiler
            .executable_from_script(source)
            .expect("Failed to compile source");
        println!(
            "Produced code: {}",
            hex::encode(&executor.executable.bytecode.clone())
        );

        let data = "00";
        let code = executor.executable.bytecode.clone();
        let data = hex::decode(data).unwrap();

        println!("Executable code: {:#}", hex::encode(code.clone()));

        let mut vm = ObservableMachine::new(Rc::new(code), Rc::new(data), 1024, 10000, None);
        vm.run();

        (vm, executor.executable)
    }

    macro_rules! expect_was_visited {
        ($vm:expr, $executable:expr, $label:expr) => {
            if let Some(label) = $executable.get_label_position($label) {
                assert!($vm.did_visit_program_counter(label));
            } else {
                panic!("{}", format!("Label '{}' not found.", $label));
            }
        };
    }

    macro_rules! expect_not_visited {
        ($vm:expr, $executable:expr, $label:expr) => {
            if let Some(label) = $executable.get_label_position($label) {
                assert!($vm.did_not_visit_program_counter(label));
            } else {
                panic!("{}", format!("Label '{}' not found.", $label));
            }
        };
    }

    #[test]
    fn test_set_true_path_in_match_nobody() {
        let (vm, executable) = create_vm_and_run_code(
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()
transition setHello ()
	x = True;
	match x with
	| True =>
	end
end
"#
            .to_string(),
        );
        println!("{:#?}", executable.label_positions);
        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_condition_1");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_block_2");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::match_exit_0");
        expect_was_visited!(vm, executable, "__entry_function__::success");
    }

    #[test]
    fn test_set_false_path_in_match_nobody() {
        let (vm, executable) = create_vm_and_run_code(
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()
transition setHello ()
	x = False;
	match x with
	| True =>
	end
end
"#
            .to_string(),
        );
        println!("{:#?}", executable.label_positions);
        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_condition_1");
        expect_not_visited!(vm, executable, "HelloWorld::setHello::clause_0_block_2");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::match_exit_0");
        expect_was_visited!(vm, executable, "__entry_function__::success");
    }

    #[test]
    fn test_set_true_path_in_match_nobody_multi_choice() {
        let (vm, executable) = create_vm_and_run_code(
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()
transition setHello ()
	x = True;
	match x with
	| True =>
	| False =>	
	end
end
"#
            .to_string(),
        );
        println!("{:#?}", executable.label_positions);
        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_condition_1");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_block_2");
        expect_not_visited!(vm, executable, "HelloWorld::setHello::clause_1_block_4");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::match_exit_0");
        expect_was_visited!(vm, executable, "__entry_function__::success");
    }

    #[test]
    fn test_cset_false_path_in_match_nobody_multi_choice() {
        let (vm, executable) = create_vm_and_run_code(
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()
transition setHello ()
	x = False;
	match x with
	| True =>
	| False =>	
	end
end
"#
            .to_string(),
        );
        println!("{:#?}", executable.label_positions);
        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_condition_1");
        expect_not_visited!(vm, executable, "HelloWorld::setHello::clause_0_block_2");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_1_block_4");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::match_exit_0");
        expect_was_visited!(vm, executable, "__entry_function__::success");
    }

    #[test]
    fn test_set_false_path_in_match_nobody_multi_choice_with_block() {
        let (vm, executable) = create_vm_and_run_code(
            r#"scilla_version 0
library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()
transition setHello ()
	msg = Uint64 12;
	x = False;
	match x with
	  | True =>
	    print msg
	  | False =>
	    print msg;
	    print msg
	end
end
"#
            .to_string(),
        );
        println!("{:#?}", executable.label_positions);
        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_0_condition_1");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_1_condition_3");
        expect_not_visited!(vm, executable, "HelloWorld::setHello::clause_0_block_2");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::clause_1_block_4");
        expect_was_visited!(vm, executable, "HelloWorld::setHello::match_exit_0");
        expect_was_visited!(vm, executable, "__entry_function__::success");
    }

    #[test]
    fn test_std_out() {
        test_compile_and_execute!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0

contract HelloWorld()

transition setHello ()
  msg = Uint64 12;
  print msg;
  print msg
end
"#,
            ""
        );
        // TODO: test output - requires a new module
        // assert!(false);
    }

    #[test]
    fn test_match_std_out() {
        test_compile_and_execute!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0

library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()

transition setHello ()
  msg = Uint64 12;

  is_owner = False;
  match is_owner with
  | True =>
    print msg
  | False =>
    print msg;
    print msg
  end

end
"#,
            ""
        );
        // TODO: test output - requires a new module
        //        assert!(false);
    }

    #[test]
    fn test_single_statement_in_match() {
        test_compile_and_execute!(
            "HelloWorld::setHello",
            "",
            r#"scilla_version 0

library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()

transition setHello (msg : Uint64)
  is_owner = True;
  match is_owner with
  | True =>
    print msg
  end
end
"#,
            ""
        );
    }

    #[test]
    fn test_set_match_uint() {
        test_compile_and_execute!(
            "HelloWorld::setHello",
            "",
            r#"scilla_version 0
library HelloWorld

contract HelloWorld()
transition setHello ()
	x = Uint64 1
end
"#,
            ""
        );
    }

    #[test]
    fn test_redefinition_of_variables() {
        test_compile_and_execute!(
            "HelloWorld::setHello",
            "[42]",
            r#"scilla_version 0

contract HelloWorld()
field welcome_msg : Uint64 = Uint64 0

transition setHelloImpl (msg : Uint64)
    welcome_msg := msg
end

transition setHello (msg : Uint64)
  setHelloImpl msg;
  msg <- welcome_msg;
  print msg
end
"#,
            ""
        );
        // TODO: test output - requires a new module
        // assert!(false);
    }
}
