use crate::block::EvmBlock;
use crate::function_signature::EvmFunctionSignature;
use evm::Opcode;
use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct EvmFunction {
    pub signature: Option<EvmFunctionSignature>,
    pub selector: Vec<u8>,
    pub blocks: Vec<EvmBlock>,
}

impl EvmFunction {
    pub fn empty() -> Self {
        Self {
            signature: None,
            selector: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn from_signature(signature: EvmFunctionSignature) -> Self {
        let selector = signature.selector();
        Self {
            signature: Some(signature),
            selector,
            blocks: Vec::new(),
        }
    }

    pub fn extract_functions(blocks: &Vec<EvmBlock>) -> (Vec<EvmFunction>, Vec<EvmBlock>) {
        let mut ret: Vec<EvmFunction> = Vec::new();
        let mut current_function = EvmFunction {
            signature: None,
            selector: Vec::new(),
            blocks: Vec::new(),
        };

        // Preparing
        let mut queue: VecDeque<&EvmBlock> = VecDeque::new();
        let mut pos = 0;
        match blocks.first() {
            Some(start) => {
                queue.push_back(start);
                pos += 1;
            }
            None => (),
        };

        let blocks: HashMap<usize, &EvmBlock> = {
            let mut ret: HashMap<usize, &EvmBlock> = HashMap::new();

            while pos < blocks.len() {
                let block = &blocks[pos];
                if let Some(position) = block.position {
                    ret.insert(position, &block);
                }
                pos += 1;
            }

            ret
        };

        let mut used_blocks = HashSet::new();

        // "main" function - extracting function starts
        let mut function_starts: Vec<(Vec<u8>, &EvmBlock)> = Vec::new();
        while !queue.is_empty() {
            let next = queue.pop_front().unwrap();
            used_blocks.insert(next.position);

            let mut last_push_value: Option<u64> = None;
            let mut last_push_args: Option<Vec<u8>> = None;
            let mut signature_value: Option<Vec<u8>> = None;

            for instr in next.instructions.iter() {
                if instr.opcode == Opcode::EQ {
                    signature_value = last_push_args;
                }

                if instr.opcode == Opcode::JUMPI || instr.opcode == Opcode::JUMP {
                    if let Some(position) = last_push_value {
                        let position = position as usize;
                        if let Some(block) = &blocks.get(&position) {
                            if let Some(selector) = signature_value {
                                function_starts.push((selector, &block));
                            } else {
                                queue.push_back(&block);
                            }
                            signature_value = None;
                        }
                    }
                }

                last_push_value = instr.push_value_as_u64();
                last_push_args = instr.push_value();
            }

            current_function.blocks.push(next.clone());
        }

        ret.push(current_function);

        // Extracting other functions
        while !function_starts.is_empty() {
            let (selector, block) = function_starts.pop().unwrap();

            current_function = EvmFunction {
                signature: None,
                selector,
                blocks: Vec::new(),
            };

            queue.push_back(block);
            while !queue.is_empty() {
                let next = queue.pop_front().unwrap();
                used_blocks.insert(next.position);

                let mut last_push_value: Option<u64> = None;
                for instr in next.instructions.iter() {
                    // TODO: Prevent cross function block inclusion (i.e. you call one function from the other)
                    if instr.opcode == Opcode::JUMPI || instr.opcode == Opcode::JUMP {
                        if let Some(position) = last_push_value {
                            let position = position as usize;
                            if let Some(block) = &blocks.get(&position) {
                                queue.push_back(&block);
                            }
                        }
                    }

                    last_push_value = instr.push_value_as_u64();
                }
                current_function.blocks.push(next.clone());
            }
            ret.push(current_function);
        }

        let mut unused_blocks = Vec::new();
        for (position, block) in blocks {
            /*
            // TODO:
            if position == None || !used_blocks.contains(&position.unwrap()) {
                unused_blocks.push(block.clone());
            }
            */
        }

        (ret, unused_blocks)
    }
}
