use anyhow::{Context, Result, bail};
use clap::Args;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::load_credentials_or_fail;

#[derive(Args, Debug)]
pub struct DeprecateArgs {
    /// Pack name and version, e.g. @example/nutrition-for-men@1.0.0
    pub name: String,

    /// Message shown to consumers who install this version (required unless --undo)
    #[arg(long, value_name = "TEXT", required_unless_present = "undo")]
    pub message: Option<String>,

    /// Clear a previous deprecation
    #[arg(long)]
    pub undo: bool,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: DeprecateArgs, cli: &CmdCtx) -> Result<()> {
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
    let deprecated = !args.undo;
    client
        .deprecate(&pack_name, &version, deprecated, args.message.as_deref())
        .await?;

    if deprecated {
        println!("{}@{} has been marked as deprecated.", pack_name, version);
        println!("Message: {}", args.message.as_deref().unwrap_or(""));
    } else {
        println!("{}@{} is no longer deprecated.", pack_name, version);
    }

    Ok(())
}
