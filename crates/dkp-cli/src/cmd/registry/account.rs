use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::cli::CmdCtx;

const CREDENTIALS_FILE: &str = ".dkp/credentials";

#[derive(Args, Debug)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub command: RegistryCommands,
}

#[derive(Subcommand, Debug)]
pub enum RegistryCommands {
    /// Create a new publisher account and save the API key to ~/.dkp/credentials
    Register {
        #[arg(long, value_name = "EMAIL")]
        email: String,
    },
    /// Authenticate with an existing account and save API key to ~/.dkp/credentials
    Login {
        #[arg(long, value_name = "EMAIL")]
        email: String,
    },
    /// Remove saved credentials
    Logout,
    /// Rotate your API key
    Token {
        #[command(subcommand)]
        action: TokenAction,
    },
    /// Manage Ed25519 public keys registered with the registry
    Keys {
        #[command(subcommand)]
        action: KeysAction,
    },
    /// Pack-level management subcommands
    Pack {
        #[command(subcommand)]
        action: PackAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum TokenAction {
    Rotate,
}

#[derive(Subcommand, Debug)]
pub enum KeysAction {
    Add {
        #[arg(long, value_name = "PATH")]
        key: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum PackAction {
    /// List all published versions of a pack
    Versions { name: String },
    /// Set pack visibility (public or private)
    SetVisibility { name: String, visibility: String },
    /// Grant access to a private pack
    Grant {
        name: String,
        #[arg(long, value_name = "EMAIL")]
        to: String,
    },
    /// Revoke access to a private pack
    Revoke {
        name: String,
        #[arg(long, value_name = "EMAIL")]
        from: String,
    },
    /// List accounts with access to a private pack
    Access { name: String },
}

pub async fn run(args: RegistryArgs, cli: &CmdCtx) -> Result<()> {
    match args.command {
        RegistryCommands::Register { email } => {
            let base = resolve_registry_url(&cli.config.registry.url);
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/register"))
                .json(&serde_json::json!({ "email": email }))
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("registration failed: {body}");
            }
            await_verification(&base, &email, &http).await?;
        }

        RegistryCommands::Login { email } => {
            let base = resolve_registry_url(&cli.config.registry.url);
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/login"))
                .json(&serde_json::json!({ "email": email }))
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("login failed: {body}");
            }
            await_verification(&base, &email, &http).await?;
        }

        RegistryCommands::Logout => {
            let path = credentials_path()?;
            if path.exists() {
                std::fs::remove_file(&path)?;
                println!("Logged out (credentials removed).");
            } else {
                println!("No credentials found.");
            }
        }

        RegistryCommands::Token {
            action: TokenAction::Rotate,
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/token/rotate"))
                .bearer_auth(&token)
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("token rotation failed: {body}");
            }
            let data: serde_json::Value = resp.json().await?;
            let key = data["api_key"].as_str().context("no api_key in response")?;
            save_credentials(&base, key)?;
            println!("API key rotated and saved.");
        }

        RegistryCommands::Keys {
            action: KeysAction::Add { key },
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let key_b64 = load_public_key(&key)?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/signing-key"))
                .bearer_auth(&token)
                .json(&serde_json::json!({ "public_key": key_b64 }))
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("key registration failed: {body}");
            }
            println!("Ed25519 public key registered with the registry.");
        }

        RegistryCommands::Pack { action } => run_pack_action(action, cli).await?,
    }
    Ok(())
}

