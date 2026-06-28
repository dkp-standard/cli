use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::templates;

#[derive(Serialize, Deserialize)]
pub struct EvalSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Serialize, Deserialize)]
pub struct EvalCaseResult {
    pub query: String,
    pub baseline_pass: bool,
    pub baseline_reason: String,
    pub grounded_pass: bool,
    pub grounded_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvalFailure {
    pub query: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvalReport {
    pub summary: EvalSummary,
    pub failures: Vec<EvalFailure>,
    pub cases: Vec<EvalCaseResult>,
}

pub async fn run(
    ctx: Arc<PipelineContext>,
    pairs: Option<usize>,
    baseline_only: bool,
) -> GenResult<EvalReport> {
    // Load eval set
    let eval_path = ctx.machine_path().join("eval_set.jsonl");
    let eval_content = std::fs::read_to_string(&eval_path)?;
    let mut cases: Vec<Value> = eval_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();

    if let Some(limit) = pairs {
        cases.truncate(limit);
    }

    // Load chunks for grounded context
    let chunks_path = ctx.machine_path().join("retrieval_chunks.jsonl");
    let grounded_context: String = std::fs::read_to_string(&chunks_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter_map(|v| v["chunk_text"].as_str().map(String::from))
        .collect::<Vec<_>>()
        .join("\n\n")
        .chars()
        .take(6000)
        .collect();

    let futs: Vec<_> = cases
        .into_iter()
        .enumerate()
        .map(|(i, case)| {
            let ctx = Arc::clone(&ctx);
            let grounded_context = grounded_context.clone();
            async move {
                let query = case["query"].as_str().unwrap_or("").to_string();
                let rubric = case["scoring_rubric"].as_str().unwrap_or("").to_string();
                let must_include: Vec<String> = case["critical_must_include"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                let (bl_sys, bl_user) =
                    templates::prompt_eval_answer(&ctx.domain, &ctx.pack_name, &query, "");
                let baseline_answer = ctx
                    .generate(&format!("eval_baseline_{i}"), &bl_sys, &bl_user)
                    .await?;
                let (sc_sys, sc_user) =
                    templates::prompt_eval_score(&query, &baseline_answer, &rubric, &must_include);
                let baseline_score_raw = ctx
                    .generate(&format!("eval_score_baseline_{i}"), &sc_sys, &sc_user)
                    .await?;
                let (baseline_pass, baseline_reason) = parse_score(&baseline_score_raw);

                let (grounded_pass, grounded_reason) = if baseline_only {
                    (baseline_pass, baseline_reason.clone())
                } else {
                    let (gr_sys, gr_user) = templates::prompt_eval_answer(
                        &ctx.domain,
                        &ctx.pack_name,
                        &query,
                        &grounded_context,
                    );
                    let grounded_answer = ctx
                        .generate(&format!("eval_grounded_{i}"), &gr_sys, &gr_user)
                        .await?;
                    let (sc2_sys, sc2_user) = templates::prompt_eval_score(
                        &query,
                        &grounded_answer,
                        &rubric,
                        &must_include,
                    );
                    let grounded_score_raw = ctx
                        .generate(&format!("eval_score_grounded_{i}"), &sc2_sys, &sc2_user)
                        .await?;
                    parse_score(&grounded_score_raw)
                };

                Ok::<EvalCaseResult, crate::error::GenError>(EvalCaseResult {
                    query,
                    baseline_pass,
                    baseline_reason,
                    grounded_pass,
                    grounded_reason,
                })
            }
        })
        .collect();

    let results: Vec<EvalCaseResult> = futures::future::try_join_all(futs).await?;

    let total = results.len();
    let passed = results.iter().filter(|r| r.grounded_pass).count();
    let failed = total - passed;

    let failures: Vec<EvalFailure> = results
        .iter()
        .filter(|r| !r.grounded_pass)
        .map(|r| EvalFailure {
            query: r.query.clone(),
            reason: r.grounded_reason.clone(),
        })
        .collect();

    let report = EvalReport {
        summary: EvalSummary {
            total,
            passed,
            failed,
        },
        failures,
        cases: results,
    };

    let report_path = ctx.build_path().join("eval_report.json");
    ctx.write_json(&report_path, &report)?;

    Ok(report)
}

fn parse_score(raw: &str) -> (bool, String) {
    // Extract JSON from LLM response (may have surrounding text)
    let start = raw.find('{').unwrap_or(0);
    let end = raw.rfind('}').map(|i| i + 1).unwrap_or(raw.len());
    let json_str = &raw[start..end];

    if let Ok(v) = serde_json::from_str::<Value>(json_str) {
        let pass = v["pass"].as_bool().unwrap_or(false);
        let reason = v["reason"]
            .as_str()
            .unwrap_or("no reason given")
            .to_string();
        return (pass, reason);
    }

    // Fallback: look for pass/fail keywords
    let lower = raw.to_lowercase();
    let pass = lower.contains("\"pass\": true") || lower.contains("pass\":true");
    (pass, raw.chars().take(200).collect())
}
