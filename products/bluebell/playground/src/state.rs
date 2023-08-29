// use strum_macros::{Display, EnumIter};
use evm_assembly::executable::EvmExecutable;
use evm_assembly::observable_machine::ObservableMachine;
use gloo_console as console;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use yewdux::prelude::Reducer;
use yewdux::store::Store;

#[derive(Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct State {
    pub source_code: String,

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
            executable: None,
            observable_machine: None,
            program_counter: 0,
        }
    }
}

pub enum StateMessage {
    ResetMachine {
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
    }, // Add other messages here if needed
    RunStep,
}

impl Reducer<State> for StateMessage {
    fn apply(self, mut orig_state: Rc<State>) -> Rc<State> {
        let state = Rc::make_mut(&mut orig_state);
        match self {
            StateMessage::ResetMachine { code, data } => {
                // console::log!("Code: {}", hex::encode(&*code));
                state.observable_machine = Some(Rc::new(RefCell::new(ObservableMachine::new(
                    code, data, 1024, 1024,
                ))));
                true
            }
            StateMessage::RunStep => {
                console::log!("Attempting Step!");
                if let Some(ref mut machine) = state.observable_machine {
                    console::log!("Step!");
                    machine.borrow_mut().step();
                    state.program_counter = if let Ok(pc) = machine.borrow_mut().machine.position()
                    {
                        *pc as u32
                    } else {
                        0
                    };

                    console::log!("New PC:", state.program_counter);

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
            executable,
            observable_machine,
            program_counter: self.program_counter,
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
    }
}

impl Eq for State {}

impl State {}
