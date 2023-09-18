use std::collections::HashMap;

use crate::bytecode_ir::EvmBytecodeIr;

pub type TypeSourceMap = HashMap<usize, (usize, usize, usize, usize)>;

#[derive(Debug, Clone)]
pub struct EvmExecutable {
    pub bytecode: Vec<u8>,
    pub label_positions: HashMap<String, u32>,
    pub ir: EvmBytecodeIr, // TODO: add abi
}

impl EvmExecutable {
    pub fn get_label_position(&self, label: &str) -> Option<u32> {
        self.label_positions.get(label).copied()
    }

    pub fn get_source_map(&self) -> TypeSourceMap {
        let mut ret = HashMap::<usize, (usize, usize, usize, usize)>::new();

        let functions = &self.ir.functions;
        for function in functions {
            for block in &function.blocks {
                for instr in &block.instructions {
                    let pc = match &instr.position {
                        Some(p) => p,
                        None => continue,
                    };
                    let source_pos = match &instr.source_position {
                        Some(p) => (p.start, p.end, p.line, p.column),
                        None => continue,
                    };
                    ret.insert(*pc as usize, source_pos);
                }
            }
        }
        ret
    }
}
