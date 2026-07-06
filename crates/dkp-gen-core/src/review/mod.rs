use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::{extract_json, templates};
use crate::tools::discovered::DiscoveredSource;
use crate::tools::web_fetch::WebFetchTool;

/// One flagged claim from the citation-checking pass: something in the
/// machine layer that doesn't trace back to any re-fetched discovered
/// source. Informational only — never a normative Gate 5/6 finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationFinding {
    pub asset: String,
    pub claim: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CitationReport {
    /// Discovered sources that could not be re-fetched (dead links, etc.).
    pub unreachable_sources: Vec<String>,
    pub unsupported_claims: Vec<CitationFinding>,
}

/// Re-fetches every URL in `build/sources_discovered.jsonl` and asks the
/// model to flag machine-layer claims that don't trace back to any of them.
/// Writes `build/citation_report.json` (non-normative, `build/`-scoped —
/// this never touches the normative `evidence/review_notes.md` sign-off).
pub async fn check_citations(ctx: &PipelineContext) -> GenResult<CitationReport> {
    let discovered_path = ctx.build_path().join("sources_discovered.jsonl");
    let discovered = read_discovered(&discovered_path)?;

    if discovered.is_empty() {
        let report = CitationReport::default();
        ctx.write_json(&ctx.build_path().join("citation_report.json"), &report)?;
        return Ok(report);
    }

    let fetcher = WebFetchTool::new(ctx.config.timeout_secs)?;
    let mut unreachable = Vec::new();
    let mut sources_text = String::new();
    for source in &discovered {
        match fetcher.fetch(&source.url).await {
            Ok(text) => {
                sources_text.push_str(&format!(
                    "Source: {}\n{}\n\n",
                    source.url,
                    text.chars().take(2000).collect::<String>()
                ));
            }
            Err(_) => unreachable.push(source.url.clone()),
        }
    }

    let claims_excerpt = build_claims_excerpt(ctx)?;
    let (sys, user) = templates::prompt_citation_check(&ctx.domain, &claims_excerpt, &sources_text);
    let raw = ctx.generate("citation_check", &sys, &user).await?;
    let value = extract_json(&raw)?;

    let unsupported_claims = value["unsupported_claims"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|c| CitationFinding {
            asset: c["asset"].as_str().unwrap_or_default().to_string(),
            claim: c["claim"].as_str().unwrap_or_default().to_string(),
            reason: c["reason"].as_str().unwrap_or_default().to_string(),
        })
        .collect();

    let report = CitationReport {
        unreachable_sources: unreachable,
        unsupported_claims,
    };
    ctx.write_json(&ctx.build_path().join("citation_report.json"), &report)?;
    Ok(report)
}

fn read_discovered(path: &std::path::Path) -> GenResult<Vec<DiscoveredSource>> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(Vec::new()),
    };
    Ok(content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect())
}

fn build_claims_excerpt(ctx: &PipelineContext) -> GenResult<String> {
    let mut excerpt = String::new();
    for name in ["rules.json", "glossary.json"] {
        if let Ok(content) = std::fs::read_to_string(ctx.machine_path().join(name))
            && let Ok(value) = serde_json::from_str::<Value>(&content) {
                excerpt.push_str(&format!("{name}:\n{value}\n\n"));
            }
    }
    if let Ok(content) = std::fs::read_to_string(ctx.machine_path().join("retrieval_chunks.jsonl"))
    {
        excerpt.push_str("retrieval_chunks.jsonl (excerpt):\n");
        excerpt.push_str(&content.chars().take(4000).collect::<String>());
        excerpt.push('\n');
    }
    Ok(excerpt.chars().take(12_000).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GenConfig;
    use crate::llm::mock::MockClient;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn test_ctx(tmp: &TempDir, fixtures: HashMap<String, String>) -> PipelineContext {
        PipelineContext {
            pack_dir: tmp.path().to_path_buf(),
            domain: "testing".to_string(),
            pack_name: "test-pack".to_string(),
            config: GenConfig::default(),
            client: Arc::new(MockClient::new(fixtures, "")),
            progress: None,
            verbose: false,
        }
    }

    #[tokio::test]
    async fn no_discovered_sources_returns_empty_report_without_llm_call() {
        let tmp = TempDir::new().unwrap();
        // No fixtures registered: if the LLM were called, this would error.
        let ctx = test_ctx(&tmp, HashMap::new());

        let report = check_citations(&ctx).await.unwrap();
        assert!(report.unsupported_claims.is_empty());
        assert!(report.unreachable_sources.is_empty());
        assert!(ctx.build_path().join("citation_report.json").exists());
    }

    #[tokio::test]
    async fn unreachable_source_is_recorded_without_aborting() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("build")).unwrap();
        std::fs::write(
            tmp.path().join("build/sources_discovered.jsonl"),
            r#"{"url":"http://127.0.0.1:1","via":"web_fetch","retrieved_at":"2026-01-01T00:00:00Z","command":"generate"}"#,
        )
        .unwrap();

        let mut fixtures = HashMap::new();
        fixtures.insert(
            "unsupported_claims".to_string(),
            r#"{"unsupported_claims": []}"#.to_string(),
        );
        let ctx = test_ctx(&tmp, fixtures);

        let report = check_citations(&ctx).await.unwrap();
        assert_eq!(report.unreachable_sources.len(), 1);
    }
}
