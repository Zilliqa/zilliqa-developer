// ! taken from ZQ2
//! A node in the Zilliqa P2P network. May coordinate multiple shard nodes.

use std::{collections::HashMap, iter};

use anyhow::{anyhow, Result};
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    gossipsub::{self, IdentTopic, MessageAuthenticity, TopicHash},
    identify,
    kad::{self, store::MemoryStore},
    mdns,
    multiaddr::{Multiaddr, Protocol},
    noise,
    swarm::{self, NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
};
use tokio::{
    select,
    signal::{self, unix::SignalKind},
    sync::{mpsc, mpsc::UnboundedSender},
    task::JoinSet,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error, info, warn};

use crate::{
    crypto::SecretKey,
    message::ExternalMessage,
    networking::{request_response, MessageCodec, MessageProtocol, ProtocolSupport},
};

#[derive(NetworkBehaviour)]
struct Behaviour {
    request_response: request_response::Behaviour<MessageCodec>,
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    identify: identify::Behaviour,
    kademlia: kad::Behaviour<MemoryStore>,
}

/// Messages circulating over the p2p network.
/// (destination, shard_id, message)
pub type OutboundMessageTuple = (Option<PeerId>, u64, ExternalMessage);

struct NodeInputChannels {
    external: UnboundedSender<(PeerId, ExternalMessage)>,
}

pub struct P2pNode {
    shard_nodes: HashMap<TopicHash, NodeInputChannels>,
    shard_threads: JoinSet<Result<()>>,
    secret_key: SecretKey,
    peer_id: PeerId,
    swarm: Swarm<Behaviour>,
    /// Shard nodes get a copy of these senders to propagate messages.
    outbound_message_sender: UnboundedSender<OutboundMessageTuple>,
    /// The p2p node keeps a handle to these receivers, to obtain messages from shards and propagate
    /// them as necessary.
    outbound_message_receiver: UnboundedReceiverStream<OutboundMessageTuple>,
}

impl P2pNode {
    pub fn new(secret_key: SecretKey) -> Result<Self> {
        let (outbound_message_sender, outbound_message_receiver) = mpsc::unbounded_channel();
        let outbound_message_receiver = UnboundedReceiverStream::new(outbound_message_receiver);

        let key_pair = secret_key.to_libp2p_keypair();
        let peer_id = PeerId::from(key_pair.public());
        info!(%peer_id);

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&key_pair)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = Behaviour {
            // TODO: Consider replacing with [request_response::json::Behaviour].
            request_response: request_response::Behaviour::with_codec(
                MessageCodec,
                iter::once((MessageProtocol, ProtocolSupport::Full)),
                Default::default(),
            ),
            gossipsub: gossipsub::Behaviour::new(
                MessageAuthenticity::Signed(key_pair.clone()),
                gossipsub::ConfigBuilder::default()
                    .max_transmit_size(524288)
                    .build()
                    .map_err(|e| anyhow!(e))?,
            )
            .map_err(|e| anyhow!(e))?,
            mdns: mdns::Behaviour::new(Default::default(), peer_id)?,
            identify: identify::Behaviour::new(identify::Config::new(
                "/ipfs/id/1.0.0".to_owned(),
                key_pair.public(),
            )),
            kademlia: kad::Behaviour::new(peer_id, MemoryStore::new(peer_id)),
        };

        let swarm = Swarm::new(
            transport,
            behaviour,
            peer_id,
            swarm::Config::with_tokio_executor(),
        );

