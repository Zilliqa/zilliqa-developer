#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod ast_converting;
pub mod ast_visitor;
pub mod constants;
pub mod formatter;
pub mod highlevel_ir;
pub mod lexer;
pub mod llvm_emitter;
pub mod ng_formatter;
pub mod type_classes;
pub mod type_inference;

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
