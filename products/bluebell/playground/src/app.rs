use std::{collections::HashMap, rc::Rc};

use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

use crate::{
    bytecode_view::ByteCodeView,
    dropdown::Dropdown,
    examples::EXAMPLES,
    logger::LoggerView,
    machine_view::MachineView,
    state::{ExecutionStatus, State, StateMessage},
    vm_remote::VmRemote,
};

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
    pub view: u32,
}

pub struct AppLayout {
    props: AppLayoutProps,
    _dispatch: Dispatch<State>,
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
            state: dispatch.get(),
            _dispatch: dispatch,
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let generate_menu = {
            let selected_view = self.props.view as u32;
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
                    <img class="h-8 w-auto" src="img/logo.png" alt="Your Company" />
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
              <img class="h-8 w-auto" src="img/logo.png" alt="Your Company" />
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
            <div class="xl:pr-96 h-full">
              <div class="relative h-full w-full">
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
                    html! {
                        <VmRemote key="remote" />

                    }
                } else {
                    html! {}
                }
            }
        </div>
        }
    }
}

fn line_to_pixel_offset(line: usize) -> usize {
    const FONT_SIZE: usize = 24; /* font size in pixels */
    const LINE_HEIGHT: f32 = 1.0; /* line height as a multiplier */
    (line - 1) * (FONT_SIZE as f32 * LINE_HEIGHT) as usize
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_store_value::<State>();
    let source_code = &state.source_code;
    let bytecode_hex = &state.bytecode_hex;
    let program_counter = state.program_counter as usize;

    let current_view_state: UseStateHandle<u32> = use_state(|| 0);
    let current_view = current_view_state.clone();
    let set_current_view = move |i| {
        current_view_state.set(i);
    };
    let exit_message = &state.exit_message;

    let compile_button_click = {
        let source_code = source_code.clone();

        Callback::from(move |_| {
            let dispatch = Dispatch::<State>::new();
            dispatch.reduce_mut(move |s| s.compiling = true);

            // Adding delay for improved UX
            let source_code = source_code.clone();
            Timeout::new(500, move || {
                let dispatch = Dispatch::<State>::new();
                dispatch.apply(StateMessage::CompileCode {
                    source_code: source_code.to_string(),
                });
            })
            .forget();
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
    let error_map: HashMap<usize, String> = HashMap::new();

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

    let is_compiling = state.compiling;
    let show_console = true;

    html! {
            <AppLayout key={*current_view} menu={menu} on_select_view={set_current_view} view={*current_view}>
            {
                if let Some((error_code, exit_message)) = exit_message {
                    html! {
                        <div class="z-10 pointer-events-none fixed inset-x-0 bottom-0 sm:flex sm:justify-center sm:px-6 sm:pb-5 lg:px-8">
                          <div class={
                                match *error_code {
                                    ExecutionStatus::Succeeded => "pointer-events-auto flex items-center justify-between gap-x-6 bg-green-600 px-6 py-2.5 sm:rounded-xl sm:py-3 sm:pl-4 sm:pr-3.5",
                                    // Unused ExecutionStatus::Paused => "pointer-events-auto flex items-center justify-between gap-x-6 bg-yellow-600 px-6 py-2.5 sm:rounded-xl sm:py-3 sm:pl-4 sm:pr-3.5",
                                    ExecutionStatus::Failed => "pointer-events-auto flex items-center justify-between gap-x-6 bg-red-600 px-6 py-2.5 sm:rounded-xl sm:py-3 sm:pl-4 sm:pr-3.5"
                                }
                            }>
                            <p class="text-sm leading-6 text-white">
                              <a href="#">
                                <strong class="font-semibold">
    {
                                match *error_code {
                                    ExecutionStatus::Succeeded => "Execution suceeded",
                                    // Unused ExecutionStatus::Paused => "Execution paused",
                                    ExecutionStatus::Failed => "Execution failed"
                                }
                            }

                                </strong>
                                <svg viewBox="0 0 2 2" class="mx-2 inline h-0.5 w-0.5 fill-current" aria-hidden="true"><circle cx="1" cy="1" r="1" /></svg>
                                {exit_message}<span aria-hidden="true"></span>
                              </a>
                            </p>
                            <button type="button" class="-m-1.5 flex-none p-1.5">
                              <span class="sr-only">{"Dismiss"}</span>
                              <svg class="h-5 w-5 text-white" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                              </svg>
                            </button>
                          </div>
                        </div>
                    }

                } else {
                     html!{}
                }
            }
                    { if *current_view == 0 {

                        html! {
                            <div class="h-full w-full flex flex-col bg-black">
                                <div class="ml-10  editor-container flex-1 w-full bg-black  text-white text-left font-mono">
                                    <div class="w-full flex items-center justify-center py-2">
                                        <Dropdown items={EXAMPLES.iter().map(|item| item.0.to_string()).collect::<Vec<_>>()}    on_item_click={|i:usize| {
                                            let value: String = EXAMPLES[i].1.to_string().clone();
                                            Dispatch::<State>::new().reduce_mut(move | s| {
                                                s.program_counter = 0;
                                                s.executable = None;
                                                s.observable_machine = None;
                                                s.pc_to_position = HashMap::new();
                                                s.current_position = None;
                                                s.source_code = value
                                            })
                                        }}    />
                                    </div>
                                    {
                                        if let Some(p) = state.current_position {
                                            let (_, _, line, _) = p;
                                            let pos = if line < usize::MAX/2 {
                                                line_to_pixel_offset(line+1)
                                            }
                                            else
                                            {
                                                0
                                            };
                                            html! {
                                            <div class="bg-blue-600 w-full h-6 absolute" style={format!("top: {}px", pos)}>
                                            </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <pre class="highlighted-code h-full w-full flex flex-col items-stretch hidden">
                                        <code class="language-scilla h-full w-full"> /* TODO: Fix highligther */
                                        {source_code.clone()}
                                        </code>
                                    </pre>
                                    <textarea
                                        class="overlay-textarea text-white outline-0 text-left font-mono h-full w-full focus:none"
                                        value={source_code}
                                        oninput={handle_source_code_change}
                                    />

                                    { for error_map.iter().map(|(line, error)| {
                                        html! {
                                            <div class="error-annotation" style={format!("top: {}px", line_to_pixel_offset(*line))}>
                                                { error }
                                            </div>
                                        }
                                    })}
                                </div>


                                {
                                    if show_console {
                                        html! { <LoggerView /> }
                                    } else {
                                        html! {}
                                    }
                                }



                                <div class="absolute -right-8 top-10 z-20 flex justify-center items-center">
                                    <button onclick={compile_button_click.clone()} class={
                                        if state.executable.is_none() {
                                            "ease-in-out delay-150 transition transition-opacity opacity-1 h-16 w-16 bg-indigo-600 text-lg text-white px-2 py-2 rounded-full shadow-sm hover:bg-indigo-700 focus:bg-indigo-800 focus:outline-none focus:ring focus:ring-indigo-200 active:bg-indigo-800 transition duration-150 flex items-center justify-center"
                                        } else {
                                            "ease-in-out delay-150 transition transition-opacity opacity-0 h-16 w-16 bg-indigo-600 text-lg text-white px-2 py-2 rounded-full shadow-sm hover:bg-indigo-700 focus:bg-indigo-800 focus:outline-none focus:ring focus:ring-indigo-200 active:bg-indigo-800 transition duration-150 flex items-center justify-center"
                                        }
                                    }>
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={
                                            if is_compiling {
                                                "w-6 h-6 animate-spin"
                                            } else {
                                                "w-6 h-6"
                                            }
                                        }>
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
                    } else  if *current_view == 2 {

                        let (before, byte_at_pc, after) = if program_counter * 2 < bytecode_hex.len() {
                            let (start, rest) = bytecode_hex.split_at(program_counter * 2);
                            let (byte, end) = rest.split_at(2);
                            (start, byte, end)
                        } else {
                            (&bytecode_hex[..], "", "")
                        };


                        html! {
                            <div class="h-full w-full pl-10 bg-black">
                                <div
                                    id="bytecode"
                                    class="text-left break-words w-full h-screen bg-black text-white font-mono focus:none outline:none"
                                >
                                    { before }
                                    <span class="bg-yellow-600 text-black px-1">{ byte_at_pc }</span>
                                    { after }
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div>{"Undefined view"}</div>
                        }
                    }
                }


            </AppLayout>
        }
}
