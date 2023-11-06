#[test]
// This test is intended to verify the capability of the Scilla to Rust compiler
// to handle import statements, library declarations and contract declaration with
// functions. It minimizes the Scilla code used, focusing mainly on the components that could
// trigger the `emit_import_declarations` error, including "import" statements.
fn test_import_handling() {
    test_compilation_and_evm_code_generation!(
        r#"scilla_version 0
            import ListUtils
            library TestLib
            contract TestContract()
            end
            "#
    );
}
