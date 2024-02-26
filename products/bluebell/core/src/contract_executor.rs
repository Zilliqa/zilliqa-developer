/// `ContractExecutor` is a trait that defines the interface for smart contract execution.
/// This is designed for compilers that create Ethereum Virtual Machine (EVM) and LLVM code, among others.
/// This trait is intended for production code and it is expected that any implementation is sandboxed with
/// a best effort to protect against undesirable side effects.
pub trait ContractExecutor {
    /// Executes the smart contract identified by `name`. This typically involves running the
    /// compiled code associated with the contract. It returns a  byte array
    /// representing the result of the execution if successful and otherwise an error string.
    fn execute(&self, name: &str) -> Result<Vec<u8>, String>;

    /// Links the given `name` to a physical address `addr`. This is typically used to
    /// resolve dynamic dependencies during the execution phase.
    fn link_symbol(&self, name: &str, addr: usize);
}

/// `UnsafeContractExecutor` is a trait similar to `ContractExecutor`, but its methods are marked unsafe.
/// This means that compilers need to make sure they're calling these methods in a safe way, as unsafe
/// blocks signal to the Rust compiler that the programmer has manually ensured correctness despite potential
/// risks of undefined behavior. This trait is intended for testing and rapid prototyping.
/// WARNING! There is no garantuee that implementations of this trait will be safe and it may expose direct access
/// to memory and/or system that should not be allowed in a production system.
pub trait UnsafeContractExecutor {
    /// An `unsafe` version of the `execute` method in the `ContractExecutor` trait. The called
    /// contract is responsible for maintaining safety since this method could potentially work directly
    /// with low-level memory and system operations.
    unsafe fn execute(&self, name: &str) -> Result<Vec<u8>, String>;

    /// An `unsafe` version of the `link_symbol` method in the `ContractExecutor` trait. This method
    /// is marked unsafe since it deals directly with memory addresses and thus requires the caller
    /// to ensure it's used correctly to prevent potential risks.
    unsafe fn link_symbol(&self, name: &str, addr: usize);
}
