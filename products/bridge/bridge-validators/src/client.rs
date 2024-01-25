use std::sync::Arc;

use crate::{ChainGateway, ValidatorManager};
use anyhow::Result;
use ethers::{
    middleware::{MiddlewareBuilder, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};

use crate::ChainConfig;

pub type Client = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ChainClient {
    pub client: Arc<Client>,
    pub validator_manager_address: Address,
    pub chain_gateway_address: Address,
    pub chain_id: U256,
    pub wallet: LocalWallet,
    pub chain_gateway_block_deployed: u64,
}

impl ChainClient {
    pub async fn new(config: &ChainConfig, wallet: LocalWallet) -> Result<Self> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())?;
        // let provider = Provider::<Ws>::connect(&config.rpc_url).await?;
        let chain_id = provider.get_chainid().await?;

        let client: Arc<Client> =
            Arc::new(provider.with_signer(wallet.clone().with_chain_id(chain_id.as_u64())));

        Ok(ChainClient {
            client,
            validator_manager_address: config.validator_manager_address.parse()?,
            chain_gateway_address: config.chain_gateway_address.parse()?,
            chain_id,
            wallet,
            chain_gateway_block_deployed: config.chain_gateway_block_deployed,
        })
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
