#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod formatter;
pub mod lexer;
pub mod type_classes;
pub mod type_inference;
lalrpop_mod!(pub parser);

use crate::formatter::ScillaFormatter;
use crate::lexer::Lexer;
use crate::type_inference::*;
use std::collections::HashMap;

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
