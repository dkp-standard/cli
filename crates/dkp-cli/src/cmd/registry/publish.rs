use anyhow::{bail, Context, Result};
use clap::Args;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::{load_credentials_from_ctx, resolve_registry_url};

#[derive(Args, Debug)]
pub struct PublishArgs {
    /// HTTPS URL to the hosted archive (publisher-controlled storage)
    #[arg(long, value_name = "URL")]
    pub url: String,

    /// Set pack visibility to private
    #[arg(long)]
    pub private: bool,

    /// Override registry URL
    #[arg(long, value_name = "URL")]
    pub registry: Option<String>,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: PublishArgs, cli: &CmdCtx) -> Result<()> {
    let pack = dkp_core::Pack::open(&std::env::current_dir()?)
        .context("failed to open pack in current directory")?;
    let manifest = pack.manifest.clone();

    let checksums_path = pack.root.join("checksums.json");
    if !checksums_path.exists() {
        bail!("checksums.json not found — run 'dkp build' first");
    }
    let checksums: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&checksums_path)?)
            .context("failed to parse checksums.json")?;

    let sig_path = pack.root.join("bundle.sig");
    if !sig_path.exists() {
        bail!("bundle.sig not found — run 'dkp sign --key <path>' first");
    }
    let bundle_sig = std::fs::read_to_string(&sig_path)?.trim().to_owned();

    let visibility = if args.private { "private" } else { "public" }.to_string();

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

    println!("Publishing {}@{} ...", manifest.name, manifest.version);

    let resp = client
        .publish(dkp_core::registry::types::PublishRequest {
            tarball_url: args.url,
            manifest,
            checksums,
            bundle_sig,
            visibility,
        })
        .await
        .context("publish failed")?;

    println!(
        "Published {}@{} ({})",
        resp.name, resp.version, resp.conformance
    );
    println!("  Gate 4: {:?}", resp.validation_report.gate_4);
    println!("  Gate 7: {:?}", resp.validation_report.gate_7);
    println!("  Gate 8: {:?}", resp.validation_report.gate_8);
    if resp.validation_report.reviewed_badge {
        println!("  [DKP-Reviewed] badge awarded");
    }

    Ok(())
}
