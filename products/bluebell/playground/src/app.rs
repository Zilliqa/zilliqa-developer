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
use crate::vm_remote::VmRemoteControl;
use crate::bytecode_view::ByteCodeView;
use crate::machine_view::MachineView;

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
    pub view: u32
}

pub struct AppLayout {
    props: AppLayoutProps,
    data: String,
    dispatch: Dispatch<State>,
    state: Rc<State>,
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
            state: dispatch.get(),
            dispatch,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true        
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppLayoutMessage::UpdateState(state) => {
                self.state = state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let step_button_click = self.dispatch.apply_callback(|_| StateMessage::RunStep);

        let run_button_click = Callback::from(move |_| {
            console::log!("Run Button clicked 2");
        });


        let generate_menu = {
            let selected_view  = self.props.view as u32;
            move |item: &MenuItem, is_desktop: bool| {
                let mut item_class = if is_desktop {
                    "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold".to_string()
                } else {
                    "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold".to_string()
                };

                if item.index == selected_view {
                    item_class.push_str(" bg-gray-700");
                }
                let index = item.index;

                html! {
                    <li>
                        <a href="#" class={item_class} onclick={self.props.on_select_view.reform(move |_| index)}>
                            { item.icon.clone() }
                            { if !is_desktop { html! { &item.text } } else { html! {} } }
                        </a>
                    </li>
                }
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
                        <VmRemoteControl>
                            <div class="space-x-4 ">
                                <button class="bg-blue-600 text-white px-2 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500" onclick={step_button_click.clone()}>
                                    {"Step"}
                                </button>
                                <button class="bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500" onclick={run_button_click.clone()}>
                                    {"Run"}
                                </button>
                                <div>{format!("0x{:02x}", self.state.program_counter)} <span>{" ("}{self.state.program_counter}{")"}</span></div>
                            </div>
                        </VmRemoteControl>
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
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
  <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
</svg>
            },
            text: "Code Editor".to_string(),
            index: 0
        },
        MenuItem {
            icon: html!{
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
</svg>

            },
            text: "Virtual Machine".to_string(),
            index: 1
        },
        MenuItem {
            icon: html!{
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0113.5 9.375v-4.5z" />
  <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 6.75h.75v.75h-.75v-.75zM6.75 16.5h.75v.75h-.75v-.75zM16.5 6.75h.75v.75h-.75v-.75zM13.5 13.5h.75v.75h-.75v-.75zM13.5 19.5h.75v.75h-.75v-.75zM19.5 13.5h.75v.75h-.75v-.75zM19.5 19.5h.75v.75h-.75v-.75zM16.5 16.5h.75v.75h-.75v-.75z" />
</svg>
            },
            text: "Bytecode".to_string(),
            index: 2
        }

    ].to_vec();

    html! {
        <AppLayout key={*current_view} menu={menu} on_select_view={set_current_view} view={*current_view}>

            { if *current_view == 0 {

                html! {
                <div class="h-full w-full pl-10 bg-black">
                <textarea
                    id="source-code"
                    class="w-full h-screen  bg-black text-white resize-none font-mono focus:none outline:none"
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
