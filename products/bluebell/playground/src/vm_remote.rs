use std::rc::Rc;

use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    state::{State, StateMessage},
    vm_remote_layout::VmRemoteControlLayout,
    vm_remote_state::{VmRemoteMessage, VmRemoteState},
};

#[derive(Properties, Clone, PartialEq)]
pub struct VmRemoteProps {}

pub struct VmRemote {
    props: VmRemoteProps,
    vm_dispatch: Dispatch<State>,
    vm_state: Rc<State>,

    dispatch: Dispatch<VmRemoteState>,
    state: Rc<VmRemoteState>,
}

pub enum VmComponentMessage {
    UpdateVmState(Rc<State>),
    UpdateVmRemoteState(Rc<VmRemoteState>),
}

impl Component for VmRemote {
    type Message = VmComponentMessage;
    type Properties = VmRemoteProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        let vm_state_callback = ctx.link().callback(VmComponentMessage::UpdateVmState);
        let vm_dispatch = Dispatch::<State>::subscribe(vm_state_callback);

        let state_callback = ctx.link().callback(VmComponentMessage::UpdateVmRemoteState);
        let dispatch = Dispatch::<VmRemoteState>::subscribe(state_callback);

        Self {
            props: props.clone(),
            vm_state: vm_dispatch.get(),
            vm_dispatch,
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
            VmComponentMessage::UpdateVmState(state) => {
                self.vm_state = state;
            }

            VmComponentMessage::UpdateVmRemoteState(state) => {
                self.state = state;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let functions = self.vm_state.functions.clone();
        let functions_clone = self.vm_state.functions.clone();
        let signature = self.state.function_signature.clone();
        let arguments = self.state.arguments.clone();

        let step_button_click = self.vm_dispatch.apply_callback(|_| StateMessage::RunStep);
        let eject_button_click = self.vm_dispatch.apply_callback(|_| StateMessage::Reset);
        // let stop_button_click = self.vm_dispatch.apply_callback(|_| StateMessage::Reset);

        let set_function_signature = self
            .dispatch
            .apply_callback(|value| VmRemoteMessage::UpdateFunctionSignature(value));
        let set_argument = self
            .dispatch
            .apply_callback(|(i, value)| VmRemoteMessage::SetArgument(i, value));

        let set_caller = self
            .dispatch
            .apply_callback(|value| VmRemoteMessage::SetCaller(value));

        let load_function = !self.vm_state.function_loaded;

        let run_button_click = {
            let load_function = !self.vm_state.function_loaded;
            let maybe_function_signature = signature.clone();
            let argument_list = arguments.clone();
            Callback::from(move |_| {
                if load_function {
                    let maybe_function_signature = maybe_function_signature.clone();
                    if let Some(function_signature) = maybe_function_signature {
                        let function_name = function_signature.name.clone();
                        let dispatch = Dispatch::<State>::new();
                        let mut arguments = "".to_string();
                        for (i, arg) in argument_list.iter().enumerate() {
                            if i > 0 {
                                arguments.push_str(",");
                            }
                            arguments.push_str(&format!("{}", arg).to_string());
                        }
                        dispatch.apply(StateMessage::PrepareFunctionCall {
                            function_name,
                            arguments,
                        });
                    }
                } else {
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
                    })
                }
            })
        };

        let (function_name, no_function_selected) = if let Some(signature) = &signature {
            (signature.name.clone(), false)
        } else {
            ("$__none__$".to_string(), true)
        };

        html! {
            <VmRemoteControlLayout>
                <div class="flex flex-col items-center space-y-4">
                    <div
                        class="p-2 bg-zinc-900 w-full text-gray-100 rounded-md space-y-2 flex flex-col"
                    >
                        { if load_function {
                            html! {
                                <>
                                    <div class="w-full flex flex-col space-y-2" onmouseover={move |e: MouseEvent| e.stop_propagation()}
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
                                            <option value="none" selected={no_function_selected}>{"(Select function)"}</option>

                                            {functions.iter().enumerate().map(|(i, v)| {
                                                if v.name == function_name {
                                                    html! {
                                                        <option value={i.to_string()} selected={true}>{v.name.clone()}</option>
                                                    }
                                                } else {
                                                    html! {
                                                        <option value={i.to_string()}>{v.name.clone()}</option>
                                                    }
                                                }
                                            }).collect::<Vec<_>>()}
                                        </select>
                                        <input type="text" placeholder="Caller" class="p-1 bg-zinc-700 text-white rounded" oninput={move |e:InputEvent| {
                                            let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                            set_caller.emit(value);
                                        }}/>
                                    </div>
                                    { if let Some(ref signature) = signature {
                                            html! {
                                                <div class="w-full flex flex-col space-y-2"  onmouseover={move |e: MouseEvent| e.stop_propagation()}
                                             onmousedown={move |e: MouseEvent| e.stop_propagation()}
                                             onmouseup={move |e: MouseEvent| e.stop_propagation()}>
                                                    {signature.arguments.iter().zip(arguments.iter()).enumerate().map(|(i, (t, v))| {
                                                        let set_argument = set_argument.clone();
                                                        html! {
                                                            <input class="p-1 bg-zinc-700 text-white rounded" placeholder={format!("{:?}", t)} key={format!("input{}",i)} value={format!("{}", v)} oninput={move |e:InputEvent| {
                                                                    let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                                                    set_argument.emit((i, value));
                                                                }} />
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }  else {
                                            html! {
                                            }
                                        }

                                    }
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <div class="w-full">
                                            {format!("{}({})", function_name, arguments.join(", "))}
                                        </div>
                                        <div class="flex items-center space-x-1">
                                            <span class="font-bold">{"PC:"}</span>
                                            <span>{format!("0x{:02x}", self.vm_state.program_counter)}</span>
                                            <span>{format!("({})", self.vm_state.program_counter)}</span>
                                        </div>
                                        <div class="flex items-center space-x-1">
                                            <span class="font-bold">{"Caller:"}</span>
                                            <span>{format!("({})", self.state.caller)}</span>
                                        </div>

                                    </>
                                }
                            }
                        }

                    </div>
                    { if let Some(_) = signature {
                        html! {
                            <>
                            <div class="flex items-center space-x-4">

                                {if load_function {
                                    html! {
                                        <>
                                            <span class="w-12 h-12"></span>
                                            <button
                                                class="flex items-center justify-center bg-blue-600 text-white w-12 h-12 rounded-full hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 text-center"
                                                onclick={run_button_click.clone()}
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                  <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3" />
                                                </svg>
                                            </button>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <>
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
                                                    if self.vm_state.playing {
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
                                        </>
                                    }
                              }
                            }
                                <button
                                    class="flex items-center justify-center bg-red-600 text-white w-8 h-8 rounded-full hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 text-center"
                                    onclick={eject_button_click.clone()}
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
            </VmRemoteControlLayout>
        }
    }
}
