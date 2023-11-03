// use strum_macros::{Display, EnumIter};

use std::rc::Rc;

use gloo_console as console;
use serde::{Deserialize, Serialize};
use yew::{prelude::*, Component, Context};
use yewdux::{prelude::Reducer, store::Store};

use crate::Dispatch;

#[derive(Store, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[store(storage = "local")]
pub struct LoggerState {
    #[serde(skip)]
    pub log: Vec<(String, String)>,
}

impl Default for LoggerState {
    fn default() -> Self {
        Self { log: [].to_vec() }
    }
}

pub enum LoggerMessage {
    Log { level: String, value: String },
}

impl Reducer<LoggerState> for LoggerMessage {
    fn apply(self, mut orig_state: Rc<LoggerState>) -> Rc<LoggerState> {
        let state = Rc::make_mut(&mut orig_state);
        match self {
            LoggerMessage::Log { level, value } => {
                console::log!("TERMINAL:", level.clone(), value.clone());
                state.log.push((level, value));
                true
            }
        };

        orig_state
    }
}

pub struct LoggerView {
    _dispatch: Dispatch<LoggerState>, // Ignore unused warning. Dispatcher needs to be present for state to update
    state: Rc<LoggerState>,
}

pub enum LoggerViewMessage {
    UpdateState(Rc<LoggerState>),
}

impl Component for LoggerView {
    type Message = LoggerViewMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state_callback = ctx.link().callback(LoggerViewMessage::UpdateState);
        let dispatch = Dispatch::<LoggerState>::subscribe(state_callback);
        Self {
            state: dispatch.get(),
            _dispatch: dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoggerViewMessage::UpdateState(state) => {
                self.state = state;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-zinc-800 w-full flex-1 p-4 flex flex-col">
                <div class="mb-4 flex flex-none">
                    <button class="py-2 px-4 border-b-2 border-blue-500 text-white hover:border-blue-700 rounded-l">
                        {"Console"}
                    </button>
                    /* Consider whether errors should have separate tab <button class="py-2 px-4 border-b-2 border-transparent text-gray-500 rounded-r hover:border-blue-700 hover:text-white">
                        {"Errors"}
                    </button> */
                </div>
                    <textarea
                        class="w-full bg-zinc-800 flex-1 text-white outline-0 text-left font-mono focus:none"
                        value={self.log_textarea_content()}
                        readonly=true
                    />
            </div>
        }
    }
}

impl LoggerView {
    fn log_textarea_content(&self) -> String {
        self.state
            .log
            .iter()
            .map(|(_level, message)| message.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
