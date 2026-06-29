use anyhow::{Context, Result};
use clap::Args;
use sha2::Digest;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::{load_credentials_from_ctx, resolve_registry_url};
use dkp_core::registry::types::LockFile;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Update only this pack (omit to update all packs in dkp.lock)
    pub name: Option<String>,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: UpdateArgs, cli: &CmdCtx) -> Result<()> {
    let lock_path = std::env::current_dir()?.join("dkp.lock");
    if !lock_path.exists() {
        println!("No dkp.lock found. Run 'dkp install <pack>' first.");
        return Ok(());
    }

    let mut lock: LockFile = serde_json::from_str(&std::fs::read_to_string(&lock_path)?)?;

    let base = resolve_registry_url(&cli.config.registry.url);
    let token = args.token.or_else(|| {
        load_credentials_from_ctx(&cli.config.registry.url)
            .ok()?
            .map(|(_, t)| t)
    });
    let client = dkp_core::registry::RegistryClient::new(base, token);

    let packs_to_update: Vec<String> = match &args.name {
        Some(n) => vec![n.clone()],
        None => lock.resolved.keys().cloned().collect(),
    };

    let mut updated = 0;
    for pack_name in &packs_to_update {
        let locked = match lock.resolved.get(pack_name) {
            Some(l) => l.clone(),
            None => {
                eprintln!("Warning: {pack_name} not in dkp.lock — skipping");
                continue;
            }
        };

        let versions = client
            .list_versions(pack_name)
            .await
            .with_context(|| format!("failed to fetch versions for {pack_name}"))?;

        let latest = match &versions.latest {
            Some(v) => v.clone(),
            None => {
                println!("{pack_name}: no non-yanked version available");
                continue;
            }
        };

        if latest == locked.version {
            println!("{pack_name}: already at latest ({latest})");
            continue;
        }

        println!("{pack_name}: {} -> {}", locked.version, latest);

        let meta = client.resolve(pack_name, &latest).await?;

        let integrity = format!(
            "sha256-{}",
            hex::encode(sha2::Sha256::digest(meta.checksums.to_string().as_bytes()))
        );

        lock.resolved.insert(
            pack_name.clone(),
            dkp_core::registry::types::LockedPack {
                version: latest,
                archive_format: meta.archive_format,
                integrity,
            },
        );
        updated += 1;
    }

    if updated > 0 {
        std::fs::write(&lock_path, serde_json::to_string_pretty(&lock)?)?;
        println!("\ndkp.lock updated ({updated} pack(s) updated). Run 'dkp install' to apply.");
    } else {
        println!("All packs are up to date.");
    }

    Ok(())
}
