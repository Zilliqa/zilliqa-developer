use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::{Chain, Result};
use ethers::{
    abi::{self, AbiEncode, Token},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, StreamExt},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes},
    utils::hash_message,
};
use ethers_contract::{abigen, providers::Provider, ContractError, ContractRevert};
use std::env;
use tokio::task::JoinSet;

const VALIDATOR_MANAGER: &str = "0xb228aa0a543204988C1A4f3fd10FEe6551f7A379";
const CHAIN_GATEWAY: &str = "0x4DF88A0dF446b2cb14Ed57d12F48255758DE842a";
const TARGET: &str = "0x287b0F2491653E5Cb93981AcF7fb30576480015D";

const RPC1: &str = "http://localhost:8545";
const RPC2: &str = "http://localhost:8546";

abigen!(
    ChainGateway,
    "abi/ChainGateway.json",
    derives(serde::Deserialize, serde::Serialize),
);
abigen!(ValidatorManager, "abi/ValidatorManager.json");
abigen!(Target, "abi/Target.json");

struct BridgeChain {
    chain_id: SupportedChains,
    rpc: &'static str,
    provider: Arc<Provider<Http>>,
}

enum SupportedChains {
    Chain1 = 1,
    Chain2 = 2,
}

struct Providers {
    providers: HashMap<SupportedChains, Arc<Provider<Http>>>,
}

async fn listen_relayed(
    source_client: Arc<Provider<Http>>,
    target_client: Arc<Provider<Http>>,
) -> Result<()> {
    let private_key =
        env::var("PRIVATE_KEY").expect("environment variable was not found hello bruno");
    let source_chain_id = source_client.get_chainid().await?;
    let target_chain_id = target_client.get_chainid().await?;

    let wallet = LocalWallet::from_str(&private_key)?;
    let source_signer = Arc::new(SignerMiddleware::new(
        source_client.clone(),
        wallet.clone().with_chain_id(source_chain_id.as_u64()),
    ));
    let target_signer = Arc::new(SignerMiddleware::new(
        target_client.clone(),
        wallet.clone().with_chain_id(target_chain_id.as_u64()),
    ));

    let chain_gateway_address: Address = CHAIN_GATEWAY.parse()?;
    let validator_manager_address: Address = VALIDATOR_MANAGER.parse()?;
    let source_chain_gateway = ChainGateway::new(chain_gateway_address, source_signer.clone());
    let target_chain_gateway = ChainGateway::new(chain_gateway_address, target_signer.clone());

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
        let sign_hash = wallet.sign_hash(data.clone()).unwrap();
        let signatures = vec![Bytes::from(sign_hash.to_vec())];

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
    // Initialise relevant clients
    let target_address: Address = TARGET.parse()?;
    let validator_manager_address: Address = VALIDATOR_MANAGER.parse()?;

    let client1 = Arc::new(Provider::try_from(RPC1)?);
    let client2 = Arc::new(Provider::try_from(RPC2)?);

    // Initialise events
    println!("Start listening");

    // Listen to events in each side
    let mut set = JoinSet::new();
    let c1 = client1.clone();
    let c2 = client2.clone();
    set.spawn(async move { listen_relayed(c1.clone(), c2.clone()).await });

    let c1 = client1.clone();
    let c2 = client2.clone();
    set.spawn(async move { listen_relayed(c2.clone(), c1.clone()).await });

    while let Some(res) = set.join_next().await {
        dbg!(res);
        println!("Finished");
    }

    // TODO: Prove the events

    // Process the events locally

    // Post actions on chain

    Ok(())
}
