use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::Result;
use ethers::{
    contract::{EthEvent, Event},
    providers::{Middleware, StreamExt},
    types::{Address, BlockNumber, Signature, U256},
};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{info, warn};

use crate::{
    block::{BlockPolling, EventListener},
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

    async fn get_historic_events<D>(
        &self,
        event: Event<Arc<Client>, Client, D>,
        to_block: BlockNumber,
    ) -> Result<Vec<D>>
    where
        D: EthEvent,
    {
        self.chain_client
            .get_events(
                event.filter,
                self.chain_client.chain_gateway_block_deployed.into(),
                to_block,
            )
            .await
    }

    pub async fn sync_historic_events(&mut self) -> Result<()> {
        let to_block = if self.chain_client.block_instant_finality {
            BlockNumber::Latest
        } else {
            BlockNumber::Finalized
        };
        info!("Getting Historic Events {}", to_block);

        let to_block_number = self
            .chain_client
            .client
            .get_block(to_block)
            .await?
            .expect("Latest finalized block should be retrieved")
            .number
            .expect("Number should be here");

        dbg!(to_block_number);

        let chain_gateway: ChainGateway<Client> = self.chain_client.get_contract();

        let dispatch_events = self
            .get_historic_events(
                chain_gateway.event::<DispatchedFilter>(),
                BlockNumber::Number(to_block_number.into()),
            )
            .await?;
        dbg!(dispatch_events.len());

        for dispatch in dispatch_events {
            self.handle_dispatch_event(dispatch)?;
        }

        let relay_events = self
            .get_historic_events(
                chain_gateway.event::<RelayedFilter>(),
                BlockNumber::Number(to_block_number.into()),
            )
            .await?;

        dbg!(relay_events.len());

        for relay in relay_events {
            self.handle_relay_event(relay)?;
        }

        unimplemented!();
    }

    pub async fn listen_events(&mut self) -> Result<()> {
        println!("Start Listening: {:?}", self.chain_client.chain_id);

        let chain_gateway: ChainGateway<Client> = self.chain_client.get_contract();

        // TODO: polling finalized events
        let relayed_filter = chain_gateway.event::<RelayedFilter>().filter;
        let dispatched_filter = chain_gateway.event::<DispatchedFilter>().filter;

        let relayed_listener: EventListener<RelayedFilter> =
            EventListener::new(self.chain_client.clone(), relayed_filter);
        let dispatched_listener: EventListener<DispatchedFilter> =
            EventListener::new(self.chain_client.clone(), dispatched_filter);

        let mut relayed_stream = relayed_listener.listen();
        let mut dispatched_stream = dispatched_listener.listen();

        loop {
            select! {
                Some(Ok(events)) = relayed_stream.next() => {
                    for event in events {
                        self.handle_relay_event(event)?;
                    }
                },
                Some(Ok(events)) = dispatched_stream.next() => {
                    for event in events {
                        self.handle_dispatch_event(event)?;
                    }
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
            info!("Address not part of the validator set, {}", address);
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
                let relay_event = if let Some(event) = &event_signatures.event {
                    event
                } else {
                    warn!("Found event_signature without event {:?}", event_signatures);
                    return Ok(());
                };

                if relay_event.hash() != event_hash {
                    warn!("Message bodies don't match, so reject {:?}", relay_event);
                    return Ok(());
                }

                event_signatures
                    .signatures
                    .add_signature(address, signature);

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
