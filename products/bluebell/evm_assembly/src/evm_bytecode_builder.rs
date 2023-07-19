use evm::Opcode;

use std::collections::HashMap;

use crate::block::EvmBlock;
use crate::evm_decompiler::EvmAssemblyGenerator;
use crate::instruction::EvmInstruction;
use crate::opcode_spec::{create_opcode_spec, OpcodeSpecification};

pub struct EvmByteCodeBuilder {
    pub bytecode: Vec<u8>,
    pub labels: HashMap<String, usize>,
    pub blocks: Vec<EvmBlock>,
    pub opcode_specs: HashMap<u8, OpcodeSpecification>,
}

impl EvmByteCodeBuilder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            blocks: Vec::new(),
            labels: HashMap::new(),
            opcode_specs: create_opcode_spec(),
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytecode: bytes,
            blocks: Vec::new(),
            labels: HashMap::new(),
            opcode_specs: create_opcode_spec(),
        }
    }

    pub fn from_asm(_script: &str) -> Self {
        unimplemented!();
    }

    pub fn push_u8(&mut self, opcode: u8) -> &mut Self {
        self.bytecode.push(opcode);
        self
    }
    pub fn push(&mut self, opcode: Opcode) -> &mut Self {
        self.bytecode.push(opcode.as_u8());
        self
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.bytecode.extend_from_slice(bytes);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytecode
    }
}

impl EvmAssemblyGenerator for EvmByteCodeBuilder {
    fn generate_evm_assembly(&self) -> String {
        let mut blocks: Vec<EvmBlock> = Vec::new();
        let mut block_counter = 0;
        let mut current_block = EvmBlock::new(format!("block{}", block_counter).to_string());
        current_block.is_entry = true;
        block_counter += 1;

        let offset = 0; // 2; // First two bytes are [version number] [magic]
        let mut i = offset;
        while i < self.bytecode.len() {
            let spec = match self.opcode_specs.get(&self.bytecode[i]) {
                Some(spec) => spec,
                _ => {
                    if let Some(instr) = current_block.instructions.last() {
                        println!("Last instruction:\n{:#?}", instr);
                        println!("Opcode name: {}", instr.opcode.to_string());
                    }
                    panic!("No spec found for opcode 0x{:02x}", self.bytecode[i]);
                }
            };

            let mut instr = EvmInstruction {
                position: i,
                opcode: Opcode(self.bytecode[i]),
                arguments: Vec::new(),
                stack_consumed: spec.stack_consumed,
                stack_produced: spec.stack_produced,
                is_terminator: spec.is_terminator,
            };

            i += 1;
            let mut collect_args = spec.bytecode_arguments;
            if i + collect_args > self.bytecode.len() {
                panic!("This is not good - we exceed the byte code");
            }

            while collect_args > 0 {
                instr.arguments.push(self.bytecode[i]);
                i += 1;
                collect_args -= 1;
            }

            if instr.opcode == Opcode::JUMPDEST {
                blocks.push(current_block);
                current_block = EvmBlock::new(format!("block{}", block_counter).to_string());
                block_counter += 1;
            }

            current_block.instructions.push(instr);

            // A terminated block followed by an invalid opcode starts the data section.
            // TODO: Find some spec to confirm this assumption
            if spec.is_terminator {
                if Opcode(self.bytecode[i]) == Opcode::INVALID {
                    i += 1;
                    // Encountered the auxilary data section
                    break;
                }
            }
        }
        println!("--Done!");
        let mut data: Vec<u8> = Vec::new();
        while i < self.bytecode.len() {
            data.push(self.bytecode[i]);
            i += 1;
        }

        blocks.push(current_block);

        let code_blocks = blocks
            .iter()
            .map(|block| {
                let code = block
                    .instructions
                    .iter()
                    .map(|instr| {
                        if instr.arguments.len() > 0 {
                            let argument: String = instr
                                .arguments
                                .iter()
                                .map(|byte| format!("{:02x}", byte).to_string())
                                .collect();
                            format!(
                                "[0x{:02x}: 0x{:02x}] {} 0x{}",
                                instr.position,
                                instr.opcode.as_u8(),
                                instr.opcode.to_string(),
                                argument
                            )
                        } else {
                            format!(
                                "[0x{:02x}: 0x{:02x}] {}",
                                instr.position,
                                instr.opcode.as_u8(),
                                instr.opcode.to_string()
                            )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{}:\n{}", block.name, code)
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let data: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();

        format!("{}\n\nauxdata: 0x{}", code_blocks, data)
    }
}
