#[cfg(test)]
mod tests {
    use evm_assembly::compiler_context::EvmCompilerContext;
    use evm_assembly::EvmAssemblyGenerator;
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

        let bytes = hex::decode("6100ff61000f6000396100ff6000f3600436101561000d57610059565b5f3560e01c3415156100ee576361047ff4811415610053576024361015156100ee57610034565b60205f600401355f6060015261004a60e061006a565b60e0610051565bf35b50610059565b5f5ffd5b604051815250610068565b565b606051608052600160605111156100e0576060516001810381811115156100ee5780905090505f6040015261009f60a061005d565b60a0516060516002810381811115156100ee5780905090505f604001526100c660c061005d565b60c05180820182811015156100ee57809050905090506080525b6080518152506100eb565b56005b5f80fda165767970657283000309000b").unwrap();
        let builder = EvmByteCodeBuilder::from_bytes(&mut specification, bytes);

        println!("{}", builder.generate_evm_assembly());
        assert!(false);
    }
}
