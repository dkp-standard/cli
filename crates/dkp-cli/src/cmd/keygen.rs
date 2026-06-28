use anyhow::{Context, Result};
use clap::Args;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::{fs, path::PathBuf};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct KeygenArgs {
    /// Write keys to a custom directory (default: ~/.dkp/)
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,

    /// Overwrite existing keys without prompting
    #[arg(long)]
    pub force: bool,
}

pub async fn run(args: KeygenArgs, _cli: &CmdCtx) -> Result<()> {
    let out_dir = match args.out {
        Some(d) => d,
        None => {
            let home = dirs::home_dir().context("could not determine home directory")?;
            home.join(".dkp")
        }
    };

    fs::create_dir_all(&out_dir)?;

    let priv_path = out_dir.join("private.key");
    let pub_path = out_dir.join("public.key");

    if priv_path.exists() && !args.force {
        anyhow::bail!(
            "{} already exists — use --force to overwrite",
            priv_path.display()
        );
    }

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let priv_hex = bytes_to_hex(&signing_key.to_bytes());
    let pub_hex = bytes_to_hex(&verifying_key.to_bytes());

    fs::write(&priv_path, &priv_hex)?;
    fs::write(&pub_path, &pub_hex)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&priv_path, fs::Permissions::from_mode(0o600))?;
    }

    println!("Generated {}", priv_path.display());
    println!("Generated {}", pub_path.display());
    println!();
    println!(
        "Register your public key with: dkp registry keys add --key {}",
        pub_path.display()
    );

    Ok(())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
