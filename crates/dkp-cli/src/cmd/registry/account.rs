use anyhow::{bail, Context, Result};
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
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
    /// Authenticate with an existing account and save API key to ~/.dkp/credentials
    Login {
        #[arg(long, value_name = "EMAIL")]
        email: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
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
    Rotate {
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum KeysAction {
    Add {
        #[arg(long, value_name = "PATH")]
        key: PathBuf,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PackAction {
    /// List all published versions of a pack
    Versions {
        name: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
    /// Set pack visibility (public or private)
    SetVisibility {
        name: String,
        visibility: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
    /// Grant access to a private pack
    Grant {
        name: String,
        #[arg(long, value_name = "EMAIL")]
        to: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
    /// Revoke access to a private pack
    Revoke {
        name: String,
        #[arg(long, value_name = "EMAIL")]
        from: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
    /// List accounts with access to a private pack
    Access {
        name: String,
        #[arg(long, value_name = "URL")]
        registry: Option<String>,
    },
}

pub async fn run(args: RegistryArgs, cli: &CmdCtx) -> Result<()> {
    match args.command {
        RegistryCommands::Register { email, registry } => {
            let base = resolve_registry_url(&cli.config.registry.url, &registry);
            let password = prompt_password("Choose a password: ")?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/register"))
                .json(&serde_json::json!({ "email": email, "password": password }))
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("registration failed: {body}");
            }
            let data: serde_json::Value = resp.json().await?;
            let key = data["api_key"].as_str().context("no api_key in response")?;
            save_credentials(&base, key)?;
            println!("Account created for {email}. API key saved to ~/{CREDENTIALS_FILE}");
        }

        RegistryCommands::Login { email, registry } => {
            let base = resolve_registry_url(&cli.config.registry.url, &registry);
            let password = prompt_password("Password: ")?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/login"))
                .json(&serde_json::json!({ "email": email, "password": password }))
                .send()
                .await
                .context("failed to contact registry")?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                bail!("login failed: {body}");
            }
            let data: serde_json::Value = resp.json().await?;
            let key = data["api_key"].as_str().context("no api_key in response")?;
            save_credentials(&base, key)?;
            println!("Logged in as {email}. API key saved to ~/{CREDENTIALS_FILE}");
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
            action: TokenAction::Rotate { registry },
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/login"))
                .bearer_auth(&token)
                .json(&serde_json::json!({}))
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
            action: KeysAction::Add { key, registry },
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
            let key_b64 = load_public_key(&key)?;
            let http = reqwest::Client::new();
            let resp = http
                .post(format!("{base}/api/v1/account/keys"))
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
        PackAction::Versions { name, registry } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
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

        PackAction::SetVisibility {
            name,
            visibility,
            registry,
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
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

        PackAction::Grant { name, to, registry } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
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

        PackAction::Revoke {
            name,
            from,
            registry,
        } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
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

        PackAction::Access { name, registry } => {
            let (base, token) = load_credentials_or_fail(&cli.config.registry.url, &registry)?;
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
        .unwrap_or_else(|| "https://registry.dkp-standard.com".into());
    if token.is_empty() {
        return Ok(None);
    }
    Ok(Some((base, token)))
}

fn load_credentials_or_fail(
    config_url: &Option<String>,
    cli_url: &Option<String>,
) -> Result<(String, String)> {
    let override_url = cli_url.clone().or_else(|| config_url.clone());
    load_credentials(&override_url)?
        .ok_or_else(|| anyhow::anyhow!("not logged in — run 'dkp registry login --email <email>'"))
}

/// Convenience for other registry commands: load (base, token) from saved creds + overrides.
pub fn load_credentials_from_ctx(
    config_url: &Option<String>,
    cli_url: &Option<String>,
) -> Result<Option<(String, String)>> {
    let override_url = cli_url.clone().or_else(|| config_url.clone());
    load_credentials(&override_url)
}

pub fn resolve_registry_url(config_url: &Option<String>, cli_url: &Option<String>) -> String {
    cli_url
        .clone()
        .or_else(|| config_url.clone())
        .unwrap_or_else(|| "https://registry.dkp-standard.com".into())
}

fn prompt_password(prompt: &str) -> Result<String> {
    rpassword::prompt_password(prompt).context("failed to read password")
}

fn load_public_key(path: &PathBuf) -> Result<String> {
    use base64::Engine;
    let bytes =
        std::fs::read(path).with_context(|| format!("reading key from {}", path.display()))?;
    // Accept raw 32-byte binary or base64-encoded text
    if bytes.len() == 32 {
        return Ok(base64::engine::general_purpose::STANDARD.encode(&bytes));
    }
    let text = String::from_utf8(bytes).context("key file is not UTF-8")?;
    let text = text.trim();
    // Validate it decodes to 32 bytes
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(text)
        .context("key file is not valid base64")?;
    if decoded.len() != 32 {
        bail!("Ed25519 public key must be 32 bytes; got {}", decoded.len());
    }
    Ok(text.to_owned())
}
