// ! taken from ZQ2
//! A node in the Zilliqa P2P network. May coordinate multiple shard nodes.

use anyhow::{anyhow, Result};
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    gossipsub::{self, IdentTopic, MessageAuthenticity},
    identify,
    kad::{self, store::MemoryStore},
    mdns,
    multiaddr::Multiaddr,
    noise,
    swarm::{self, NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
};
use tokio::{
    select,
    sync::{mpsc, mpsc::UnboundedSender},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error, info};

use crate::{
    crypto::SecretKey,
    message::ExternalMessage,
    validator_node::{ValidatorNode, ValidatorNodeConfig},
};

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    identify: identify::Behaviour,
    kademlia: kad::Behaviour<MemoryStore>,
}

pub struct P2pNode {
    peer_id: PeerId,
    // swarm: Swarm<Behaviour>,
    /// Forward messages to the bridge validators. Only initialised once BridgeNode is created
    bridge_inbound_message_sender: Option<UnboundedSender<ExternalMessage>>,
    /// Bridge nodes get a copy of these senders to propagate messages across the network.
    bridge_outbound_message_sender: UnboundedSender<ExternalMessage>,
    /// The p2p node keeps a handle to these receivers, to obtain messages from bridge nodes and propagate
    /// them as necessary.
    bridge_outbound_message_receiver: UnboundedReceiverStream<ExternalMessage>,
}

impl P2pNode {
    pub fn new(secret_key: SecretKey) -> Result<Self> {
        let (bridge_outbound_message_sender, bridge_outbound_message_receiver) =
            mpsc::unbounded_channel();
        let bridge_outbound_message_receiver =
            UnboundedReceiverStream::new(bridge_outbound_message_receiver);

        let key_pair = secret_key.to_libp2p_keypair();
        let peer_id = PeerId::from(key_pair.public());
        info!(%peer_id);

        // let transport = tcp::tokio::Transport::new(tcp::Config::default())
        //     .upgrade(upgrade::Version::V1)
        //     .authenticate(noise::Config::new(&key_pair)?)
        //     .multiplex(yamux::Config::default())
        //     .boxed();

        // let behaviour = Behaviour {
        //     gossipsub: gossipsub::Behaviour::new(
        //         MessageAuthenticity::Signed(key_pair.clone()),
        //         gossipsub::ConfigBuilder::default()
        //             .max_transmit_size(524288)
        //             .build()
        //             .map_err(|e| anyhow!(e))?,
        //     )
        //     .map_err(|e| anyhow!(e))?,
        //     mdns: mdns::Behaviour::new(Default::default(), peer_id)?,
        //     identify: identify::Behaviour::new(identify::Config::new(
        //         "/ipfs/id/1.0.0".to_owned(),
        //         key_pair.public(),
        //     )),
        //     kademlia: kad::Behaviour::new(peer_id, MemoryStore::new(peer_id)),
        // };

        // let swarm = Swarm::new(
        //     transport,
        //     behaviour,
        //     peer_id,
        //     swarm::Config::with_tokio_executor(),
        // );

        Ok(Self {
            peer_id,
            // swarm,
            bridge_outbound_message_sender,
            bridge_outbound_message_receiver,
            bridge_inbound_message_sender: None,
        })
    }

    fn forward_external_message_to_bridge_node(
        &self,
        _source: PeerId,
        message: ExternalMessage,
    ) -> Result<()> {
        self.bridge_inbound_message_sender
            .as_ref()
            .expect("Bridge should be initialized")
            .send(message)?;
        Ok(())
    }

    async fn create_and_start_validator_node(&mut self, config: ValidatorNodeConfig) -> Result<()> {
        // let topic = IdentTopic::new("bridge"); // TODO: change to more specific bridge chains

        // self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Initialise bridge node
        let mut bridge_node =
            ValidatorNode::new(config, self.bridge_outbound_message_sender.clone()).await?;

        self.bridge_inbound_message_sender = Some(bridge_node.get_bridge_inbound_message_sender());

        tokio::task::spawn(async move { bridge_node.listen_p2p().await.unwrap() });

        Ok(())
    }

    pub async fn start(&mut self, config: ValidatorNodeConfig) -> Result<()> {
        // let addr: Multiaddr = "/ip4/0.0.0.0/tcp/3334".parse().unwrap();

        // if let Some((peer, address)) = &config.bootstrap_address {
        //     self.swarm
        //         .behaviour_mut()
        //         .kademlia
        //         .add_address(peer, address.clone());
        //     self.swarm.behaviour_mut().kademlia.bootstrap()?;
        // }

        self.create_and_start_validator_node(config).await?;

        // self.swarm.listen_on(addr)?;

        println!("Started");

        loop {
            select! {
                // event = self.swarm.select_next_some() => match event {
                //     SwarmEvent::NewListenAddr { address, .. } => {
                //         info!(%address, "started listening");
                //     }
                //     SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                //         for (peer_id, addr) in list {
                //         info!(%peer_id, %addr, "discovered peer via mDNS");
                //         self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                //         self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                //         }
                //     }
                //     SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                //         for (peer_id, addr) in list {
                //             self.swarm.behaviour_mut().kademlia.remove_address(&peer_id, &addr);
                //         }
                //     }
                //     SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received { info: identify::Info { observed_addr, listen_addrs, .. }, peer_id })) => {
                //         info!("discovered peer via kad, {}, {:?}", &peer_id, &listen_addrs);
                //         for addr in listen_addrs {
                //             self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                //         }
                //         // Mark the address observed for us by the external peer as confirmed.
                //         // TODO: We shouldn't trust this, instead we should confirm our own address manually or using
                //         // `libp2p-autonat`.
                //         self.swarm.add_external_address(observed_addr);
                //                             }
                //     SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message{
                //         message: gossipsub::Message {
                //             source,
                //             data,
                //             ..
                //         }, ..
                //     })) => {
                //         let source = source.expect("message should have a source");
                //         let message = serde_json::from_slice::<ExternalMessage>(&data).unwrap();
                //         let message_type = message.name();
                //         let to = self.peer_id;
                //         debug!(%source, %to, message_type, "broadcast received");
                //         self.forward_external_message_to_bridge_node(source, message)?;
                //     }

                //     _ => {},
                // },
                message = self.bridge_outbound_message_receiver.next() => {
                    let message = message.expect("message stream should be infinite");
                    let message_type = message.name();
                    let data = serde_json::to_vec(&message).unwrap();
                    let from = self.peer_id;

                    // let topic = IdentTopic::new("bridge");

                    // if self.swarm.behaviour().gossipsub.all_peers().count() > 0 {
                    // debug!(%from, message_type, "broadcasting");
                    // match self.swarm.behaviour_mut().gossipsub.publish(topic.hash(), data)  {
                    //     Ok(_) => {},
                    //     Err(e) => {
                    //         error!(%e, "failed to publish message");
                    //     }
                    // }
                    // } else{
                    //     info!("Not broadcasting");
                    // };


                    // Also broadcast the message to ourselves.
                    self.forward_external_message_to_bridge_node(from, message)?;
                },
            }
        }
    }
}
