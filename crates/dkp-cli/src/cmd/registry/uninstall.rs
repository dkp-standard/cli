use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Pack name, e.g. @mathis/nutrition-for-men or @mathis/pack@1.2.0
    pub name: String,

    /// Remove from global store
    #[arg(long, short = 'g')]
    pub global: bool,

    /// Remove from a custom directory
    #[arg(long, value_name = "DIR")]
    pub dest: Option<PathBuf>,

    /// Remove all installed versions
    #[arg(long)]
    pub all_versions: bool,
}

pub async fn run(args: UninstallArgs, _cli: &CmdCtx) -> Result<()> {
    // TODO: locate installed pack and remove; update dkp.lock
    let _ = args;
    Ok(())
}
