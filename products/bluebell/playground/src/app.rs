use std::cell::RefCell;
use std::rc::Rc;
use yew::virtual_dom::VNode;

use evm_assembly::executable::EvmExecutable;
use gloo_console as console;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::{State, StateMessage};
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

pub struct ByteCodeViewInstruction {
    pub label: Option<String>,
    pub bytecode_position: u32,
    pub hex_bytecode_position: String,
    pub text: String,
    pub comment: String,
}

pub struct ByteCodeView {
    props: ByteCodeViewProps,
    selected_tab: usize,
    instructions: Vec<ByteCodeViewInstruction>,
}
pub enum ByteCodeViewMessage {
    SelectTab(usize),
    SetInstructions(Vec<ByteCodeViewInstruction>),
}
#[derive(Properties)]
pub struct ByteCodeViewProps {
    pub executable: Rc<RefCell<EvmExecutable>>,
    pub data: String,
    pub program_counter: u32,
}

impl Clone for ByteCodeViewProps {
    fn clone(&self) -> Self {
        Self {
            executable: Rc::clone(&self.executable),
            data: self.data.clone(),
            program_counter: self.program_counter,
        }
    }
}

impl PartialEq for ByteCodeViewProps {
    fn eq(&self, other: &Self) -> bool {
        // Logic to compare two ByteCodeViewProps.
        // If you only need reference equality for the Rc:
        Rc::ptr_eq(&self.executable, &other.executable)
            && self.data == other.data
            && self.program_counter == other.program_counter
    }
}

impl Component for ByteCodeView {
    type Message = ByteCodeViewMessage;
    type Properties = ByteCodeViewProps;

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

