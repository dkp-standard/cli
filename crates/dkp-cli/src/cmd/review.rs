use anyhow::{Context, Result};
use clap::Args;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_gen_core::{CliOverrides, GenConfig, OpenAiClient, PipelineContext};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ReviewArgs {
    /// Path to the DKP pack directory
    pub pack: PathBuf,

    /// Skip the citation-checking pass even if an API key and discovered
    /// sources are available
    #[arg(long)]
    pub no_citations: bool,

    #[arg(
        long,
        value_name = "KEY",
        env = "DKP_GEN_API_KEY",
        hide_env_values = true
    )]
    pub api_key: Option<String>,

    #[arg(long, value_name = "URL")]
    pub base_url: Option<String>,

    #[arg(long, value_name = "MODEL")]
    pub model: Option<String>,
}

pub async fn run(args: ReviewArgs, ctx: &CmdCtx) -> Result<()> {
    run_citation_check(&args, ctx).await?;

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

/// Runs the citation-checking pass (§2.5) if a discovered-sources staging
/// file exists and an LLM API key is available. Skippable by design: with
/// no key or no discovered sources, `dkp review` falls back to its original
/// eval-report-only behavior.
async fn run_citation_check(args: &ReviewArgs, ctx: &CmdCtx) -> Result<()> {
    if args.no_citations {
        return Ok(());
    }
    let discovered_path = args.pack.join("build").join("sources_discovered.jsonl");
    if !discovered_path.exists() {
        return Ok(());
    }

    let config = GenConfig::load(CliOverrides {
        base_url: args.base_url.clone(),
        api_key: args.api_key.clone(),
        model: args.model.clone(),
        overwrite: false,
        no_tools: true,
        instructions: None,
    })?;
    if config.api_key.is_empty() {
        // No key configured: silently skip rather than failing the whole
        // `dkp review` run over an optional, additive check.
        return Ok(());
    }

    let pack = dkp_core::Pack::open(&args.pack)?;
    let domain = pack.manifest.domain.clone();
    let pack_name = pack.manifest.name.clone();
    let client = Arc::new(OpenAiClient::new(&config)?);
    let gen_ctx = PipelineContext {
        pack_dir: args.pack.clone(),
        domain,
        pack_name,
        config,
        client,
        progress: None,
        verbose: !ctx.quiet,
    };

    if !ctx.quiet {
        println!("Checking citations against discovered sources...");
    }
    let report = dkp_gen_core::review::check_citations(&gen_ctx).await?;

    if !ctx.quiet {
        if !report.unreachable_sources.is_empty() {
            println!(
                "  {} source(s) could not be re-fetched (dead links).",
                report.unreachable_sources.len()
            );
        }
        if report.unsupported_claims.is_empty() {
            println!("  No unsupported claims flagged.\n");
        } else {
            println!(
                "  {} claim(s) flagged as unsupported by discovered sources:",
                report.unsupported_claims.len()
            );
            for finding in &report.unsupported_claims {
                println!("    - [{}] {}", finding.asset, finding.claim);
                println!("      {}", finding.reason);
            }
            println!();
        }
    }

    Ok(())
}
