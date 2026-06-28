use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct TuiArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,
}

pub async fn run(args: TuiArgs, _cli: &CmdCtx) -> Result<()> {
    #[cfg(not(feature = "tui"))]
    {
        let _ = args;
        anyhow::bail!("TUI support was not compiled in. Rebuild with: cargo build --features tui");
    }
    #[cfg(feature = "tui")]
    {
        tokio::task::block_in_place(|| run_tui(args.pack))
    }
}

#[cfg(feature = "tui")]
mod app;
#[cfg(feature = "tui")]
mod event;
#[cfg(feature = "tui")]
mod panels;
#[cfg(feature = "tui")]
mod ui;

#[cfg(feature = "tui")]
fn run_tui(pack_path: PathBuf) -> Result<()> {
    use crossterm::{
        cursor, execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::{io, time::Duration};

    let state = build_state(pack_path)?;

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = terminal::disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
        }
    }
    let _restore = Restore;

    let mut state = state;
    let tick = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui::draw(f, &state))?;
        if let Some(ev) = event::next_event(tick)? {
            state.handle_event(ev);
        }
        if state.should_quit {
            break;
        }
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn build_state(pack_path: PathBuf) -> Result<app::AppState> {
    use dkp_core::{search::SearchIndex, Pack};

    let pack = Pack::open(&pack_path)?;
    let search_index = SearchIndex::build(&pack)?;

    let glossary_count = pack.load_glossary()?.map(|g| g.terms.len()).unwrap_or(0);
    let rules_count = pack.load_rules()?.map(|r| r.rules.len()).unwrap_or(0);
    let chunks = pack.load_chunks()?;
    let chunk_count = chunks.len();
    let eval_cases = pack.load_eval_set()?;
    let eval_count = eval_cases.len();

    let mcp_manifest: Option<dkp_core::types::mcp_manifest::McpManifest> =
        if pack.has_mcp_manifest() {
            let path = pack.machine_dir().join("mcp_manifest.json");
            let bytes = std::fs::read(path)?;
            Some(serde_json::from_slice(&bytes)?)
        } else {
            None
        };

    let mcp_config = pack.manifest.mcp.clone();

    let asset_summary = app::AssetSummary {
        glossary_count,
        chunk_count,
        rule_count: rules_count,
        eval_count,
        has_graph: pack.has_knowledge_graph(),
        has_skills: pack.has_skills(),
        has_l10n: pack.has_l10n(),
        has_mcp_manifest: pack.has_mcp_manifest(),
        has_assets_file: pack.has_assets(),
        has_cross_refs: pack.has_cross_refs(),
        has_okf: pack.has_okf(),
    };

    Ok(app::AppState {
        pack_name: pack.manifest.name.clone(),
        pack_version: pack.manifest.version.clone(),
        pack_domain: pack.manifest.domain.clone(),
        pack_update_date: pack.manifest.update_date.clone(),
        active_panel: app::Panel::Assets,
        detail_view: None,
        asset_summary,
        assets_selected: 0,
        search_input: String::new(),
        search_input_focused: false,
        search_results: Vec::new(),
        search_selected: 0,
        search_index,
        chunks,
        chunks_selected: 0,
        chunks_offset: 0,
        eval_cases,
        eval_selected: 0,
        eval_offset: 0,
        mcp_config,
        mcp_manifest,
        mcp_selected: 0,
        should_quit: false,
    })
}
