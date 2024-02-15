use std::collections::VecDeque;

use crate::{block::EvmBlock, function::EvmFunction};

#[derive(Debug, Clone)]
pub struct EvmBytecodeIr {
    pub functions: VecDeque<EvmFunction>,
    pub data: Vec<(String, Vec<u8>)>,
    pub unused_blocks: Vec<EvmBlock>,
}
impl EvmBytecodeIr {
    pub fn new() -> EvmBytecodeIr {
        return EvmBytecodeIr {
            functions: VecDeque::new(),
            data: Vec::new(),
            unused_blocks: Vec::new(),
        };
    }

    pub fn to_string(&self) -> String {
        let mut script = "Unused blocks:\n\n".to_string();

        let unused_blocks = self
            .unused_blocks
            .iter()
            .map(|block| {
                let code = block
                    .instructions
                    .iter()
                    .map(|instr| {
                        let position = match instr.position {
                            Some(v) => v,
                            None => 0,
                        };
                        let instruction_value = if instr.arguments.len() > 0 {
                            let argument: String = instr
                                .arguments
                                .iter()
                                .map(|byte| format!("{:02x}", byte).to_string())
                                .collect();

                            format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                        } else {
                            instr.opcode.to_string()
                        };

                        format!(
                            "[0x{:02x}: 0x{:02x}] {:<width$}  ;; Stack: {}, Comment: {}, Source: {:?}",
                            position,
                            instr.opcode.as_u8(),
                            instruction_value,
                            instr.stack_size,
                            instr.comment.clone().unwrap_or("".to_string()).trim(),
                            instr.source_position,
                            width = 40
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let position = match block.position {
                    Some(v) => v,
                    None => 0,
                };
                format!(
                    "{}: ;; Starts at 0x{:02x}  u8[{}] \n{}",
                    block.name, position, block.consumes, code
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        script.push_str(&unused_blocks);

        for function in &self.functions {
            let code_blocks = function
                .blocks
                .iter()
                .map(|block| {
                    let code = block
                        .instructions
                        .iter()
                        .map(|instr| {
                            let position = match instr.position {
                                Some(v) => v,
                                None => 0,
                            };

                            let instruction_value = if instr.arguments.len() > 0 {
                                let argument: String = instr
                                    .arguments
                                    .iter()
                                    .map(|byte| format!("{:02x}", byte).to_string())
                                    .collect();

                                format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                            } else {
                                instr.opcode.to_string()
                            };

                            format!(
                                "[0x{:02x}: 0x{:02x}] {:<width$} ;; Stack: {}, Comment: {}, Source: {:?}",
                                position,
                                instr.opcode.as_u8(),
                                instruction_value,
                                instr.stack_size,
                                instr.comment.clone().unwrap_or("".to_string()).trim(),
                                instr.source_position,
                                width = 40
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    let position = match block.position {
                        Some(v) => v,
                        None => 0,
                    };
                    format!(
                        "{}: ;; Starts at 0x{:02x} u8[{}]\n{}",
                        block.name, position, block.consumes, code
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            script.push_str("\n\nFunction:\n");
            script.push_str(&code_blocks);
        }

        script
    }
}
