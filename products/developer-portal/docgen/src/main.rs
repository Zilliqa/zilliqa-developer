use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs, process::Command};

#[derive(Clone, Deserialize)]
struct Zq2Spec {
    versions: Vec<String>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let here = String::from(&args[1]);

    // Find the zq2 versions that we need to collect.

    println!("here = {here}");

    let root_path = PathBuf::from(&here);
    let versions: Zq2Spec =
        serde_yaml::from_str(&fs::read_to_string(format!("{}/zq2_spec.yaml", here))?)?;
    for v in &versions.versions {
        println!("Compiling zq2 version {v}");
        // Check out the zq2 version
        let cache_dir: PathBuf = root_path.clone().join("cache");
        let zq2_checkout_dir: PathBuf = cache_dir.clone().join("zq2");
        let id_prefix = format!("versions/{v}/");
        let target_dir = root_path
            .clone()
            .join("zq2")
            .join("docs")
            .join("versions")
            .join(v);
        let target_dir_str = target_dir
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("unprintable path"))?
            .to_string();

        println!("  Check out zq2 into {zq2_checkout_dir:?}");
        // Does it exist?
        // Use https so that those (me!) with yubikeys don't need to keep touching them.
        let mut ok = if fs::metadata(zq2_checkout_dir.clone().join(".git")).is_ok() {
            // Update.
            Command::new("git")
                .args(["fetch", "https://github.com/zilliqa/zq2"])
                .current_dir(zq2_checkout_dir.clone())
                .status()?
                .success()
        } else {
            // Clone
            Command::new("git")
                .args(["clone", "https://github.com/zilliqa/zq2"])
                .current_dir(cache_dir.clone())
                .status()?
                .success()
        };
        if !ok {
            return Err(anyhow!("Couldn't update zq2 repository"));
        }
        // Check out
        ok = Command::new("git")
            .args(["checkout", v])
            .current_dir(zq2_checkout_dir.clone())
            .status()?
            .success();
        if !ok {
            return Err(anyhow!("Couldn't check out {v} in {cache_dir:?}"));
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
        let key_prefix = format!("nav/{v}/api");
        println!(" Generating documentation from {v} into {target_dir_str}...");
        let z2_dir = zq2_checkout_dir.clone();
        println!(" Running {z2_dir:?}/z2 .. ");
        // Now we can run the docgen
        ok = Command::new("scripts/z2")
            .args([
                "doc-gen",
                &target_dir_str,
                "--id-prefix",
                &id_prefix,
                "--index-file",
                &index_file_name,
                "--key-prefix",
                &key_prefix,
            ])
            .current_dir(z2_dir.clone())
            .status()?
            .success();
        if !ok {
            return Err(anyhow!("couldn't generate documentation"));
        }
    }
    Ok(())
}
