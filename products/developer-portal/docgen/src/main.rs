use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};
use zqutils::commands::CommandBuilder;

#[derive(Clone, Deserialize)]
struct Version {
    refspec: String,
    name: Option<String>,
}

#[derive(Clone, Deserialize)]
struct Zq2Spec {
    versions: Vec<Version>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let here = String::from(&args[1]);

    // Set NO_CHECKOUT to skip the checkout steps - this allows you to do debugging with symlinks or similar.
    let checkout = !std::env::var("NO_CHECKOUT").is_ok();

    // Find the zq2 versions that we need to collect.

    println!("here = {here}");

    let root_path = PathBuf::from(&here);
    let versions: Zq2Spec =
        serde_yaml::from_str(&fs::read_to_string(format!("{}/zq2_spec.yaml", here))?)?;
    for vrec in &versions.versions {
        let refspec = &vrec.refspec;
        let name: String = match vrec.name {
            None => {
                if refspec.len() > 8 {
                    refspec[..7].to_string()
                } else {
                    refspec.to_string()
                }
            }
            Some(ref val) => val.to_string(),
        };
        println!("Compiling zq2 version {name}");
        let cache_dir: PathBuf = root_path.clone().join("cache");
        let zq2_checkout_dir: PathBuf = cache_dir.clone().join("zq2");
        let id_prefix = format!("versions/{name}/");
        let target_dir = root_path
            .clone()
            .join("zq2")
            .join("docs")
            .join("versions")
            .join(&name);
        let target_dir_str = target_dir
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("unprintable path"))?
            .to_string();
        if checkout {
            // Check out the zq2 version
            println!("  Check out zq2 into {zq2_checkout_dir:?}");
            // Does it exist?
            // Use https so that those (me!) with yubikeys don't need to keep touching them.
            if fs::metadata(zq2_checkout_dir.clone().join(".git")).is_ok() {
                // Update.
                CommandBuilder::new()
                    .cmd("git", &["fetch", "https://github.com/zilliqa/zq2", refspec])
                    .current_dir(&zq2_checkout_dir.clone())?
                    .run()
                    .await?
                    .success_or("Cannot run git fetch")?
            } else {
                // Clone
                CommandBuilder::new()
                    .cmd("git", &["clone", "https://github.com/zilliqa/zq2"])
                    .current_dir(&cache_dir.clone())?
                    .run()
                    .await?
                    .success_or("Cannot run git clone")?
            };
            // Check out
            CommandBuilder::new()
                .cmd("git", &["checkout", refspec])
                .current_dir(&zq2_checkout_dir.clone())?
                .run()
                .await?
                .success_or("Cannot run git checkout")?;
        }
        // First, zap the target
        println!(" Removing {target_dir:?} ... ");
        if fs::metadata(&target_dir).is_ok() {
            fs::remove_dir_all(&target_dir)?;
        }

        let index_file_path = root_path.clone().join("zq2").join("mkdocs.yaml");
        let index_file_name = index_file_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("unprintable index file path"))?
            .to_string();
        let key_prefix = format!("nav");
        println!(" Generating documentation from {refspec} into {target_dir_str}...");
        let z2_dir = zq2_checkout_dir.clone();
        println!(" Running {z2_dir:?}/z2 .. ");
        // Now we can run the docgen
        CommandBuilder::new()
            .cmd(
                "scripts/z2",
                &[
                    "doc-gen",
                    &target_dir_str,
                    "--id-prefix",
                    &id_prefix,
                    "--index-file",
                    &index_file_name,
                    "--key-prefix",
                    &key_prefix,
                ],
            )
            .current_dir(&z2_dir.clone())?
            .run()
            .await?
            .success_or("Couldn't run z2")?;
    }
    Ok(())
}
