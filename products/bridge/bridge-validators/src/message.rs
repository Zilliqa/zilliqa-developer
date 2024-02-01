// ! taken from ZQ2

use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use ethers::types::{Address, Signature, U256};
use serde::{Deserialize, Serialize};

use crate::event::RelayEvent;

/// A message intended to be sent over the network as part of p2p communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalMessage {
    BridgeEcho(Relay),
}

impl ExternalMessage {
    pub fn name(&self) -> &'static str {
        match self {
            ExternalMessage::BridgeEcho(_) => "BridgeEcho",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Relay {
    pub event: RelayEvent,
    pub signature: Signature,
}

impl Debug for Relay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Relay event: [source_chain: {}, target_chain: {}, nonce: {}]",
            self.event.source_chain_id, self.event.target_chain_id, self.event.nonce
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatched {
    pub chain_id: U256,
    pub nonce: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatch {
    pub event: RelayEvent,
    pub signatures: HashMap<Address, Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InboundBridgeMessage {
    Dispatched(Dispatched),
    Relay(Relay),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutboundBridgeMessage {
    Dispatch(Dispatch),
    Dispatched(Dispatched),
    Relay(Relay),
}
