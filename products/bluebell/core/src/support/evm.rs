use crate::ast::nodes::NodeProgram;
use crate::support::modules::BluebellModule;

use crate::evm_bytecode_generator::EvmBytecodeGenerator;

use crate::intermediate_representation::emitter::IrEmitter;
use crate::intermediate_representation::pass_manager::PassManager;

use crate::parser::lexer;
use crate::parser::lexer::Lexer;
use crate::parser::parser;
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executor::{EvmExecutable, EvmExecutor};

pub struct EvmCompiler {
    context: EvmCompilerContext,
    ir_emitter: IrEmitter,
    pass_manager: PassManager,
    abi_support: bool,
}

impl EvmCompiler {
    pub fn new() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            ir_emitter: IrEmitter::new(),
            pass_manager: PassManager::default_pipeline(),
            abi_support: true,
        }
    }

    pub fn new_no_abi_support() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            ir_emitter: IrEmitter::new(),
            pass_manager: PassManager::default_pipeline(),
            abi_support: false,
        }
    }

    pub fn pass_manager_mut(&mut self) -> &mut PassManager {
        &mut self.pass_manager
    }

    pub fn attach(&mut self, module: &dyn BluebellModule) {
        module.attach(&mut self.context);
    }

    pub fn compile(&mut self, script: String) -> Result<EvmExecutable, String> {
        let mut errors: Vec<lexer::ParseError> = [].to_vec();
        let lexer = Lexer::new(&script);
        let parser = parser::ProgramParser::new();
        let ast = match parser.parse(&mut errors, lexer) {
            Ok(ast) => ast,
            Err(error) => {
                let message = format!("Syntax error {:?}", error);
                return Err(message.to_string());
            }
        };

        self.compile_ast(&ast)
    }

    // TODO: Remove &mut self - needs to be removed from a number of places first
    pub fn compile_ast(&mut self, ast: &NodeProgram) -> Result<EvmExecutable, String> {
        let mut ir = self.ir_emitter.emit(ast)?;
        self.pass_manager.run(&mut ir)?;

        let mut generator = EvmBytecodeGenerator::new(&mut self.context, ir, self.abi_support);

        generator.build_executable()
    }

    pub fn executable_from_ast(&mut self, ast: &NodeProgram) -> Result<EvmExecutor, String> {
        let executable = self.compile_ast(ast)?;
        Ok(EvmExecutor::new(&self.context, executable))
    }

    pub fn executable_from_script(&mut self, script: String) -> Result<EvmExecutor, String> {
        let executable = self.compile(script)?;
        Ok(EvmExecutor::new(&self.context, executable))
    }
}
