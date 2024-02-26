use std::collections::{HashMap, HashSet, VecDeque};

use evm::Opcode;

use crate::{block::EvmBlock, function_signature::EvmFunctionSignature};

#[derive(Debug, Clone)]
pub struct EvmFunction {
    pub signature: Option<EvmFunctionSignature>,
    pub selector: Vec<u8>,
    pub blocks: Vec<EvmBlock>,
    pub consumes: i32,
    pub produces: i32,
}

impl EvmFunction {
    /// Function that computes how many stack elements are consumed and produced
    /// as well as checking stack integrety for block jumps
    pub fn compute_stack_difference(&mut self) -> Result<(), String> {
        let function_name = self.signature.clone().unwrap().name;
        let mut block_map: HashMap<String, &EvmBlock> = HashMap::new();
        for block in self.blocks.iter() {
            if block_map.contains_key(&block.name) {
                return Err(format!("Multiple blocks with name {}", block.name));
            }
            block_map.insert(block.name.clone(), &block);
        }

        let binding = self.blocks.first();
        let first_block = match &binding {
            Some(v) => v,
            None => {
                return Err("No entry block found for function".to_string());
            }
        };

        let consumes = first_block.consumes;
        let mut produces: Option<i32> = None;

        let mut traversal_queue: Vec<(&EvmBlock, i32)> = [(*first_block, consumes)].to_vec();
        while !traversal_queue.is_empty() {
            let (next, passed_in) = traversal_queue.pop().unwrap();
            if passed_in < next.consumes {
                return Err(format!(
                    "Block requires at least {} but only {} was provided",
                    passed_in, next.consumes
                ));
            }

            // TODO: This needs to be done outside of the loop
            // It is basically to prevent that placeholder blocks are analyzed
            if next.instructions.len() == 1 {
                // TODO: and check that it is a jumpdest
                produces = Some(0);
                continue;
            }

            for instr in next.instructions.iter() {
                let block_production = passed_in + next.produces - next.consumes;

                let label = match instr.opcode {
                    Opcode::RETURN | Opcode::REVERT => {
                        produces = Some(block_production);
                        // TODO: Check 0
                        continue;
                    }
                    Opcode::JUMP | Opcode::JUMPI => &instr.unresolved_argument_label,
                    _ => continue,
                };

                let label = match label {
                    Some(l) => l,
                    _ => {
                        produces = Some(block_production);

                        // Returns to a pointer and there is nothing to resolve
                        continue;
                        // todo!(); // This is actually an internal function end
                        // return Err(format!("No label specified for jump in {}", next.name));
                    }
                };

                let &block = match block_map.get(label) {
                    Some(v) => v,
                    None => {
                        return Err(format!(
                            "Label '{}' not found in block '{}'",
                            label, next.name
                        ))
                    }
                };

                traversal_queue.push((block, block_production));
            }
        }

        let produces = match produces {
            Some(v) => v,
            None => {
                return Err(format!(
                    "Unable to determine the produced number of blocks in {}",
                    function_name
                ));
            }
        };
        self.produces = produces;
        self.consumes = consumes;

        // TODO: Check signature

        Ok(())
    }

    pub fn empty() -> Self {
        Self {
            signature: None,
            selector: Vec::new(),
            blocks: Vec::new(),
            consumes: 0,
            produces: 0,
        }
    }

    pub fn from_signature(signature: EvmFunctionSignature) -> Self {
        let selector = signature.selector();
        Self {
            signature: Some(signature),
            selector,
            blocks: Vec::new(),
            consumes: 0,
            produces: 0,
        }
    }

    pub fn extract_functions(blocks: &Vec<EvmBlock>) -> (Vec<EvmFunction>, Vec<EvmBlock>) {
        let mut ret: Vec<EvmFunction> = Vec::new();
        let mut current_function = EvmFunction {
            signature: None,
            selector: Vec::new(),
            blocks: Vec::new(),
            consumes: 0,
            produces: 0,
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

        let blocks: HashMap<u32, &EvmBlock> = {
            let mut ret: HashMap<u32, &EvmBlock> = HashMap::new();

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
                        let position = position as u32;
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
                consumes: 0,
                produces: 0,
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
                            let position = position as u32;
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

        let unused_blocks = Vec::new();
        for (_position, _block) in blocks {
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
