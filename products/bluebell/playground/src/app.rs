use evm_assembly::function_signature::EvmFunctionSignature;
use std::collections::HashMap;
use std::rc::Rc;

use yew::virtual_dom::VNode;

use gloo_console as console;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::bytecode_view::ByteCodeView;
use crate::dropdown::Dropdown;
use crate::examples::EXAMPLES;
use crate::machine_view::MachineView;
use crate::state::{State, StateMessage};
use crate::vm_remote::VmRemoteControl;
use gloo_timers::callback::Timeout;

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
    data: String,
    dispatch: Dispatch<State>,
    state: Rc<State>,
    function_signature: Option<EvmFunctionSignature>,
    arguments: Vec<String>,
}

pub enum AppLayoutMessage {
    UpdateState(Rc<State>),
    UpdateFunctionSignature(Option<EvmFunctionSignature>),
    SetArguments(Vec<String>),
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
            function_signature: None,
            arguments: [].to_vec(),
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
            AppLayoutMessage::UpdateFunctionSignature(signature) => {
                self.function_signature = signature;
                let n = if let Some(signature) = &self.function_signature {
                    signature.arguments.len()
                } else {
                    0
                };
                self.arguments = vec![String::new(); n];
            }
            AppLayoutMessage::SetArguments(args) => {
                self.arguments = args;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let step_button_click = self.dispatch.apply_callback(|_| StateMessage::RunStep);
        let stop_button_click = self.dispatch.apply_callback(|_| StateMessage::Reset);
        let set_function_signature = ctx
            .link()
            .callback(|value| AppLayoutMessage::UpdateFunctionSignature(value));
        let set_arguments = ctx
            .link()
            .callback(|value| AppLayoutMessage::SetArguments(value));

        let run_button_click = Callback::from(move |_| {
            let dispatch = Dispatch::<State>::new();
            dispatch.reduce_mut(move |s| {
                s.playing = !s.playing;
                if s.playing {
                    Timeout::new(500, move || {
                        let dispatch = Dispatch::<State>::new();
                        dispatch.apply(StateMessage::RunStep);
                    })
                    .forget();
                }
            });
        });

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

        let functions = self.state.functions.clone();
        let functions_clone = self.state.functions.clone();
        let signature = self.function_signature.clone();
        let arguments = self.arguments.clone();

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
                        <VmRemoteControl>
                            <div class="flex flex-col items-center space-y-4">
                                <div
                                    class="p-2 bg-zinc-900 w-full text-gray-100 rounded-md space-x-2 flex flex-col"
                                >
                                    <div class="w-full" onmouseover={move |e: MouseEvent| e.stop_propagation()}
                                     onmousedown={move |e: MouseEvent| e.stop_propagation()}
                                     onmouseup={move |e: MouseEvent| e.stop_propagation()}>
                                        <select class="w-full bg-zinc-900 py-2" onchange={move |e: Event| {
                                            let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                            if value != "none" {
                                                let value : usize = value.parse::<usize>().unwrap();
                                                set_function_signature.emit(functions_clone.get(value).cloned())
                                            } else {
                                                set_function_signature.emit(None)
                                            }
                                        }}>
                                            <option value="none" selected={true}>{"(Select function)"}</option>
                                            {functions.iter().enumerate().map(|(i, v)| html! {
                                                <option value={i.to_string()}>{v.name.clone()}</option>
                                            }).collect::<Vec<_>>()}
                                        </select>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="font-bold">{"PC:"}</span>
                                        <span>{format!("0x{:02x}", self.state.program_counter)}</span>
                                        <span>{format!("({})", self.state.program_counter)}</span>
                                    </div>
                                </div>
                                { if let Some(signature) = signature {
                                    html! {
                                        <>
                                        <div class="w-full"  onmouseover={move |e: MouseEvent| e.stop_propagation()}
                                     onmousedown={move |e: MouseEvent| e.stop_propagation()}
                                     onmouseup={move |e: MouseEvent| e.stop_propagation()}>
                                            {signature.arguments.iter().zip(arguments.iter()).enumerate().map(|(i, (t, v))| {
                                                let arguments = arguments.clone();
                                                let set_arguments = set_arguments.clone();
                                                html! {
                                                    <div>
                                                        <label>{format!("{:?}", t)}</label>
                                                        <input key={format!("input{}",i)} value={format!("{}", v)} onkeyup={move |e:KeyboardEvent| {
                                                                let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                                                let mut arguments = arguments.clone();
                                                                arguments[i] = value.clone();
                                                                set_arguments.emit(arguments);
                                                                console::log!(format!("Arg {} is '{}'", i, value));
                                                            }} />
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                        <div class="flex items-center space-x-4">

                                            <button
                                                class="flex items-center justify-center bg-blue-600 text-white w-12 h-12 rounded-full hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 text-center"
                                                onclick={step_button_click.clone()}
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                  <path stroke-linecap="round" stroke-linejoin="round" d="M15 15l6-6m0 0l-6-6m6 6H9a6 6 0 000 12h3" />
                                                </svg>

                                            </button>

                                            <button
                                                class="flex items-center justify-center bg-green-600 text-white w-14 h-14 rounded-full hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 text-center"
                                                onclick={run_button_click.clone()}
                                            >
                                                {
                                                    if self.state.playing {
                                                        html!{
                                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                              <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
                                                            </svg>
                                                        }
                                                    } else {
                                                        html! {
                                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                              <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
                                                            </svg>
                                                        }
                                                    }
                                                }

                                            </button>

                                            <button
                                                class="flex items-center justify-center bg-red-600 text-white w-10 h-10 rounded-full hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 text-center"
                                                onclick={stop_button_click.clone()}
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                  <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 7.5A2.25 2.25 0 017.5 5.25h9a2.25 2.25 0 012.25 2.25v9a2.25 2.25 0 01-2.25 2.25h-9a2.25 2.25 0 01-2.25-2.25v-9z" />
                                                </svg>

                                            </button>

                                        </div>
                                        </>
                                    }
                                } else {
                                    html!{}
                                }}

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

fn line_to_pixel_offset(line: usize) -> usize {
    const FONT_SIZE: usize = 24; /* font size in pixels */
    const LINE_HEIGHT: f32 = 1.0; /* line height as a multiplier */
    console::log!("Line ", line);
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

    html! {
        <AppLayout key={*current_view} menu={menu} on_select_view={set_current_view} view={*current_view}>

            { if *current_view == 0 {

                html! {
                    <div class="h-full w-full pl-10 bg-black">
                            <div class="editor-container h-full w-full bg-black  text-white text-left font-mono">
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
