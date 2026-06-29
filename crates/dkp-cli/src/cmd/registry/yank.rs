use anyhow::{bail, Context, Result};
use clap::Args;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::load_credentials_or_fail;

#[derive(Args, Debug)]
pub struct YankArgs {
    /// Pack name and version, e.g. @example/nutrition-for-men@1.0.0
    pub name: String,

    /// Reason shown to consumers who attempt to install this version
    #[arg(long, value_name = "TEXT")]
    pub reason: String,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: YankArgs, cli: &CmdCtx) -> Result<()> {
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

    let (base, token) = if let Some(t) = args.token {
        (
            cli.config
                .registry
                .url
                .clone()
                .unwrap_or_else(|| "https://registry.dkp.directory".into()),
            t,
        )
    } else {
        load_credentials_or_fail(&cli.config.registry.url)?
    };

    let client = dkp_core::registry::RegistryClient::new(base, Some(token));
    client.yank(&pack_name, &version, &args.reason).await?;

    println!("{}@{} has been yanked.", pack_name, version);
    println!("Reason: {}", args.reason);
    println!("Note: this operation is irreversible.");

    Ok(())
}
