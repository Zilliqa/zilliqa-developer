#[cfg(test)]
mod tests {
    use evm_assembly::compiler_context::EvmCompilerContext;

    use evm_assembly::EvmByteCodeBuilder;

    #[test]
    fn blah() {
        let mut specification = EvmCompilerContext::new();
        specification.declare_integer("Int8", 8);
        specification.declare_integer("Int16", 16);
        specification.declare_integer("Int32", 32);
        specification.declare_integer("Int64", 64);
        specification.declare_unsigned_integer("Uint8", 8);
        specification.declare_unsigned_integer("Uint16", 16);
        specification.declare_unsigned_integer("Uint32", 32);
        specification.declare_unsigned_integer("Uint64", 64);
        specification.declare_unsigned_integer("Uint256", 256);

        let bytes = hex::decode("608060405234801561001057600080fd5b506004361061002b5760003560e01c80633a19a7c614610030575b600080fd5b61003861004e565b6040516100459190610107565b60405180910390f35b60606000604051806101400160405280610114815260200161012a610114913990508091505090565b600081519050919050565b600082825260208201905092915050565b60005b838110156100b1578082015181840152602081019050610096565b60008484015250505050565b6000601f19601f8301169050919050565b60006100d982610077565b6100e38185610082565b93506100f3818560208601610093565b6100fc816100bd565b840191505092915050565b6000602082019050818103600083015261012181846100ce565b90509291505056fe48656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c642048656c6c6f20576f726c6420a26469706673582212209e44d7f3c5ad5ed44f2d09f524e9aea6f2a72997367b5def3f0952f557cf658864736f6c63430008140033").unwrap();
        let _builder = EvmByteCodeBuilder::from_bytes(&mut specification, bytes);

        assert!(false);
    }
}
