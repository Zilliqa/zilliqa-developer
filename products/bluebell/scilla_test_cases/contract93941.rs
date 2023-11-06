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

        end
        
        "#
    );
}
