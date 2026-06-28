use anyhow::Result;
use clap::Args;
use comfy_table::{presets::UTF8_FULL, Table};
use std::path::PathBuf;

use dkp_core::{procedures, Pack};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,
}

pub async fn run(args: ListArgs, _ctx: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let defs = procedures::list(&pack)?;

    if defs.is_empty() {
        println!("No procedures found in machine/procedures/");
        return Ok(());
    }

    let timeout_label = |_id: &str| -> String {
        pack.manifest
            .procedure_capabilities
            .as_ref()
            .and_then(|c| c.max_runtime_ms)
            .map(|ms| format!("{ms}ms"))
            .unwrap_or_else(|| "5000ms (default)".to_string())
    };

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(["ID", "WASM", "TITLE", "TIMEOUT"]);

    for def in &defs {
        let wasm_icon = if def.wasm_path.is_some() {
            "✓"
        } else {
            "✗"
        };
        let title = &def.schema.title;
        let timeout = timeout_label(&def.id);
        table.add_row([def.id.as_str(), wasm_icon, title.as_str(), timeout.as_str()]);
    }

    println!("{table}");
    Ok(())
}
