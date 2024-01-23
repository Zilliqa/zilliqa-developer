use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::Result;
use ethers::{
    contract::{EthEvent, Event},
    providers::{Middleware, StreamExt},
    types::{Address, Block, BlockNumber, Signature, U256},
};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{info, warn};

use crate::{
    client::{ChainClient, Client, ContractInitializer},
    event::{RelayEvent, RelayEventSignatures},
    message::{Dispatch, Dispatched, InboundBridgeMessage, OutboundBridgeMessage, Relay},
    signature::SignatureTracker,
    ChainGateway, DispatchedFilter, RelayedFilter, ValidatorManager,
};

#[derive(Debug)]
pub struct BridgeNode {
    event_signatures: HashMap<U256, RelayEventSignatures>,
    outbound_message_sender: UnboundedSender<OutboundBridgeMessage>,
    inbound_message_receiver: UnboundedReceiverStream<InboundBridgeMessage>,
    inbound_message_sender: UnboundedSender<InboundBridgeMessage>,
    pub chain_client: ChainClient,
    validators: HashSet<Address>,
    is_leader: bool,
}

impl BridgeNode {
    pub async fn new(
        chain_client: ChainClient,
        outbound_message_sender: UnboundedSender<OutboundBridgeMessage>,
        is_leader: bool,
    ) -> Result<Self> {
        let (inbound_message_sender, inbound_message_receiver) = mpsc::unbounded_channel();
        let inbound_message_receiver = UnboundedReceiverStream::new(inbound_message_receiver);

        let mut bridge_node = BridgeNode {
            event_signatures: HashMap::new(),
            chain_client,
            validators: HashSet::new(),
            outbound_message_sender,
            inbound_message_receiver,
            inbound_message_sender,
            is_leader,
        };

        bridge_node.update_validators().await?;

        Ok(bridge_node)
    }

    pub fn get_inbound_message_sender(&self) -> UnboundedSender<InboundBridgeMessage> {
        self.inbound_message_sender.clone()
    }

    async fn query_recent_finalised_events<D>(
        &self,
        event: Event<Arc<Client>, Client, D>,
    ) -> Result<(Vec<D>, u64)>
    where
        D: EthEvent,
    {
        let latest_finalised_block = self
            .chain_client
            .client
            .get_block(BlockNumber::Finalized)
            .await?;

        if let Some(Block {
            number: Some(block_number),
            ..
        }) = latest_finalised_block
        {
            println!("Latest Finalised Block {}", block_number);

            let events = event
                .address(self.chain_client.chain_gateway_address.into())
                .from_block(self.chain_client.chain_gateway_block_deployed)
                .to_block(BlockNumber::Finalized)
                .query()
                .await?;

            return Ok((events, block_number.as_u64()));
        }

        Ok((vec![], 0))
    }

    pub async fn listen_events(&mut self) -> Result<()> {
        println!("Start Listening: {:?}", self.chain_client.chain_id);

        let chain_gateway: ChainGateway<Client> = self.chain_client.get_contract();

        let x = self
            .query_recent_finalised_events(chain_gateway.event::<RelayedFilter>())
            .await?;
        dbg!(x);

        // TODO: test if polling finalised block events work
        let relayed_events = chain_gateway.event::<RelayedFilter>();

        let dispatched_events = chain_gateway.event::<DispatchedFilter>();

        let mut relayed_stream = relayed_events.stream().await?;
        let mut dispatched_stream = dispatched_events.stream().await?;

        loop {
            select! {
                Some(Ok(event)) = relayed_stream.next() => {
                    self.handle_relay_event(event)?;
                },
                Some(Ok(event)) = dispatched_stream.next() => {
                    self.handle_dispatch_event(event)?;
                }
                Some(message) = self.inbound_message_receiver.next() => {
                    self.handle_bridge_message(message).await?;
                }
            }
        }
    }

    /// Handles incoming bridge related messages, either Relay from other validators or Dispatch from another chain
    /// running on a separate thread locally
    async fn handle_bridge_message(&mut self, message: InboundBridgeMessage) -> Result<()> {
        match message {
            InboundBridgeMessage::Dispatched(dispatch) => {
                info!(
                    "Register event as dispatched Chain {}, Nonce: {}",
                    dispatch.chain_id, dispatch.nonce
                );
                match self.event_signatures.get_mut(&dispatch.nonce) {
                    Some(event_signature) => {
                        event_signature.dispatched = true;
                    }
                    None => {
                        // Create new one instance if does not yet exist
                        self.event_signatures.insert(
                            dispatch.nonce,
                            RelayEventSignatures {
                                dispatched: true,
                                ..RelayEventSignatures::default()
                            },
                        );
                    }
                }
            }
            InboundBridgeMessage::Relay(relay) => {
                self.handle_relay(&relay).await?;
            }
        }

        Ok(())
    }

