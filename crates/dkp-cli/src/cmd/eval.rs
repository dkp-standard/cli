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

    let config = GenConfig::load(CliOverrides {
        base_url: args.base_url,
        api_key: args.api_key,
        model: args.model,
        overwrite: true,
    })?;
    if config.api_key.is_empty() {
        anyhow::bail!(
            "API key required: pass --api-key, set DKP_GEN_API_KEY, \
             or add api_key to ~/.dkp/gen.toml"
        );
    }

    let client = Arc::new(OpenAiClient::new(&config)?);
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
            "Running eval ({} cases)...",
            if let Some(pairs) = args.pairs {
                format!("up to {}", pairs)
            } else {
                "all".into()
            }
        );
    }

    let report = dkp_gen_core::eval::run(gen_ctx, args.pairs, args.baseline_only).await?;

    let pct = (report.summary.passed * 100)
        .checked_div(report.summary.total)
        .unwrap_or(0);

    println!(
        "Eval complete: {}/{} passed ({}%)",
        report.summary.passed, report.summary.total, pct
    );

    if !report.failures.is_empty() {
        println!("\nFailed cases:");
        for f in &report.failures {
            println!("  ✗ {}", f.query);
            println!("    Reason: {}", f.reason);
        }
        println!("\nRun `dkp fix <pack>` to address failures.");
    }

    Ok(())
}
