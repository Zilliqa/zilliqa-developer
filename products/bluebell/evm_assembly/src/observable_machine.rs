use crate::executable::TypeSourceMap;
use crate::io_interface::EvmIoInterface;
use evm::executor::stack::PrecompileFn;
use evm::Capture;
use evm::ExitReason;
use evm::Trap;
use log::warn;
use primitive_types::U256;
use std::collections::BTreeMap;
use std::str::FromStr;

use evm::Capture::Exit;
use evm::Capture::Trap as CaptureTrap;
use evm::Context;
use evm::Machine;
use evm::Opcode;

use primitive_types::{H160, H256};

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub type EvmPrecompileSet = BTreeMap<H160, PrecompileFn>;

pub struct ObservableMachine {
    pub machine: Machine,
    pub positions_visited: HashMap<u32, u32>,
    pub lines_visited_ordered: Vec<u32>,
    pub lines_visited: HashSet<u32>,
    pub position_to_line: HashMap<usize, usize>,
    pub failed: bool,
    pub error_message: Option<String>,
    pub storage: HashMap<H256, H256>,
    pub precompile_set: Option<EvmPrecompileSet>,

    pub caller: H160,
}

fn h160_to_usize(address: H160) -> usize {
    let bytes = address.as_fixed_bytes();
    let mut result = 0usize;

    for &byte in bytes.iter().rev().take(std::mem::size_of::<usize>()).rev() {
        result = (result << 8) | (byte as usize);
    }

    result
}

impl ObservableMachine {
    /// Create a new machine with given code and data.
    pub fn new(
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
        stack_limit: usize,
        memory_limit: usize,
        precompile_set: Option<EvmPrecompileSet>,
    ) -> Self {
        Self {
            machine: Machine::new(code, data, stack_limit, memory_limit),
            positions_visited: HashMap::new(),
            lines_visited_ordered: Vec::new(),
            lines_visited: HashSet::new(),
            position_to_line: HashMap::new(),
            failed: false,
            error_message: None,
            storage: HashMap::new(),
            precompile_set,
            caller: H160::zero(),
        }
    }

    pub fn set_caller(&mut self, caller: String) {
        let caller_address = H160::from_str(&caller).expect("Failed to parse caller address");
        self.caller = caller_address;
    }

    pub fn set_source_map(&mut self, source_map: &TypeSourceMap) {
        self.position_to_line = source_map
            .iter()
            .map(|(k, (_, _, line, _))| (*k, *line))
            .collect::<HashMap<_, _>>();
    }

