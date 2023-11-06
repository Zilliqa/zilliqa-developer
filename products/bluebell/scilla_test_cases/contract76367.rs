#[test]
// Test the handling of a Map type in field declarations
fn test_map_field_type() {
    test_compilation_and_evm_code_generation!(
        r#"scilla_version 0

        library Dummy
        contract Dummy()
        field _map : Map Uint256 Uint256 = Emp Uint256 Uint256 
"#
    );
}
