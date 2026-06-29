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
    /// Pack name, e.g. @example/nutrition-for-men or @example/pack@1.2.0.
    /// Omit to install all packs from dkp.lock.
    pub name: Option<String>,

    /// Install directly from a URL without involving the registry.
    /// Pass --checksums and/or --sig to verify integrity.
    #[arg(long, value_name = "URL")]
    pub url: Option<String>,

    /// Path to a checksums.json for verifying a --url install
    #[arg(long, value_name = "PATH", requires = "url")]
    pub checksums: Option<PathBuf>,

    /// Path to a bundle.sig for verifying a --url install
    #[arg(long, value_name = "PATH", requires = "url")]
    pub sig: Option<PathBuf>,

    /// Publisher Ed25519 public key for verifying a --url install (hex, base64, or raw 32-byte file)
    #[arg(long, value_name = "PATH", requires = "sig")]
    pub pubkey: Option<PathBuf>,

    /// Install to global store (~/.dkp/packs/)
    #[arg(long, short = 'g')]
    pub global: bool,

    /// Install to a custom directory
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,

    /// Registry API token
    #[arg(long, value_name = "KEY", env = "DKP_TOKEN")]
    pub token: Option<String>,

    /// Skip signature verification for registry installs (NOT RECOMMENDED)
    #[arg(long)]
    pub no_verify: bool,
}

pub async fn run(args: InstallArgs, cli: &CmdCtx) -> Result<()> {
    // --- Direct URL install (no registry) ---
    if let Some(ref url) = args.url {
        return install_from_url(url, &args, cli).await;
    }

    let base = resolve_registry_url(&cli.config.registry.url);
    let token = args.token.clone().or_else(|| {
        load_credentials_from_ctx(&cli.config.registry.url)
            .ok()?
            .map(|(_, t)| t)
    });
    let client = dkp_core::registry::RegistryClient::new(base, token);

    match &args.name {
        Some(name) => install_one(name, &args, cli, &client).await,
        None => install_from_lock(&args, cli, &client).await,
    }
}

