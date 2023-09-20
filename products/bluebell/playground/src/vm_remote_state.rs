use evm_assembly::function_signature::EvmFunctionSignature;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yewdux::prelude::Reducer;
use yewdux::store::Store;

#[derive(Store, Serialize, Deserialize, Clone)]
#[store(storage = "local")]
pub struct VmRemoteState {
    #[serde(skip)]
    pub function_signature: Option<EvmFunctionSignature>,
    #[serde(skip)]
    pub arguments: Vec<String>,
}

impl Default for VmRemoteState {
    fn default() -> Self {
        Self {
            function_signature: None,
            arguments: [].to_vec(),
        }
    }
}

impl PartialEq for VmRemoteState {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Eq for VmRemoteState {}
impl VmRemoteState {}

pub enum VmRemoteMessage {
    UpdateFunctionSignature(Option<EvmFunctionSignature>),
    SetArgument(usize, String),
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
        }

        orig_state
    }
}
