// use strum_macros::{Display, EnumIter};

use gloo_console as console;

use serde::{Deserialize, Serialize};

use std::rc::Rc;

use yewdux::prelude::Reducer;
use yewdux::store::Store;

#[derive(Store, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[store(storage = "local")]
pub struct LoggerState {
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
                console::log!("LLOGG X: {} - {}", level.clone(), value.clone());
                state.log.push((level, value));
                true
            }
        };

        orig_state
    }
}
