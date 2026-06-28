use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_core::{okf::exporter::export_okf, Pack};
use dkp_gen_core::{CliOverrides, GenConfig, OpenAiClient, PipelineContext};

use crate::cli::CmdCtx;
use crate::cmd::init::{run as init_run, InitArgs};

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Pack name (e.g. "Kubernetes Networking")
    pub name: String,

    /// Domain category (e.g. "Kubernetes", "Clinical Nutrition")
    #[arg(long)]
    pub domain: String,

    /// Output directory (default: ./<name-slug>/)
    #[arg(long)]
    pub dir: Option<PathBuf>,

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

    /// Overwrite already-generated assets
    #[arg(long, alias = "force")]
    pub overwrite: bool,

    /// Skip validation step
    #[arg(long)]
    pub skip_validate: bool,

    /// Skip packaging step
    #[arg(long)]
    pub skip_package: bool,
}

pub async fn run(args: NewArgs, ctx: &CmdCtx) -> Result<()> {
    let slug = slugify(&args.name);
    let pack_dir = args.dir.clone().unwrap_or_else(|| PathBuf::from(&slug));

    // Step 1: scaffold (same as `dkp init`)
    init_run(
        InitArgs {
            name: args.name.clone(),
            domain: args.domain.clone(),
            out: Some(pack_dir.clone()),
            extras: true,
            force: args.overwrite,
            stubs: false,
        },
        ctx,
    )
    .await?;

    // Step 2: build LLM pipeline context
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
    let gen_ctx = PipelineContext {
        pack_dir: pack_dir.clone(),
        domain: args.domain.clone(),
        pack_name: args.name.clone(),
        config,
        client,
        progress: None,
        verbose: !ctx.quiet,
    };

    // Step 3: machine layer
    if !ctx.quiet {
        println!("Generating machine layer...");
    }
    dkp_gen_core::pipeline::machine::run(&gen_ctx).await?;

    // Step 4: human layer
    if !ctx.quiet {
        println!("Generating human layer...");
    }
    dkp_gen_core::pipeline::human::run(&gen_ctx).await?;

    // Step 5: manifest metadata
    dkp_gen_core::pipeline::manifest::update_meta(&gen_ctx).await?;

    // Step 6: OKF export
    if !ctx.quiet {
        println!("Exporting OKF layer...");
    }
    let pack = Pack::open(&pack_dir)?;
    export_okf(&pack, &pack_dir.join("okf"))?;

    // Step 7: validate
    if !args.skip_validate {
        if !ctx.quiet {
            println!("Validating...");
        }
        crate::cmd::validate::run(
            crate::cmd::validate::ValidateArgs {
                pack: pack_dir.clone(),
                strict: false,
                gate: None,
            },
            ctx,
        )
        .await?;
    }

    // Step 8: package
    if !args.skip_package {
        if !ctx.quiet {
            println!("Packaging...");
        }
        crate::cmd::build::run(
            crate::cmd::build::BuildArgs {
                pack: pack_dir.clone(),
                format: "zip".into(),
                out: None,
                no_human: false,
                gen_mcp_manifest: false,
            },
            ctx,
        )
        .await?;
    }

    if !ctx.quiet {
        println!("\nDone! Pack created at '{}'", pack_dir.display());
    }
    Ok(())
}

fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
