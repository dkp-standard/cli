use anyhow::{Context, Result};
use clap::Args;
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct SignArgs {
    /// Path to the built archive to sign
    pub archive: PathBuf,

    /// Path to Ed25519 private key (default: ~/.dkp/private.key)
    #[arg(long, value_name = "PATH")]
    pub key: Option<PathBuf>,

    /// Write signature to a custom path (default: <archive-dir>/bundle.sig)
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

pub async fn run(args: SignArgs, _cli: &CmdCtx) -> Result<()> {
    // Resolve private key path
    let key_path = match args.key {
        Some(p) => p,
        None => {
            let home = dirs::home_dir().context("could not determine home directory")?;
            home.join(".dkp").join("private.key")
        }
    };

    if !key_path.exists() {
        anyhow::bail!(
            "private key not found at {} — run `dkp keygen` first",
            key_path.display()
        );
    }

    // Load and decode private key (hex-encoded 32-byte seed)
    let key_hex =
        fs::read_to_string(&key_path).with_context(|| format!("reading {}", key_path.display()))?;
    let key_hex = key_hex.trim();
    let key_bytes = hex_to_bytes(key_hex)
        .with_context(|| format!("decoding private key at {}", key_path.display()))?;
    if key_bytes.len() != 32 {
        anyhow::bail!(
            "invalid private key: expected 32 bytes, got {}",
            key_bytes.len()
        );
    }
    let key_array: [u8; 32] = key_bytes.try_into().unwrap();
    let signing_key = SigningKey::from_bytes(&key_array);

    // Find checksums.json next to the archive
    let archive_dir = args
        .archive
        .parent()
        .context("archive path has no parent directory")?;
    let checksums_path = archive_dir.join("checksums.json");
    if !checksums_path.exists() {
        anyhow::bail!(
            "checksums.json not found at {} — run `dkp build` first",
            checksums_path.display()
        );
    }

    // The canonical payload is the checksums.json bytes
    // build.rs writes it via BTreeMap + serde_json::to_string_pretty, so it's deterministic
    let checksums_bytes = fs::read(&checksums_path)
        .with_context(|| format!("reading {}", checksums_path.display()))?;

    // Hash the payload then sign the digest
    let digest = Sha256::digest(&checksums_bytes);
    let signature = signing_key.sign(&digest);

    // Write bundle.sig (64 raw bytes)
    let sig_path = args.out.unwrap_or_else(|| archive_dir.join("bundle.sig"));
    fs::write(&sig_path, signature.to_bytes())?;

    println!("Wrote {}", sig_path.display());
    println!(
        "  Signed:   {}",
        args.archive
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    println!("  Key:      {}", key_path.display());
    println!(
        "  Pub key:  {}",
        key_path.with_file_name("public.key").display()
    );

    Ok(())
}

fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        anyhow::bail!("odd-length hex string");
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| anyhow::anyhow!("invalid hex byte at position {}: {}", i, e))
        })
        .collect()
}
