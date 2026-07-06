use anyhow::Result;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
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
    /// "brave" (default) | "tavily" | "serpapi" | ...
    pub search_provider: String,
    pub search_api_key: String,
    pub max_tool_turns: u32,
    /// Tool use (web_fetch/web_search) is on by default; `--no-tools` or
    /// `tools_enabled = false` in gen.toml disables it.
    pub tools_enabled: bool,
    /// Free-text guidance appended to every asset's user prompt (e.g. tone,
    /// focus areas, things to emphasize or avoid). Empty means no guidance.
    pub instructions: String,
    /// Render `human/handbook.md` to `handbook.pdf`/`handbook.epub` after
    /// generation. On by default (spec §11.2 "SHOULD be regenerated");
    /// `--no-render-formats` or `render_formats = false` in gen.toml disables it.
    pub render_formats: bool,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".into(),
            api_key: String::new(),
            model: "gpt-4o-mini".into(),
            overwrite: false,
            timeout_secs: 300,
            search_provider: "brave".into(),
            search_api_key: String::new(),
            max_tool_turns: 6,
            tools_enabled: true,
            instructions: String::new(),
            render_formats: true,
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
    /// `--no-tools` — negative-only flag, so a plain `bool` (not `Option`) is enough.
    pub no_tools: bool,
    pub instructions: Option<String>,
    /// `--no-render-formats` — negative-only flag, mirrors `no_tools`.
    pub no_render_formats: bool,
    pub max_tool_turns: Option<u32>,
}

#[derive(Debug, serde::Deserialize, Default)]
struct SearchFileConfig {
    provider: Option<String>,
    api_key: Option<String>,
}

#[derive(Debug, serde::Deserialize, Default)]
struct FileConfig {
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    timeout_secs: Option<u64>,
    max_tool_turns: Option<u32>,
    tools_enabled: Option<bool>,
    instructions: Option<String>,
    render_formats: Option<bool>,
    search: Option<SearchFileConfig>,
    // Flat fallbacks: `Env::prefixed("DKP_GEN_")` maps DKP_GEN_SEARCH_PROVIDER /
    // DKP_GEN_SEARCH_API_KEY to top-level keys, not the nested [search] table,
    // so both shapes are accepted.
    search_provider: Option<String>,
    search_api_key: Option<String>,
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

        let search_provider = file
            .search_provider
            .or_else(|| file.search.as_ref().and_then(|s| s.provider.clone()))
            .unwrap_or(defaults.search_provider);
        let search_api_key = file
            .search_api_key
            .or_else(|| file.search.as_ref().and_then(|s| s.api_key.clone()))
            .unwrap_or_default();

        Ok(GenConfig {
            base_url: cli.base_url.or(file.base_url).unwrap_or(defaults.base_url),
            api_key: cli.api_key.or(file.api_key).unwrap_or_default(),
            model: cli.model.or(file.model).unwrap_or(defaults.model),
            overwrite: cli.overwrite,
            timeout_secs: file.timeout_secs.unwrap_or(defaults.timeout_secs),
            search_provider,
            search_api_key,
            max_tool_turns: cli
                .max_tool_turns
                .or(file.max_tool_turns)
                .unwrap_or(defaults.max_tool_turns),
            tools_enabled: if cli.no_tools {
                false
            } else {
                file.tools_enabled.unwrap_or(defaults.tools_enabled)
            },
            instructions: cli.instructions.or(file.instructions).unwrap_or_default(),
            render_formats: if cli.no_render_formats {
                false
            } else {
                file.render_formats.unwrap_or(defaults.render_formats)
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use figment::Jail;

    fn no_overrides() -> CliOverrides {
        CliOverrides {
            base_url: None,
            api_key: None,
            model: None,
            overwrite: false,
            no_tools: false,
            instructions: None,
            no_render_formats: false,
            max_tool_turns: None,
        }
    }

    #[test]
    fn defaults_have_tools_enabled_and_brave_provider() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            let config = GenConfig::load(no_overrides()).unwrap();
            assert!(config.tools_enabled);
            assert_eq!(config.search_provider, "brave");
            assert_eq!(config.max_tool_turns, 6);
            assert!(config.search_api_key.is_empty());
            Ok(())
        });
    }

    #[test]
    fn no_tools_cli_flag_overrides_file_default() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            let config = GenConfig::load(CliOverrides {
                no_tools: true,
                ..no_overrides()
            })
            .unwrap();
            assert!(!config.tools_enabled);
            Ok(())
        });
    }

    #[test]
    fn nested_search_table_in_gen_toml_is_read() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.create_dir(".dkp")?;
            jail.create_file(
                ".dkp/gen.toml",
                r#"
                [search]
                provider = "brave"
                api_key = "table-key"
                "#,
            )?;
            let config = GenConfig::load(no_overrides()).unwrap();
            assert_eq!(config.search_api_key, "table-key");
            Ok(())
        });
    }

    #[test]
    fn env_var_tools_enabled_false_is_respected() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.set_env("DKP_GEN_TOOLS_ENABLED", "false");
            let config = GenConfig::load(no_overrides()).unwrap();
            assert!(!config.tools_enabled);
            Ok(())
        });
    }

    #[test]
    fn env_var_search_api_key_is_read_as_flat_key() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.set_env("DKP_GEN_SEARCH_API_KEY", "env-key");
            let config = GenConfig::load(no_overrides()).unwrap();
            assert_eq!(config.search_api_key, "env-key");
            Ok(())
        });
    }

    #[test]
    fn cli_instructions_override_file_instructions() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.create_dir(".dkp")?;
            jail.create_file(".dkp/gen.toml", r#"instructions = "from file""#)?;
            let config = GenConfig::load(CliOverrides {
                instructions: Some("from cli".into()),
                ..no_overrides()
            })
            .unwrap();
            assert_eq!(config.instructions, "from cli");
            Ok(())
        });
    }

    #[test]
    fn file_instructions_used_when_no_cli_override() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.create_dir(".dkp")?;
            jail.create_file(".dkp/gen.toml", r#"instructions = "from file""#)?;
            let config = GenConfig::load(no_overrides()).unwrap();
            assert_eq!(config.instructions, "from file");
            Ok(())
        });
    }

    #[test]
    fn cli_max_tool_turns_overrides_file_default() {
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            let config = GenConfig::load(CliOverrides {
                max_tool_turns: Some(20),
                ..no_overrides()
            })
            .unwrap();
            assert_eq!(config.max_tool_turns, 20);
            Ok(())
        });
    }

    #[test]
    fn unknown_search_provider_config_value_is_still_loaded_verbatim() {
        // GenConfig::load itself never validates search_provider — that
        // happens at tool-executor construction time (make_search_provider).
        Jail::expect_with(|jail| {
            let home = jail.directory().display().to_string();
            jail.set_env("HOME", &home);
            jail.set_env("DKP_GEN_SEARCH_PROVIDER", "not-a-real-provider");
            let config = GenConfig::load(no_overrides()).unwrap();
            assert_eq!(config.search_provider, "not-a-real-provider");
            Ok(())
        });
    }
}