        Ok(Self {
            shard_nodes: HashMap::new(),
            peer_id,
            secret_key,
            swarm,
            shard_threads: JoinSet::new(),
            outbound_message_sender,
            outbound_message_receiver,
        })
    }

    pub fn shard_id_to_topic(shard_id: u64) -> IdentTopic {
        IdentTopic::new(shard_id.to_string())
    }

    fn forward_external_message_to_node(
        &self,
        topic_hash: &TopicHash,
        source: PeerId,
        message: ExternalMessage,
    ) -> Result<()> {
        match self.shard_nodes.get(topic_hash) {
            Some(inbound_message_sender) => {
                inbound_message_sender.external.send((source, message))?
            }
            None => warn!(
                ?topic_hash,
                ?source,
                ?message,
                "Message received for unknown shard/topic"
            ),
        };
        Ok(())
    }

    pub fn create_validator_node(&mut self) -> Result<()> {
        let topic = "bridge"; // TODO: change to more specific bridge chains

        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let mut addr: Multiaddr = "/ip4/0.0.0.0".parse().unwrap();
        addr.push(Protocol::Tcp(0));

        self.swarm.listen_on(addr)?;

        let mut terminate = signal::unix::signal(SignalKind::terminate())?;

        loop {
            select! {
                event = self.swarm.next() => match event.expect("swarm stream should be infinite") {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!(%address, "started listening");
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, addr) in list {
                            info!(%peer_id, %addr, "discovered peer via mDNS");
                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, addr) in list {
                            self.swarm.behaviour_mut().kademlia.remove_address(&peer_id, &addr);
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received { info: identify::Info { observed_addr, listen_addrs, .. }, peer_id })) => {
                        for addr in listen_addrs {
                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                        }
                        // Mark the address observed for us by the external peer as confirmed.
                        // TODO: We shouldn't trust this, instead we should confirm our own address manually or using
                        // `libp2p-autonat`.
                        self.swarm.add_external_address(observed_addr);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message{
                        message: gossipsub::Message {
                            source,
                            data,
                            topic: topic_hash, ..
                        }, ..
                    })) => {
                        let source = source.expect("message should have a source");
                        let message = serde_json::from_slice::<ExternalMessage>(&data).unwrap();
                        let message_type = message.name();
                        let to = self.peer_id;
                        debug!(%source, %to, message_type, "broadcast received");
                        self.forward_external_message_to_node(&topic_hash, source, message)?;
                    }

                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(request_response::Event::Message { message, peer: source })) => {
                        match message {
                            request_response::Message::Request {request, channel, ..} => {
                                let to = self.peer_id;
                                let (shard_id, external_message) = request;
                                let message_type = external_message.name();
                                debug!(%source, %to, message_type, "message received");
                                let topic = Self::shard_id_to_topic(shard_id);
                                self.forward_external_message_to_node(&topic.hash(), source, external_message)?;
                                let _ = self.swarm.behaviour_mut().request_response.send_response(channel, (shard_id, ExternalMessage::RequestResponse));
                            }
                            request_response::Message::Response {..} => {}
                        }
                    }

                    _ => {},
                },
                message = self.outbound_message_receiver.next() => {
                    let (dest, shard_id, message) = message.expect("message stream should be infinite");
                    let message_type = message.name();
                    let data = serde_json::to_vec(&message).unwrap();
                    let from = self.peer_id;

                    let topic = Self::shard_id_to_topic(shard_id);

                    match dest {
                        Some(dest) => {
                            debug!(%from, %dest, message_type, "sending direct message");
                            if from == dest {
                                self.forward_external_message_to_node(&topic.hash(), from, message)?;
                            } else {
                                let _ = self.swarm.behaviour_mut().request_response.send_request(&dest, (shard_id, message));
                            }
                        },
                        None => {
                            debug!(%from, message_type, "broadcasting");
                            match self.swarm.behaviour_mut().gossipsub.publish(topic.hash(), data)  {
                                Ok(_) => {},
                                Err(e) => {
                                    error!(%e, "failed to publish message");
                                }
                            }
                            // Also broadcast the message to ourselves.
                            self.forward_external_message_to_node(&topic.hash(), from, message)?;
                        },
                    }
                },
                Some(res) = self.shard_threads.join_next() => {
                    if let Err(e) = res {
                        // Currently, abort everything should a single shard fail.
                        error!(%e);
                        break;
                    }
                }
                _ = terminate.recv() => {
                    self.shard_threads.shutdown().await;
                    break;
                },
                _ = signal::ctrl_c() => {
                    self.shard_threads.shutdown().await;
                    break;
                },
            }
        }
        Ok(())
    }
}