    fn handle_relay_event(&self, event: RelayedFilter) -> Result<()> {
        info!(
            "Chain: {} event found to be broadcasted: {}",
            self.chain_client.chain_id, event
        );

        if let Some(RelayEventSignatures {
            dispatched: true, ..
        }) = self.event_signatures.get(&event.nonce)
        {
            info!("Already dispatched, no need to broadcast");
            return Ok(());
        }

        let relay_event = RelayEvent::from(event, self.chain_client.chain_id);

        self.broadcast_message(Relay {
            signature: relay_event.sign(&self.chain_client.wallet)?,
            event: relay_event,
        })?;

        Ok(())
    }

    fn handle_dispatch_event(&mut self, event: DispatchedFilter) -> Result<()> {
        info!(
            "Found dispatched event chain: {}, nonce: {}",
            event.source_chain_id, event.nonce
        );
        self.outbound_message_sender
            .send(OutboundBridgeMessage::Dispatched(Dispatched {
                chain_id: event.source_chain_id,
                nonce: event.nonce,
            }))?;

        Ok(())
    }

    fn broadcast_message(&self, relay: Relay) -> Result<()> {
        info!("Broadcasting: {:?}", relay);
        // Send out echo message
        self.outbound_message_sender
            .send(OutboundBridgeMessage::Relay(relay))?;

        Ok(())
    }

    async fn update_validators(&mut self) -> Result<()> {
        let validator_manager: ValidatorManager<Client> = self.chain_client.get_contract();

        let validators: Vec<Address> = validator_manager.get_validators().call().await?;

        self.validators = validators.into_iter().collect();

        Ok(())
    }

    fn has_supermajority(&self, signature_count: usize) -> bool {
        signature_count * 3 > self.validators.len() * 2
    }

    /// Handle message, verify and add to storage.
    /// If has supermajority then submit the transaction.
    /// TODO: Also check if it is current leader to dispatch
    async fn handle_relay(&mut self, echo: &Relay) -> Result<()> {
        let Relay { signature, event } = echo;
        let nonce = event.nonce;
        let event_hash = event.hash();

        let signature = Signature::try_from(signature.to_vec().as_slice())?;

        // update validator set in case it has changed
        self.update_validators().await?;

        let address = match signature.recover(event_hash) {
            Ok(addr) => addr,
            Err(err) => {
                info!("Address not part of the validator set: {:?}", err);
                return Ok(());
            }
        };

        if !self.validators.contains(&address) {
            info!("Address not part of the validator set");
            return Ok(());
        }

        // TODO: handle case where validators sign different data to the same event
        let event_signatures = match self.event_signatures.get_mut(&nonce) {
            None => {
                let event_signatures = RelayEventSignatures::new(event.clone(), address, signature);
                self.event_signatures
                    .insert(nonce, event_signatures.clone());

                event_signatures
            }
            Some(event_signatures) => {
                // Only insert if it is the same event as the one stored
                if let Some(relay_event) = &mut event_signatures.event {
                    if relay_event.hash() == event_hash {
                        event_signatures
                            .signatures
                            .add_signature(address, signature);
                    } else {
                        warn!("Message bodies don't match, so reject {:?}", relay_event);
                        return Ok(());
                    }
                } else {
                    warn!("Found event_signature without event {:?}", event_signatures);
                    return Ok(());
                }

                event_signatures.clone()
            }
        };

        info!(
            "Handling received: {:?}, collected: {:?}",
            &echo,
            event_signatures.signatures.len()
        );

        // if leader and majority, create request to dispatch
        if self.is_leader && self.has_supermajority(event_signatures.signatures.len()) {
            // TODO: Verify if any signatures became invalid due to validator changes
            info!("Sending out dispatch request for {:?}", &echo);

            self.outbound_message_sender
                .send(OutboundBridgeMessage::Dispatch(Dispatch {
                    event: event.clone(),
                    signatures: event_signatures.signatures,
                }))?;
        }

        Ok(())
    }
}
