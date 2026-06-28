use anyhow::Result;
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use dkp_core::{search::SearchIndex, Pack};

use comfy_table::{presets::UTF8_FULL, Table};

use crate::{cli::CmdCtx, output::Render};

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Path to the DKP bundle (omit when using --registry)
    pub pack: Option<PathBuf>,

    /// Search query
    pub query: Option<String>,

    /// Search the registry index instead of a local pack
    #[arg(long)]
    pub registry: bool,

    /// Narrow results to a specific asset type
    #[arg(long, value_name = "TYPE")]
    pub r#type: Option<String>,

    /// Maximum number of results
    #[arg(long, default_value = "10")]
    pub limit: usize,

    // Registry-only flags
    #[arg(long, value_name = "NAME", requires = "registry")]
    pub domain: Option<String>,

    #[arg(long, value_name = "LEVEL", requires = "registry")]
    pub conformance: Option<String>,
}

#[derive(Debug, Serialize)]
struct SearchResults {
    query: String,
    results: Vec<dkp_core::search::SearchResult>,
}

impl Render for SearchResults {
    fn render_plain(&self) -> String {
        if self.results.is_empty() {
            return format!("No results for \"{}\"\n", self.query);
        }
        let mut out = String::new();
        for (i, r) in self.results.iter().enumerate() {
            out.push_str(&format!(
                "[{}] {} (score: {:.2})\n  …{}\n  Source: {}\n\n",
                r.id, r.title, r.score, r.excerpt, r.source_file
            ));
            let _ = i;
        }
        out
    }

    fn render_table(&self) -> String {
        if self.results.is_empty() {
            return format!("No results for \"{}\"\n", self.query);
        }
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(["ID", "Type", "Title", "Score", "Excerpt"]);
        for r in &self.results {
            table.add_row([
                &r.id,
                &r.asset_type,
                &r.title,
                &format!("{:.2}", r.score),
                &r.excerpt,
            ]);
        }
        table.to_string()
    }
}

pub async fn run(args: SearchArgs, cli: &CmdCtx) -> Result<()> {
    if args.registry {
        // TODO: query registry search API
        anyhow::bail!("registry search not yet implemented");
    }

    let pack_path = args.pack.ok_or_else(|| {
        anyhow::anyhow!("pack path required for local search (usage: dkp search <pack> <query>)")
    })?;
    let query = args
        .query
        .ok_or_else(|| anyhow::anyhow!("query required (usage: dkp search <pack> <query>)"))?;
    let pack = Pack::open(&pack_path)?;
    let index = SearchIndex::build(&pack)?;

    let fetch_limit = if args.r#type.is_some() {
        args.limit * 10
    } else {
        args.limit
    };
    let mut results = index.search(&query, fetch_limit)?;
    if let Some(ref type_filter) = args.r#type {
        results.retain(|r| r.asset_type == type_filter.as_str());
        results.truncate(args.limit);
    }

    SearchResults { query, results }.print(cli.output);
    Ok(())
}
