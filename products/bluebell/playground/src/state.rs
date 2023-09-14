// use strum_macros::{Display, EnumIter};

use bluebell::support::evm::EvmCompiler;
use bluebell::support::modules::ScillaDebugBuiltins;
use bluebell::support::modules::ScillaDefaultBuiltins;
use bluebell::support::modules::ScillaDefaultTypes;
use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executable::EvmExecutable;
use evm_assembly::function_signature::EvmFunctionSignature;
use evm_assembly::observable_machine::ObservableMachine;
use evm_assembly::types::EvmTypeValue;
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
}

impl Default for State {
    fn default() -> Self {
        console::log!("Creating new state");
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
                console::log!("Compiling code");

                if let Ok(exec) = compiler.executable_from_script(source_code.to_string()) {
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
                        None, // TODO: Add prefcompiles
                    ))));

                    // Preserving the context so it can be used later
                    let context = Rc::new(RefCell::new(EvmCompilerContext::new()));
                    {
                        let mut context = context.borrow_mut();
                        std::mem::swap(&mut *context, &mut compiler.context)
                    }
                    state.context = Some(context);
                } else {
                    console::error!("Compilation failed!");
                }

                true
            }
            StateMessage::PrepareFunctionCall {
                function_name,
                arguments,
            } => {
                // console::log!("Code: {}", hex::encode(&*code));
                let code: Vec<u8> = if let Some(exec) = &state.executable {
                    exec.borrow().bytecode.to_vec()
                } else {
                    [].to_vec()
                };

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

                console::log!("Resetting machine");
                state.observable_machine = Some(Rc::new(RefCell::new(ObservableMachine::new(
                    code.into(),
                    data,
                    1024,
                    1024,
                    precompiles,
                ))));
                state.compiling = false;
                state.playing = false;
                state.function_loaded = true;

                true
            }
            StateMessage::RunStep => {
                if let Some(ref mut machine) = state.observable_machine {
                    let mut machine = machine.borrow_mut();
                    machine.step();

                    state.program_counter = if let Ok(pc) = machine.machine.position() {
                        if let Some(pos) = state.pc_to_position.get(pc) {
                            state.current_position = Some(*pos);
                        }
                        *pc as u32
                    } else {
                        state.current_position = None;
                        0
                    };

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
        console::log!("Cloning 1");
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
        console::log!(format!("Cloning 2: {}", self.function_loaded));
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
    }
}

impl Eq for State {}

impl State {}
