use anyhow::{bail, Result};
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;

use dkp_core::{procedures, Pack};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,
}

pub async fn run(args: ValidateArgs, _ctx: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    if !pack.has_procedures() {
        println!("No machine/procedures/ directory found — nothing to validate.");
        return Ok(());
    }

    let errors = procedures::validate_all(&pack)?;

    if errors.is_empty() {
        println!("{} All procedure checks passed.", "✓".green());
        return Ok(());
    }

    eprintln!("{} Procedure validation failed:\n", "✗".red());
    for e in &errors {
        eprintln!("  • {e}");
    }
    bail!("{} procedure error(s) found", errors.len());
}
