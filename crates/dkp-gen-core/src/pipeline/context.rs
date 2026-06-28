use std::path::{Path, PathBuf};
use std::sync::Arc;

use indicatif::ProgressBar;
use serde::Serialize;

use crate::config::GenConfig;
use crate::error::GenResult;
use crate::llm::LlmClient;

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
        if let Some(pb) = &self.progress {
            pb.set_message(format!("generating {label}..."));
        } else if self.verbose {
            eprintln!("  → {label}...");
        }
        let result = self.client.complete(system, user).await?;
        if let Some(pb) = &self.progress {
            pb.set_message(format!("{label} done"));
        } else if self.verbose {
            eprintln!("  ✓ {label}");
        }
        Ok(result)
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
