use anyhow::{Context, Result};
use clap::Args;
use serde_json::Value;
use std::path::PathBuf;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ReviewArgs {
    /// Path to the DKP pack directory
    pub pack: PathBuf,
}

pub async fn run(args: ReviewArgs, ctx: &CmdCtx) -> Result<()> {
    let report_path = args.pack.join("build").join("eval_report.json");
    if !report_path.exists() {
        anyhow::bail!(
            "No eval report found at '{}'. Run `dkp eval <pack>` first.",
            report_path.display()
        );
    }

    let content = std::fs::read_to_string(&report_path)
        .with_context(|| format!("reading '{}'", report_path.display()))?;
    let report: Value =
        serde_json::from_str(&content).with_context(|| "parsing eval_report.json")?;

    let pack_name = args
        .pack
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| args.pack.display().to_string());

    let total = report["summary"]["total"].as_u64().unwrap_or(0);
    let passed = report["summary"]["passed"].as_u64().unwrap_or(0);
    let failed = report["summary"]["failed"].as_u64().unwrap_or(0);
    let pct = (passed * 100).checked_div(total).unwrap_or(0);

    if !ctx.quiet {
        println!("Pack: {pack_name}");
        println!("Eval: {passed}/{total} passed ({pct}%)\n");
    }

    let cases = report["cases"].as_array().cloned().unwrap_or_default();

    let passed_cases: Vec<&Value> = cases
        .iter()
        .filter(|c| c["grounded_pass"].as_bool().unwrap_or(false))
        .collect();
    let failed_cases: Vec<&Value> = cases
        .iter()
        .filter(|c| !c["grounded_pass"].as_bool().unwrap_or(false))
        .collect();

    if !passed_cases.is_empty() {
        println!("PASSED ({}):", passed_cases.len());
        for c in &passed_cases {
            let query = c["query"].as_str().unwrap_or("");
            println!("  ✓ {query}");
        }
        println!();
    }

    if !failed_cases.is_empty() {
        println!("FAILED ({}):", failed_cases.len());
        for c in &failed_cases {
            let query = c["query"].as_str().unwrap_or("");
            let reason = c["grounded_reason"].as_str().unwrap_or("");
            println!("  ✗ {query}");
            println!("    Reason: {reason}");
        }
        println!();
        println!(
            "Run `dkp fix <pack>` to address {} failure(s).",
            failed_cases.len()
        );
    }

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
