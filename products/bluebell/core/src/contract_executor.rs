pub trait ContractExecutor {
    fn execute(&self, name: &str) -> f64;
    fn link_symbol(&self, name: &str, addr: usize);
}

pub trait UnsafeContractExecutor {
    unsafe fn execute(&self, name: &str) -> f64;
    unsafe fn link_symbol(&self, name: &str, addr: usize);
}
