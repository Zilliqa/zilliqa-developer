use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::Result;
use ethers::{
    abi::{self, Token},
    middleware::{MiddlewareBuilder, SignerMiddleware},
    providers::{Http, Middleware, Provider, StreamExt},
    signers::{LocalWallet, Signer},
    types::{Address, Block, BlockNumber, Bytes, Signature, H256, U256},
    utils::hash_message,
};
use ethers_contract::{EthEvent, Event};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{info, warn};

use crate::{
    crypto::SecretKey,
    message::{
        Dispatch, Dispatched, ExternalMessage, InboundBridgeMessage, OutboundBridgeMessage, Relay,
    },
    ChainConfig, ChainGateway, ChainGatewayErrors, DispatchedFilter, RelayedFilter,
    ValidatorManager,
};

pub type Client = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayEvent {
    pub source_chain_id: U256,
    pub target_chain_id: U256,
    target: Address,
    call: Bytes,
    gas_limit: U256,
    pub nonce: U256,
}

impl RelayEvent {
    fn hash(&self) -> H256 {
        hash_message(abi::encode(&[
            Token::Uint(self.source_chain_id),
            Token::Uint(self.target_chain_id),
            Token::Address(self.target),
            Token::Bytes(self.call.to_vec()),
            Token::Uint(self.gas_limit),
            Token::Uint(self.nonce),
        ]))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventSignatures {
    event: Option<RelayEvent>,
    dispatched: bool,
    signatures: SignedSignatures,
}

impl EventSignatures {
    fn new(event: RelayEvent, address: Address, signature: Signature) -> Self {
        EventSignatures {
            event: Some(event),
            dispatched: false,
            signatures: SignedSignatures(HashMap::from([(address, signature)])),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignedSignatures(HashMap<Address, Signature>);

impl SignedSignatures {
    fn add_signature(&mut self, address: Address, signature: Signature) -> Option<Signature> {
        self.0.insert(address, signature)
    }

    fn into_ordered_signatures(self) -> Vec<Bytes> {
        let mut list = self.0.into_iter().collect::<Vec<(Address, Signature)>>();
        list.sort_by_cached_key(|(address, _)| address.clone());
        list.into_iter()
            .map(|(_, signature)| Bytes::from(signature.to_vec()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ChainClient {
    client: Arc<Client>,
    validator_manager_address: Address,
    chain_gateway_address: Address,
    chain_id: U256,
    wallet: LocalWallet,
    block_query_limit: Option<u64>,
}

impl ChainClient {
    async fn new(config: &ChainConfig, wallet: LocalWallet) -> Result<Self> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())?;
        let chain_id = provider.get_chainid().await?;

        let client: Arc<Client> =
            Arc::new(provider.with_signer(wallet.clone().with_chain_id(chain_id.as_u64())));

        Ok(ChainClient {
            client,
            validator_manager_address: config.validator_manager_address.parse()?,
            chain_gateway_address: config.chain_gateway_address.parse()?,
            chain_id,
            wallet,
            block_query_limit: config.block_query_limit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BridgeNodeConfig {
    pub chain_configs: Vec<ChainConfig>,
    pub private_key: SecretKey,
    pub is_leader: bool,
}

#[derive(Debug)]
pub struct BridgeChainNode {
    event_signatures: HashMap<U256, EventSignatures>,
    outbound_message_sender: UnboundedSender<OutboundBridgeMessage>,
    inbound_message_receiver: UnboundedReceiverStream<InboundBridgeMessage>,
    inbound_message_sender: UnboundedSender<InboundBridgeMessage>,
    chain_client: ChainClient,
    validators: HashSet<Address>,
    is_leader: bool,
}

impl BridgeChainNode {
    async fn new(
        chain_client: ChainClient,
        outbound_message_sender: UnboundedSender<OutboundBridgeMessage>,
        is_leader: bool,
    ) -> Result<Self> {
        let (inbound_message_sender, inbound_message_receiver) = mpsc::unbounded_channel();
        let inbound_message_receiver = UnboundedReceiverStream::new(inbound_message_receiver);

        let mut bridge_node = BridgeChainNode {
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

    // WIP
    fn get_inbound_message_sender(&self) -> UnboundedSender<InboundBridgeMessage> {
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
            let from = if let Some(block_query_limit) = self.chain_client.block_query_limit {
                dbg!(block_query_limit);
                BlockNumber::Number(block_number - block_query_limit)
            } else {
                BlockNumber::Earliest
            };

            dbg!(from);
            dbg!(block_number);

            let res = event
                .from_block(from)
                .to_block(block_number)
                .query()
                .await?;
            return Ok((res, block_number.as_u64()));
        }

        return Ok((vec![], 0));
    }

    async fn listen_events(&mut self) -> Result<()> {
        println!("Start Listening: {:?}", self.chain_client.chain_id);

        let chain_gateway: ChainGateway<Client> = self.chain_client.get_contract();

        // TODO: test if polling finalised block events work
        let relayed_events = chain_gateway
            .event::<RelayedFilter>()
            .to_block(BlockNumber::Finalized);

        let dispatched_events = chain_gateway
            .event::<DispatchedFilter>()
            .to_block(BlockNumber::Finalized);

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
                if let Some(event_signature) = self.event_signatures.get_mut(&dispatch.nonce) {
                    event_signature.dispatched = true;
                } else {
                    // Create new one instance if does not yet exist
                    self.event_signatures.insert(
                        dispatch.nonce,
                        EventSignatures {
                            dispatched: true,
                            ..EventSignatures::default()
                        },
                    );
                };
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

        if let Some(EventSignatures {
            dispatched: true, ..
        }) = self.event_signatures.get(&event.nonce)
        {
            info!("Already dispatched, no need to broadcast");
            return Ok(());
        }

        let relay_event = RelayEvent {
            source_chain_id: self.chain_client.chain_id,
            target_chain_id: event.target_chain_id,
            target: event.target,
            call: event.call,
            gas_limit: event.gas_limit,
            nonce: event.nonce,
        };

        self.broadcast_message(Relay {
            signature: self.chain_client.wallet.sign_event(&relay_event)?,
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

        if let Ok(address) = signature.recover(event_hash) {
            if self.validators.contains(&address) {
                // TODO: handle case where validators sign different data to the same event
                let event_signatures = if let Some(event_signatures) =
                    self.event_signatures.get_mut(&nonce)
                {
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
                } else {
                    let event_signatures = EventSignatures::new(event.clone(), address, signature);
                    self.event_signatures
                        .insert(nonce, event_signatures.clone());

                    event_signatures
                };
                info!(
                    "Handling received: {:?}, collected: {:?}",
                    &echo,
                    event_signatures.signatures.0.len()
                );

                if self.is_leader && self.has_supermajority(event_signatures.signatures.0.len()) {
                    // Send message to BridgeNode to be handled and sent out
                    // TODO: Verify if any signatures became invalid due to validator changes
                    info!("Sending out dispatch request for {:?}", &echo);
                    self.outbound_message_sender
                        .send(OutboundBridgeMessage::Dispatch(Dispatch {
                            event: event.clone(),
                            signatures: event_signatures.signatures,
                        }))?;
                }
            } else {
                info!("Address not part of the validator set");
                return Ok(());
            }
        } else {
            info!("Invalid message signature");
            return Ok(());
        }

        Ok(())
    }
}

type ChainID = U256;

#[derive(Debug)]
pub struct BridgeNode {
    /// The following two message streams are used for networked messages.
    /// The sender is provided to the p2p coordinator, to forward messages to the node.
    bridge_outbound_message_sender: UnboundedSender<ExternalMessage>,
    bridge_inbound_message_receiver: UnboundedReceiverStream<ExternalMessage>,
    bridge_inbound_message_sender: UnboundedSender<ExternalMessage>,
    bridge_message_receiver: UnboundedReceiverStream<OutboundBridgeMessage>,
    chain_node_senders: HashMap<ChainID, UnboundedSender<InboundBridgeMessage>>,
    chain_clients: HashMap<ChainID, ChainClient>,
}

impl BridgeNode {
    pub async fn new(
        config: BridgeNodeConfig,
        bridge_outbound_message_sender: UnboundedSender<ExternalMessage>,
    ) -> Result<Self> {
        let mut chain_node_senders = HashMap::new();
        let mut chain_clients = HashMap::new();
        let wallet = config.private_key.as_wallet()?;

        let (bridge_message_sender, bridge_message_receiver) = mpsc::unbounded_channel();
        let bridge_message_receiver = UnboundedReceiverStream::new(bridge_message_receiver);

        for chain_config in config.chain_configs {
            let chain_client = ChainClient::new(&chain_config, wallet.clone()).await?;

            let mut validator_chain_node = BridgeChainNode::new(
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
                validator_chain_node.listen_events().await.unwrap();
            });
        }

        let (bridge_inbound_message_sender, bridge_inbound_message_receiver) =
            mpsc::unbounded_channel();
        let bridge_inbound_message_receiver =
            UnboundedReceiverStream::new(bridge_inbound_message_receiver);

        Ok(BridgeNode {
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

    // TODO: solidify into all events together
    pub async fn listen_events(&mut self) -> Result<()> {
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

        if let Some(client) = self.chain_clients.get(&event.target_chain_id) {
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

            match function_call.call().await {
                Ok(_) => {
                    function_call.send().await?.await?;
                    println!("Transaction Sent {}.{}", event.target_chain_id, event.nonce);
                }
                Err(contract_err) => {
                    match contract_err.decode_contract_revert::<ChainGatewayErrors>() {
                        Some(ChainGatewayErrors::AlreadyDispatched(_)) => {
                            println!(
                                "Already Dispatched {}.{}",
                                event.target_chain_id, event.nonce
                            );
                        }
                        Some(x) => {
                            println!("ChainGatewayError: {:?}", x);
                        }
                        None => {
                            dbg!("Some unknown error", contract_err);
                        }
                    }
                }
            };
        } else {
            warn!("Invalid chain id")
        }

        Ok(())
    }
}

pub trait EventSigner {
    fn sign_event(&self, event: &RelayEvent) -> Result<Signature>;
}

impl EventSigner for LocalWallet {
    fn sign_event(&self, event: &RelayEvent) -> Result<Signature> {
        let data = event.hash();
        let signature = self.sign_hash(data)?;

        Ok(signature)
    }
}

pub trait ContractInitializer<T> {
    fn get_contract(&self) -> T;
}

impl ContractInitializer<ValidatorManager<Client>> for ChainClient {
    fn get_contract(&self) -> ValidatorManager<Client> {
        ValidatorManager::new(self.validator_manager_address, self.client.clone())
    }
}

impl ContractInitializer<ChainGateway<Client>> for ChainClient {
    fn get_contract(&self) -> ChainGateway<Client> {
        ChainGateway::new(self.chain_gateway_address, self.client.clone())
    }
}
