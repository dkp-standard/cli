use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_gen_core::{CliOverrides, GenConfig, OpenAiClient, PipelineContext};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct FixUntilArgs {
    /// Path to the DKP pack directory
    pub pack: PathBuf,

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

    /// Disable web_fetch/web_search tool use (on by default)
    #[arg(long)]
    pub no_tools: bool,

    /// Skip rendering human/handbook.md to handbook.pdf/handbook.epub (on by default)
    #[arg(long)]
    pub no_render_formats: bool,

    /// Extra free-text guidance appended to every generation prompt
    #[arg(long, value_name = "TEXT")]
    pub instructions: Option<String>,

    /// Max tool round-trips per generation call before giving up (default: 6)
    #[arg(long, value_name = "N")]
    pub max_tool_turns: Option<u32>,

    /// Target pass rate to stop at (0.0–1.0). Defaults to min_eval_delta from
    /// the manifest, or 1.0 if unset.
    #[arg(long, value_name = "F")]
    pub threshold: Option<f64>,

    /// Maximum eval+fix rounds before giving up (default: 5)
    #[arg(long, value_name = "N", default_value = "5")]
    pub max_rounds: u32,

    /// Run only first N eval pairs per round (default: all)
    #[arg(long, value_name = "N")]
    pub pairs: Option<usize>,

    /// Score without DKP context (baseline only)
    #[arg(long)]
    pub baseline_only: bool,
}

pub async fn run(args: FixUntilArgs, ctx: &CmdCtx) -> Result<()> {
    let pack = dkp_core::Pack::open(&args.pack)?;
    let domain = pack.manifest.domain.clone();
    let pack_name = pack.manifest.name.clone();
    let pack_version = pack.manifest.version.clone();
    let min_eval_delta = pack.manifest.min_eval_delta.unwrap_or(0.0);

    let threshold = args.threshold.unwrap_or_else(|| {
        if min_eval_delta > 0.0 {
            min_eval_delta
        } else {
            1.0
        }
    });

    let fix_config = GenConfig::load(CliOverrides {
        base_url: args.base_url.clone(),
        api_key: args.api_key.clone(),
        model: args.model.clone(),
        overwrite: true,
        no_tools: args.no_tools,
        instructions: args.instructions.clone(),
        no_render_formats: args.no_render_formats,
        max_tool_turns: args.max_tool_turns,
    })?;
    if fix_config.api_key.is_empty() {
        anyhow::bail!(
            "API key required: pass --api-key, set DKP_GEN_API_KEY, \
             or add api_key to ~/.dkp/gen.toml"
        );
    }
    if fix_config.tools_enabled && fix_config.search_api_key.is_empty() {
        anyhow::bail!(
            "Tool use is enabled but no search API key is set: set DKP_GEN_SEARCH_API_KEY, \
             add [search] api_key to ~/.dkp/gen.toml, or pass --no-tools."
        );
    }

    // eval always disables tools (contamination) and render formats
    let eval_config = GenConfig::load(CliOverrides {
        base_url: args.base_url,
        api_key: args.api_key,
        model: args.model,
        overwrite: true,
        no_tools: true,
        instructions: None,
        no_render_formats: true,
        max_tool_turns: None,
    })?;

    let client = Arc::new(OpenAiClient::new(&fix_config)?);
    let client_ref = Arc::clone(&client);
    let display_name = pack_name.clone();

    // eval::run takes Arc<PipelineContext> with its own config; fix::run takes
    // &PipelineContext with the fix config. We keep two contexts that share the
    // same underlying client (as Arc<dyn LlmClient>) so token counts accumulate
    // across all rounds. client_ref (Arc<OpenAiClient>) is kept separately to
    // call token_usage() after the loop.
    let client_dyn: Arc<dyn dkp_gen_core::LlmClient> = client;
    let eval_ctx = Arc::new(PipelineContext {
        pack_dir: args.pack.clone(),
        domain: domain.clone(),
        pack_name: pack_name.clone(),
        config: eval_config,
        client: Arc::clone(&client_dyn),
        progress: None,
        verbose: !ctx.quiet,
    });
    let fix_ctx = PipelineContext {
        pack_dir: args.pack.clone(),
        domain,
        pack_name,
        config: fix_config,
        client: Arc::clone(&client_dyn),
        progress: None,
        verbose: !ctx.quiet,
    };

    let mut last_passed = 0usize;
    let mut last_total = 0usize;

    for round in 0..=args.max_rounds {
        if !ctx.quiet {
            println!(
                "[{}] Round {}/{}: running eval...",
                display_name,
                round + 1,
                args.max_rounds
            );
        }

        let (pre_eval_p, pre_eval_c) = client_ref.token_usage();
        let report = dkp_gen_core::eval::run(
            Arc::clone(&eval_ctx),
            &pack_version,
            min_eval_delta,
            args.pairs,
            args.baseline_only,
        )
        .await?;
        let (post_eval_p, post_eval_c) = client_ref.token_usage();

        let passed = report.summary.passed;
        let total = report.summary.total;
        let pct = (passed * 100).checked_div(total).unwrap_or(0);
        let pass_rate = if total > 0 {
            passed as f64 / total as f64
        } else {
            0.0
        };
        last_passed = passed;
        last_total = total;

        if !ctx.quiet {
            let ep = post_eval_p - pre_eval_p;
            let ec = post_eval_c - pre_eval_c;
            println!(
                "[{}] Eval: {}/{} passed ({}%) — {} prompt + {} completion tokens",
                display_name, passed, total, pct, ep, ec
            );
        }

        if pass_rate >= threshold {
            if !ctx.quiet {
                println!(
                    "[{}] Threshold met ({:.0}%). Stopping.",
                    display_name,
                    threshold * 100.0
                );
            }
            break;
        }

        if round == args.max_rounds {
            if !ctx.quiet {
                println!(
                    "[{}] Max rounds ({}) reached. Final pass rate: {}%.",
                    display_name, args.max_rounds, pct
                );
            }
            break;
        }

        if !ctx.quiet {
            println!(
                "[{}] Round {}/{}: running fix...",
                display_name,
                round + 1,
                args.max_rounds
            );
        }

        let (pre_fix_p, pre_fix_c) = client_ref.token_usage();
        let fix_report = dkp_gen_core::fix::run(&fix_ctx).await?;
        let (post_fix_p, post_fix_c) = client_ref.token_usage();

        if !ctx.quiet {
            let fp = post_fix_p - pre_fix_p;
            let fc = post_fix_c - pre_fix_c;
            println!(
                "[{}] Fix: {} failures addressed, {} chunks written, {} eval cases written — {} prompt + {} completion tokens",
                display_name,
                fix_report.failed_count,
                fix_report.chunks_written,
                fix_report.eval_cases_written,
                fp,
                fc,
            );
        }
    }

    if !ctx.quiet {
        let final_pct = (last_passed * 100).checked_div(last_total).unwrap_or(0);
        let (prompt, completion) = client_ref.token_usage();
        println!(
            "[{}] fix-until complete: {}/{} passed ({}%) — {} prompt + {} completion tokens total",
            display_name, last_passed, last_total, final_pct, prompt, completion,
        );
    }

    Ok(())
}
