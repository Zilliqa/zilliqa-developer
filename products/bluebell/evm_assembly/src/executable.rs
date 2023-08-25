use std::collections::HashMap;

use crate::bytecode_ir::EvmBytecodeIr;

#[derive(Debug, Clone)]
pub struct EvmExecutable {
    pub bytecode: Vec<u8>,
    pub label_positions: HashMap<String, usize>,
    pub ir: EvmBytecodeIr, // TODO: add abi
}

impl EvmExecutable {
    pub fn get_label_position(&self, label: &str) -> Option<usize> {
        self.label_positions.get(label).copied()
    }
}
