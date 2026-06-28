use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DkpConfig {
    #[serde(default)]
    pub defaults: ConfigDefaults,
    #[serde(default)]
    pub registry: RegistryConfig,
    #[serde(default)]
    pub install: InstallConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigDefaults {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryConfig {
    pub url: Option<String>,
    pub token_env: Option<String>,
    pub key_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InstallConfig {
    /// Project-local install directory name (default: "dkps")
    pub local_dir: Option<String>,
    /// Global install directory (default: ~/.dkp/packs)
    pub global_dir: Option<String>,
}

impl DkpConfig {
    pub fn load() -> Self {
        let mut figment = Figment::new();

        if let Some(home_cfg) = dirs::home_dir()
            .map(|h| h.join(".dkp").join("config.toml"))
            .filter(|p| p.exists())
        {
            figment = figment.merge(Toml::file(home_cfg));
        }

        figment
            .merge(Toml::file(".dkp.toml"))
            .merge(Env::prefixed("DKP_").split("_"))
            .extract()
            .unwrap_or_default()
    }

    pub fn registry_url(&self) -> &str {
        self.registry
            .url
            .as_deref()
            .unwrap_or("https://registry.dkp-standard.com")
    }

    pub fn local_install_dir(&self) -> &str {
        self.install.local_dir.as_deref().unwrap_or("dkps")
    }
}
