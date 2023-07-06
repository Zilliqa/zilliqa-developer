use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;

use inkwell::OptimizationLevel;

pub struct UnsafeLlvmTestExecutor<'ctx, 'module> {
    execution_engine: ExecutionEngine<'ctx>,
    module: &'module mut Module<'ctx>,
}

impl<'ctx, 'module> UnsafeLlvmTestExecutor<'ctx, 'module> {
    pub fn new(module: &'module mut Module<'ctx>) -> Self {
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Unable to create execution engine");
        UnsafeLlvmTestExecutor {
            execution_engine,
            module,
        }
    }
}

pub trait ContractExecutor {
    fn execute(&self, name: &str) -> f64;
    fn link_symbol(&self, name: &str, addr: usize);
}

pub trait UnsafeContractExecutor {
    unsafe fn execute(&self, name: &str) -> f64;
    unsafe fn link_symbol(&self, name: &str, addr: usize);
}

impl<'ctx, 'module> UnsafeContractExecutor for UnsafeLlvmTestExecutor<'ctx, 'module> {
    unsafe fn execute(&self, name: &str) -> f64 {
        let function = self
            .execution_engine
            .get_function::<unsafe extern "C" fn() -> f64>(name)
            .expect("Unable to find the function");
        function.call()
    }

    unsafe fn link_symbol(&self, name: &str, addr: usize) {
        let function = self
            .module
            .get_function(name)
            .expect("Unable to link the function");
        self.execution_engine
            .add_global_mapping(&function, addr as usize);
    }
}
