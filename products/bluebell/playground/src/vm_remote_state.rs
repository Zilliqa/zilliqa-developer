use std::rc::Rc;

use evm_assembly::function_signature::EvmFunctionSignature;
use serde::{Deserialize, Serialize};
use yewdux::{prelude::Reducer, store::Store};

#[derive(Store, Serialize, Deserialize, Clone)]
#[store(storage = "local")]
pub struct VmRemoteState {
    pub caller: String,

    #[serde(skip)]
    pub function_signature: Option<EvmFunctionSignature>,
    #[serde(skip)]
    pub arguments: Vec<String>,
}

impl Default for VmRemoteState {
    fn default() -> Self {
        Self {
            caller: "".to_string(),
            function_signature: None,
            arguments: [].to_vec(),
        }
    }
}

impl PartialEq for VmRemoteState {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for VmRemoteState {}
impl VmRemoteState {}

pub enum VmRemoteMessage {
    UpdateFunctionSignature(Option<EvmFunctionSignature>),
    SetArgument(usize, String),
    SetCaller(String),
}

impl Reducer<VmRemoteState> for VmRemoteMessage {
    fn apply(self, mut orig_state: Rc<VmRemoteState>) -> Rc<VmRemoteState> {
        let state = Rc::make_mut(&mut orig_state);

        match self {
            VmRemoteMessage::UpdateFunctionSignature(signature) => {
                state.function_signature = signature;
                let n = if let Some(signature) = &state.function_signature {
                    signature.arguments.len()
                } else {
                    0
                };

                state.arguments = vec![String::new(); n];
            }
            VmRemoteMessage::SetArgument(i, v) => {
                state.arguments[i] = v;
            }
            VmRemoteMessage::SetCaller(v) => {
                state.caller = v;
            }
        }

        orig_state
    }
}
