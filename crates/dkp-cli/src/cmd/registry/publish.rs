use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use base64::Engine;
use clap::Args;

use crate::cli::CmdCtx;
use crate::cmd::registry::account::load_credentials_or_fail;

#[derive(Args, Debug)]
pub struct PublishArgs {
    /// Path to the pack directory (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Directory containing the archive (.tar.gz or .zip), checksums.json, and bundle.sig (default: <pack>/build/)
    #[arg(long, value_name = "DIR")]
    pub build_dir: Option<PathBuf>,

    /// Set pack visibility to private
    #[arg(long)]
    pub private: bool,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_REGISTRY_TOKEN")]
    pub token: Option<String>,
}

pub async fn run(args: PublishArgs, cli: &CmdCtx) -> Result<()> {
    let pack_dir = match args.path {
        Some(ref p) => p.clone(),
        None => std::env::current_dir()?,
    };
    let pack = dkp_core::Pack::open(&pack_dir).context("failed to open pack")?;
    let manifest = pack.manifest.clone();

    let artifact_dir = args.build_dir.unwrap_or_else(|| pack.root.join("build"));

    let checksums_path = artifact_dir.join("checksums.json");
    if !checksums_path.exists() {
        bail!(
            "checksums.json not found in {} — run 'dkp build' first (or pass --build-dir)",
            artifact_dir.display()
        );
    }
    let checksums: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&checksums_path)?)
            .context("failed to parse checksums.json")?;

    let sig_path = artifact_dir.join("bundle.sig");
    if !sig_path.exists() {
        bail!(
            "bundle.sig not found in {} — run 'dkp sign' first (or pass --build-dir)",
            artifact_dir.display()
        );
    }
    let sig_raw = std::fs::read(&sig_path)?;
    let bundle_sig = base64::engine::general_purpose::STANDARD.encode(&sig_raw);

    // Locate the archive
    let archive_path = find_archive(&artifact_dir)?;
    let archive_bytes = std::fs::read(&archive_path)
        .with_context(|| format!("failed to read archive {}", archive_path.display()))?;
    let size_bytes = archive_bytes.len() as i64;

    let visibility = if args.private { "private" } else { "public" }.to_string();

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

    println!("Publishing {}@{} ...", manifest.name, manifest.version);

    // Step 1: POST metadata → receive presigned upload URL
    let resp = client
        .publish(dkp_core::registry::types::PublishRequest {
            manifest: manifest.clone(),
            checksums,
            bundle_sig,
            visibility,
            size_bytes,
        })
        .await
        .context("publish failed")?;

    println!("Uploading archive ({} bytes) ...", size_bytes);

    // Step 2: PUT archive directly to R2 via presigned URL
    let http = reqwest::Client::new();
    let upload_resp = http
        .put(&resp.upload_url)
        .body(archive_bytes)
        .send()
        .await
        .context("failed to upload archive to storage")?;

    if !upload_resp.status().is_success() {
        bail!(
            "archive upload failed: {} {}",
            upload_resp.status(),
            upload_resp.text().await.unwrap_or_default()
        );
    }

    println!("Archive uploaded. Confirming ...");

    // Step 3: POST confirm → registry validates and marks the version live
    let confirm = client
        .confirm_publish(&resp.name, &resp.version)
        .await
        .context("confirm failed")?;

    println!(
        "Published {}@{} ({})",
        confirm.name, confirm.version, confirm.conformance
    );
    println!("  Size: {} bytes", confirm.size_bytes);
    println!("  Gate 4: {:?}", confirm.validation_report.gate_4);
    println!("  Gate 7: {:?}", confirm.validation_report.gate_7);
    println!("  Gate 8: {:?}", confirm.validation_report.gate_8);
    if confirm.validation_report.reviewed_badge {
        println!("  [DKP-Reviewed] badge awarded");
    }

    Ok(())
}

fn find_archive(dir: &std::path::Path) -> Result<PathBuf> {
    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("cannot read build dir {}", dir.display()))?;
    for entry in entries.flatten() {
        let p = entry.path();
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        if name.ends_with(".tar.gz") || name.ends_with(".zip") {
            return Ok(p);
        }
    }
    bail!(
        "no .tar.gz or .zip archive found in {} — run 'dkp build' first (or pass --build-dir)",
        dir.display()
    )
}