    pub fn step(&mut self) -> Result<(), Capture<ExitReason, Trap>> {
        match self.machine.step() {
            Ok(()) => (),
            Err(code) => match code {
                Exit(ref _value) => {
                    return Err(code);
                }
                CaptureTrap(opcode) => {
                    match opcode {
                        Opcode::SSTORE => {
                            let stack = self.machine.stack_mut();
                            let address = match stack.pop() {
                                Ok(v) => v,
                                Err(_) => panic!("Stack empty!"),
                            };
                            let value = match stack.pop() {
                                Ok(v) => v,
                                Err(_) => panic!("Stack empty!"),
                            };

                            self.storage.insert(address, value);
                        }

                        Opcode::SLOAD => {
                            let stack = self.machine.stack_mut();
                            let address = match stack.pop() {
                                Ok(v) => v,
                                Err(_) => panic!("Stack empty!"),
                            };

                            let value = match self.storage.get(&address) {
                                Some(v) => v.clone(),
                                None => panic!("Unable to find value!"),
                            };

                            let _ = stack.push(value);
                        }
                        Opcode::CALLER => {
                            let stack = self.machine.stack_mut();

                            let mut h256_bytes = [0u8; 32]; // Create h256_bytes[12..].copy_from_slice(&self.caller.0);

                            stack.push(H256::from_slice(&h256_bytes));
                        }
                        Opcode::CALLVALUE => {
                            let stack = self.machine.stack_mut();
                            // We always assume zero value caller (root call)
                            let v = stack.push(H256::zero());
                            if v.is_err() {
                                panic!("Failed to push result to stack");
                            }
                        }
                        Opcode::CALLDATASIZE => {
                            panic!("Call size not set.")
                        }
                        Opcode::CALLDATALOAD => {
                            panic!("Call data not loadable.")
                        }

                        Opcode::STATICCALL => {
                            // Emulating static call
                            // TODO: Attach runtime!

                            let (gas, address, args_offset, args_size, ret_offset, ret_size) = {
                                let stack = self.machine.stack_mut();
                                let gas: u64 = match stack.pop() {
                                    Ok(g) => h160_to_usize(g.into()) as u64,
                                    Err(_) => {
                                        panic!("Gas argument missing");
                                    }
                                };
                                let address: H160 = match stack.pop() {
                                    Ok(g) => g.into(),
                                    Err(_) => {
                                        panic!("Address argument missing");
                                    }
                                };
                                let args_offset: usize = match stack.pop() {
                                    Ok(g) => h160_to_usize(g.into()),
                                    Err(_) => {
                                        panic!("Args offset argument missing");
                                    }
                                };
                                let args_size: usize = match stack.pop() {
                                    Ok(g) => h160_to_usize(g.into()),
                                    Err(_) => {
                                        panic!("Args size argument missing");
                                    }
                                };
                                let ret_offset: usize = match stack.pop() {
                                    Ok(g) => h160_to_usize(g.into()),
                                    Err(_) => {
                                        panic!("Return offset argument missing");
                                    }
                                };
                                let ret_size: usize = match stack.pop() {
                                    Ok(g) => h160_to_usize(g.into()),
                                    Err(_) => {
                                        panic!("Return size argument missing");
                                    }
                                };

                                (gas, address, args_offset, args_size, ret_offset, ret_size)
                            };

                            let ret = if let Some(precompile_set) = &self.precompile_set {
                                if let Some(f) = precompile_set.get(&address) {
                                    let ret = {
                                        let mem = self.machine.memory().data();
                                        let end = args_offset + args_size;
                                        //let
                                        let input = &mem[args_offset..end]; //args_offset, args_offset+args_size);

                                        // TODO: Integrate these properly
                                        let dummy_context = Context {
                                            address: H160::zero(),
                                            caller: self.caller.clone(),
                                            apparent_value: U256::zero(),
                                        };
                                        let dummy_backend = EvmIoInterface::new(BTreeMap::new());

                                        f(input, Some(gas), &dummy_context, &dummy_backend, true)
                                    };

                                    if let Ok(_ret) = ret {
                                        let _mem = self.machine.memory_mut().data();
                                        // TODO: Write ret to memory
                                        H256::zero()
                                    } else {
                                        H256::zero()
                                    }
                                } else {
                                    H256::zero()
                                }
                            } else {
                                H256::zero()
                            };

                            let stack = self.machine.stack_mut();
                            if stack.push(ret).is_err() {
                                panic!("Failed to push result to stack");
                            }
                        }

                        _ => {
                            self.failed = true;
                            self.error_message = Some(format!("{:?}", opcode).to_string());
                            panic!("Unhandled trap opcode {:?}", opcode)
                        }
                    }
                }
            },
        }

        if let Ok(p) = self.machine.position() {
            if let Some(value) = self.positions_visited.get_mut(&(*p as u32)) {
                *value = *value + 1;
            } else {
                self.positions_visited.insert(*p as u32, 1);
            }

            if let Some(line) = self.position_to_line.get(p) {
                self.lines_visited.insert(*line as u32);
                let should_add = if let Some(last) = self.lines_visited_ordered.last() {
                    *last != *line as u32
                } else {
                    true
                };

                if should_add {
                    self.lines_visited_ordered.push(*line as u32);
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            match self.step() {
                Ok(()) => (),
                Err(code) => match code {
                    Exit(_value) => {
                        return;
                    }
                    _ => (),
                },
            }
            if self.machine.position().is_err() {
                // Breaking only when we've reached an invalid position
                // This is to ignore issues of traps with static calls
                return;
            }
        }
    }

    pub fn did_visit_program_counter(&self, pc: u32) -> bool {
        None != self.positions_visited.get(&pc)
    }

    pub fn did_not_visit_program_counter(&self, pc: u32) -> bool {
        None == self.positions_visited.get(&pc)
    }

    pub fn did_visit_line(&self, pc: u32) -> bool {
        None != self.lines_visited.get(&pc)
    }

    pub fn did_not_visit_line(&self, pc: u32) -> bool {
        None == self.lines_visited.get(&pc)
    }

    pub fn has_record(&self, key: &H256, value: &H256) -> bool {
        // Check if the key exists in storage and if its value matches the given value
        match self.storage.get(&key) {
            Some(stored_value) => *stored_value == *value,
            None => false,
        }
    }
}
