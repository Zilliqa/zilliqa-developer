mod block;
mod bridge_node;
mod client;
mod crypto;
mod event;
mod message;
mod p2p_node;
mod signature;
mod validator_node;

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use ethers::{contract::abigen, types::Address};
use libp2p::{Multiaddr, PeerId};
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::EnvFilter;
use validator_node::ValidatorNodeConfig;

use crate::{crypto::SecretKey, p2p_node::P2pNode};

abigen!(ChainGateway, "abi/ChainGateway.json",);
abigen!(ValidatorManager, "abi/ValidatorManager.json");

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub validator_manager_address: Address,
    pub chain_gateway_address: Address,
    pub chain_gateway_block_deployed: u64,
    pub block_instant_finality: Option<bool>,
    pub legacy_gas_estimation: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub chain_configs: Vec<ChainConfig>,
    pub bootstrap_address: Option<(PeerId, Multiaddr)>,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = SecretKey::from_hex)]
    secret_key: SecretKey,
    #[clap(long, short, default_value = "config.toml")]
    config_file: PathBuf,
    #[clap(long)]
    is_leader: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let config = if args.config_file.exists() {
        fs::read_to_string(&args.config_file)?
    } else {
        panic!("There needs to be a config file provided");
    };
    let config: Config = toml::from_str(&config)?;

    if args.is_leader {
        info!("Running as leader");
    }

    let config = ValidatorNodeConfig {
        chain_configs: config.chain_configs,
        private_key: args.secret_key,
        is_leader: args.is_leader,
        bootstrap_address: config.bootstrap_address,
    };

    let mut node = P2pNode::new(args.secret_key)?;

    node.start(config).await?;

    Ok(())
}
