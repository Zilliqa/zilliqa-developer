#[test]
// Testing the failure when handling NodeTypeNameIdentifier::EventType in the emit_type_name_identifier() function.
fn test_event_type_not_implemented() {
    test_compilation_and_evm_code_generation!(
        r#"scilla_version 0

        library DummyRefinery

        contract DummyRefinery()
        
        transition Register(claimer : ByStr20)
        end

        transition Refine(to: ByStr20, amount: Uint128)
        end
"#
    );
}
