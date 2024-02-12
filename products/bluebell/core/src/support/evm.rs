use evm_assembly::{
    compiler_context::EvmCompilerContext, executable::EvmExecutable, executor::EvmExecutor,
};
use scilla_parser::{
    ast::nodes::NodeProgram,
    parser::{lexer, lexer::Lexer, parser},
};

use crate::{
    evm_bytecode_generator::EvmBytecodeGenerator,
    intermediate_representation::{
        emitter::IrEmitter, pass_manager::PassManager, symbol_table::SymbolTableConstructor,
    },
    support::modules::BluebellModule,
};

pub struct EvmCompiler {
    pub context: EvmCompilerContext,
    pass_manager: PassManager,
    abi_support: bool,
}

impl EvmCompiler {
    pub fn new() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            pass_manager: PassManager::default_pipeline(),
            abi_support: true,
        }
    }

    pub fn new_no_abi_support() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
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
        let symbol_table = self.context.new_symbol_table();
        let mut ir_emitter = IrEmitter::new(symbol_table);
        let mut ir = ir_emitter.emit(ast)?;
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
