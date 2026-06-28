use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Procedure ID (stem name without extension, e.g. "macro-calc")
    pub procedure: String,

    /// JSON object to pass as stdin to the procedure (default: {})
    #[arg(long, value_name = "JSON")]
    pub input: Option<String>,

    /// Override the wall-clock timeout in milliseconds
    #[arg(long, value_name = "MS")]
    pub timeout_ms: Option<u64>,

    /// Allow running non-WASM procedures from unsigned bundles (dev/testing only)
    #[arg(long)]
    pub allow_unsigned: bool,
}

pub async fn run(args: RunArgs, _ctx: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    #[cfg(not(feature = "procedures"))]
    {
        let _ = (args, pack);
        anyhow::bail!(
            "procedure execution requires the 'procedures' feature (enabled by default in dkp)"
        );
    }

    #[cfg(feature = "procedures")]
    {
        use dkp_core::procedures;

        let defs = procedures::list(&pack)?;
        let def = defs
            .into_iter()
            .find(|d| d.id == args.procedure)
            .ok_or_else(|| dkp_core::DkpError::ProcedureNotFound {
                id: args.procedure.clone(),
                pack: pack.manifest.name.clone(),
            })?;

        let input: serde_json::Value = match &args.input {
            Some(s) => serde_json::from_str(s)
                .map_err(|e| anyhow::anyhow!("--input is not valid JSON: {e}"))?,
            None => serde_json::Value::Object(Default::default()),
        };

        let opts = procedures::executor::RunOptions {
            input,
            timeout_ms: args.timeout_ms,
            allow_unsigned: args.allow_unsigned,
        };

        let result =
            tokio::task::spawn_blocking(move || procedures::executor::run(&pack, &def, opts))
                .await
                .map_err(|e| anyhow::anyhow!("procedure task panicked: {e}"))??;

        println!("{}", serde_json::to_string_pretty(&result)?);

        Ok(())
    }
}
