use std::{env, net::IpAddr, sync::Arc};

use anyhow::{anyhow, Result};
use askama::Template;
use axum::{extract::State, routing::get, Form, Router};
use ethers::{
    middleware::{signer::SignerMiddlewareError, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::{TransactionRequest, H256},
    utils::{parse_checksummed, parse_ether, to_checksum, ConversionError, WEI_IN_ETHER},
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    from_addr: String,
    native_token_symbol: String,
    amount: String,
    explorer_url: Option<String>,
    pending_txn_hash: Option<String>,
    error: Option<String>,
}

enum RequestStatus {
    Sent(H256),
    AddrErr(ConversionError),
    SendErr(SignerMiddlewareError<Provider<Http>, LocalWallet>),
}

async fn home_inner(State(state): State<Arc<AppState>>, status: Option<RequestStatus>) -> Home {
    Home {
        from_addr: to_checksum(&state.signer.address(), None),
        native_token_symbol: state.config.native_token_symbol.clone(),
        amount: state.config.eth_amount.clone(),
        explorer_url: state.config.explorer_url.clone(),
        pending_txn_hash: match status {
            Some(RequestStatus::Sent(t)) => Some(format!("{t:?}")),
            _ => None,
        },
        error: match status {
            Some(RequestStatus::AddrErr(e)) => Some(format!("Invalid address: {e}")),
            Some(RequestStatus::SendErr(e)) => Some(format!("Failed to send transaction: {e}")),
            _ => None,
        },
    }
}

async fn home(state: State<Arc<AppState>>) -> Home {
    home_inner(state, None).await
}

#[derive(Deserialize)]
struct Request {
    address: String,
}

async fn request(State(state): State<Arc<AppState>>, Form(request): Form<Request>) -> Home {
    let address = match parse_checksummed(&request.address, None) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e:?}");
            return home_inner(State(state), Some(RequestStatus::AddrErr(e))).await;
        },
    };

    let value = parse_ether(&state.config.eth_amount).unwrap_or(WEI_IN_ETHER);
    let tx = TransactionRequest::pay(
        address,
        value,
    );
    let status = match state.signer.send_transaction(tx, None).await {
        Ok(t) => RequestStatus::Sent(t.tx_hash()),
        Err(e) => {
            eprintln!("{e:?}");
            return home_inner(State(state.clone()), Some(RequestStatus::SendErr(e))).await;
        }
    };

    println!("Sent {value} to {address:?}");

    home_inner(State(state), Some(status)).await
}

struct Config {
    http_port: u16,
    rpc_url: String,
    native_token_symbol: String,
    private_key: String,
    eth_amount: String,
    explorer_url: Option<String>,
}

fn env_var(key: &'static str) -> Result<String> {
    env::var(key).map_err(|_| anyhow!("environment variable {key} not set"))
}

impl Config {
    fn from_env() -> Result<Config> {
        Ok(Config {
            http_port: env_var("HTTP_PORT")
                .and_then(|p| Ok(p.parse()?))
                .unwrap_or(80),
            rpc_url: env_var("RPC_URL")?,
            native_token_symbol: env_var("NATIVE_TOKEN_SYMBOL")
                .unwrap_or_else(|_| "ETH".to_owned()),
            private_key: env_var("PRIVATE_KEY")?,
            eth_amount: env_var("ETH_AMOUNT").unwrap_or_else(|_| "1".to_owned()),
            explorer_url: env_var("EXPLORER_URL").ok(),
        })
    }
}

struct AppState {
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let provider = Provider::try_from(&config.rpc_url)?;
    let wallet: LocalWallet = config.private_key.parse()?;
    let signer = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;

    let addr = ("0.0.0.0".parse::<IpAddr>()?, config.http_port).into();
    let state = Arc::new(AppState { signer, config });

    let app = Router::new()
        .route("/", get(home).post(request))
        .with_state(state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
