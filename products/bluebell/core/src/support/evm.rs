use crate::ast::nodes::NodeProgram;
use crate::support::modules::BluebellModule;

use crate::evm_bytecode_generator::EvmBytecodeGenerator;

use crate::intermediate_representation::emitter::IrEmitter;
use crate::intermediate_representation::pass_manager::PassManager;

use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executor::EvmExecutor;

pub struct EvmCompiler {
    context: EvmCompilerContext,
    ir_emitter: IrEmitter,
    pass_manager: PassManager,
}

impl EvmCompiler {
    pub fn new() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            ir_emitter: IrEmitter::new(),
            pass_manager: PassManager::default_pipeline(),
        }
    }

    pub fn attach(&mut self, module: &dyn BluebellModule) {
        module.attach(&mut self.context);
    }

    pub fn compile(&self, _script: String) -> Result<Vec<u8>, String> {
        todo!()
    }

    // TODO: Remove &mut self - needs to be removed from a number of places first
    pub fn compile_ast(&mut self, ast: &NodeProgram) -> Result<Vec<u8>, String> {
        let mut ir = self.ir_emitter.emit(ast)?;
        self.pass_manager.run(&mut ir)?;

        let mut generator = EvmBytecodeGenerator::new(&mut self.context, ir);

        generator.build_executable()
    }

    pub fn executable_from_ast(&mut self, ast: &NodeProgram) -> Result<EvmExecutor, String> {
        let executable = self.compile_ast(ast)?;
        Ok(EvmExecutor::new(&self.context, executable))
    }
}
