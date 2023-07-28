#[macro_use]
extern crate lalrpop_util;
pub mod ast;

pub mod constants;
pub mod contract_executor;
pub mod formatter;
pub mod intermediate_representation;
pub mod passes;

pub mod evm_ir_generator;

pub mod intermediate_name_generator;
pub mod lexer;
pub mod llvm_ir_generator;
pub mod support;
pub mod symbol_table;

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
