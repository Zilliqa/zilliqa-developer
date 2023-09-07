// use strum_macros::{Display, EnumIter};

use bluebell::support::evm::EvmCompiler;
use bluebell::support::modules::ScillaDebugBuiltins;
use bluebell::support::modules::ScillaDefaultBuiltins;
use bluebell::support::modules::ScillaDefaultTypes;
use evm_assembly::executable::EvmExecutable;
use evm_assembly::observable_machine::ObservableMachine;
use gloo_console as console;
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yewdux::prelude::Dispatch;
use yewdux::prelude::Reducer;
use yewdux::store::Store;

#[derive(Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct State {
    pub source_code: String,

    pub pc_to_position: HashMap<usize, (usize, usize, usize, usize)>,
    pub current_position: Option<(usize, usize, usize, usize)>,

    #[serde(skip)]
    pub compiling: bool,

    #[serde(skip)]
    pub playing: bool,

    #[serde(skip)]
    pub bytecode_hex: String,

    #[serde(skip)]
    pub executable: Option<Rc<RefCell<EvmExecutable>>>,

    #[serde(skip)]
    pub observable_machine: Option<Rc<RefCell<ObservableMachine>>>,

    #[serde(skip)]
    pub program_counter: u32, // Forces state update on step
}

impl Default for State {
    fn default() -> Self {
        State {
            source_code: String::from(
                r#"scilla_version 0

library HelloWorld
(* New source *)
type Bool = 
  | True
  | False

contract HelloWorld()

transition setHello ()
  msg = Uint64 12;

  is_owner = False;
  match is_owner with
  | True =>
    x = builtin print__impl msg
  | False =>
    x = builtin print__impl msg;
    y = builtin print__impl msg
  end

end
"#,
            ),
            compiling: false,
            playing: false,
            bytecode_hex: "".to_string(),
            executable: None,
            observable_machine: None,
            program_counter: 0,
            current_position: None,
            pc_to_position: HashMap::new(),
        }
    }
}

pub enum StateMessage {
    Reset,
    ResetMachine {
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
    }, // Add other messages here if needed
    RunStep,
    CompileCode {
        source_code: String,
    },
}

impl Reducer<State> for StateMessage {
    fn apply(self, mut orig_state: Rc<State>) -> Rc<State> {
        let state = Rc::make_mut(&mut orig_state);
        match self {
            StateMessage::Reset => {
                state.observable_machine = None;
                state.executable = None;
                state.bytecode_hex = "".to_string();
                true
            }
            StateMessage::CompileCode { source_code } => {
                let mut compiler = EvmCompiler::new_no_abi_support();
                compiler.pass_manager_mut().enable_debug_printer();

                let default_types = ScillaDefaultTypes {};
                let default_builtins = ScillaDefaultBuiltins {};
                let debug = ScillaDebugBuiltins {};

                compiler.attach(&default_types);
                compiler.attach(&default_builtins);
                compiler.attach(&debug);

                state.compiling = false;
                state.playing = false;
                if let Ok(exec) = compiler.executable_from_script(source_code.to_string()) {
                    state.source_code = source_code.clone();
                    state.bytecode_hex = hex::encode(&exec.executable.bytecode.clone());
                    let code: Vec<u8> = (&*exec.executable.bytecode).to_vec();

                    // Creating PC to source map
                    state.pc_to_position = HashMap::new();
                    let functions = &exec.executable.ir.functions;
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
                                state.pc_to_position.insert(*pc as usize, source_pos);
                            }
                        }
                    }

                    state.executable = Some(Rc::new(RefCell::new(exec.executable)));
                    state.observable_machine = Some(Rc::new(RefCell::new(ObservableMachine::new(
                        code.into(),
                        [].to_vec().into(),
                        1024,
                        1024,
                    ))));
                } else {
                    console::error!("Compilation failed!");
                }

                true
            }
            StateMessage::ResetMachine { code, data } => {
                // console::log!("Code: {}", hex::encode(&*code));
                console::log!("Resetting machine");
                state.observable_machine = Some(Rc::new(RefCell::new(ObservableMachine::new(
                    code, data, 1024, 1024,
                ))));
                state.compiling = false;
                state.playing = false;
                true
            }
            StateMessage::RunStep => {
                console::log!("Attempting Step!");
                if let Some(ref mut machine) = state.observable_machine {
                    console::log!("Step!");
                    machine.borrow_mut().step();
                    state.program_counter = if let Ok(pc) = machine.borrow_mut().machine.position()
                    {
                        if let Some(pos) = state.pc_to_position.get(pc) {
                            state.current_position = Some(*pos);
                        }
                        *pc as u32
                    } else {
                        state.current_position = None;
                        0
                    };

                    console::log!("New PC:", state.program_counter);
                    if state.playing {
                        Timeout::new(500, move || {
                            let dispatch = Dispatch::<State>::new();
                            dispatch.apply(StateMessage::RunStep);
                        })
                        .forget();
                    }
                    true
                } else {
                    false
                }
            }
        };

        orig_state
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        let executable = if let Some(e) = &self.executable {
            Some(Rc::clone(&e))
        } else {
            None
        };
        let observable_machine = if let Some(e) = &self.observable_machine {
            Some(Rc::clone(&e))
        } else {
            None
        };
        Self {
            source_code: self.source_code.clone(),
            compiling: self.compiling,
            playing: self.playing,
            executable,
            observable_machine,
            program_counter: self.program_counter,
            bytecode_hex: self.bytecode_hex.clone(),
            pc_to_position: self.pc_to_position.clone(),
            current_position: self.current_position.clone(),
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let e1 = match &self.executable {
            Some(e) => e,
            None => return false,
        };

        let e2 = match &other.executable {
            Some(e) => e,
            None => return false,
        };

        let o1 = match &self.observable_machine {
            Some(e) => e,
            None => return false,
        };

        let o2 = match &other.observable_machine {
            Some(e) => e,
            None => return false,
        };

        Rc::ptr_eq(&e1, &e2)
            && Rc::ptr_eq(&o1, &o2)
            && self.source_code == other.source_code
            && self.program_counter == other.program_counter
            && self.bytecode_hex == other.bytecode_hex
            && self.pc_to_position == other.pc_to_position
            && self.current_position == other.current_position
            && self.compiling == other.compiling
            && self.playing == other.playing
    }
}

impl Eq for State {}

impl State {}
