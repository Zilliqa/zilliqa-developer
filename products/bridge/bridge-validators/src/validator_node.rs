use std::collections::HashMap;

use anyhow::Result;
use ethers::{providers::StreamExt, types::U256};
use libp2p::{Multiaddr, PeerId};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{info, warn};

use crate::{
    bridge_node::BridgeNode,
    client::{ChainClient, Client, ContractInitializer},
    crypto::SecretKey,
    message::{Dispatch, ExternalMessage, InboundBridgeMessage, OutboundBridgeMessage},
    signature::SignatureTracker,
    ChainConfig, ChainGateway, ChainGatewayErrors,
};

type ChainID = U256;

#[derive(Debug, Clone)]
pub struct ValidatorNodeConfig {
    pub chain_configs: Vec<ChainConfig>,
    pub private_key: SecretKey,
    pub is_leader: bool,
    pub bootstrap_address: Option<(PeerId, Multiaddr)>,
}

#[derive(Debug)]
pub struct ValidatorNode {
    /// The following two message streams are used for networked messages.
    /// The sender is provided to the p2p coordinator, to forward messages to the node.
    bridge_outbound_message_sender: UnboundedSender<ExternalMessage>,
    bridge_inbound_message_receiver: UnboundedReceiverStream<ExternalMessage>,
    bridge_inbound_message_sender: UnboundedSender<ExternalMessage>,
    bridge_message_receiver: UnboundedReceiverStream<OutboundBridgeMessage>,
    chain_node_senders: HashMap<ChainID, UnboundedSender<InboundBridgeMessage>>,
    chain_clients: HashMap<ChainID, ChainClient>,
}

impl ValidatorNode {
    pub async fn new(
        config: ValidatorNodeConfig,
        bridge_outbound_message_sender: UnboundedSender<ExternalMessage>,
    ) -> Result<Self> {
        let mut chain_node_senders = HashMap::new();
        let mut chain_clients = HashMap::new();
        let wallet = config.private_key.as_wallet()?;

        let (bridge_message_sender, bridge_message_receiver) = mpsc::unbounded_channel();
        let bridge_message_receiver = UnboundedReceiverStream::new(bridge_message_receiver);

        for chain_config in config.chain_configs {
            let chain_client = ChainClient::new(&chain_config, wallet.clone()).await?;

            let mut validator_chain_node = BridgeNode::new(
                chain_client.clone(),
                bridge_message_sender.clone(),
                config.is_leader,
            )
            .await?;
            chain_node_senders.insert(
                validator_chain_node.chain_client.chain_id,
                validator_chain_node.get_inbound_message_sender(),
            );

            chain_clients.insert(validator_chain_node.chain_client.chain_id, chain_client);

            tokio::spawn(async move {
                // Fill all historic events first
                // validator_chain_node.sync_historic_events().await.unwrap();
                // Then start listening to new ones
                validator_chain_node.listen_events().await.unwrap();
            });
        }

        let (bridge_inbound_message_sender, bridge_inbound_message_receiver) =
            mpsc::unbounded_channel();
        let bridge_inbound_message_receiver =
            UnboundedReceiverStream::new(bridge_inbound_message_receiver);

        Ok(ValidatorNode {
            bridge_outbound_message_sender,
            bridge_inbound_message_receiver,
            bridge_inbound_message_sender,
            bridge_message_receiver,
            chain_node_senders,
            chain_clients,
        })
    }

    pub fn get_bridge_inbound_message_sender(&self) -> UnboundedSender<ExternalMessage> {
        self.bridge_inbound_message_sender.clone()
    }

    pub async fn listen_p2p(&mut self) -> Result<()> {
        loop {
            select! {
               Some(message) = self.bridge_inbound_message_receiver.next() => {
                    // forward messages to bridge_chain_node
                    match message {
                        ExternalMessage::BridgeEcho(echo) => {
                            // Send echo to respective source_chain_id to be verified, only if chain is supported
                            if let Some(sender) = self.chain_node_senders.get(&echo.event.source_chain_id) {
                                sender.send(InboundBridgeMessage::Relay(echo))?;
                            }
                        }
                    }
                }
                Some(message) = self.bridge_message_receiver.next() => {
                    match message {
                        OutboundBridgeMessage::Dispatch(dispatch) => {
                            // Send relay event to target chain
                            self.dispatch_message(dispatch).await?;
                        },
                        OutboundBridgeMessage::Dispatched(dispatched) => {
                            // Forward message to another chain_node
                            if let Some(sender) = self.chain_node_senders.get(&dispatched.chain_id) {
                                sender.send(InboundBridgeMessage::Dispatched(dispatched))?;
                            }
                        },
                        OutboundBridgeMessage::Relay(relay) => {
                            // Forward message to broadcast
                            self.bridge_outbound_message_sender.send(ExternalMessage::BridgeEcho(relay))?;
                        },
                    }
                }
            }
        }
    }

    async fn dispatch_message(&self, dispatch: Dispatch) -> Result<()> {
        let Dispatch {
            event, signatures, ..
        } = dispatch;

        let client = match self.chain_clients.get(&event.target_chain_id) {
            Some(client) => client,
            None => {
                warn!("Unsupported Chain ID");
                return Ok(());
            }
        };

        let chain_gateway: ChainGateway<Client> = client.get_contract();

        let function_call = chain_gateway.dispatch(
            event.source_chain_id,
            event.target,
            event.call,
            event.gas_limit,
            event.nonce,
            signatures.into_ordered_signatures(),
        );
        info!(
            "Preparing to send dispatch {}.{}",
            event.target_chain_id, event.nonce
        );

        let function_call = if client.legacy_gas_estimation {
            function_call.legacy()
        } else {
            function_call
        };

        // Simulate call, if fails decode error and exit early
        if let Err(contract_err) = function_call.call().await {
            match contract_err.decode_contract_revert::<ChainGatewayErrors>() {
                Some(ChainGatewayErrors::AlreadyDispatched(_)) => {
                    info!(
                        "Already Dispatched {}.{}",
                        event.target_chain_id, event.nonce
                    );
                }
                Some(err) => {
                    warn!("ChainGatewayError: {:?}", err);
                }
                None => {
                    warn!("Some unknown error, {:?}", contract_err);
                }
            }
            return Ok(());
        }

        // Make the actual call
        let txn = function_call.send().await?.log_msg("Pending txn hash");
        println!("Transaction Sent {}.{}", event.target_chain_id, event.nonce);

        Ok(())
    }
}
