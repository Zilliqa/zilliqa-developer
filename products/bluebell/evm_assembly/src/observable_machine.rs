use crate::io_interface::EvmIoInterface;
use evm::executor::stack::PrecompileFn;
use log::info;
use primitive_types::U256;
use std::collections::BTreeMap;

use evm::Capture::Exit;
use evm::Capture::Trap;
use evm::Context;
use evm::Machine;
use evm::Opcode;

use primitive_types::{H160, H256};

use std::collections::HashMap;
use std::rc::Rc;

pub type EvmPrecompileSet = BTreeMap<H160, PrecompileFn>;

pub struct ObservableMachine {
    pub machine: Machine,
    pub positions_visited: HashMap<u32, u32>,
    pub failed: bool,
    pub error_message: Option<String>,
    pub storage: HashMap<H256, H256>,
    pub precompile_set: Option<EvmPrecompileSet>,
}

fn h160_to_usize(address: H160) -> usize {
    let bytes = address.as_fixed_bytes();
    let mut result = 0usize;

    // Depending on architecture (32-bit or 64-bit), you may need fewer bytes
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
            failed: false,
            error_message: None,
            storage: HashMap::new(),
            precompile_set,
        }
    }

    pub fn step(&mut self) {
        match self.machine.step() {
            Ok(()) => (),
            Err(code) => match code {
                Exit(_value) => {
                    return;
                }
                Trap(opcode) => {
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

                            info!(
                                "{}",
                                format!(
                                    "Call:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
                                    gas, address, args_offset, args_size, ret_offset, ret_size
                                )
                            );
                            info!("{:#?}", self.precompile_set);

                            let ret = if let Some(precompile_set) = &self.precompile_set {
                                if let Some(f) = precompile_set.get(&address) {
                                    let mem = self.machine.memory().data();
                                    let end = args_offset + args_size;
                                    //let
                                    let input = &mem[args_offset..end]; //args_offset, args_offset+args_size);
                                    info!("{:#?}", input);
                                    // TODO: Integrate these properly
                                    let dummy_context = Context {
                                        address: H160::zero(),
                                        caller: H160::zero(),
                                        apparent_value: U256::zero(),
                                    };
                                    let dummy_backend = EvmIoInterface::new(BTreeMap::new());

                                    let ret =
                                        f(input, Some(gas), &dummy_context, &dummy_backend, true);

                                    // TODO: Write ret to memory
                                    H256::zero()
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
    }

    pub fn run(&mut self) {
        // TODO:  Refactor to use self.step
        loop {
            match self.machine.step() {
                Ok(()) => (),
                Err(code) => match code {
                    Exit(_value) => {
                        return;
                    }
                    Trap(opcode) => {
                        match opcode {
                            Opcode::STATICCALL => {
                                // Emulating static call
                                // TODO: Attach runtime!
                                let stack = self.machine.stack_mut();
                                for _i in 0..6 {
                                    if stack.pop().is_err() {
                                        panic!("Stack empty!");
                                    }
                                }
                                let v = stack.push(H256::zero());
                                if v.is_err() {
                                    panic!("Failed to push result to stack");
                                }
                            }

                            _ => {
                                panic!("Unhandled trap opcode.")
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
            } else {
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
}
