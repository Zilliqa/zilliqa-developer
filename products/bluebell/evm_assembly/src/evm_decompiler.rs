pub trait EvmAssemblyGenerator {
    fn generate_evm_assembly(&self) -> String;
}
