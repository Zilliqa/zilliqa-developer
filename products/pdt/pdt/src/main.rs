mod bqimport;
mod importer;
mod importers;
mod multi_import;
mod psqlimport;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use pdtlib::context::Context;
use pdtlib::exporter::Exporter;
use pdtlib::historical::Historical;
use pdtlib::incremental::Incremental;
use pdtlib::render::Renderer;
use pdtlisten::{listen_bq, listen_psql};
use pdtparse::parse_zrc2;

#[derive(Parser)]
/// Download and import Zilliqa 1 persistence.
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "/home/rrw/tmp/test")]
    download_dir: String,

    #[arg(long, default_value = "/home/rrw/tmp/unpacked")]
    unpack_dir: String,

    #[arg(long, default_value = "testnet-925")]
    network: String,

    #[arg(long, value_enum)]
    network_type: Option<NetworkType>,

    #[arg(long, value_enum)]
    token_type: Option<TokenType>,

    #[arg(long)]
    postgres_url: Option<String>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NetworkType {
    Testnet,
    Mainnet,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TokenType {
    ZRC2,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "download")]
    DownloadPersistence,
    #[command(name = "dump")]
    DumpPersistence,
    #[command(name = "bqmulti")]
    BQImportMulti(MultiOptions),
    #[command(name = "psqlmulti")]
    PSQLImportMulti(PSQLMultiOptions),
    #[command(name = "reconcile-blocks")]
    ReconcileBlocks(ReconcileOptions),
    #[command(name = "parse-events")]
    ParseEvents,
    #[command(name = "psqllisten")]
    PSQLListen,
    #[command(name = "bqlisten")]
    BQListen(ListenOptions),
}

// #[derive(Debug, Args)]
// struct ParseOptions {
//     #[arg(long), value_enum]
//     token_type: Option<TokenType>,
// }

#[derive(Debug, Args)]
struct ReconcileOptions {
    #[clap(long)]
    batch_blocks: i64,

    #[arg(long)]
    dataset_id: String,

    #[arg(long, default_value = "rrw-bigquery-test-id")]
    project_id: String,

    #[arg(long)]
    service_account_key_file: Option<String>,
}

#[derive(Debug, Args)]
struct MultiOptions {
    #[clap(long)]
    nr_threads: i64,

    #[clap(long)]
    batch_blocks: i64,

    #[clap(long, default_value = None) ]
    start_block: Option<i64>,

    #[arg(long)]
    dataset_id: String,

    #[arg(long, default_value = "rrw-bigquery-test-id")]
    project_id: String,

    #[arg(long)]
    no_dup: bool,

    #[arg(long)]
    service_account_key_file: Option<String>,
}
#[derive(Debug, Args)]
struct PSQLMultiOptions {
    #[clap(long)]
    nr_threads: i64,

    #[clap(long)]
    batch_blocks: i64,

    #[clap(long, default_value = None) ]
    start_block: Option<i64>,

    #[arg(long)]
    no_dup: bool,

    #[arg(long)]
    partition_size: i64,
}
#[derive(Debug, Args)]
struct ListenOptions {
    #[arg(long)]
    dataset_id: String,

    #[arg(long, default_value = "prj-c-data-analytics-3xs14wez")]
    project_id: String,

    #[arg(long, default_value = "5")]
    buffer_size: usize,
}

const TESTNET_BUCKET: &str = "zq1-testnet-persistence";
const MAINNET_BUCKET: &str = "c5b68604-8540-4887-ad29-2ab9e680f997";

const DEV_API_URL: &str = "https://dev-api.zilliqa.com/";
const MAINNET_API_URL: &str = "https://api.zilliqa.com/";

async fn download_persistence(
    download_dir: &str,
    unpack_dir: &str,
    network_name: &str,
    network_type: &Option<NetworkType>,
) -> Result<()> {
    let network_type = if let Some(t) = network_type {
        t
    } else {
        if network_name.starts_with("mainnet-") {
            &NetworkType::Mainnet
        } else if network_name.starts_with("testnet-") {
            &NetworkType::Testnet
        } else {
            panic!("Unexpected network type")
        }
    };
    let bucket_name = match network_type {
        NetworkType::Mainnet => MAINNET_BUCKET,
        NetworkType::Testnet => TESTNET_BUCKET,
    };
    let ctx = Context::new(bucket_name, network_name, &download_dir).await?;
    println!("Downloading persistence to {}", download_dir);
    let historical = Historical::new(&ctx)?;
    historical.download().await?;
    println!("Download persistence .. ");
    let incr = Incremental::new(&ctx)?;
    let max_block = incr.get_max_block().await?;
    incr.download_persistence().await?;
    incr.download_incr_persistence().await?;
    incr.download_incr_state().await?;
    incr.save_meta(max_block)?;
    println!("Max block {}", max_block);

    let render = Renderer::new(network_name, download_dir, unpack_dir)?;
    let recovery_points = render.get_recovery_points()?;
    println!(
        "persistence blocks: {:?} , state blocks: {:?}",
        recovery_points.persistence_blocks, recovery_points.state_delta_blocks
    );
    println!("RP : {:?} ", recovery_points.recovery_points);

    render.unpack(&recovery_points, None)?;

    println!("Persistence was rendered successfully");
    Ok(())
}

