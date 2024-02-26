use std::path::Path;

use anyhow::{anyhow, Result};
use pdtdb::utils::ProcessCoordinates;
use pdtlib::exporter::Exporter;
use pdtpsql::psql::ZilliqaPSQLProject;
use sqlx::{postgres::PgPoolOptions, query, PgPool};
use tokio::task::JoinSet;

use crate::{importer::Importer, importers::TransactionMicroblockImporter, multi_import::import};

pub async fn psql_multi_import(
    unpack_dir: &str,
    nr_threads: i64,
    batch_blocks: i64,
    start_blk: Option<i64>,
    db_url: &str,
    duplicate_persistence: bool,
    partition_size: i64,
) -> Result<()> {
    let mut jobs = JoinSet::new();
    let coords = ProcessCoordinates {
        nr_machines: nr_threads,
        batch_blks: batch_blocks,
        machine_id: 0,
        client_id: "schema_creator".to_string(),
    };

    println!("instatiating connection pool..");

    let pool = PgPoolOptions::new()
        .max_connections((nr_threads + 1).try_into()?)
        .connect(db_url)
        .await?;

    // create the schema
    println!("Creating schema .. ");

    ZilliqaPSQLProject::ensure_schema(&pool).await?;

    for idx in 0..nr_threads {
        let my_pool = pool.clone();
        let orig_unpack_dir = unpack_dir.to_string();
        let my_unpack_dir = Path::new(unpack_dir).join("..").join(format!("t{}", idx));
        let my_start_blk = start_blk.clone();
        let my_coords = coords
            .with_client_id(&format!("t{}_{}", idx, coords.nr_machines))
            .with_machine_id(idx);
        let url = db_url.to_string();
        jobs.spawn(async move {
            println!("Sync persistence to {:?}", &my_unpack_dir);
            let unpack_str = my_unpack_dir
                .to_str()
                .ok_or(anyhow!("Cannot render thread-specific data dir"))?;
            // Sync persistence
            if duplicate_persistence {
                pdtlib::utils::dup_directory(&orig_unpack_dir, &unpack_str)?;
            }
            println!("Starting thread {}/{}", idx, nr_threads);
            let exporter = Exporter::new(&unpack_str)?;
            let mut imp = TransactionMicroblockImporter::new();
            let max_block = imp.get_max_block(&exporter).await?;
            let nr_blocks: i64 = (max_block + 1).try_into()?;
            let project =
                ZilliqaPSQLProject::new(&url, my_pool, &my_coords, nr_blocks, partition_size)?;
            while !import(
                &mut imp,
                &exporter,
                &project,
                &my_coords,
                None,
                my_start_blk,
            )
            .await?
            {
                // Go round again!
            }
            let result: anyhow::Result<()> = Ok(());
            result
        });
    }
    println!("Waiting for jobs to complete .. ");
    while jobs.len() > 0 {
        let result = jobs.join_next().await;
        if let Some(res) = result {
            let value = res?;
            println!("Job finished with {:?}, {} remain", value, jobs.len())
        }
    }
    async fn print_imported_ranges(pool: &PgPool) -> Result<()> {
        let txn_range = query!(
            "select * from (select unnest(imported_ranges) as range from transactions_meta) as r"
        )
        .fetch_all(pool)
        .await?;
        println!("txns: imported {:#?} ranges", txn_range);
        let mb_range = query!(
            "select * from (select unnest(imported_ranges) as range from microblocks_meta) as r"
        )
        .fetch_all(pool)
        .await?;
        println!("microblocks: imported {:#?} ranges", mb_range);
        Ok(())
    }
    print_imported_ranges(&pool).await?;
    println!("cleaning up...");
    // run one more pass, single threaded, cleaning up any missed batches.
    let my_coords = ProcessCoordinates {
        nr_machines: 1,
        batch_blks: batch_blocks,
        machine_id: 0,
        client_id: "t_clean".to_string(),
    };
    let my_unpack_dir = Path::new(unpack_dir).join("..").join(format!("t{}", 0));
    let unpack_str = my_unpack_dir
        .to_str()
        .ok_or(anyhow!("Cannot render thread-specific data dir"))?;
    let exporter = Exporter::new(&unpack_str)?;
    let mut imp = TransactionMicroblockImporter::new();
    let max_block = imp.get_max_block(&exporter).await?;
    let nr_blocks: i64 = (max_block + 1).try_into()?;
    let project = ZilliqaPSQLProject::new(
        &db_url.to_string(),
        pool.clone(),
        &my_coords,
        nr_blocks,
        partition_size,
    )?;
    while !import(&mut imp, &exporter, &project, &my_coords, None, start_blk).await? {
        // Go round again!
    }
    println!("cleaned up. imported ranges:");
    print_imported_ranges(&pool).await?;
    Ok(())
}
