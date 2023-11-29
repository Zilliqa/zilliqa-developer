// use strum_macros::{Display, EnumIter};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use bluebell::support::{
    evm::EvmCompiler,
    modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
};
use evm::{Capture::Exit, ExitReason};
use evm_assembly::{
    compiler_context::EvmCompilerContext, executable::EvmExecutable,
    function_signature::EvmFunctionSignature, observable_machine::ObservableMachine,
    types::EvmTypeValue,
};
use gloo_console as console;
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use yewdux::{
    prelude::{Dispatch, Reducer},
    store::Store,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Succeeded,
    // Unused Paused,
    Failed,
}

#[derive(Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct State {
    pub source_code: String,

    pub pc_to_position: HashMap<usize, (usize, usize, usize, usize)>,
    pub current_position: Option<(usize, usize, usize, usize)>,
    pub data: String,

    #[serde(skip)]
    pub context: Option<Rc<RefCell<EvmCompilerContext>>>,

    #[serde(skip)]
    pub compiling: bool,

    #[serde(skip)]
    pub playing: bool,

    #[serde(skip)]
    pub function_loaded: bool,

    #[serde(skip)]
    pub bytecode_hex: String,

    #[serde(skip)]
    pub executable: Option<Rc<RefCell<EvmExecutable>>>,

    #[serde(skip)]
    pub functions: Vec<EvmFunctionSignature>,

    #[serde(skip)]
    pub observable_machine: Option<Rc<RefCell<ObservableMachine>>>,

    #[serde(skip)]
    pub program_counter: u32, // Forces state update on step

    #[serde(skip)]
    pub exit_message: Option<(ExecutionStatus, String)>,

    #[serde(skip)]
    pub breakpoints: HashSet<u32>,
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
    print msg
  | False =>
    print msg;
    print msg
  end

end
"#,
            ),
            context: None,
            compiling: false,
            playing: false,
            function_loaded: false,
            bytecode_hex: "".to_string(),
            executable: None,
            functions: [].to_vec(),
            observable_machine: None,
            program_counter: 0,
            current_position: None,
            pc_to_position: HashMap::new(),
            data: "".to_string(),
            exit_message: None,
            breakpoints: HashSet::new(),
        }
    }
}

