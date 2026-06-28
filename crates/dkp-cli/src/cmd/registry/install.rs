use anyhow::{bail, Context, Result};
use base64::Engine;
use clap::Args;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, io::Cursor, path::PathBuf};

use crate::cli::CmdCtx;
use crate::cmd::registry::account::{load_credentials_from_ctx, resolve_registry_url};
use dkp_core::registry::types::{LockFile, LockedPack};

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Pack name, e.g. @example/nutrition-for-men or @example/pack@1.2.0
    pub name: String,

    /// Install to global store (~/.dkp/packs/)
    #[arg(long, short = 'g')]
    pub global: bool,

    /// Install to a custom directory
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,

    /// Override registry URL
    #[arg(long, value_name = "URL")]
    pub registry: Option<String>,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_TOKEN")]
    pub token: Option<String>,

    /// Skip signature verification (NOT RECOMMENDED)
    #[arg(long)]
    pub no_verify: bool,
}

pub async fn run(args: InstallArgs, cli: &CmdCtx) -> Result<()> {
    // Parse name@version
    let (pack_name, version) = parse_pack_arg(&args.name);

    let base = resolve_registry_url(&cli.config.registry.url, &args.registry);
    let token = args.token.clone().or_else(|| {
        load_credentials_from_ctx(&cli.config.registry.url, &args.registry)
            .ok()?
            .map(|(_, t)| t)
    });

    let client = dkp_core::registry::RegistryClient::new(base, token);

    println!("Resolving {pack_name}@{version} ...");
    let meta = client.resolve(&pack_name, &version).await?;

    if meta.yanked {
        eprintln!(
            "Warning: {}@{} is yanked: {}",
            meta.name,
            meta.version,
            meta.yank_reason.as_deref().unwrap_or("no reason given")
        );
    }

    // Determine install directory
    let install_dir = resolve_install_dir(&args, cli, &meta.name, &meta.version)?;
    if install_dir.exists() {
        println!("Already installed at {}", install_dir.display());
        return Ok(());
    }

    // Download archive
    println!("Downloading {} ...", meta.tarball_url);
    let http = reqwest::Client::new();
    let resp = http
        .get(&meta.tarball_url)
        .send()
        .await
        .context("failed to download pack archive")?;
    if !resp.status().is_success() {
        bail!("download failed: {}", resp.status());
    }
    let archive_bytes = resp.bytes().await.context("failed to read archive body")?;

    if !args.no_verify {
        // Verify checksums
        let expected: HashMap<String, String> =
            serde_json::from_value(meta.checksums.clone()).context("invalid checksums")?;
        let actual = hash_archive(&archive_bytes, &meta.archive_format)?;
        for (path, expected_hash) in &expected {
            let actual_hash = actual
                .get(path)
                .with_context(|| format!("'{path}' in checksums not found in archive"))?;
            if actual_hash != expected_hash {
                bail!(
                    "checksum mismatch for '{path}': expected {expected_hash}, got {actual_hash}"
                );
            }
        }

        // Verify Ed25519 signature
        let sig_bytes = base64::engine::general_purpose::STANDARD
            .decode(&meta.bundle_sig)
            .context("invalid bundle_sig base64")?;
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&meta.publisher_public_key)
            .context("invalid publisher_public_key base64")?;
        verify_signature(&sig_bytes, &key_bytes, &expected)?;

        println!("Checksums and signature verified.");
    } else {
        eprintln!("Warning: skipping verification (--no-verify)");
    }

    // Extract to a temp dir then move into place
    let tmp = tempfile::tempdir().context("failed to create temp dir")?;
    extract_archive(&archive_bytes, &meta.archive_format, tmp.path())?;

    // Find the pack root inside the extracted tree
    let extracted_root = find_pack_root(tmp.path());

    std::fs::create_dir_all(&install_dir)
        .with_context(|| format!("creating install dir {}", install_dir.display()))?;

    // Move contents
    for entry in std::fs::read_dir(&extracted_root)? {
        let entry = entry?;
        let dest = install_dir.join(entry.file_name());
        std::fs::rename(entry.path(), dest)?;
    }

    println!("Installed to {}", install_dir.display());

    // Write / update dkp.lock
    update_lock_file(
        &meta.name,
        &meta.version,
        &meta.tarball_url,
        &meta.archive_format,
        &meta.checksums,
    )?;

    Ok(())
}

fn parse_pack_arg(arg: &str) -> (String, String) {
    // Split on the last '@' that is preceded by at least one non-@ character
    // e.g. "@example/pack@1.2.0" -> ("@example/pack", "1.2.0")
    //      "@example/pack" -> ("@example/pack", "latest")
    if let Some(pos) = arg.rfind('@').filter(|&p| p > 0) {
        (arg[..pos].to_owned(), arg[pos + 1..].to_owned())
    } else {
        (arg.to_owned(), "latest".to_owned())
    }
}

