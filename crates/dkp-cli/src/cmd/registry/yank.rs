use anyhow::{bail, Context, Result};
use clap::Args;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::{load_credentials_from_ctx, resolve_registry_url};

#[derive(Args, Debug)]
pub struct YankArgs {
    /// Pack name and version, e.g. @example/nutrition-for-men@1.0.0
    pub name: String,

    /// Reason shown to consumers who attempt to install this version
    #[arg(long, value_name = "TEXT")]
    pub reason: String,

    /// Override registry URL
    #[arg(long, value_name = "URL")]
    pub registry: Option<String>,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: YankArgs, cli: &CmdCtx) -> Result<()> {
    // Parse name@version
    let pos = args
        .name
        .rfind('@')
        .filter(|&p| p > 0)
        .context("specify pack as @scope/name@version")?;
    let pack_name = args.name[..pos].to_owned();
    let version = args.name[pos + 1..].to_owned();

    if version.is_empty() {
        bail!("version required — use @scope/name@version");
    }

    let base = resolve_registry_url(&cli.config.registry.url, &args.registry);
    let token = args
        .token
        .or_else(|| {
            load_credentials_from_ctx(&cli.config.registry.url, &args.registry)
                .ok()?
                .map(|(_, t)| t)
        })
        .context("no registry token — run 'dkp registry login' or set DKP_REGISTRY_TOKEN")?;

    let client = dkp_core::registry::RegistryClient::new(base, Some(token));
    client.yank(&pack_name, &version, &args.reason).await?;

    println!("{}@{} has been yanked.", pack_name, version);
    println!("Reason: {}", args.reason);
    println!("Note: this operation is irreversible.");

    Ok(())
}
