use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::{State, StateMessage};

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
                <div class="grid grid-rows-2 lg:grid-rows-1 lg:grid-cols-2 gap-4">
                    <div class="p-4">
                        <h3 class="text-lg font-semibold mb-2">{"Stack"}</h3>
                        <ul class="list-decimal pl-5">
                            { for machine.stack().data().iter().map(|item| {
                                html! {
                                    <li class="mb-2">
                                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-1">
                                            { for item.0.chunks(4).map(|chunk| {
                                                let segment: String = chunk.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
                                                html! {
                                                    <div class="border rounded p-1 text-sm border-gray-800">
                                                        <p class="break-all">{segment}</p>
                                                    </div>
                                                }
                                            }) }
                                        </div>
                                    </li>
                                }
                            }) }
                        </ul>
                    </div>

                    <div class="p-4">
                        <h3 class="text-lg font-semibold mb-2">{"Memory"}</h3>
                        <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-4 xl:grid-cols-4 gap-2">
                            { for machine.memory().data().chunks(4).enumerate().map(|(idx, chunk)| {
                                let address = idx * 4;
                                let segment: String = chunk.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
                                let is_start_of_32_byte_boundary = address % 32 == 0;

                                html! {
                                    <div class={if is_start_of_32_byte_boundary { "border-l-4 border-l-green-700 border rounded p-2 border-gray-800" } else { "border rounded p-2 border-gray-800" }}>
                                        <p class={ "text-sm font-bold" }>{format!("0x{:04x}", address)}</p>
                                        <p class="break-all">{segment}</p>
                                    </div>
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