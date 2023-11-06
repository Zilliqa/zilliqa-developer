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
