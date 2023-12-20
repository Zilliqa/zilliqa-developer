// ! taken from ZQ2

use serde::{Deserialize, Serialize};

/// A message intended to be sent over the network as part of p2p communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalMessage {
    RequestResponse,
}

impl ExternalMessage {
    pub fn name(&self) -> &'static str {
        match self {
            ExternalMessage::RequestResponse => "RequestResponse",
        }
    }
}