pub enum StateMessage {
    Reset,
    PrepareFunctionCall {
        function_name: String,
        arguments: String,
    }, // Add other messages here if needed
    RunStep,
    CompileCode {
        source_code: String,
    },
    AddBreakPoint(u32),
    RemoveBreakPoint(u32),
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
            StateMessage::AddBreakPoint(point) => {
                state.breakpoints.insert(point);
                true
            }
            StateMessage::RemoveBreakPoint(point) => {
                state.breakpoints.remove(&point);
                true
            }
            StateMessage::CompileCode { source_code } => {
                let mut compiler = EvmCompiler::new();
                compiler.pass_manager_mut().enable_debug_printer();

                let default_types = ScillaDefaultTypes {};
                let default_builtins = ScillaDefaultBuiltins {};
                let debug = ScillaDebugBuiltins {};

                compiler.attach(&default_types);
                compiler.attach(&default_builtins);
                compiler.attach(&debug);

                state.compiling = false;
                state.playing = false;
                state.function_loaded = false;

                state.data = "0x00".to_string();

                let result = compiler.executable_from_script(source_code.to_string());
                if let Ok(exec) = result {
                    state.functions = exec
                        .context
                        .function_declarations
                        .iter()
                        .map(|(_k, v)| v.clone())
                        .collect();
                    state.source_code = source_code.clone();
                    state.bytecode_hex = hex::encode(&exec.executable.bytecode.clone());
                    let code: Vec<u8> = (&*exec.executable.bytecode).to_vec();

                    // Creating PC to source map
                    state.pc_to_position = exec.executable.get_source_map();

                    state.executable = Some(Rc::new(RefCell::new(exec.executable)));
                    let mut observable_machine = ObservableMachine::new(
                        code.into(),
                        [].to_vec().into(),
                        1024,
                        1024,
                        None, // TODO: Add prefcompiles
                    );
                    observable_machine.set_source_map(&state.pc_to_position);

                    state.observable_machine = Some(Rc::new(RefCell::new(observable_machine)));
                    state.breakpoints = HashSet::new();

                    // Preserving the context so it can be used later
                    let context = Rc::new(RefCell::new(EvmCompilerContext::new()));
                    {
                        let mut context = context.borrow_mut();
                        std::mem::swap(&mut *context, &mut compiler.context)
                    }
                    state.context = Some(context);
                } else {
                    if let Err(e) = result {
                        console::error!(format!("{:#?}", e));
                    }
                    console::error!("Compilation failed!");
                }

                true
            }
            StateMessage::PrepareFunctionCall {
                function_name,
                arguments,
            } => {
                let code: Vec<u8> = if let Some(exec) = &state.executable {
                    exec.borrow().bytecode.to_vec()
                } else {
                    [].to_vec()
                };

                let arguments = format!("[{}]", arguments);

                let args: Vec<EvmTypeValue> = if arguments == "" {
                    [].to_vec()
                } else {
                    serde_json::from_str(&arguments).expect("Failed to deserialize arguments")
                };

                let data: Rc<Vec<u8>> = if let Some(context) = &state.context {
                    context
                        .borrow_mut()
                        .get_function(&function_name)
                        .expect(&format!("Function name {} not found", function_name).to_string())
                        .generate_transaction_data(args)
                        .into()
                } else {
                    Rc::new([].to_vec())
                };

                let precompiles = if let Some(context) = &state.context {
                    Some(context.borrow_mut().get_precompiles())
                } else {
                    None
                };

                state.data = hex::encode(&*data);

                let mut observable_machine =
                    ObservableMachine::new(code.into(), data, 1024, 1024, precompiles);

                observable_machine.set_source_map(&state.pc_to_position);

                state.observable_machine = Some(Rc::new(RefCell::new(observable_machine)));
                state.compiling = false;
                state.playing = false;
                state.function_loaded = true;
                state.exit_message = None;
                true
            }
            StateMessage::RunStep => {
                if let Some(ref mut machine) = state.observable_machine {
                    let mut machine = machine.borrow_mut();

                    match machine.step() {
                        Ok(()) => (),
                        Err(code) => match code {
                            Exit(value) => {
                                state.playing = false;
                                state.function_loaded = false;
                                match value {
                                    ExitReason::Succeed(value) => {
                                        state.exit_message = Some((
                                            ExecutionStatus::Succeeded,
                                            format!(
                                                "{:?} at {:#02x}",
                                                value, state.program_counter
                                            )
                                            .to_string(),
                                        ));
                                    }
                                    _ => {
                                        state.exit_message = Some((
                                            ExecutionStatus::Failed,
                                            format!(
                                                "{:?} at {:#02x}",
                                                value, state.program_counter
                                            )
                                            .to_string(),
                                        ));
                                    }
                                }
                            }
                            _ => (),
                        },
                    }

                    state.program_counter = if let Ok(pc) = machine.machine.position() {
                        if let Some(pos) = state.pc_to_position.get(pc) {
                            state.current_position = Some(*pos);
                        }
                        *pc as u32
                    } else {
                        state.current_position = None;
                        0
                    };

                    if state.breakpoints.contains(&state.program_counter) {
                        state.playing = false;
                    }

                    if state.playing {
                        Timeout::new(5, move || {
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
        let context = if let Some(c) = &self.context {
            Some(Rc::clone(&c))
        } else {
            None
        };
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
            function_loaded: self.function_loaded,
            executable,
            context,
            functions: self.functions.clone(),
            observable_machine,
            program_counter: self.program_counter,
            bytecode_hex: self.bytecode_hex.clone(),
            pc_to_position: self.pc_to_position.clone(),
            current_position: self.current_position.clone(),
            data: self.data.clone(),
            exit_message: self.exit_message.clone(),
            breakpoints: self.breakpoints.clone(),
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
            && self.function_loaded == other.function_loaded
            && self.data == other.data
            && self.exit_message == other.exit_message
            && self.breakpoints == other.breakpoints
    }
}

impl Eq for State {}

impl State {}
