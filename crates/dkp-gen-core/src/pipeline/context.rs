use std::path::{Path, PathBuf};
use std::sync::Arc;

use indicatif::ProgressBar;
use serde::Serialize;

use crate::config::GenConfig;
use crate::error::GenResult;
use crate::llm::LlmClient;
use crate::tools::GenToolExecutor;
use crate::tools::discovered::DiscoveredLog;

pub struct PipelineContext {
    pub pack_dir: PathBuf,
    pub domain: String,
    pub pack_name: String,
    pub config: GenConfig,
    pub client: Arc<dyn LlmClient>,
    pub progress: Option<ProgressBar>,
    pub verbose: bool,
}

impl PipelineContext {
    pub fn machine_path(&self) -> PathBuf {
        self.pack_dir.join("machine")
    }

    pub fn human_path(&self) -> PathBuf {
        self.pack_dir.join("human")
    }

    pub fn evidence_path(&self) -> PathBuf {
        self.pack_dir.join("evidence")
    }

    pub fn build_path(&self) -> PathBuf {
        self.pack_dir.join("build")
    }

    /// Returns true if the asset at `path` should be (re)generated.
    pub fn should_generate(&self, path: &Path) -> bool {
        self.config.overwrite
            || !path.exists()
            || path.metadata().map(|m| m.len() == 0).unwrap_or(true)
    }

    /// Call the LLM and return the raw text response.
    pub async fn generate(&self, label: &str, system: &str, user: &str) -> GenResult<String> {
        self.report_start(label);
        let user = self.with_instructions(user);
        let result = self.client.complete(system, &user).await?;
        self.report_done(label);
        Ok(result)
    }

    /// Call the LLM with tool access (web_fetch/web_search) and return the
    /// raw text response, bounded to `config.max_tool_turns` tool round-trips.
    pub async fn generate_with_tools(
        &self,
        label: &str,
        system: &str,
        user: &str,
        tools: &GenToolExecutor,
    ) -> GenResult<String> {
        self.report_start(label);
        let user = self.with_instructions(user);
        let result = self
            .client
            .complete_with_tools(system, &user, tools, self.config.max_tool_turns)
            .await?;
        self.report_done(label);
        Ok(result)
    }

    /// Appends the user-supplied `--instructions`/`instructions =` guidance
    /// (if any) to a generated user prompt, so it applies uniformly across
    /// every asset without each prompt builder needing to know about it.
    fn with_instructions(&self, user: &str) -> String {
        if self.config.instructions.trim().is_empty() {
            user.to_string()
        } else {
            format!(
                "{user}\n\nAdditional user guidance (apply unless it conflicts with the \
                 instructions above): {}",
                self.config.instructions
            )
        }
    }

    /// Convenience for call sites that only conditionally have a tool
    /// executor available (i.e. tool use may be disabled via config).
    pub async fn generate_maybe_tools(
        &self,
        label: &str,
        system: &str,
        user: &str,
        tools: Option<&GenToolExecutor>,
    ) -> GenResult<String> {
        match tools {
            Some(t) => self.generate_with_tools(label, system, user, t).await,
            None => self.generate(label, system, user).await,
        }
    }

    /// Build the tool executor for this run, or `None` if tool use is
    /// disabled (`--no-tools` / `tools_enabled = false`). `command` tags
    /// provenance entries (e.g. "generate", "fix").
    pub fn build_tool_executor(&self, command: &str) -> GenResult<Option<GenToolExecutor>> {
        if !self.config.tools_enabled {
            return Ok(None);
        }
        let discovered = Arc::new(DiscoveredLog::new(
            self.build_path().join("sources_discovered.jsonl"),
        ));
        let search = crate::tools::make_search_provider(&self.config)?;
        let executor = GenToolExecutor::new(
            self.config.timeout_secs,
            discovered,
            command,
            search,
            self.verbose,
        )?;
        Ok(Some(executor))
    }

    fn report_start(&self, label: &str) {
        if let Some(pb) = &self.progress {
            pb.set_message(format!("generating {label}..."));
        } else if self.verbose {
            eprintln!("  → {label}...");
        }
    }

    fn report_done(&self, label: &str) {
        if let Some(pb) = &self.progress {
            pb.set_message(format!("{label} done"));
        } else if self.verbose {
            eprintln!("  ✓ {label}");
        }
    }

    /// Surface a non-fatal warning to the user, regardless of `verbose`.
    pub fn report_warning(&self, msg: &str) {
        if let Some(pb) = &self.progress {
            pb.println(format!("  ⚠ {msg}"));
        } else {
            eprintln!("  ⚠ {msg}");
        }
    }

    /// Write JSON to path atomically (tmp → rename).
    pub fn write_json<T: Serialize>(&self, path: &Path, value: &T) -> GenResult<()> {
        let content = serde_json::to_string_pretty(value)? + "\n";
        write_atomic(path, content.as_bytes())
    }

    /// Write JSONL to path atomically.
    pub fn write_jsonl<T: Serialize>(&self, path: &Path, records: &[T]) -> GenResult<()> {
        let mut content = String::new();
        for r in records {
            content.push_str(&serde_json::to_string(r)?);
            content.push('\n');
        }
        write_atomic(path, content.as_bytes())
    }

    /// Write raw text to path atomically.
    pub fn write_text(&self, path: &Path, text: &str) -> GenResult<()> {
        write_atomic(path, text.as_bytes())
    }
}

fn write_atomic(path: &Path, data: &[u8]) -> GenResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GenConfig;
    use crate::llm::mock::MockClient;
    use std::collections::HashMap;

    fn test_ctx(instructions: &str) -> PipelineContext {
        PipelineContext {
            pack_dir: PathBuf::new(),
            domain: "testing".to_string(),
            pack_name: "test-pack".to_string(),
            config: GenConfig {
                instructions: instructions.to_string(),
                ..GenConfig::default()
            },
            client: Arc::new(MockClient::new(HashMap::new(), "")),
            progress: None,
            verbose: false,
        }
    }

    #[test]
    fn with_instructions_is_noop_when_empty() {
        let ctx = test_ctx("");
        assert_eq!(ctx.with_instructions("Write a poem."), "Write a poem.");
    }

    #[test]
    fn with_instructions_appends_guidance_when_present() {
        let ctx = test_ctx("Use a formal tone and cite sources.");
        let result = ctx.with_instructions("Write a poem.");
        assert!(result.starts_with("Write a poem."));
        assert!(result.contains("Use a formal tone and cite sources."));
    }
}