async fn dump_persistence(unpack_dir: &str) -> Result<()> {
    let exporter = Exporter::new(&unpack_dir)?;
    let max_block = exporter.get_max_block();
    println!("max_block is {}", max_block);

    // exporter.import_txns(4, max_block, None);
    for val in exporter.micro_blocks(max_block, None)? {
        if let Ok((key, blk)) = val {
            // println!("Blk! at {}/{}", key.epochnum, key.shardid);
            for (_hash, maybe_txn) in exporter.txns(&key, &blk)? {
                if let Some(txn) = maybe_txn {
                    if let Some(_actually_txn) = txn.transaction {
                        // println!("Txn {}", H256::from_slice(&actually_txn.tranid));
                    }
                }
            }
        } else {
            println!("Error!");
        }
    }
    Ok(())
}

async fn bigquery_import_multi(unpack_dir: &str, opts: &MultiOptions) -> Result<()> {
    bqimport::bq_multi_import(
        unpack_dir,
        opts.nr_threads,
        opts.batch_blocks,
        opts.start_block,
        &opts.project_id,
        &opts.dataset_id,
    )
    .await
}
async fn psql_import_multi(
    unpack_dir: &str,
    postgres_url: &Option<String>,
    opts: &PSQLMultiOptions,
) -> Result<()> {
    let db_url = postgres_url
        .as_ref()
        .expect("postgres url should be set with the --postgres-url option");
    psqlimport::psql_multi_import(
        unpack_dir,
        opts.nr_threads,
        opts.batch_blocks,
        opts.start_block,
        &db_url,
        !opts.no_dup,
        opts.partition_size,
    )
    .await
}

async fn parse_events(token_type: &Option<TokenType>, postgres_url: &Option<String>) -> Result<()> {
    let t_type = token_type.expect("the token type to be parsed should be given!");
    let postgres_url = postgres_url
        .as_ref()
        .expect("postgres url should be set with the --postgres-url option");
    match t_type {
        TokenType::ZRC2 => parse_zrc2(postgres_url).await,
    }
}

async fn bigquery_reconcile_blocks(unpack_dir: &str, opts: &ReconcileOptions) -> Result<()> {
    bqimport::reconcile_blocks(
        unpack_dir,
        opts.batch_blocks,
        &opts.project_id,
        &opts.dataset_id,
    )
    .await
}

async fn psql_listen_outer(postgres_url: &str, network_type: &NetworkType) -> Result<()> {
    let api_url = match network_type {
        NetworkType::Testnet => DEV_API_URL,
        NetworkType::Mainnet => MAINNET_API_URL,
    };
    listen_psql(postgres_url, api_url).await
}

async fn bigquery_listen_outer(
    bq_project_id: &str,
    bq_dataset_id: &str,
    network_type: &NetworkType,
    block_buffer_size: usize,
) -> Result<()> {
    let api_url = match network_type {
        NetworkType::Testnet => DEV_API_URL,
        NetworkType::Mainnet => MAINNET_API_URL,
    };
    listen_bq(bq_project_id, bq_dataset_id, api_url, block_buffer_size).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::DownloadPersistence => {
            download_persistence(
                &cli.download_dir,
                &cli.unpack_dir,
                &cli.network,
                &cli.network_type,
            )
            .await
        }
        Commands::DumpPersistence => dump_persistence(&cli.unpack_dir).await,
        Commands::BQImportMulti(opts) => bigquery_import_multi(&cli.unpack_dir, opts).await,
        Commands::PSQLImportMulti(opts) => {
            psql_import_multi(&cli.unpack_dir, &cli.postgres_url, opts).await
        }
        Commands::ReconcileBlocks(opts) => bigquery_reconcile_blocks(&cli.unpack_dir, opts).await,
        Commands::ParseEvents => parse_events(&cli.token_type, &cli.postgres_url).await,
        Commands::PSQLListen => {
            psql_listen_outer(
                &cli.postgres_url
                    .expect("no postgres connection url -- did you forget to set --postgres-url?"),
                &cli.network_type
                    .expect("no network type -- did forget to set --network-type?"),
            )
            .await
        }
        Commands::BQListen(opts) => {
            bigquery_listen_outer(
                &opts.project_id,
                &opts.dataset_id,
                &cli.network_type
                    .expect("no network type -- did forget to set --network-type?"),
                opts.buffer_size,
            )
            .await
        }
    }
}
