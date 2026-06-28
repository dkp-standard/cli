use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_gen_core::{CliOverrides, GenConfig, OpenAiClient, PipelineContext};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Path to the DKP pack directory
    pub pack: PathBuf,

    /// Specific assets to regenerate (default: all machine assets)
    #[arg(value_name = "ASSET")]
    pub assets: Vec<String>,

    /// Overwrite existing assets
    #[arg(long)]
    pub overwrite: bool,

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

pub async fn run(args: GenerateArgs, ctx: &CmdCtx) -> Result<()> {
    // Read domain and pack name from manifest
    let pack = dkp_core::Pack::open(&args.pack)?;
    let domain = pack.manifest.domain.clone();
    let pack_name = pack.manifest.name.clone();

    let config = GenConfig::load(CliOverrides {
        base_url: args.base_url,
        api_key: args.api_key,
        model: args.model,
        overwrite: args.overwrite,
    })?;
    if config.api_key.is_empty() {
        anyhow::bail!(
            "API key required: pass --api-key, set DKP_GEN_API_KEY, \
             or add api_key to ~/.dkp/gen.toml"
        );
    }

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
        println!("Generating machine layer...");
    }
    dkp_gen_core::pipeline::machine::run(&gen_ctx).await?;

    if !ctx.quiet {
        println!("Generating human layer...");
    }
    dkp_gen_core::pipeline::human::run(&gen_ctx).await?;

    dkp_gen_core::pipeline::manifest::update_meta(&gen_ctx).await?;

    if !ctx.quiet {
        println!("Done.");
    }
    Ok(())
}
