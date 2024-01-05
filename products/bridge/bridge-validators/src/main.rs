mod crypto;
mod message;
mod p2p_node;
mod validator_node;

use anyhow::Result;
use clap::Parser;
use ethers::contract::abigen;
use tracing_subscriber::EnvFilter;
use validator_node::BridgeNodeConfig;

use crate::{crypto::SecretKey, p2p_node::P2pNode};

const _TARGET: &str = "0x9cB4b20da1fA0caA96221aD7a80139DdbBEC266e";

const RPC1: &str = "http://localhost:8545";
const RPC2: &str = "http://localhost:8546";

abigen!(
    ChainGateway,
    "abi/ChainGateway.json",
    derives(serde::Deserialize, serde::Serialize)
);
abigen!(ValidatorManager, "abi/ValidatorManager.json");
abigen!(Target, "abi/Target.json");

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = SecretKey::from_hex)]
    secret_key: SecretKey,
    #[clap(short, long, default_value = "false")]
    leader: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let config = BridgeNodeConfig {
        rpc_urls: vec![RPC1.to_string(), RPC2.to_string()],
        private_key: args.secret_key,
        is_leader: args.leader,
    };

    let mut node = P2pNode::new(args.secret_key)?;

    node.start(config).await?;

    Ok(())
}
