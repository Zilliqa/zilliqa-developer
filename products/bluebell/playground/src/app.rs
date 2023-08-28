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

pub struct FloatingCard {
    props: FloatingCardProps,
    is_dragging: bool,
    position: (i32, i32),
    last_mouse_position: (i32, i32),
}

#[derive(Properties, Clone, PartialEq)]
pub struct FloatingCardProps {
    pub children: Children,
}

pub enum FloatingCardMessage {
    MouseDown((i32, i32)),
    MouseMove((i32, i32)),
    MouseUp,
}

impl Component for FloatingCard {
    type Message = FloatingCardMessage;
    type Properties = FloatingCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            props,
            is_dragging: false,
            position: (300, 100),
            last_mouse_position: (0, 0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FloatingCardMessage::MouseDown(coords) => {
                self.is_dragging = true;
                self.last_mouse_position = coords;
            }
            FloatingCardMessage::MouseMove(coords) => {
                if self.is_dragging {
                    let dx = coords.0 - self.last_mouse_position.0;
                    let dy = coords.1 - self.last_mouse_position.1;
                    self.position.0 -= dx;
                    self.position.1 -= dy;
                    self.last_mouse_position = coords;
                }
            }
            FloatingCardMessage::MouseUp => {
                self.is_dragging = false;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &FloatingCardProps) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class="cursor-grab border rounded border-gray-300 p-4 w-64 min-w-48 max-w-96 h-24 select-none bg-white shadow-md"
                style={format!("position: absolute; right: {}px; bottom: {}px;", self.position.0, self.position.1)}
                onmousedown={ctx.link().callback(|e: MouseEvent| FloatingCardMessage::MouseDown((e.client_x(), e.client_y())))}
                onmousemove={ctx.link().callback(|e: MouseEvent| FloatingCardMessage::MouseMove((e.client_x(), e.client_y())))}
                onmouseup={ctx.link().callback(|_| FloatingCardMessage::MouseUp)}
            >
                    { self.props.children.clone() }
            </div>
        }
    }
}

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

pub struct ExecutorViewInstruction {
    pub bytecode_position: u32,
    pub hex_bytecode_position: String,
    pub text: String,
    pub ir_origin: String,
}

pub struct ExecutorView {
    props: ExecutorViewProps,
    selected_tab: usize,
    instructions: Vec<ExecutorViewInstruction>,
}
pub enum ExecutorViewMessage {
    SelectTab(usize),
    SetInstructions(Vec<ExecutorViewInstruction>),
}
#[derive(Properties)]
pub struct ExecutorViewProps {
    pub executable: Rc<RefCell<EvmExecutable>>,
    pub data: String,
    pub program_counter: u32,
}

impl Clone for ExecutorViewProps {
    fn clone(&self) -> Self {
        Self {
            executable: Rc::clone(&self.executable),
            data: self.data.clone(),
            program_counter: self.program_counter,
        }
    }
}

impl PartialEq for ExecutorViewProps {
    fn eq(&self, other: &Self) -> bool {
        // Logic to compare two ExecutorViewProps.
        // If you only need reference equality for the Rc:
        Rc::ptr_eq(&self.executable, &other.executable)
            && self.data == other.data
            && self.program_counter == other.program_counter
    }
}

