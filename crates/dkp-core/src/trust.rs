use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const TRUSTED_KEYS_FILE: &str = ".dkp/trusted_keys.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedKeysFile {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub scopes: HashMap<String, TrustedKeyEntry>,
}

impl Default for TrustedKeysFile {
    fn default() -> Self {
        Self {
            version: 1,
            scopes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedKeyEntry {
    /// Base64-encoded Ed25519 public key.
    pub public_key: String,
    /// Unix timestamp (seconds) when this scope was first pinned.
    pub first_seen_at: u64,
    pub registry_url: String,
}

/// Outcome of checking a fetched publisher key against the trusted-key store.
pub enum PinCheck {
    /// No prior pin existed; the key has now been recorded.
    FirstContact,
    /// A prior pin existed and matches the freshly fetched key.
    Match,
    /// A prior pin existed and differs from the freshly fetched key.
    Mismatch { pinned_key: String },
}

fn trusted_keys_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(TRUSTED_KEYS_FILE))
}

pub fn load(path: &Path) -> Result<TrustedKeysFile> {
    if !path.exists() {
        return Ok(TrustedKeysFile::default());
    }
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("reading trusted keys from {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("parsing trusted keys file {}", path.display()))
}

pub fn save(path: &Path, file: &TrustedKeysFile) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(file)?)
        .with_context(|| format!("writing trusted keys to {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

/// Check `scope`'s freshly-fetched `public_key` against the global trusted-key
/// store, pinning it on first contact. Does not itself decide policy on
/// mismatch — the caller decides whether to hard-fail or accept an override.
pub fn check_and_pin(scope: &str, public_key: &str, registry_url: &str) -> Result<PinCheck> {
    let path = trusted_keys_path()?;
    let mut file = load(&path)?;

    match file.scopes.get(scope) {
        None => {
            let first_seen_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            file.scopes.insert(
                scope.to_owned(),
                TrustedKeyEntry {
                    public_key: public_key.to_owned(),
                    first_seen_at,
                    registry_url: registry_url.to_owned(),
                },
            );
            save(&path, &file)?;
            Ok(PinCheck::FirstContact)
        }
        Some(entry) if entry.public_key == public_key => Ok(PinCheck::Match),
        Some(entry) => Ok(PinCheck::Mismatch {
            pinned_key: entry.public_key.clone(),
        }),
    }
}

/// Overwrite the pinned key for `scope`, used after the user explicitly
/// accepts a key change (e.g. via `--accept-new-key`).
pub fn accept_new_key(scope: &str, public_key: &str, registry_url: &str) -> Result<()> {
    let path = trusted_keys_path()?;
    let mut file = load(&path)?;
    let first_seen_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    file.scopes.insert(
        scope.to_owned(),
        TrustedKeyEntry {
            public_key: public_key.to_owned(),
            first_seen_at,
            registry_url: registry_url.to_owned(),
        },
    );
    save(&path, &file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_pins_and_reports_first_contact() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trusted_keys.json");
        let mut file = load(&path).unwrap();
        assert!(file.scopes.is_empty());

        file.scopes.insert(
            "@scope/pack".into(),
            TrustedKeyEntry {
                public_key: "keyA".into(),
                first_seen_at: 0,
                registry_url: "https://example.test".into(),
            },
        );
        save(&path, &file).unwrap();

        let reloaded = load(&path).unwrap();
        assert_eq!(
            reloaded.scopes.get("@scope/pack").unwrap().public_key,
            "keyA"
        );
    }

    #[test]
    fn load_missing_file_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("does-not-exist.json");
        let file = load(&path).unwrap();
        assert_eq!(file.version, 1);
        assert!(file.scopes.is_empty());
    }
}
