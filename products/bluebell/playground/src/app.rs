use std::cell::RefCell;
use std::rc::Rc;

use evm_assembly::executable::EvmExecutable;
use gloo_console as console;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use bluebell::support::evm::EvmCompiler;
use bluebell::support::modules::ScillaDebugBuiltins;
use bluebell::support::modules::ScillaDefaultBuiltins;
use bluebell::support::modules::ScillaDefaultTypes;
use evm_assembly::observable_machine::ObservableMachine;
use evm_assembly::types::EvmTypeValue;
use serde_json;

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let Props {
        selected_tab,
        on_tab_selected,
        tabs,
    } = props;

    html! {
        <div class="border-b border-gray-200">
            <nav class="-mb-px flex">
                { for tabs.iter().enumerate().map(|(index, tab)| {
                    let is_selected = *selected_tab == index;
                    let button_class = if is_selected {
                        "border-indigo-500 text-indigo-600"
                    } else {
                        "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                    };

                    html! {
                        <button
                            class={"inline-flex items-center px-4 py-2 border-b-2 font-medium".to_owned() + button_class}
                            onclick={on_tab_selected.reform(move |_| index)}
                        >
                            { tab }
                        </button>
                    }
                }) }
            </nav>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub selected_tab: usize,
    pub on_tab_selected: Callback<usize>,
    pub tabs: Vec<String>,
}

pub struct ExecutorView {
    props: ExecutorViewProps,
    selected_tab: usize,
    observable_machine: Option<ObservableMachine>,
}
pub enum ExecutorViewMessage {
    SelectTab(usize),
    ResetMachine {
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
    }, // Add other messages here if needed
    RunStep,
}
#[derive(Properties)]
pub struct ExecutorViewProps {
    pub executable: Rc<RefCell<EvmExecutable>>,
    pub data: String,
}

impl Clone for ExecutorViewProps {
    fn clone(&self) -> Self {
        Self {
            executable: Rc::clone(&self.executable),
            data: self.data.clone(),
        }
    }
}

impl PartialEq for ExecutorViewProps {
    fn eq(&self, other: &Self) -> bool {
        // Logic to compare two ExecutorViewProps.
        // If you only need reference equality for the Rc:
        Rc::ptr_eq(&self.executable, &other.executable)
    }
}

impl Component for ExecutorView {
    type Message = ExecutorViewMessage;
    type Properties = ExecutorViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let data = if props.data != "" {
            hex::decode(props.data.clone())
        } else {
            Ok(Vec::<u8>::new())
        };

        if let Ok(data) = data {
            let code = (*props.executable).clone().into_inner().bytecode;
            ctx.link().send_message(ExecutorViewMessage::ResetMachine {
                code: code.into(),
                data: data.into(),
            });
        }

        Self {
            props,
            selected_tab: 0,
            observable_machine: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &ExecutorViewProps) -> bool {
        let data = if props.data != "" {
            hex::decode(props.data.clone())
        } else {
            Ok(Vec::<u8>::new())
        };

        if let Ok(data) = data {
            let code = (*props.executable).clone().into_inner().bytecode;
            ctx.link().send_message(ExecutorViewMessage::ResetMachine {
                code: code.into(),
                data: data.into(),
            });
            true
        } else {
            false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log!("Was here?");
        match msg {
            ExecutorViewMessage::SelectTab(index) => {
                self.selected_tab = index;
                true // Should the component re-render?
            }
            ExecutorViewMessage::ResetMachine { code, data } => {
                self.observable_machine = Some(ObservableMachine::new(code, data, 1024, 1024));
                true
            } // Handle other messages here
            ExecutorViewMessage::RunStep => {
                if let Some(ref mut machine) = self.observable_machine {
                    console::log!("Stepping");
                    machine.step();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let observable_machine = if let Some(om) = &self.observable_machine {
            om
        } else {
            return html! {
                <div>{"Something went wrong"}</div>
            };
        };

        let machine = &observable_machine.machine;
        let program_counter = if let Ok(pc) = machine.position() {
            pc
        } else {
            &0
        };

        if observable_machine.failed {
            if let Some(msg) = &observable_machine.error_message {
                return html! {
                    <div>{msg}</div>
                };
            }

            return html! {
                <div>{"VM execution failed for unknown reasons"}</div>
            };
        }

        let functions = (*self.props.executable).clone().into_inner().ir.functions;

        let mut bytecode: Vec<String> = Vec::new();
        for function in &functions {
            for block in &function.blocks {
                for instr in &block.instructions {
                    let position = match instr.position {
                        Some(v) => v,
                        None => 0,
                    };

                    let instruction_value = if instr.arguments.len() > 0 {
                        let argument: String = instr
                            .arguments
                            .iter()
                            .map(|byte| format!("{:02x}", byte).to_string())
                            .collect();

                        format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                    } else {
                        instr.opcode.to_string()
                    };

                    let value = format!(
                        "[0x{:02x}] {} ;; Stack: {}, Comment: {}",
                        position,
                        instruction_value,
                        instr.stack_size,
                        instr.comment.clone().unwrap_or("".to_string()).trim(),
                    );
                    bytecode.push(value);
                }
            }
        }

        let stack: Vec<String> = vec![];
        let instruction: String = "PUSH1 0x60".into();
        let memory: Vec<String> = vec![];

        let _script = (*self.props.executable).clone().into_inner().ir.to_string();

        let step_button_click = ctx.link().callback(move |_| ExecutorViewMessage::RunStep);

        let run_button_click = Callback::from(move |_| {
            console::log!("Run Button clicked");
        });

        let handle_tab_selected = ctx
            .link()
            .callback(move |index: usize| ExecutorViewMessage::SelectTab(index));

          html! {         
            <div class="h-full w-full flex flex-col relative">
                <div class="space-x-4 right-0 top-0 absolute">
                    <button class="bg-blue-600 text-white px-2 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500" onclick={step_button_click.clone()}>
                        {"Step"}
                    </button>
                    <button class="bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500" onclick={run_button_click.clone()}>
                        {"Run"}
                    </button>
                    {program_counter}
                </div>
                <Tabs
                    selected_tab={self.selected_tab.clone()}
                    on_tab_selected={handle_tab_selected}
                    tabs={vec!["EVM Bytecode".to_string(), "Intermediate Representation".to_string()]}
                />
                <div class="space-y-6 flex flex-col flex-grow">

                    { if self.selected_tab == 0 {
                        html! {
                            <>
                            <div class="p-4 bg-white border rounded-md shadow-sm space-y-2 overflow-y-auto">
                                { for bytecode.iter().map(|instr| html! {
                                    <div class="flex items-center space-x-4 text-sm text-gray-700">
                                        <input type="checkbox" class="form-checkbox h-5 w-5 text-blue-600 rounded-full" />
                                        <div>{instr}</div>
                                    </div>
                                }) }
                            </div>
                            <div>
                                <h2 class="text-xl font-semibold mb-3 text-gray-800">{"Stack"}</h2>
                                <div class="p-4 bg-white border rounded-md shadow-sm space-y-2 overflow-y-auto h-48">
                                    { for stack.iter().map(|s| html! { <div class="text-sm text-gray-700">{s}</div> }) }
                                </div>
                            </div>
                            <div>
                                <h2 class="text-xl font-semibold mb-3 text-gray-800">{"Memory"}</h2>
                                <div class="p-4 bg-white border rounded-md shadow-sm space-y-2">
                                    { for memory.iter().map(|mem| html! { <div class="text-sm text-gray-700">{mem}</div> }) }
                                </div>
                            </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="space-y-6">
                                // "Intermediate Representation" tab content
                            </div>
                        }
                    } }
                </div>                
            </div>
        }

    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MainCardProps {
    pub children: Children,
}

pub struct MainCard {
    props: MainCardProps,
}

impl Component for MainCard {
    type Message = ();
    type Properties = MainCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            props,
        }
    }
    fn changed(&mut self, ctx: &Context<Self>, props: &MainCardProps) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex h-screen w-screen p-16 justify-center items-center">
                <div class="h-full w-full bg-white p-6 rounded-lg flex flex-col justify-center items-center">
                    { for self.props.children.iter() }
                </div>
            </div>
        }            
    }
}


#[function_component(App)]
pub fn app() -> Html {
    let source_code = use_state(|| {
        r#"scilla_version 0

library HelloWorld
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
"#
        .to_string()
    });
    let executable_state: UseStateHandle<Option<Rc<RefCell<EvmExecutable>>>> = use_state(|| None);
    let executable = executable_state.clone();

    let compile_button_click = {
        let source_code = source_code.clone();
        Callback::from(move |_| {
            let mut compiler = EvmCompiler::new_no_abi_support();
            compiler.pass_manager_mut().enable_debug_printer();

            let default_types = ScillaDefaultTypes {};
            let default_builtins = ScillaDefaultBuiltins {};
            let debug = ScillaDebugBuiltins {};

            compiler.attach(&default_types);
            compiler.attach(&default_builtins);
            compiler.attach(&debug);
            if let Ok(exec) = compiler.executable_from_script(source_code.to_string()) {
                executable_state.set(Some(Rc::new(RefCell::new(exec.executable))));
            } else {
                console::error!("Compilation failed!");
            }
        })
    };

    let target_input_value = |e: &Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value()
    };

    let handle_source_code_change = {
        let source_code = source_code.clone();
        Callback::from(move |e: InputEvent| {
            let value = target_input_value(&e);
            console::log!(value.clone());
            source_code.set(value);
        })
    };

    if let Some(executable) = &*executable {

        /*
        return html! {
            <div class="container mx-auto mt-12 p-6 bg-gray-50 rounded-lg shadow-md">
                <ExecutorView executable={executable} data={""} />
            </div>
        };
        */
        return     html! {
            <MainCard key="second">
                <ExecutorView executable={executable} data={""} />
            </MainCard>
        };
    }

    let source_code: String = source_code.to_string();


html! {
    <MainCard key="first">
        <div class="h-full w-full flex flex-col space-y-6">
            <h1 class="text-2xl font-semibold text-gray-900 mb-4">{"Scilla Compiler"}</h1>

            <textarea
                id="source-code"
                class="flex-1 w-full p-4 bg-black text-white border rounded-md shadow-sm resize-none focus:border-teal-500 focus:ring focus:ring-teal-200 transition-shadow duration-150 font-mono"
                style="tab-size: 4; white-space: pre;"
                rows="20"
                value={source_code}
                oninput={handle_source_code_change}
                placeholder="Enter Scilla source code here..."
            />


            <div class="mt-4">
                <button class="bg-indigo-600 text-lg text-white px-6 py-2 rounded-lg shadow-sm hover:bg-indigo-700 focus:bg-indigo-800 focus:outline-none focus:ring focus:ring-indigo-200 active:bg-indigo-800 transition duration-150" onclick={compile_button_click.clone()}>
                    {"Compile"}
                </button>
            </div>
        </div>
    </MainCard>
}


}
