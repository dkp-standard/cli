use crate::cli::CmdCtx;
use anyhow::Result;
use clap::{Args, Subcommand};

pub mod list;
pub mod new;
pub mod validate;

#[derive(Args, Debug)]
pub struct ProceduresArgs {
    #[command(subcommand)]
    pub command: ProceduresCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProceduresCommands {
    /// List all procedures defined in machine/procedures/
    List(list::ListArgs),

    /// Validate procedure file completeness and schema correctness
    Validate(validate::ValidateArgs),

    /// Scaffold a new Rust WASI procedure project
    New(new::NewArgs),
}

pub async fn run(args: ProceduresArgs, ctx: &CmdCtx) -> Result<()> {
    match args.command {
        ProceduresCommands::List(a) => list::run(a, ctx).await,
        ProceduresCommands::Validate(a) => validate::run(a, ctx).await,
        ProceduresCommands::New(a) => new::run(a, ctx).await,
    }
}
