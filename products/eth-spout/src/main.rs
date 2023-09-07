use std::{
    collections::HashMap,
    env, mem,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use askama::Template;
use axum::{extract::State, routing::get, Form, Router};
use ethers::{
    middleware::{nonce_manager::NonceManagerError, NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::{Address, TransactionRequest, H256},
    utils::{parse_checksummed, parse_ether, to_checksum, ConversionError, WEI_IN_ETHER},
};
use serde::Deserialize;
use tokio::sync::Mutex;

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
    SendErr(NonceManagerError<SignerMiddleware<Provider<Http>, LocalWallet>>),
    RateLimitErr(Duration),
}

async fn home_inner(State(state): State<Arc<AppState>>, status: Option<RequestStatus>) -> Home {
    Home {
        from_addr: to_checksum(&state.provider.inner().address(), None),
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
            Some(RequestStatus::RateLimitErr(d)) => Some(format!(
                "Request made too recently, please wait {} seconds before trying again",
                d.as_secs()
            )),
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
        }
    };

    let minimum_time_between_requests = state.config.minimum_time_between_requests;
    // Avoid accessing mutex if minimum time is zero.
    if !minimum_time_between_requests.is_zero() {
        let guard = state.last_request.lock().await;
        if let Some(last_request) = guard.get(&address) {
            let elapsed = last_request.elapsed();
            mem::drop(guard);
            if elapsed <= minimum_time_between_requests {
                eprintln!("Last request for {address:?} was too recent ({elapsed:?} ago)");
                return home_inner(
                    State(state),
                    Some(RequestStatus::RateLimitErr(
                        minimum_time_between_requests - elapsed,
                    )),
                )
                .await;
            }
        }
    }

    let value = parse_ether(&state.config.eth_amount).unwrap_or(WEI_IN_ETHER);
    let tx = TransactionRequest::pay(address, value);
    let status = match state.provider.send_transaction(tx, None).await {
        Ok(t) => RequestStatus::Sent(t.tx_hash()),
        Err(e) => {
            eprintln!("{e:?}");
            return home_inner(State(state.clone()), Some(RequestStatus::SendErr(e))).await;
        }
    };

    println!("Sent {value} to {address:?}");

    // No need to keep this state if minimum time is zero
    if !minimum_time_between_requests.is_zero() {
        state
            .last_request
            .lock()
            .await
            .insert(address, Instant::now());
    }

    home_inner(State(state), Some(status)).await
}

struct Config {
    http_port: u16,
    rpc_url: String,
    native_token_symbol: String,
    private_key: String,
    eth_amount: String,
    explorer_url: Option<String>,
    minimum_time_between_requests: Duration,
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
            minimum_time_between_requests: env_var("MINIMUM_SECONDS_BETWEEN_REQUESTS")
                .and_then(|t| Ok(Duration::from_secs(t.parse()?)))
                .unwrap_or(Duration::from_secs(300)),
        })
    }
}

struct AppState {
    provider: NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
    config: Config,
    last_request: Mutex<HashMap<Address, Instant>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let provider = Provider::try_from(&config.rpc_url)?;
    let wallet: LocalWallet = config.private_key.parse()?;
    let provider = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;
    let address = provider.address();
    let provider = NonceManagerMiddleware::new(provider, address);

    let addr = ("0.0.0.0".parse::<IpAddr>()?, config.http_port).into();
    let state = Arc::new(AppState {
        provider,
        config,
        last_request: Default::default(),
    });

    let app = Router::new()
        .route("/", get(home).post(request))
        .with_state(state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
