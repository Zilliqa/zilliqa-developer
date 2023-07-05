#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod ast_converting;
pub mod ast_visitor;
pub mod constants;
pub mod formatter;
pub mod highlevel_ir;
pub mod highlevel_ir_pass;
pub mod highlevel_ir_pass_executor;
pub mod passes;

// pub mod highlevel_ir_pass_manager;

pub mod highlevel_ir_debug_printer;
pub mod highlevel_ir_emitter;
pub mod intermediate_name_generator;
pub mod lexer;
pub mod llvm_ir_generator;
pub mod symbol_table;

// pub mod type_classes;
// pub mod type_inference;

lalrpop_mod!(pub parser);

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} (line {}, column {})",
            self.message, self.line, self.column
        )
    }
}
