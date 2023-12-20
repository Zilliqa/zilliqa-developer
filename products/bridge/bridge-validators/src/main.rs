mod crypto;
mod message;
mod networking;
mod p2p_node;
mod validator_node;

use anyhow::Result;
use ethers::{
    abi::{self, Token},
    contract::abigen,
    providers::StreamExt,
    types::{Address, Bytes, U256},
    utils::hash_message,
};
use std::{env, sync::Arc};
use tokio::task::JoinSet;
use validator_node::{Client, ValidatorNode};

use crate::{crypto::SecretKey, p2p_node::P2pNode, validator_node::ContractInitializer};

const _VALIDATOR_MANAGER: &str = "0xb228aa0a543204988C1A4f3fd10FEe6551f7A379";
const CHAIN_GATEWAY: &str = "0x4DF88A0dF446b2cb14Ed57d12F48255758DE842a";
const _TARGET: &str = "0x287b0F2491653E5Cb93981AcF7fb30576480015D";

const RPC1: &str = "http://localhost:8545";
const RPC2: &str = "http://localhost:8546";

abigen!(
    ChainGateway,
    "abi/ChainGateway.json",
    derives(serde::Deserialize, serde::Serialize)
);
abigen!(ValidatorManager, "abi/ValidatorManager.json");
abigen!(Target, "abi/Target.json");

async fn listen_relayed(source_chain_id: U256, validator_node: Arc<ValidatorNode>) -> Result<()> {
    let chain_gateway_address = CHAIN_GATEWAY.parse::<Address>()?;

    let source_chain_gateway: ChainGateway<Client> = validator_node
        .get_contract(source_chain_id, chain_gateway_address)
        .unwrap();

    let relayed_events = source_chain_gateway.event::<RelayedFilter>().from_block(1);

    let mut stream = relayed_events.stream().await?;

    while let Some(Ok(event)) = stream.next().await {
        println!("Chain: {} event found: {}", source_chain_id, event);

        // TODO: Broadcast signature and gather signatures

        let data = hash_message(abi::encode(&[
            Token::Int(source_chain_id),
            Token::Int(event.target_chain_id),
            Token::Address(event.target),
            Token::Bytes(Bytes::from(event.call.clone()).to_vec()),
            Token::Uint(event.gas_limit),
            Token::Uint(event.nonce),
        ]));
        let sign_hash = validator_node.wallet.sign_hash(data).unwrap();
        let signatures = vec![Bytes::from(sign_hash.to_vec())];

        let target_chain_gateway: ChainGateway<Client> = validator_node
            .get_contract(event.target_chain_id, chain_gateway_address)
            .unwrap();

        let function_call = target_chain_gateway.dispatch(
            source_chain_id,
            event.target,
            event.call,
            event.gas_limit,
            event.nonce,
            signatures.clone(),
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
                        dbg!("Some unkown error", contract_err);
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let private_key =
        env::var("PRIVATE_KEY").expect("environment variable was not found hello bruno");
    // Initialise relevant clients

    let validator_node = Arc::new(ValidatorNode::new(&[RPC1, RPC2], &private_key).await?);

    // Initialise events
    println!("Start listening");

    // Listen to events in each side
    let mut set = JoinSet::new();

    for chain_ids in validator_node.chain_clients.clone().into_keys() {
        let validator_node = validator_node.clone();
        set.spawn(async move { listen_relayed(chain_ids.clone(), validator_node).await });
    }

    while let Some(res) = set.join_next().await {
        dbg!(&res);
        println!("Finished");
    }

    // Verify events

    // Process the events locally

    // Post actions on chain

    // Setup P2P
    let secret_key = SecretKey::from_hex(&private_key)?;
    let mut node = P2pNode::new(secret_key)?;

    node.start().await?;

    Ok(())
}
