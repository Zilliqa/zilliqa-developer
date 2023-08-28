use evm::Machine;

use std::collections::HashMap;
use std::rc::Rc;

pub struct ObservableMachine {
    pub machine: Machine,
    pub positions_visited: HashMap<u32, u32>,
    pub failed: bool,
    pub error_message: Option<String>,
}

impl ObservableMachine {
    /// Create a new machine with given code and data.
    pub fn new(
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
        stack_limit: usize,
        memory_limit: usize,
    ) -> Self {
        Self {
            machine: Machine::new(code, data, stack_limit, memory_limit),
            positions_visited: HashMap::new(),
            failed: false,
            error_message: None,
        }
    }

    pub fn step(&mut self) {
        match self.machine.step() {
            Ok(()) => (),
            Err(res) => {
                self.failed = true;
                self.error_message = Some(format!("{:?}", res).to_string());
            }
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.machine.step() {
                Ok(()) => (),
                Err(_res) => return,
            }
            if let Ok(p) = self.machine.position() {
                if let Some(value) = self.positions_visited.get_mut(&(*p as u32)) {
                    *value = *value + 1;
                } else {
                    self.positions_visited.insert(*p as u32, 1);
                }
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
