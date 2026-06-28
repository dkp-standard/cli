use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;

use crate::cli::CmdCtx;
use dkp_core::registry::types::LockFile;

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Pack name, e.g. @example/nutrition-for-men or @example/pack@1.2.0
    pub name: String,

    /// Remove from global store
    #[arg(long, short = 'g')]
    pub global: bool,

    /// Remove from a custom directory
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,

    /// Remove all installed versions
    #[arg(long)]
    pub all_versions: bool,
}

pub async fn run(args: UninstallArgs, cli: &CmdCtx) -> Result<()> {
    let (pack_name, version) = parse_pack_arg(&args.name);

    let base_dir = resolve_base_dir(&args, cli)?;

    if args.all_versions {
        let pack_dir = base_dir.join(&pack_name);
        if !pack_dir.exists() {
            anyhow::bail!(
                "pack '{}' is not installed in {}",
                pack_name,
                base_dir.display()
            );
        }
        std::fs::remove_dir_all(&pack_dir)
            .with_context(|| format!("failed to remove {}", pack_dir.display()))?;
        println!(
            "Removed all versions of '{}' from {}",
            pack_name,
            pack_dir.display()
        );
    } else {
        let install_dir = base_dir.join(&pack_name).join(&version);
        if !install_dir.exists() {
            anyhow::bail!(
                "pack '{}@{}' is not installed in {}",
                pack_name,
                version,
                base_dir.display()
            );
        }
        std::fs::remove_dir_all(&install_dir)
            .with_context(|| format!("failed to remove {}", install_dir.display()))?;
        println!(
            "Removed '{}@{}' from {}",
            pack_name,
            version,
            install_dir.display()
        );

        // Clean up empty parent directory
        let pack_dir = base_dir.join(&pack_name);
        if pack_dir
            .read_dir()
            .map(|mut d| d.next().is_none())
            .unwrap_or(false)
        {
            let _ = std::fs::remove_dir(&pack_dir);
        }
    }

    remove_from_lock(&pack_name)?;

    Ok(())
}

fn parse_pack_arg(arg: &str) -> (String, String) {
    if let Some(pos) = arg.rfind('@').filter(|&p| p > 0) {
        (arg[..pos].to_owned(), arg[pos + 1..].to_owned())
    } else {
        (arg.to_owned(), "latest".to_owned())
    }
}

fn resolve_base_dir(args: &UninstallArgs, cli: &CmdCtx) -> Result<PathBuf> {
    if let Some(out) = &args.out {
        return Ok(out.clone());
    }
    if args.global {
        return cli
            .config
            .install
            .global_dir
            .as_deref()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".dkp").join("packs")))
            .context("cannot determine global install dir");
    }
    let local_name = cli.config.install.local_dir.as_deref().unwrap_or("dkps");
    Ok(std::env::current_dir()?.join(local_name))
}

fn remove_from_lock(pack_name: &str) -> Result<()> {
    let lock_path = std::env::current_dir()?.join("dkp.lock");
    if !lock_path.exists() {
        return Ok(());
    }
    let mut lock: LockFile =
        serde_json::from_str(&std::fs::read_to_string(&lock_path)?).context("invalid dkp.lock")?;
    if lock.resolved.remove(pack_name).is_some() {
        std::fs::write(&lock_path, serde_json::to_string_pretty(&lock)?)?;
    }
    Ok(())
}
