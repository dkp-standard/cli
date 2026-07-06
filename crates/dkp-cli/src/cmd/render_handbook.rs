use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use dkp_core::Pack;
use dkp_gen_core::render_handbook_formats;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct RenderHandbookArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,
}

/// Renders `human/handbook.md` to `handbook.pdf`/`handbook.epub` (DKP spec
/// §11.2), independent of a full `dkp generate`/`dkp build` run. A failure
/// to render either format prints a warning and exits 0 — these are
/// optional artifacts.
pub async fn run(args: RenderHandbookArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let human_dir = pack.root.join("human");
    let md_path = human_dir.join("handbook.md");

    let markdown = std::fs::read_to_string(&md_path).map_err(|_| {
        anyhow::anyhow!(
            "human/handbook.md not found at {} — nothing to render",
            md_path.display()
        )
    })?;

    let report = render_handbook_formats(&markdown, &human_dir)?;
    for w in &report.warnings {
        eprintln!("  ⚠ {w}");
    }
    if report.pdf_written {
        println!("  Written: {}", human_dir.join("handbook.pdf").display());
    }
    if report.epub_written {
        println!("  Written: {}", human_dir.join("handbook.epub").display());
    }
    Ok(())
}