async fn install_one(
    arg: &str,
    args: &InstallArgs,
    cli: &CmdCtx,
    client: &dkp_core::registry::RegistryClient,
) -> Result<()> {
    let (pack_name, version) = parse_pack_arg(arg);

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

    let install_dir = resolve_install_dir(args, cli, &meta.name, &meta.version)?;
    if install_dir.exists() {
        println!("Already installed at {}", install_dir.display());
        return Ok(());
    }

    // Fetch the CDN download URL from the registry
    let dl = client.get_download_url(&meta.name, &meta.version).await?;

    println!("Downloading from {} ...", dl.url);
    let http = reqwest::Client::new();
    let resp = http
        .get(&dl.url)
        .send()
        .await
        .context("failed to download pack archive")?;
    if !resp.status().is_success() {
        bail!("download failed: {}", resp.status());
    }
    let archive_bytes = resp.bytes().await.context("failed to read archive body")?;

    if !args.no_verify {
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

    let tmp = tempfile::tempdir().context("failed to create temp dir")?;
    extract_archive(&archive_bytes, &meta.archive_format, tmp.path())?;
    let extracted_root = find_pack_root(tmp.path());

    std::fs::create_dir_all(&install_dir)
        .with_context(|| format!("creating install dir {}", install_dir.display()))?;
    for entry in std::fs::read_dir(&extracted_root)? {
        let entry = entry?;
        std::fs::rename(entry.path(), install_dir.join(entry.file_name()))?;
    }

    println!("Installed to {}", install_dir.display());

    let integrity = format!(
        "sha256-{}",
        hex::encode(Sha256::digest(meta.checksums.to_string().as_bytes()))
    );
    update_lock_file(&meta.name, &meta.version, &meta.archive_format, &integrity)?;

    Ok(())
}

async fn install_from_lock(
    args: &InstallArgs,
    cli: &CmdCtx,
    client: &dkp_core::registry::RegistryClient,
) -> Result<()> {
    let lock_path = std::env::current_dir()?.join("dkp.lock");
    if !lock_path.exists() {
        println!("No dkp.lock found. Specify a pack name to install.");
        return Ok(());
    }
    let lock: LockFile = serde_json::from_str(&std::fs::read_to_string(&lock_path)?)?;
    for (name, locked) in &lock.resolved {
        let install_dir = resolve_install_dir(args, cli, name, &locked.version)?;
        if install_dir.exists() {
            println!("{name}@{} already installed", locked.version);
            continue;
        }
        // Re-use install_one logic
        install_one(&format!("{name}@{}", locked.version), args, cli, client).await?;
    }
    Ok(())
}

// --- Direct URL install ---

async fn install_from_url(url: &str, args: &InstallArgs, cli: &CmdCtx) -> Result<()> {
    println!("Downloading from {url} ...");
    let http = reqwest::Client::new();
    let resp = http
        .get(url)
        .send()
        .await
        .context("failed to download archive")?;
    if !resp.status().is_success() {
        bail!("download failed: {}", resp.status());
    }
    let archive_bytes = resp.bytes().await.context("failed to read archive body")?;

    // Detect format from bytes
    let archive_format = detect_format_from_bytes(&archive_bytes);

    // Optional checksums verification
    let mut verified_checksums = false;
    if let Some(ref checksums_path) = args.checksums {
        let expected: HashMap<String, String> =
            serde_json::from_str(&std::fs::read_to_string(checksums_path)?)
                .context("failed to parse checksums file")?;
        let actual = hash_archive(&archive_bytes, &archive_format)?;
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
        verified_checksums = true;
        println!("Checksums verified.");

        // Optional signature verification (requires --sig and --pubkey)
        if let (Some(ref sig_path), Some(ref pubkey_path)) = (&args.sig, &args.pubkey) {
            let sig_bytes = std::fs::read(sig_path).context("failed to read .sig file")?;
            let key_bytes = load_public_key(pubkey_path)?;
            verify_signature(&sig_bytes, &key_bytes, &expected)?;
            println!("Signature verified.");
        } else if args.sig.is_some() {
            eprintln!("Warning: --sig provided without --pubkey; skipping signature verification.");
        }
    } else {
        eprintln!(
            "Warning: installing without integrity verification. \
             Pass --checksums <path> to verify this archive."
        );
    }

    // Determine pack name and version from archive filename or prompt
    let filename = url.rsplit('/').next().unwrap_or("unknown.tar.gz");
    let (pack_name, version) = parse_dkp_filename(filename);

    let install_dir = resolve_install_dir(args, cli, &pack_name, &version)?;

    let tmp = tempfile::tempdir().context("failed to create temp dir")?;
    extract_archive(&archive_bytes, &archive_format, tmp.path())?;
    let extracted_root = find_pack_root(tmp.path());

    std::fs::create_dir_all(&install_dir)
        .with_context(|| format!("creating install dir {}", install_dir.display()))?;
    for entry in std::fs::read_dir(&extracted_root)? {
        let entry = entry?;
        std::fs::rename(entry.path(), install_dir.join(entry.file_name()))?;
    }

    println!("Installed to {}", install_dir.display());

    if verified_checksums {
        // We don't have a registry checksums JSON value; use a placeholder integrity
        let integrity = format!("sha256-{}", hex::encode(Sha256::digest(&archive_bytes)));
        update_lock_file(&pack_name, &version, &archive_format, &integrity)?;
    }

    Ok(())
}

fn parse_dkp_filename(name: &str) -> (String, String) {
    // Expected: "{name}-{version}.tar.gz" or "{name}-{version}.zip"
    let base = name
        .strip_suffix(".tar.gz")
        .or_else(|| name.strip_suffix(".zip"))
        .unwrap_or(name);
    if let Some(pos) = base.rfind('-') {
        let maybe_version = &base[pos + 1..];
        if maybe_version
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            return (base[..pos].to_owned(), maybe_version.to_owned());
        }
    }
    (base.to_owned(), "unknown".to_owned())
}

// --- Shared helpers ---

fn parse_pack_arg(arg: &str) -> (String, String) {
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

fn detect_format_from_bytes(bytes: &[u8]) -> String {
    if bytes.starts_with(b"PK\x03\x04") {
        return "zip".into();
    }
    if bytes.starts_with(&[0x1f, 0x8b]) {
        return "tar.gz".into();
    }
    "tar.gz".into()
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

fn load_public_key(path: &PathBuf) -> Result<Vec<u8>> {
    use base64::Engine;
    let bytes =
        std::fs::read(path).with_context(|| format!("reading key from {}", path.display()))?;
    if bytes.len() == 32 {
        return Ok(bytes);
    }
    let text = String::from_utf8(bytes).context("key file is not UTF-8")?;
    let text = text.trim();
    if text.len() == 64 && text.chars().all(|c| c.is_ascii_hexdigit()) {
        return hex::decode(text).context("invalid hex in key file");
    }
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(text)
        .context("key file is not hex, raw bytes, or base64")?;
    if decoded.len() != 32 {
        bail!("Ed25519 public key must be 32 bytes; got {}", decoded.len());
    }
    Ok(decoded)
}

fn update_lock_file(
    name: &str,
    version: &str,
    archive_format: &str,
    integrity: &str,
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

    lock.resolved.insert(
        name.to_owned(),
        LockedPack {
            version: version.to_owned(),
            archive_format: archive_format.to_owned(),
            integrity: integrity.to_owned(),
        },
    );

    std::fs::write(&lock_path, serde_json::to_string_pretty(&lock)?)?;
    Ok(())
}
