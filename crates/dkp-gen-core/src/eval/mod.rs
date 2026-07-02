use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

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
    pub baseline_score: f64,
    pub grounded_pass: bool,
    pub grounded_reason: String,
    pub grounded_score: f64,
    pub expected_dimensions: Vec<String>,
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

/// Spec-defined `evidence/eval_results/eval_summary.json` shape (SPEC.md Appendix B §B.13).
#[derive(Serialize, Deserialize)]
pub struct EvalSummaryDoc {
    pub last_run_date: String,
    pub pack_version: String,
    pub model: String,
    pub mean_delta: f64,
    pub pass_rate: f64,
    pub gate7_pass: bool,
}

/// Spec-defined `evidence/eval_results/{date}-{model}.jsonl` line shape (SPEC.md §12.4).
#[derive(Serialize, Deserialize)]
pub struct EvalResultLine {
    pub query_hash: String,
    pub model: String,
    pub pack_version: String,
    pub run_date: String,
    pub with_pack_score: f64,
    pub baseline_score: f64,
    pub delta: f64,
    pub dimensions_met: Vec<String>,
    pub dimensions_missed: Vec<String>,
}

pub async fn run(
    ctx: Arc<PipelineContext>,
    pack_version: &str,
    min_eval_delta: f64,
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
                let expected_dimensions: Vec<String> = case["expected_dimensions"]
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
                let (baseline_pass, baseline_reason, baseline_score) =
                    parse_score(&baseline_score_raw);

                let (grounded_pass, grounded_reason, grounded_score) = if baseline_only {
                    (baseline_pass, baseline_reason.clone(), baseline_score)
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
                    baseline_score,
                    grounded_pass,
                    grounded_reason,
                    grounded_score,
                    expected_dimensions,
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

    let mean_delta = if total > 0 {
        results
            .iter()
            .map(|r| r.grounded_score - r.baseline_score)
            .sum::<f64>()
            / total as f64
    } else {
        0.0
    };
    let eval_pass_rate = if total > 0 {
        results
            .iter()
            .filter(|r| (r.grounded_score - r.baseline_score) >= min_eval_delta)
            .count() as f64
            / total as f64
    } else {
        0.0
    };
    let gate7_pass = mean_delta >= min_eval_delta;
    let run_date = now_iso8601();

    let summary_doc = EvalSummaryDoc {
        last_run_date: run_date.clone(),
        pack_version: pack_version.to_string(),
        model: ctx.config.model.clone(),
        mean_delta,
        pass_rate: eval_pass_rate,
        gate7_pass,
    };
    let summary_path = ctx.evidence_path().join("eval_results").join("eval_summary.json");
    // Best-effort: bundle directory may be read-only (SPEC.md §12.4).
    let _ = ctx.write_json(&summary_path, &summary_doc);

    let result_lines: Vec<EvalResultLine> = results
        .iter()
        .map(|r| {
            let dimensions_met = r.expected_dimensions.clone();
            EvalResultLine {
                query_hash: format!("{:x}", Sha256::digest(r.query.as_bytes())),
                model: ctx.config.model.clone(),
                pack_version: pack_version.to_string(),
                run_date: run_date.clone(),
                with_pack_score: r.grounded_score,
                baseline_score: r.baseline_score,
                delta: r.grounded_score - r.baseline_score,
                dimensions_met: if r.grounded_pass {
                    dimensions_met
                } else {
                    Vec::new()
                },
                dimensions_missed: if r.grounded_pass {
                    Vec::new()
                } else {
                    r.expected_dimensions.clone()
                },
            }
        })
        .collect();
    let date_str = run_date[..10].to_string();
    let lines_path = ctx
        .evidence_path()
        .join("eval_results")
        .join(format!("{date_str}-{}.jsonl", ctx.config.model));
    let _ = ctx.write_jsonl(&lines_path, &result_lines);

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

/// Current UTC time as an RFC3339 datetime string (`std`-only, no chrono dependency).
fn now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86400) as i64;
    let time_of_day = secs % 86400;
    let (hh, mm, ss) = (time_of_day / 3600, (time_of_day % 3600) / 60, time_of_day % 60);

    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn parse_score(raw: &str) -> (bool, String, f64) {
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
        let score = v["score"]
            .as_f64()
            .unwrap_or(if pass { 1.0 } else { 0.0 })
            .clamp(0.0, 1.0);
        return (pass, reason, score);
    }

    // Fallback: look for pass/fail keywords
    let lower = raw.to_lowercase();
    let pass = lower.contains("\"pass\": true") || lower.contains("pass\":true");
    (
        pass,
        raw.chars().take(200).collect(),
        if pass { 1.0 } else { 0.0 },
    )
}
