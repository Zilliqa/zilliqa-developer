use std::{rc::Rc, str::FromStr};

use evm_assembly::{
    executable::EvmExecutable, observable_machine::ObservableMachine, types::EvmTypeValue,
};
use primitive_types::H256;
use serde_json;

use crate::support::{
    evm::EvmCompiler,
    modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
};

pub fn create_vm_and_run_code(
    function_name: &str,
    args: &str,
    source: String,
    initial_storage: &str,
) -> (ObservableMachine, EvmExecutable) {
    let mut compiler = EvmCompiler::new();
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

    let code = executor.executable.bytecode.clone();
    let arguments: Vec<EvmTypeValue> = if args == "" {
        [].to_vec()
    } else {
        serde_json::from_str(&args).expect("Failed to deserialize arguments")
    };

    let data: Rc<Vec<u8>> = {
        executor
            .context
            .get_function(&function_name)
            .expect(&format!("Function name {} not found", function_name).to_string())
            .generate_transaction_data(arguments)
            .into()
    };

    let source_map = executor.executable.get_source_map();
    let mut vm = ObservableMachine::new(Rc::new(code), data, 1024, 10000, None);
    vm.set_source_map(&source_map);

    let initial_storage: Vec<&str> = initial_storage.lines().collect();
    for record in initial_storage {
        let record = record.trim_start();
        if record.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = record.split(":").map(|s| s.trim()).collect();
        if parts.len() != 2 {
            panic!(
                "Invalid storage record format. Expected 'key: value', got '{}'",
                record
            );
        }

        let key = H256::from_str(&format_hex_string(parts[0]).expect("Invalid key"))
            .expect(&format!("Failed to parse key from '{}'", parts[0]));
        let value = H256::from_str(&format_hex_string(parts[1]).expect("Invalid value"))
            .expect(&format!("Failed to parse value from '{}'", parts[1]));

        vm.storage.insert(key, value);
    }

    vm.run();

    (vm, executor.executable)
}

fn format_hex_string(input: &str) -> Result<String, &'static str> {
    if !input.starts_with("0x") {
        return Err("String does not start with 0x prefix");
    }

    if input.contains("...") {
        // 66 = 2 for "0x" + 64 for H256
        let required_zeros: i64 = 66 - input.len() as i64 + 3; // +3 for the length of "..."

        if required_zeros < 0 {
            return Err("Input string is too long to be a valid H256 value.");
        }

        let zeros = "0".repeat(required_zeros as usize);
        Ok(input.replace("...", &zeros))
    } else {
        Ok(input.to_string())
    }
}

pub fn test_execution_path(
    entry: &str,
    args: &str,
    source: &str,
    initial_storage: &str,
    storage_checks: &str,
) {
    // 1. Extract lines with *:> and their line numbers
    let lines: Vec<&str> = source.lines().collect();
    let mut expected_visited_lines = Vec::new();
    let mut expected_not_visited_lines = Vec::new();
    let mut cleaned_code = String::new();

    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("--|") {
            expected_not_visited_lines.push(index);
            cleaned_code.push_str(&line.replace("--|", ""));
        } else if line.starts_with("-->") {
            expected_visited_lines.push(index);
            cleaned_code.push_str(&line.replace("-->", ""));
        } else {
            cleaned_code.push_str(line);
        }
        cleaned_code.push('\n');
    }

    // 2. Compile and run the cleaned code
    let (vm, _executable) = create_vm_and_run_code(entry, args, cleaned_code, initial_storage);

    // 3. Check if the expected lines were visited
    for line in &expected_visited_lines {
        if vm.did_not_visit_line((*line).try_into().unwrap()) {
            println!("{:#?} vs {:#?}", vm.lines_visited, expected_visited_lines);
            panic!("Expected line {} to be visited but it wasn't.", line);
        }
    }

    for line in &expected_not_visited_lines {
        if vm.did_visit_line((*line).try_into().unwrap()) {
            println!(
                "{:#?} vs {:#?}",
                vm.lines_visited, expected_not_visited_lines
            );
            panic!("Expected line {} to be not visited but it was.", line);
        }
    }

    // 4. Check if the VM's storage contains the expected key-value pairs
    let storage_checks: Vec<&str> = storage_checks.lines().collect();
    for check in storage_checks {
        let check = check.trim_start();
        if check.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = check.split(":").map(|s| s.trim()).collect();
        if parts.len() != 2 {
            panic!(
                "Invalid storage check format. Expected 'key: value', got '{}'",
                check
            );
        }

        let key = H256::from_str(&format_hex_string(parts[0]).expect("Invalid key"))
            .expect(&format!("Failed to parse key from '{}'", parts[0]));
        let value = H256::from_str(&format_hex_string(parts[1]).expect("Invalid value"))
            .expect(&format!("Failed to parse value from '{}'", parts[1]));

        if !vm.has_record(&key, &value) {
            panic!(
                "Storage check failed. Key '{}' does not have expected value '{}'.",
                key, value
            );
        }
    }
}
