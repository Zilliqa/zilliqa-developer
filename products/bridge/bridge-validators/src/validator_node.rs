use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::{Address, U256},
};
use futures::future::join_all;

use crate::{ChainGateway, ValidatorManager};

pub type Client = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ValidatorNode {
    pub chain_clients: HashMap<U256, Client>,
    pub wallet: LocalWallet,
    // /// The following two message streams are used for networked messages.
    // /// The sender is provided to the p2p coordinator, to forward messages to the node.
    // pub inbound_message_sender: UnboundedSender<(PeerId, ExternalMessage)>,
    // /// The corresponding receiver is handled here, forwarding messages to the node struct.
    // pub inbound_message_receiver: UnboundedReceiverStream<(PeerId, ExternalMessage)>,
}

impl ValidatorNode {
    pub async fn new(rpc_urls: &[&str], private_key: &str) -> Result<ValidatorNode> {
        let wallet: LocalWallet = private_key.try_into()?;

        let chain_clients: HashMap<U256, Client> =
            join_all(rpc_urls.into_iter().map(|&rpc_url| async move {
                let provider = Provider::<Http>::try_from(rpc_url)?;
                let client: Client =
                    SignerMiddleware::new(provider, LocalWallet::from_str(private_key)?);
                let chain_id = client.get_chainid().await?;

                Ok((chain_id, client))
            }))
            .await
            .into_iter()
            .collect::<Result<HashMap<U256, Client>>>()
            .unwrap();

        Ok(ValidatorNode {
            chain_clients,
            wallet,
        })
    }
}

pub trait ContractInitializer<T> {
    fn get_contract(&self, chain_id: U256, contract_address: Address) -> Option<T>;
}

impl ContractInitializer<ValidatorManager<Client>> for ValidatorNode {
    fn get_contract(
        &self,
        chain_id: U256,
        contract_address: Address,
    ) -> Option<ValidatorManager<Client>> {
        self.chain_clients
            .get(&chain_id)
            .map(|client| ValidatorManager::new(contract_address, Arc::new(client.clone())))
    }
}

impl ContractInitializer<ChainGateway<Client>> for ValidatorNode {
    fn get_contract(
        &self,
        chain_id: U256,
        contract_address: Address,
    ) -> Option<ChainGateway<Client>> {
        self.chain_clients
            .get(&chain_id)
            .map(|client| ChainGateway::new(contract_address, Arc::new(client.clone())))
    }
}
