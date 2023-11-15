use std::collections::HashMap;

use evm_assembly::{
    compiler_context::EvmCompilerContext, executable::EvmExecutable, executor::EvmExecutor,
};

use crate::{
    ast::nodes::NodeProgram,
    evm_bytecode_generator::EvmBytecodeGenerator,
    intermediate_representation::{
        ast_queue::AstQueue, emitter::IrEmitter, pass_manager::PassManager,
        symbol_table::SymbolTableConstructor,
    },
    parser::{lexer, lexer::Lexer, parser},
    support::modules::BluebellModule,
};

/// Example implementation of AstQueue.
pub struct SourceImporter {
    queue: Vec<NodeProgram>,
    preloaded_scripts: HashMap<String, String>,
}

impl AstQueue for SourceImporter {
    fn enqueue(&mut self, filename: &str) -> Result<(), String> {
        let script = self.load_script_from_filename(filename)?;
        self.load_script(script)
    }

    fn enqueue_with_alias(&mut self, filename: &str, alias_name: &str) -> Result<(), String> {
        let script = self.load_script_from_filename(filename)?;
        println!("TODO: Alias not implemented");
        self.load_script(script)
    }

    fn pop_front(&mut self) -> Option<NodeProgram> {
        self.queue.pop()
    }
}

impl SourceImporter {
    fn new() -> Self {
        let mut preloaded_scripts = HashMap::new();
        // TODO: Move this such that it is defined in the module.
        preloaded_scripts.insert(
            "ListUtils".to_string(),
            r#"scilla_version 0
            library ListUtils
            contract ListUtils()
            "#
            .to_string(),
        );

        preloaded_scripts.insert(
            "BoolUtils".to_string(),
            r#"scilla_version 0
            library BoolUtils
            contract BoolUtils()
            "#
            .to_string(),
        );

        preloaded_scripts.insert(
            "IntUtils".to_string(),
            r#"scilla_version 0
            library IntUtils
            contract IntUtils()
            "#
            .to_string(),
        );

        preloaded_scripts.insert(
            "IntUtils".to_string(),
            r#"scilla_version 0
            library IntUtils
            contract IntUtils()
            "#
            .to_string(),
        );
        SourceImporter {
            queue: Vec::new(),
            preloaded_scripts,
        }
    }

    fn load_script_from_filename(&self, filename: &str) -> Result<String, String> {
        if let Some(script) = self.preloaded_scripts.get(filename) {
            Ok(script.clone())
        } else {
            std::fs::read_to_string(filename).map_err(|err| format!("{}: {}", err, filename))
        }
    }

    fn load_script(&mut self, script: String) -> Result<(), String> {
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

        self.queue.push(ast);
        Ok(())
    }
}

pub struct EvmCompiler {
    pub context: EvmCompilerContext,
    pass_manager: PassManager,
    abi_support: bool,
    source_importer: SourceImporter,
}

impl EvmCompiler {
    pub fn new() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            pass_manager: PassManager::default_pipeline(),
            abi_support: true,
            source_importer: SourceImporter::new(),
        }
    }

    pub fn new_no_abi_support() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            pass_manager: PassManager::default_pipeline(),
            abi_support: false,
            source_importer: SourceImporter::new(),
        }
    }

    pub fn pass_manager_mut(&mut self) -> &mut PassManager {
        &mut self.pass_manager
    }

    pub fn attach(&mut self, module: &dyn BluebellModule) {
        module.attach(&mut self.context);
    }

    pub fn compile(&mut self, script: String) -> Result<EvmExecutable, String> {
        self.source_importer.load_script(script)?;
        let symbol_table = self.context.new_symbol_table();

        // TODO: Change to while loop. This requires that IRs can be merged
        if let Some(ast) = self.source_importer.pop_front() {
            let ast_queue = &mut self.source_importer;
            let mut ir_emitter = IrEmitter::new(symbol_table, ast_queue);
            let mut ir = ir_emitter.emit(&ast)?;
            self.pass_manager.run(&mut ir)?;
            let mut generator = EvmBytecodeGenerator::new(&mut self.context, ir, self.abi_support);
            generator.build_executable()
        } else {
            Err("No AST found.".to_string())
        }
    }

    // TODO: Remove &mut self - needs to be removed from a number of places first
    pub fn compile_ast(&mut self, ast: &NodeProgram) -> Result<EvmExecutable, String> {
        let symbol_table = self.context.new_symbol_table();
        let ast_queue = &mut self.source_importer;

        let mut ir_emitter = IrEmitter::new(symbol_table, ast_queue);
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
