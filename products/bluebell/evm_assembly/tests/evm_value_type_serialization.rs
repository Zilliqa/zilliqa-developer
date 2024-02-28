#[cfg(test)]
mod tests {
    use evm_assembly::types::EvmTypeValue;

    #[test]
    fn blah() {
        let values: Vec<EvmTypeValue> = vec![
            EvmTypeValue::Uint64(123),
            EvmTypeValue::String("Hello".to_string()),
            EvmTypeValue::Uint64(11),
        ];
        // TODO: Update the serialization format
        let json_string = serde_json::to_string(&values).expect("Failed to serialize");
        assert!(json_string == "[123,\"Hello\",11]");

        let deserialized: Vec<EvmTypeValue> =
            serde_json::from_str(&json_string).expect("Failed to deserialize");
        assert!(deserialized == values);
    }
}