async fn run_pack_action(action: PackAction, cli: &CmdCtx) -> Result<()> {
    match action {
        PackAction::Versions { name } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let client = dkp_core::registry::RegistryClient::new(base, Some(token));
            let resp = client.list_versions(&name).await?;
            println!("Versions for {}:", resp.name);
            for v in &resp.versions {
                let yank = if v.yanked {
                    format!(" [YANKED: {}]", v.yank_reason.as_deref().unwrap_or(""))
                } else {
                    String::new()
                };
                println!(
                    "  {} ({})  {}{}",
                    v.version, v.conformance, v.published_at, yank
                );
            }
        }

        PackAction::SetVisibility { name, visibility } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let http = reqwest::Client::new();
            let resp = http
                .patch(format!("{base}/api/v1/packages/{name}/visibility"))
                .bearer_auth(&token)
                .json(&serde_json::json!({ "visibility": visibility }))
                .send()
                .await?;
            if !resp.status().is_success() {
                bail!(
                    "set-visibility failed: {}",
                    resp.text().await.unwrap_or_default()
                );
            }
            println!("{name} visibility set to {visibility}.");
        }

        PackAction::Grant { name, to } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/packages/{name}/access"))
                .bearer_auth(&token)
                .json(&serde_json::json!({ "email": to }))
                .send()
                .await?;
            if !resp.status().is_success() {
                bail!("grant failed: {}", resp.text().await.unwrap_or_default());
            }
            println!("Access granted to {to} for {name}.");
        }

        PackAction::Revoke { name, from } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let http = reqwest::Client::new();
            let resp = http
                .delete(format!("{base}/api/v1/packages/{name}/access/{from}"))
                .bearer_auth(&token)
                .send()
                .await?;
            if !resp.status().is_success() {
                bail!("revoke failed: {}", resp.text().await.unwrap_or_default());
            }
            println!("Access revoked from {from} for {name}.");
        }

        PackAction::Access { name } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url)?;
            let http = reqwest::Client::new();
            let resp = http
                .get(format!("{base}/api/v1/packages/{name}/access"))
                .bearer_auth(&token)
                .send()
                .await?;
            if !resp.status().is_success() {
                bail!(
                    "access list failed: {}",
                    resp.text().await.unwrap_or_default()
                );
            }
            let data: serde_json::Value = resp.json().await?;
            if let Some(entries) = data.as_array() {
                if entries.is_empty() {
                    println!("No additional accounts have access to {name}.");
                } else {
                    println!("Accounts with access to {name}:");
                    for e in entries {
                        println!(
                            "  {} (granted {})",
                            e["email"].as_str().unwrap_or("?"),
                            e["granted_at"].as_str().unwrap_or("?")
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

// --- Pending session polling ---

async fn await_verification(base: &str, email: &str, http: &reqwest::Client) -> Result<()> {
    // Create a pending session so the registry can deliver the api_key to us
    let resp = http
        .post(format!("{base}/api/v1/account/session"))
        .json(&serde_json::json!({ "email": email }))
        .send()
        .await
        .context("failed to create pending session")?;
    if !resp.status().is_success() {
        // Non-fatal: fall back to instructions-only mode
        println!("Check your email and click the verification link to complete sign-in.");
        return Ok(());
    }
    let data: serde_json::Value = resp.json().await?;
    let session_id = data["session_id"]
        .as_str()
        .context("no session_id in response")?
        .to_owned();

    println!("Check your email and click the verification link. Waiting...");

    // Poll every 3 seconds for up to 5 minutes (100 attempts)
    for _ in 0..100 {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let poll = http
            .get(format!("{base}/api/v1/account/session/{session_id}"))
            .send()
            .await;

        let Ok(poll_resp) = poll else { continue };

        if poll_resp.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("Session expired. Run 'dkp registry login' again.");
        }

        if !poll_resp.status().is_success() {
            continue;
        }

        let poll_data: serde_json::Value = poll_resp.json().await.unwrap_or_default();
        if poll_data["status"] == "complete" {
            let api_key = poll_data["api_key"]
                .as_str()
                .context("no api_key in session response")?;
            save_credentials(base, api_key)?;
            println!("Logged in successfully. Credentials saved to ~/.dkp/credentials");
            return Ok(());
        }
        // status == "pending" — keep waiting, print a dot
        print!(".");
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }

    bail!(
        "Timed out waiting for verification. Run 'dkp registry login' again once you've clicked the link."
    );
}

// --- Credential storage helpers ---

fn credentials_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(CREDENTIALS_FILE))
}

fn save_credentials(registry_url: &str, api_key: &str) -> Result<()> {
    let path = credentials_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = format!("registry={registry_url}\ntoken={api_key}\n");
    std::fs::write(&path, contents)?;
    Ok(())
}

fn load_credentials(registry_url_override: &Option<String>) -> Result<Option<(String, String)>> {
    let path = credentials_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&path)?;
    let mut registry = String::new();
    let mut token = String::new();
    for line in contents.lines() {
        if let Some(v) = line.strip_prefix("registry=") {
            registry = v.to_owned();
        } else if let Some(v) = line.strip_prefix("token=") {
            token = v.to_owned();
        }
    }
    let base = registry_url_override
        .clone()
        .or({
            if registry.is_empty() {
                None
            } else {
                Some(registry)
            }
        })
        .unwrap_or_else(|| "https://registry.dkp.directory".into());
    if token.is_empty() {
        return Ok(None);
    }
    Ok(Some((base, token)))
}

pub fn load_credentials_or_fail(config_url: &Option<String>) -> Result<(String, String)> {
    load_credentials(config_url)?
        .ok_or_else(|| anyhow::anyhow!("not logged in — run 'dkp registry login --email <email>'"))
}

/// Convenience for other registry commands: load (base, token) from saved creds.
pub fn load_credentials_from_ctx(config_url: &Option<String>) -> Result<Option<(String, String)>> {
    load_credentials(config_url)
}

pub fn resolve_registry_url(config_url: &Option<String>) -> String {
    config_url
        .clone()
        .unwrap_or_else(|| "https://registry.dkp.directory".into())
}

fn load_public_key(path: &PathBuf) -> Result<String> {
    use base64::Engine;
    let bytes =
        std::fs::read(path).with_context(|| format!("reading key from {}", path.display()))?;
    if bytes.len() == 32 {
        return Ok(base64::engine::general_purpose::STANDARD.encode(&bytes));
    }
    let text = String::from_utf8(bytes).context("key file is not UTF-8")?;
    let text = text.trim();
    if text.len() == 64 && text.chars().all(|c| c.is_ascii_hexdigit()) {
        let raw = hex::decode(text).context("invalid hex in key file")?;
        return Ok(base64::engine::general_purpose::STANDARD.encode(&raw));
    }
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(text)
        .context("key file is not hex, raw bytes, or base64")?;
    if decoded.len() != 32 {
        bail!("Ed25519 public key must be 32 bytes; got {}", decoded.len());
    }
    Ok(text.to_owned())
}
