#[macro_use]
extern crate lalrpop_util;
pub mod ast;

pub mod constants;
pub mod contract_executor;
pub mod errors;
pub mod formatter;
pub mod intermediate_representation;
pub mod passes;

pub mod evm_bytecode_generator;
// pub mod llvm_ir_generator;
pub mod parser;
pub mod support;
pub mod testing;
