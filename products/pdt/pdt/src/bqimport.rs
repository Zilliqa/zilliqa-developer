use pdtdb::utils::ProcessCoordinates;
// Bigquery importer.
use crate::importer::Importer;
use crate::importers::TransactionMicroblockImporter;
use crate::multi_import::import;
use anyhow::{anyhow, Result};
use pdtbq::bq::ZilliqaBQProject;
use pdtbq::bq_utils::BigQueryDatasetLocation;
use pdtdb::zqproj::ZilliqaDBProject;
use pdtlib::exporter::Exporter;
use std::path::Path;
use tokio::task::JoinSet;

//    "/home/rrw/work/zilliqa/src/secrets/rrw-bigquery-test-id-8401353b2800.json";

pub async fn bq_multi_import(
    unpack_dir: &str,
    nr_threads: i64,
    batch_blocks: i64,
    start_blk: Option<i64>,
    project_id: &str,
    dataset_id: &str,
) -> Result<()> {
    // OK. Just go ..
    let mut jobs = JoinSet::new();
    let location = BigQueryDatasetLocation {
        project_id: project_id.to_string(),
        dataset_id: dataset_id.to_string(),
    };
    let coords = ProcessCoordinates {
        nr_machines: nr_threads,
        batch_blks: batch_blocks,
        machine_id: 0,
        client_id: "schema_creator".to_string(),
    };

    // Start off by creating the schema. Allocating a new BQ Object will do ..
    println!("Creating schema .. ");
    ZilliqaBQProject::ensure_schema(&location).await?;

    for idx in 0..nr_threads {
        let orig_unpack_dir = unpack_dir.to_string();
        let my_unpack_dir = Path::new(unpack_dir).join("..").join(format!("t{}", idx));
        let my_start_blk = start_blk.clone();
        let my_coords = coords
            .with_client_id(&format!("t{}_{}", idx, coords.nr_machines))
            .with_machine_id(idx);
        let location = BigQueryDatasetLocation {
            project_id: project_id.to_string(),
            dataset_id: dataset_id.to_string(),
        };

        jobs.spawn(async move {
            println!("Sync persistence to {:?}", &my_unpack_dir);
            let unpack_str = if nr_threads > 1 {
                let new_unpack_dir = my_unpack_dir
                    .to_str()
                    .ok_or(anyhow!("Cannot render thread-specific data dir"))?;
                // Sync persistence
                pdtlib::utils::dup_directory(&orig_unpack_dir, &new_unpack_dir)?;
                new_unpack_dir
            } else {
                &orig_unpack_dir
            };

            println!("Starting thread {}/{}", idx, nr_threads);
            let exporter = Exporter::new(&unpack_str)?;
            let mut imp = TransactionMicroblockImporter::new();
            let max_block = imp.get_max_block(&exporter).await?;
            let nr_blocks: i64 = (max_block + 1).try_into()?;
            let project = ZilliqaBQProject::new(&location, &my_coords, nr_blocks).await?;
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
    Ok(())
}

pub async fn reconcile_blocks(
    unpack_dir: &str,
    scale: i64,
    project_id: &str,
    dataset_id: &str,
) -> Result<()> {
    let client_id = format!("reconcile-blocks");
    let exporter = Exporter::new(&unpack_dir)?;
    let max_block: i64 = exporter.get_max_block().try_into()?;
    let location = BigQueryDatasetLocation {
        project_id: project_id.to_string(),
        dataset_id: dataset_id.to_string(),
    };
    let coords = ProcessCoordinates {
        nr_machines: 1,
        batch_blks: 0,
        machine_id: 0,
        client_id: client_id.to_string(),
    };
    let project = ZilliqaBQProject::new(&location, &coords, max_block + 1).await?;
    // We should have coverage for every block, extant or not, up to the
    // last batch, which will be short.
    let mut blk: i64 = 0;
    while (blk + scale) < max_block {
        let span = std::cmp::min(scale, max_block - (blk + scale));
        println!("blk {} span {}", blk, span);
        match project.is_txn_range_covered_by_entry(blk, span).await? {
            None => {
                println!(
                    "Help! Block range {} + {} is not covered by any imported entry",
                    blk, span
                );
                blk += span;
            }
            Some((nr_blks, _c)) => {
                println!("Skipping {} blocks @ {}", nr_blks, blk);
                blk += nr_blks;
            }
        };
    }

    Ok(())
}
