use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct WebuiArgs {
    /// Path to the DKP pack directory or archive
    pub pack: PathBuf,

    /// Port to listen on (0 = OS-assigned random port)
    #[arg(long, default_value = "0")]
    pub port: u16,

    /// Do not automatically open the browser
    #[arg(long)]
    pub no_open: bool,
}

pub async fn run(args: WebuiArgs, _ctx: &CmdCtx) -> Result<()> {
    #[cfg(not(feature = "webui"))]
    {
        let _ = args;
        anyhow::bail!(
            "Web UI support was not compiled in. Rebuild with: cargo build --features webui"
        );
    }
    #[cfg(feature = "webui")]
    {
        server::serve(args).await
    }
}

#[cfg(feature = "webui")]
mod embed;
#[cfg(feature = "webui")]
mod routes;
#[cfg(feature = "webui")]
mod server;
