use anyhow::Result;
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use dkp_core::{search::index::SearchIndex, Pack};

use crate::cli::CmdCtx;
use crate::output::{OutputFormat, Render};

#[derive(Args, Debug)]
pub struct ChunkArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Query string
    pub query: String,

    /// Number of chunks to return
    #[arg(long, default_value = "5")]
    pub top: usize,

    /// Filter by minimum confidence score
    #[arg(long, value_name = "F")]
    pub min_confidence: Option<f64>,
}

#[derive(Debug, Serialize)]
struct ChunkResult {
    rank: usize,
    id: String,
    title: String,
    confidence: f32,
    excerpt: String,
}

#[derive(Debug, Serialize)]
struct ChunkOutput {
    query: String,
    results: Vec<ChunkResult>,
}

impl Render for ChunkOutput {
    fn render_plain(&self) -> String {
        if self.results.is_empty() {
            return format!("No chunks found for query: {}\n", self.query);
        }
        let mut out = String::new();
        for r in &self.results {
            out.push_str(&format!(
                "{}. [{}] {} (confidence: {:.2})\n   {}\n\n",
                r.rank, r.id, r.title, r.confidence, r.excerpt
            ));
        }
        out
    }
}

pub async fn run(args: ChunkArgs, cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let index = SearchIndex::build(&pack)?;

    // Search with extra headroom then filter to chunk type
    let raw = index.search(&args.query, args.top * 4)?;

    let results: Vec<ChunkResult> = raw
        .into_iter()
        .filter(|r| r.asset_type == "chunk")
        .filter(|r| {
            args.min_confidence
                .map(|min| r.score as f64 >= min)
                .unwrap_or(true)
        })
        .take(args.top)
        .enumerate()
        .map(|(i, r)| ChunkResult {
            rank: i + 1,
            id: r.id,
            title: r.title,
            confidence: r.score,
            excerpt: r.excerpt,
        })
        .collect();

    if results.is_empty() && cli.output == OutputFormat::Plain {
        eprintln!("No chunks matched '{}'", args.query);
        return Ok(());
    }

    ChunkOutput {
        query: args.query,
        results,
    }
    .print(cli.output);
    Ok(())
}
