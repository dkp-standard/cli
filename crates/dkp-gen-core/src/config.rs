use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

/// Configuration for generation commands.
/// Constructed via `GenConfig::load()`: CLI flags > DKP_GEN_* env vars > ~/.dkp/gen.toml > defaults.
#[derive(Debug, Clone)]
pub struct GenConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub overwrite: bool,
    pub timeout_secs: u64,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".into(),
            api_key: String::new(),
            model: "gpt-4o-mini".into(),
            overwrite: false,
            timeout_secs: 300,
        }
    }
}

/// Values the user explicitly set on the CLI (all Option — None means absent).
pub struct CliOverrides {
    pub base_url: Option<String>,
    /// Already captures DKP_GEN_API_KEY via clap's `env` attribute.
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub overwrite: bool,
}

#[derive(Debug, serde::Deserialize, Default)]
struct FileConfig {
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    timeout_secs: Option<u64>,
}

impl GenConfig {
    /// Merge CLI overrides, DKP_GEN_* env vars, ~/.dkp/gen.toml, and built-in defaults.
    pub fn load(cli: CliOverrides) -> Result<Self> {
        let config_path = dirs::home_dir().unwrap_or_default().join(".dkp/gen.toml");

        let defaults = GenConfig::default();

        // Layer: file first, then DKP_GEN_* env vars override file
        let file: FileConfig = Figment::new()
            .merge(Toml::file(&config_path))
            .merge(Env::prefixed("DKP_GEN_"))
            .extract()
            .unwrap_or_default();

        Ok(GenConfig {
            base_url: cli.base_url.or(file.base_url).unwrap_or(defaults.base_url),
            api_key: cli.api_key.or(file.api_key).unwrap_or_default(),
            model: cli.model.or(file.model).unwrap_or(defaults.model),
            overwrite: cli.overwrite,
            timeout_secs: file.timeout_secs.unwrap_or(defaults.timeout_secs),
        })
    }
}