    fn changed(&mut self, ctx: &Context<Self>, props: &ByteCodeViewProps) -> bool {
        let functions = (*props.executable).clone().into_inner().ir.functions;

        let mut instructions: Vec<ByteCodeViewInstruction> = Vec::new();
        for function in &functions {
            for block in &function.blocks {
                let mut next_label = Some(block.name.clone());

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

                    let next_instr = ByteCodeViewInstruction {
                        label: next_label,
                        bytecode_position,
                        hex_bytecode_position: format!("0x{:02x}", bytecode_position).to_string(),
                        text: instruction_value,
                        comment: instr.comment.clone().unwrap_or("".to_string()),
                    };
                    instructions.push(next_instr);
                    next_label = None;
                }
            }
        }
        self.props = ctx.props().clone();
        ctx.link()
            .send_message(ByteCodeViewMessage::SetInstructions(instructions));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ByteCodeViewMessage::SelectTab(index) => {
                self.selected_tab = index;
                true
            }
            ByteCodeViewMessage::SetInstructions(instructions) => {
                self.instructions = instructions;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let program_counter = self.props.program_counter;

        html! {
                <>
                    <h2 class="text-2xl ">{"EVM Bytecode"}</h2>
                    <div class="px-4 bg-white overflow-auto space-y-1">
                        {
                            for self.instructions.iter().map(|instr| html! {
                                <>
                                    {
                                        if let Some(label) = &instr.label {
                                            html! {
                                                <div class="pt-6 font-medium flex items-center space-x-4 text-sm text-gray-700">
                                                {label}
                                                </div>
                                            }
                                        } else {
                                            html!{}
                                        }
                                    }

                                {
                                    if instr.comment.len() > 0 {
                                        html! {
                                            <div class="flex items-center space-x-4 text-sm text-gray-700 whitespace-nowrap">
                                                <span class="h-5 w-5"></span>
                                                <div class="ml-4 font-mono text-xs text-gray-400">{instr.comment.clone()}</div>
                                            </div>
                                        }
                                    } else {
                                        html!{}
                                    }
                                }

                                <div class={
                                    if program_counter == instr.bytecode_position {
                                      "flex items-center space-x-4 text-sm text-gray-700 bg-green-600 whitespace-nowrap"
                                  }
                                  else
                                  {
                                     "flex items-center space-x-4 text-sm text-gray-700 whitespace-nowrap"
                                  }
                                }>
                                    <input type="checkbox" class="form-checkbox h-5 w-5 text-blue-600 rounded-full" />
                                    <div>{instr.hex_bytecode_position.clone()}</div>
                                    <div>{instr.text.clone()}</div>
                                </div>

                            </>
                        })
                    }
                    </div>
                </>

        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub icon: VNode,
    pub text: String,
    pub index: u32,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AppLayoutProps {
    pub children: Children,
    pub on_select_view: Callback<u32>,
    pub menu: Vec<MenuItem>,
}

pub struct AppLayout {
    props: AppLayoutProps,
    data: String,
    view: usize,
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

pub struct MachineView {
    dispatch: Dispatch<State>,
    state: Rc<State>,
}

pub enum MachineViewMessage {
    UpdateState(Rc<State>),
}

impl Component for MachineView {
    type Message = MachineViewMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state_callback = ctx.link().callback(MachineViewMessage::UpdateState);
        let dispatch = Dispatch::<State>::subscribe(state_callback);
        Self {
            state: dispatch.get(),
            dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MachineViewMessage::UpdateState(state) => {
                self.state = state;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let observable_machine = match &self.state.observable_machine {
            Some(o) => o,
            None => {
                return html! {
                    <div>{"No machine started yet."}</div>
                }
            }
        };
        let machine = &(observable_machine.borrow()).machine;

        html! {
            <div class="bg-black min-h-full max-h-full h-screen w-full p-4">
                <h2 class="text-xl font-bold mb-4">{"EVM Machine State"}</h2>
                <div class="grid grid-rows-2 lg:grid-cols-2 gap-4">
                    <div class="border p-4">
                        <h3 class="text-lg font-semibold mb-2">{"Stack"}</h3>
                        <ul>
                            { for machine.stack().data().iter().rev().map(|item| html! { <li>{format!("{:?}", item)}</li> }) }
                        </ul>
                    </div>
                    <div class="border p-4">
                        <h3 class="text-lg font-semibold mb-2">{"Memory"}</h3>
                        <div class="grid grid-cols-12">
                            { for machine.memory().data().chunks(32).enumerate().map(|(idx, chunk)| {
                                let address = idx * 32;
                                let segment: String = chunk.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
                                html! {
                                    <>
                                    <div class="col-span-2">{format!("0x{:02x}", address)}</div>
                                    <div class="col-span-10">{format!("{}", segment)}</div>
                                    </>
                                }
                            }) }
                        </div>

                    </div>
                </div>
                <div class="mt-4">
                    <h3 class="text-lg font-semibold mb-2">{"Program Counter"}</h3>
                    <p>{format!("{:?}", machine.position())}</p>
                </div>
            </div>
        }
    }
}

pub enum AppLayoutMessage {
    UpdateState(Rc<State>),
}

impl Component for AppLayout {
    type Message = AppLayoutMessage;
    type Properties = AppLayoutProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        let state_callback = ctx.link().callback(AppLayoutMessage::UpdateState);
        let dispatch = Dispatch::<State>::subscribe(state_callback);

        Self {
            props: props.clone(),
            data: "".to_string(),
            view: 0,
            state: dispatch.get(),
            dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppLayoutMessage::UpdateState(state) => {
                self.state = state;
            }
        }
        true
    }

    /*
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppLayoutMessage::SelectView(index) => {
                self.view = index;
                true
            }
            AppLayoutMessage::ResetMachine { code, data } => {
                console::log!("Code: {}", hex::encode(&*code));
                self.observable_machine = Some(Rc::new(RefCell::new(ObservableMachine::new(
                    code, data, 1024, 1024,
                ))));
                true
            }
            AppLayoutMessage::RunStep => {
                if let Some(ref mut machine) = self.observable_machine {
                    machine.borrow_mut().step();
                    true
                } else {
                    false
                }
            }
        }
    }
    */

    fn view(&self, ctx: &Context<Self>) -> Html {
        let step_button_click = self.dispatch.apply_callback(|_| StateMessage::RunStep);

        let run_button_click = Callback::from(move |_| {
            console::log!("Run Button clicked");
        });

        let generate_menu = |item: &MenuItem, is_desktop: bool| {
            let item_class = if is_desktop {
                "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold"
            } else {
                "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
            };

            let index = item.index;

            html! {
                <li>
                    <a href="#" class={item_class} onclick={self.props.on_select_view.reform(move |_| index)}>
                        { item.icon.clone() }
                        { if !is_desktop { html! { &item.text } } else { html! {} } }
                    </a>
                </li>
            }
        };

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
                        { for self.props.menu.iter().map(|item| generate_menu(item, false)) }
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
                        { for self.props.menu.iter().map(|item| generate_menu(item, true)) }
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
            <div>{self.state.program_counter}</div>
            {
                if let Some(executable) = &self.state.executable {
                    html! { <ByteCodeView executable={executable} data={""} program_counter={self.state.program_counter} /> }
                } else {
                    html! {
                        <div>{"Nothing compiled yet."}</div>
                    }
                }
            }
          </aside>
            {
                      if let Some(_) = &self.state.executable {
                        html!{
                        <FloatingCard>
                            <div class="space-x-4 ">
                                <button class="bg-blue-600 text-white px-2 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500" onclick={step_button_click.clone()}>
                                    {"Step"}
                                </button>
                                <button class="bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500" onclick={run_button_click.clone()}>
                                    {"Run"}
                                </button>
                                <div>{format!("0x{:02x}", self.state.program_counter)} <span>{" ("}{self.state.program_counter}{")"}</span></div>
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
    let state = use_store_value::<State>();
    let source_code = &state.source_code;

    let current_view_state: UseStateHandle<u32> = use_state(|| 0);
    let current_view = current_view_state.clone();
    let set_current_view = move |i| {
        current_view_state.set(i);
    };

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

                let code: Vec<u8> = (&*exec.executable.bytecode).to_vec();

                let dispatch = Dispatch::<State>::new();
                dispatch.reduce_mut(move |s| {
                    s.executable = Some(Rc::new(RefCell::new(exec.executable)))
                });

                dispatch.apply(StateMessage::ResetMachine {
                    code: code.into(),
                    data: [].to_vec().into(), // TODO:
                });
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
        Dispatch::<State>::new().reduce_mut_callback_with(move |s, e: InputEvent| {
            let value = target_input_value(&e);

            s.source_code = value
        })
    };

    let source_code: String = source_code.to_string();

    let menu : Vec< MenuItem > = [
        MenuItem {
            icon: html!{
<svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                    </svg>
            },
            text: "Code Editor".to_string(),
            index: 0
        },
        MenuItem {
            icon: html!{
<svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                    </svg>
            },
            text: "Virtual Machine".to_string(),
            index: 1
        },
        MenuItem {
            icon: html!{
<svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                    </svg>
            },
            text: "Bytecode".to_string(),
            index: 2
        }

    ].to_vec();

    html! {
        <AppLayout key={*current_view} menu={menu} on_select_view={set_current_view}>

            { if *current_view == 0 {

                html! {
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
            }
        } else if *current_view == 1 {
                    html! {
                        <MachineView  />
                    }
        }  else {
            html! {
                <div>{"Undefined view"}</div>
            }
        }
    }
        </AppLayout>
    }
}
