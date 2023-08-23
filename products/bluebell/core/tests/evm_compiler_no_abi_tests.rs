use evm::Capture;
use evm::ExitReason;
use evm::Machine;
use evm::Trap;
use std::collections::HashMap;
use std::rc::Rc;

struct ObservableMachine {
    pub machine: Machine,
    pub positions_visited: HashMap<usize, usize>,
}

impl ObservableMachine {
    /// Create a new machine with given code and data.
    pub fn new(
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
        stack_limit: usize,
        memory_limit: usize,
    ) -> Self {
        Self {
            machine: Machine::new(code, data, stack_limit, memory_limit),
            positions_visited: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.machine.step() {
                Ok(()) => (),
                Err(_res) => return,
            }
            if let Ok(p) = self.machine.position() {
                if let Some(value) = self.positions_visited.get_mut(p) {
                    *value = *value + 1;
                } else {
                    self.positions_visited.insert(*p, 1);
                }
            }
        }
    }

    pub fn did_visit_program_counter(&self, pc: usize) -> bool {
        None != self.positions_visited.get(&pc)
    }

    pub fn did_not_visit_program_counter(&self, pc: usize) -> bool {
        None == self.positions_visited.get(&pc)
    }
}

#[cfg(test)]
mod tests {
    use super::ObservableMachine;
    use bluebell::support::evm::EvmCompiler;
    use bluebell::support::modules::ScillaDebugBuiltins;
    use bluebell::support::modules::ScillaDefaultBuiltins;
    use bluebell::support::modules::ScillaDefaultTypes;
    use evm_assembly::executor::EvmExecutable;
    use evm_assembly::executor::ExecutorResult;
    use evm_assembly::types::EvmTypeValue;
    use serde_json;

    use evm::{Capture, ExitSucceed, Machine};
    use std::rc::Rc;

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
        let data = "00";
        let code = executor.executable.bytecode.clone();
        let data = hex::decode(data).unwrap();

        println!("Executable code: {:#}", hex::encode(code.clone()));

        let mut vm = ObservableMachine::new(Rc::new(code), Rc::new(data), 1024, 10000);
        vm.run();

        (vm, executor.executable)
    }

    macro_rules! expect_was_visited {
        ($vm:expr, $executable:expr, $label:expr) => {
            assert!($vm.did_visit_program_counter($executable.get_label_position($label).unwrap()));
        };
    }

    macro_rules! expect_not_visited {
        ($vm:expr, $executable:expr, $label:expr) => {
            assert!(
                $vm.did_not_visit_program_counter($executable.get_label_position($label).unwrap())
            );
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

        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "clause_0_condition_1");
        expect_was_visited!(vm, executable, "clause_0_block_2");
        expect_was_visited!(vm, executable, "match_exit_0");
        expect_was_visited!(vm, executable, "success");
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

        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "clause_0_condition_1");
        expect_not_visited!(vm, executable, "clause_0_block_2");
        expect_was_visited!(vm, executable, "match_exit_0");
        expect_was_visited!(vm, executable, "success");
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

        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "clause_0_condition_1");
        expect_was_visited!(vm, executable, "clause_0_block_2");
        expect_not_visited!(vm, executable, "clause_1_block_4");
        expect_was_visited!(vm, executable, "match_exit_0");
        expect_was_visited!(vm, executable, "success");
    }

    #[test]
    fn test_set_false_path_in_match_nobody_multi_choice() {
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

        expect_was_visited!(vm, executable, "HelloWorld::setHello");
        expect_was_visited!(vm, executable, "clause_0_condition_1");
        expect_not_visited!(vm, executable, "clause_0_block_2");
        expect_was_visited!(vm, executable, "clause_1_block_4");
        expect_was_visited!(vm, executable, "match_exit_0");
        expect_was_visited!(vm, executable, "success");
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
}
