mod crypto;
mod message;
mod p2p_node;
mod validator_node;

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use ethers::contract::abigen;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;
use validator_node::BridgeNodeConfig;

use crate::{crypto::SecretKey, p2p_node::P2pNode};

const _TARGET: &str = "0x9cB4b20da1fA0caA96221aD7a80139DdbBEC266e";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub chain_configs: Vec<ChainConfig>,
}

abigen!(ChainGateway, "abi/ChainGateway.json",);
abigen!(ValidatorManager, "abi/ValidatorManager.json");
abigen!(Target, "abi/Target.json");

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub validator_manager_address: String,
    pub chain_gateway_address: String,
    pub block_query_limit: Option<u64>,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = SecretKey::from_hex)]
    secret_key: SecretKey,
    #[clap(short, long, default_value = "false")]
    leader: bool,
    #[clap(long, short, default_value = "config.toml")]
    config_file: PathBuf,
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

    let config = BridgeNodeConfig {
        chain_configs: config.chain_configs,
        private_key: args.secret_key,
        is_leader: args.leader,
    };

    let mut node = P2pNode::new(args.secret_key)?;

    node.start(config).await?;

    Ok(())
}