impl Component for ExecutorView {
    type Message = ExecutorViewMessage;
    type Properties = ExecutorViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        let mut ret = Self {
            props: props.clone(),
            selected_tab: 0,
            instructions: Vec::new(),
        };
        Component::changed(&mut ret, ctx, &props);
        ret
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &ExecutorViewProps) -> bool {
        let functions = (*props.executable).clone().into_inner().ir.functions;

        let mut instructions: Vec<ExecutorViewInstruction> = Vec::new();
        for function in &functions {
            for block in &function.blocks {
                for instr in &block.instructions {
                    let bytecode_position = match instr.position {
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

                    let next_instr = ExecutorViewInstruction {
                        bytecode_position,
                        hex_bytecode_position: format!("0x{:02x}", bytecode_position).to_string(),
                        text: instruction_value,
                        ir_origin: "".to_string(),
                    };
                    instructions.push(next_instr);
                }
            }
        }
        self.props = ctx.props().clone();
        ctx.link()
            .send_message(ExecutorViewMessage::SetInstructions(instructions));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExecutorViewMessage::SelectTab(index) => {
                self.selected_tab = index;
                true
            }
            ExecutorViewMessage::SetInstructions(instructions) => {
                self.instructions = instructions;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let program_counter = self.props.program_counter;

        html! {
            <div class="h-full w-full flex flex-col relative">
                <div class="space-y-6 flex flex-col flex-grow">
                            <div class="p-4 bg-white border rounded-md shadow-sm space-y-2 overflow-y-auto">
                                { for self.instructions.iter().map(|instr| html! {
                                    <div class={
                                        if program_counter == instr.bytecode_position {
                                          "flex items-center space-x-4 text-sm text-gray-700 bg-green-600"
                                      }
                                      else
                                      {
                                         "flex items-center space-x-4 text-sm text-gray-700"
                                      }
                                    }>
                                        <input type="checkbox" class="form-checkbox h-5 w-5 text-blue-600 rounded-full" />
                                        <div>{instr.hex_bytecode_position.clone()}</div>
                                        <div>{instr.text.clone()}</div>
                                    </div>
                                }) }
                            </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties)]
pub struct MainCardProps {
    pub children: Children,
    pub executable: Option<Rc<RefCell<EvmExecutable>>>,
}

impl Clone for MainCardProps {
    fn clone(&self) -> Self {
        let executable = if let Some(e) = &self.executable {
            Some(Rc::clone(&e))
        } else {
            None
        };
        Self {
            executable,
            children: self.children.clone(),
        }
    }
}

impl PartialEq for MainCardProps {
    fn eq(&self, other: &Self) -> bool {
        if self.executable.is_none() && other.executable.is_none() {
            return self.children == other.children;
        }
        let e1 = match &self.executable {
            Some(e) => e,
            None => return false,
        };

        let e2 = match &other.executable {
            Some(e) => e,
            None => return false,
        };

        Rc::ptr_eq(&e1, &e2) && self.children == other.children
    }
}

pub struct MainCard {
    props: MainCardProps,
    observable_machine: Option<ObservableMachine>,
    data: String,
    view: usize,
}

pub enum MainCardMessage {
    ResetMachine {
        code: Rc<Vec<u8>>,
        data: Rc<Vec<u8>>,
    }, // Add other messages here if needed
    RunStep,
    SelectView(usize),
}

impl Component for MainCard {
    type Message = MainCardMessage;
    type Properties = MainCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        let mut ret = Self {
            props: props.clone(),
            observable_machine: None,
            data: "".to_string(),
            view: 0,
        };

        Component::changed(&mut ret, ctx, &props);
        ret
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainCardMessage::SelectView(index) => {
                self.view = index;
                true
            }
            MainCardMessage::ResetMachine { code, data } => {
                console::log!("Code: {}", hex::encode(&*code));
                self.observable_machine = Some(ObservableMachine::new(code, data, 1024, 1024));
                true
            }
            MainCardMessage::RunStep => {
                if let Some(ref mut machine) = self.observable_machine {
                    machine.step();
                    true
                } else {
                    false
                }
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>, _props: &MainCardProps) -> bool {
        let data = if self.data != "" {
            hex::decode(self.data.clone())
        } else {
            Ok(Vec::<u8>::new())
        };

        if let Some(ref e) = &ctx.props().executable {
            if let Ok(data) = data {
                let code: Vec<u8> = (&*e.borrow().bytecode).to_vec(); //.into_inner().bytecode;
                ctx.link().send_message(MainCardMessage::ResetMachine {
                    code: code.into(),
                    data: data.into(),
                });
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (program_counter, error_message) =
            if let Some(observable_machine) = &self.observable_machine {
                let machine = &observable_machine.machine;
                let program_counter = if let Ok(pc) = machine.position() {
                    pc
                } else {
                    &0
                };

                let error = if observable_machine.failed {
                    &observable_machine.error_message
                } else {
                    &None
                };
                ((*program_counter as u32), error)
            } else {
                (0, &None)
            };
        let step_button_click = ctx.link().callback(move |_| MainCardMessage::RunStep);

        let run_button_click = Callback::from(move |_| {
            console::log!("Run Button clicked");
        });

        html! {
        <div class="pl-20 h-screen w-screen">
          <div class="relative z-50 lg:hidden" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-gray-900/80"></div>

            <div class="fixed inset-0 flex">
              <div class="relative mr-16 flex w-full max-w-xs flex-1">
                <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
                  <button type="button" class="-m-2.5 p-2.5">
                    <span class="sr-only">{"Close sidebar"}</span>
                    <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>

                <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-2 ring-1 ring-white/10">
                  <div class="flex h-16 shrink-0 items-center">
                    <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company" />
                  </div>
                  <nav class="flex flex-1 flex-col">
                    <ul role="list" class="-mx-2 flex-1 space-y-1">
                      <li>
                        <a href="#" class="bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                          </svg>
                          {"Dashboard"}
                        </a>
                      </li>
                      <li>
                        <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
                          </svg>
                          {"Team"}
                        </a>
                      </li>
                      <li>
                        <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                          </svg>
                          {"Projects"}
                        </a>
                      </li>
                      <li>
                        <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5" />
                          </svg>
                          {"Calendar"}
                        </a>
                      </li>
                      <li>
                        <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75" />
                          </svg>
                          {"Documents"}
                        </a>
                      </li>
                      <li>
                        <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold">
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z" />
                          </svg>
                          {"Reports"}
                        </a>
                      </li>
                    </ul>
                  </nav>
                </div>
              </div>
            </div>
          </div>

          /* Static sidebar for desktop */
          <div class="hidden lg:fixed lg:inset-y-0 lg:left-0 lg:z-50 lg:block lg:w-20 lg:overflow-y-auto lg:bg-gray-900 lg:pb-4">
            <div class="flex h-16 shrink-0 items-center justify-center">
              <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company" />
            </div>
            <nav class="mt-8">
              <ul role="list" class="flex flex-col items-center space-y-1">
                <li>
                  /* Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" */
                  <a href="#" class="bg-gray-800 text-white group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                    </svg>
                    <span class="sr-only">{"Dashboard"}</span>
                  </a>
                </li>
                <li>
                  <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
                    </svg>
                    <span class="sr-only">{"Team"}</span>
                  </a>
                </li>
                <li>
                  <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                    </svg>
                    <span class="sr-only">{"Projects"}</span>
                  </a>
                </li>
                <li>
                  <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5" />
                    </svg>
                    <span class="sr-only">{"Calendar"}</span>
                  </a>
                </li>
                <li>
                  <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75" />
                    </svg>
                    <span class="sr-only">{"Documents"}</span>
                  </a>
                </li>
                <li>
                  <a href="#" class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold">
                    <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z" />
                      <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z" />
                    </svg>
                    <span class="sr-only">{"Reports"}</span>
                  </a>
                </li>
              </ul>
            </nav>
          </div>

          <div class="sticky top-0 z-40 flex items-center gap-x-6 bg-gray-900 px-4 py-4 shadow-sm sm:px-6 lg:hidden">
            <button type="button" class="-m-2.5 p-2.5 text-gray-400 lg:hidden">
              <span class="sr-only">{"Open sidebar"}</span>
              <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
              </svg>
            </button>
            <div class="flex-1 text-sm font-semibold leading-6 text-white">{"Dashboard"}</div>
            <a href="#">
              <span class="sr-only">{"Your profile"}</span>
              <img class="h-8 w-8 rounded-full bg-gray-800" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
            </a>
          </div>

          <main class="h-full w-full">
            <div class="xl:pr-96">
              <div class="relative">
                { for self.props.children.iter() }
                /* Main area */
              </div>
            </div>
          </main>
          <aside class="bg-white fixed inset-y-0 right-0 hidden w-96 overflow-y-auto border-l border-gray-200 px-4 py-6 sm:px-6 lg:px-8 xl:block">
            {
                if let Some(executable) = &self.props.executable {
                    html! { <ExecutorView executable={executable} data={""} program_counter={program_counter} /> }
                } else {
                    html! {
                        <div>{"Nothing compiled yet."}</div>
                    }
                }
            }
          </aside>
            {
                      if let Some(_) = &self.props.executable {
                        html!{
                        <FloatingCard>
                            <div class="space-x-4 ">
                                <button class="bg-blue-600 text-white px-2 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500" onclick={step_button_click.clone()}>
                                    {"Step"}
                                </button>
                                <button class="bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500" onclick={run_button_click.clone()}>
                                    {"Run"}
                                </button>
                                <div>{format!("0x{:02x}", program_counter)} <span>{" ("}{program_counter}{")"}</span></div>
                            </div>
                        </FloatingCard>
                    }
                    } else {
                        html! {}

                    }

                }
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
                console::log!(
                    "Produced code: {}",
                    hex::encode(&exec.executable.bytecode.clone())
                );
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

            source_code.set(value);
        })
    };

    let source_code: String = source_code.to_string();

    // TODO: Hack to fix the fact that MainCard is not updating correctly with
    // the executable

    let key = if let Some(_executable) = &*executable {
        "second"
    } else {
        "first"
    };
    html! {
        <MainCard key={key} executable={&*executable}>
                <div class="h-full w-full">
                <textarea
                    id="source-code"
                    class="w-full h-screen  bg-black text-white resize-none font-mono"
                    style="tab-size: 4; white-space: pre;"

                    value={source_code}
                    oninput={handle_source_code_change}
                    placeholder="Enter Scilla source code here..."
                />


                <div class="absolute -right-8 top-10 z-20 flex justify-center items-center">
                    <button class="h-16 w-16 bg-indigo-600 text-lg text-white px-6 py-2 rounded-full shadow-sm hover:bg-indigo-700 focus:bg-indigo-800 focus:outline-none focus:ring focus:ring-indigo-200 active:bg-indigo-800 transition duration-150" onclick={compile_button_click.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 00-3.7-3.7 48.678 48.678 0 00-7.324 0 4.006 4.006 0 00-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3l-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 003.7 3.7 48.656 48.656 0 007.324 0 4.006 4.006 0 003.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3l-3 3" />
                        </svg>
                    </button>
                </div>
                </div>
        </MainCard>
    }
}
