#[test]
fn test_type_arg_error() {
    // This test is meant to reproduce the `EnclosedTypeArgument` error
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
            end
            "#
    );
}
