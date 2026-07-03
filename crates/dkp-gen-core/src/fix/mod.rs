use serde_json::Value;

use dkp_core::types::eval::{EvalCase, EvalVersionMeta};

use crate::chunk;
use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::{extract_jsonl, templates};

pub struct FixReport {
    pub failed_count: usize,
    pub chunks_written: usize,
    pub eval_cases_written: usize,
}

pub async fn run(ctx: &PipelineContext) -> GenResult<FixReport> {
    let eval_report_path = ctx.build_path().join("eval_report.json");
    let report_content = std::fs::read_to_string(&eval_report_path)?;
    let report: Value = serde_json::from_str(&report_content)?;

    let failed_entries = collect_failures(&report);
    let failed_count = failed_entries.len();
    if failed_count == 0 {
        return Ok(FixReport {
            failed_count: 0,
            chunks_written: 0,
            eval_cases_written: 0,
        });
    }

    let failure_summary = failed_entries.join("\n");

    // Load existing chunks for context
    let chunks_path = ctx.machine_path().join("retrieval_chunks.jsonl");
    let corpus: String = std::fs::read_to_string(&chunks_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter_map(|v| v["chunk_text"].as_str().map(String::from))
        .collect::<Vec<_>>()
        .join("\n\n")
        .chars()
        .take(4000)
        .collect();

    let (sys, user) =
        templates::prompt_fix_chunks(&ctx.domain, &ctx.pack_name, &failure_summary, &corpus);
    let raw = ctx.generate("fix_chunks", &sys, &user).await?;
    let new_chunks = chunk::split(&raw, &ctx.domain, &ctx.pack_name);
    let chunks_written = new_chunks.len();
    ctx.write_jsonl(&chunks_path, &new_chunks)?;

    // Regenerate eval set grounded on fresh corpus
    let corpus_excerpt: String = new_chunks
        .iter()
        .map(|c| c.chunk_text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
        .chars()
        .take(4000)
        .collect();

    let (eval_sys, eval_user) =
        templates::prompt_eval_set(&ctx.domain, &ctx.pack_name, &corpus_excerpt);
    let eval_raw = ctx.generate("eval_set", &eval_sys, &eval_user).await?;
    let rows = extract_jsonl(&eval_raw)?;
    let eval_cases_written = rows.len();

    let dataset_version = read_manifest_version(&ctx.pack_dir);
    let eval_cases: Vec<EvalCase> = rows
        .iter()
        .map(|row| EvalCase {
            query: row["query"].as_str().unwrap_or("").to_string(),
            expected_dimensions: row["expected_dimensions"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            critical_must_include: row["critical_must_include"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            scoring_rubric: row["scoring_rubric"].as_str().unwrap_or("").to_string(),
            version_meta: EvalVersionMeta {
                prompt_hash: String::new(),
                model_version: ctx.config.model.clone(),
                dataset_version: dataset_version.clone(),
            },
            tags: vec![],
            audience: vec![],
        })
        .collect();

    ctx.write_jsonl(&ctx.machine_path().join("eval_set.jsonl"), &eval_cases)?;

    Ok(FixReport {
        failed_count,
        chunks_written,
        eval_cases_written,
    })
}

fn collect_failures(report: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(arr) = report["failures"].as_array() {
        for f in arr {
            let query = f["query"].as_str().unwrap_or("");
            let reason = f["reason"].as_str().unwrap_or("");
            if !query.is_empty() {
                out.push(format!("- Query: {query}\n  Reason: {reason}"));
            }
        }
    }
    out
}

fn read_manifest_version(pack_dir: &std::path::Path) -> String {
    std::fs::read_to_string(pack_dir.join("manifest.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["version"].as_str().map(String::from))
        .unwrap_or_else(|| "0.1.0".into())
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

    #[test]
    fn collect_failures_extracts_query_and_reason() {
        let report = serde_json::json!({
            "failures": [
                {"query": "q1", "reason": "missing detail"},
                {"query": "q2", "reason": "wrong answer"},
            ]
        });
        let failures = collect_failures(&report);
        assert_eq!(failures.len(), 2);
        assert!(failures[0].contains("q1"));
        assert!(failures[0].contains("missing detail"));
    }

    #[test]
    fn collect_failures_skips_entries_with_empty_query() {
        let report = serde_json::json!({
            "failures": [{"query": "", "reason": "irrelevant"}]
        });
        assert!(collect_failures(&report).is_empty());
    }

    #[test]
    fn collect_failures_handles_missing_failures_key() {
        let report = serde_json::json!({});
        assert!(collect_failures(&report).is_empty());
    }

    #[test]
    fn read_manifest_version_defaults_when_absent() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(read_manifest_version(tmp.path()), "0.1.0");
    }

    #[test]
    fn read_manifest_version_reads_actual_version() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("manifest.json"), r#"{"version": "2.3.4"}"#).unwrap();
        assert_eq!(read_manifest_version(tmp.path()), "2.3.4");
    }

    #[tokio::test]
    async fn run_no_failures_returns_zeroed_report_without_llm_call() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("build")).unwrap();
        std::fs::write(
            tmp.path().join("build/eval_report.json"),
            r#"{"failures": []}"#,
        )
        .unwrap();
        // No fixtures registered: if the LLM were called, this would error.
        let ctx = test_ctx(&tmp, HashMap::new());

        let report = run(&ctx).await.unwrap();
        assert_eq!(report.failed_count, 0);
        assert_eq!(report.chunks_written, 0);
        assert_eq!(report.eval_cases_written, 0);
    }

    #[tokio::test]
    async fn run_missing_eval_report_errors() {
        let tmp = TempDir::new().unwrap();
        let ctx = test_ctx(&tmp, HashMap::new());
        assert!(run(&ctx).await.is_err());
    }
}
