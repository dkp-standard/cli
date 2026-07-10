use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_gen_core::{CliOverrides, GenConfig, OpenAiClient, PipelineContext};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct EvalArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[arg(long, value_name = "PROVIDER")]
    pub provider: Option<String>,

    #[arg(long, value_name = "MODEL")]
    pub model: Option<String>,

    #[arg(long, value_name = "URL")]
    pub base_url: Option<String>,

    #[arg(
        long,
        value_name = "KEY",
        env = "DKP_GEN_API_KEY",
        hide_env_values = true
    )]
    pub api_key: Option<String>,

    /// Run only first N eval pairs (default: all)
    #[arg(long, value_name = "N")]
    pub pairs: Option<usize>,

    /// Score without DKP context (baseline only)
    #[arg(long)]
    pub baseline_only: bool,
}

pub async fn run(args: EvalArgs, ctx: &CmdCtx) -> Result<()> {
    let pack = dkp_core::Pack::open(&args.pack)?;
    let domain = pack.manifest.domain.clone();
    let pack_name = pack.manifest.name.clone();
    let pack_version = pack.manifest.version.clone();
    let min_eval_delta = pack.manifest.min_eval_delta.unwrap_or(0.0);

    let config = GenConfig::load(CliOverrides {
        base_url: args.base_url,
        api_key: args.api_key,
        model: args.model,
        overwrite: true,
        // `dkp eval` measures what the pack adds over an unaided baseline
        // answer; tool access would contaminate that comparison, so it's
        // always disabled here regardless of global config.
        no_tools: true,
        instructions: None,
        no_render_formats: true,
        max_tool_turns: None,
    })?;
    if config.api_key.is_empty() {
        anyhow::bail!(
            "API key required: pass --api-key, set DKP_GEN_API_KEY, \
             or add api_key to ~/.dkp/gen.toml"
        );
    }

    let client = Arc::new(OpenAiClient::new(&config)?);
    let client_ref = Arc::clone(&client);
    let display_name = pack_name.clone();
    let gen_ctx = Arc::new(PipelineContext {
        pack_dir: args.pack.clone(),
        domain,
        pack_name,
        config,
        client,
        progress: None,
        verbose: !ctx.quiet,
    });

    if !ctx.quiet {
        println!(
            "[{}] Running eval ({} cases)...",
            display_name,
            if let Some(pairs) = args.pairs {
                format!("up to {}", pairs)
            } else {
                "all".into()
            }
        );
    }

    let report = dkp_gen_core::eval::run(
        gen_ctx,
        &pack_version,
        min_eval_delta,
        args.pairs,
        args.baseline_only,
    )
    .await?;

    let pct = (report.summary.passed * 100)
        .checked_div(report.summary.total)
        .unwrap_or(0);

    let (prompt, completion) = client_ref.token_usage();
    println!(
        "[{}] Eval complete: {}/{} passed ({}%) — {} prompt + {} completion tokens",
        display_name,
        report.summary.passed,
        report.summary.total,
        pct,
        prompt,
        completion,
    );

    if !report.failures.is_empty() {
        println!("[{}] Failed cases:", display_name);
        for f in &report.failures {
            println!("  [{}] ✗ {}", display_name, f.query);
            println!("         Reason: {}", f.reason);
        }
        println!("  Run `dkp fix {}` to address failures.", args.pack.display());
    }

    Ok(())
}