fn resolve_install_dir(
    args: &InstallArgs,
    cli: &CmdCtx,
    pack_name: &str,
    version: &str,
) -> Result<PathBuf> {
    if let Some(out) = &args.out {
        return Ok(out.join(pack_name).join(version));
    }
    if args.global {
        let global = cli
            .config
            .install
            .global_dir
            .as_deref()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".dkp").join("packs")))
            .context("cannot determine global install dir")?;
        return Ok(global.join(pack_name).join(version));
    }
    // Default: local dkps/ directory
    let local_name = cli.config.install.local_dir.as_deref().unwrap_or("dkps");
    Ok(std::env::current_dir()?
        .join(local_name)
        .join(pack_name)
        .join(version))
}

fn strip_top_component(path: &str) -> String {
    match path.find('/') {
        Some(i) => path[i + 1..].to_string(),
        None => String::new(),
    }
}

fn hash_archive(bytes: &[u8], format: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    match format {
        "zip" => {
            let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                if file.is_dir() {
                    continue;
                }
                let name = strip_top_component(file.name());
                if name.is_empty() {
                    continue;
                }
                let mut hasher = Sha256::new();
                std::io::copy(&mut file, &mut hasher)?;
                map.insert(name, hex::encode(hasher.finalize()));
            }
        }
        "tar.gz" => hash_tar(flate2::read::GzDecoder::new(Cursor::new(bytes)), &mut map)?,
        _ => bail!("unsupported archive format: {format}"),
    }
    Ok(map)
}

fn hash_tar<R: std::io::Read>(reader: R, map: &mut HashMap<String, String>) -> Result<()> {
    let mut archive = tar::Archive::new(reader);
    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.header().entry_type().is_dir() {
            continue;
        }
        let raw = entry.path()?.to_string_lossy().into_owned();
        let name = strip_top_component(&raw);
        if name.is_empty() {
            continue;
        }
        let mut hasher = Sha256::new();
        std::io::copy(&mut entry, &mut hasher)?;
        map.insert(name, hex::encode(hasher.finalize()));
    }
    Ok(())
}

fn verify_signature(
    sig_bytes: &[u8],
    key_bytes: &[u8],
    checksums: &HashMap<String, String>,
) -> Result<()> {
    let key_arr: [u8; 32] = key_bytes
        .try_into()
        .context("Ed25519 key must be 32 bytes")?;
    let key = VerifyingKey::from_bytes(&key_arr).context("invalid Ed25519 public key")?;
    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .context("Ed25519 signature must be 64 bytes")?;
    let sig = Signature::from_bytes(&sig_arr);
    let canonical = serde_json::to_string_pretty(
        &checksums
            .iter()
            .collect::<std::collections::BTreeMap<_, _>>(),
    )?;
    let digest = Sha256::digest(canonical.as_bytes());
    key.verify(&digest, &sig)
        .context("signature verification failed — pack may have been tampered with")?;
    Ok(())
}

fn extract_archive(bytes: &[u8], format: &str, dest: &std::path::Path) -> Result<()> {
    match format {
        "zip" => {
            zip::ZipArchive::new(Cursor::new(bytes))?.extract(dest)?;
        }
        "tar.gz" => {
            tar::Archive::new(flate2::read::GzDecoder::new(Cursor::new(bytes))).unpack(dest)?;
        }
        _ => bail!("unsupported format: {format}"),
    }
    Ok(())
}

fn find_pack_root(dir: &std::path::Path) -> PathBuf {
    if let Ok(entries) = std::fs::read_dir(dir) {
        let entries: Vec<_> = entries.flatten().collect();
        if entries.len() == 1 && entries[0].path().is_dir() {
            return entries[0].path();
        }
    }
    dir.to_path_buf()
}

fn update_lock_file(
    name: &str,
    version: &str,
    tarball_url: &str,
    archive_format: &str,
    checksums: &serde_json::Value,
) -> Result<()> {
    let lock_path = std::env::current_dir()?.join("dkp.lock");
    let mut lock: LockFile = if lock_path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&lock_path)?)?
    } else {
        LockFile {
            lockfile_version: 1,
            resolved: HashMap::new(),
        }
    };

    // Compute aggregate integrity string from checksums
    let integrity = format!(
        "sha256-{}",
        hex::encode(Sha256::digest(checksums.to_string().as_bytes()))
    );

    lock.resolved.insert(
        name.to_owned(),
        LockedPack {
            version: version.to_owned(),
            tarball_url: tarball_url.to_owned(),
            archive_format: archive_format.to_owned(),
            integrity,
        },
    );

    std::fs::write(&lock_path, serde_json::to_string_pretty(&lock)?)?;
    Ok(())
}
