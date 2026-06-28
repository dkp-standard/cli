use anyhow::Result;
use clap::Args;
use serde::Serialize;
use std::path::{Path, PathBuf};

use dkp_core::Pack;

use crate::{cli::CmdCtx, output::Render};

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Root directory to scan for DKP packs (or path to a single pack)
    pub root: PathBuf,

    /// Filter by domain name
    #[arg(long, value_name = "NAME")]
    pub domain: Option<String>,

    /// Filter by tier tag (e.g. starter, pro)
    #[arg(long, value_name = "TIER")]
    pub tier: Option<String>,
}

#[derive(Debug, Serialize)]
struct PackList {
    packs: Vec<PackSummary>,
}

#[derive(Debug, Serialize)]
struct PackSummary {
    domain: String,
    name: String,
    version: String,
    path: String,
    tags: Vec<String>,
    terms: usize,
    rules: usize,
    chunks: usize,
    has_okf: bool,
    has_eval: bool,
    has_graph: bool,
}

impl Render for PackList {
    fn render_plain(&self) -> String {
        if self.packs.is_empty() {
            return "No packs found.\n".to_string();
        }
        let mut out = format!(
            "{:<20} {:<40} {:<12} {}\n",
            "Domain", "Pack Name", "Version", "Assets"
        );
        out.push_str(&"─".repeat(82));
        out.push('\n');
        for p in &self.packs {
            let name = if p.name.len() > 38 {
                format!("{}…", &p.name[..37])
            } else {
                p.name.clone()
            };
            let assets = format!("{}t {}r {}c", p.terms, p.rules, p.chunks);
            out.push_str(&format!(
                "{:<20} {:<40} {:<12} {}\n",
                p.domain, name, p.version, assets
            ));
        }
        out
    }

    fn render_table(&self) -> String {
        use comfy_table::{presets::UTF8_FULL, Table};
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header([
            "Domain", "Name", "Version", "Terms", "Rules", "Chunks", "OKF", "Eval", "Graph",
        ]);
        for p in &self.packs {
            table.add_row([
                p.domain.as_str(),
                p.name.as_str(),
                p.version.as_str(),
                &p.terms.to_string(),
                &p.rules.to_string(),
                &p.chunks.to_string(),
                if p.has_okf { "✓" } else { "-" },
                if p.has_eval { "✓" } else { "-" },
                if p.has_graph { "✓" } else { "-" },
            ]);
        }
        if self.packs.is_empty() {
            return "No packs found.\n".to_string();
        }
        table.to_string()
    }
}

fn pack_to_summary(pack: &Pack, path: &Path) -> PackSummary {
    PackSummary {
        domain: pack.manifest.domain.clone(),
        name: pack.manifest.name.clone(),
        version: pack.manifest.version.clone(),
        path: path.display().to_string(),
        tags: pack.manifest.tags.clone(),
        terms: pack
            .load_glossary()
            .ok()
            .flatten()
            .map_or(0, |g| g.terms.len()),
        rules: pack
            .load_rules()
            .ok()
            .flatten()
            .map_or(0, |r| r.rules.len()),
        chunks: pack.load_chunks().map(|v| v.len()).unwrap_or(0),
        has_okf: pack.has_okf(),
        has_eval: pack.has_eval_set(),
        has_graph: pack.has_knowledge_graph(),
    }
}

fn passes_filters(pack: &Pack, domain: &Option<String>, tier: &Option<String>) -> bool {
    if let Some(ref d) = domain {
        if !pack.manifest.domain.eq_ignore_ascii_case(d) {
            return false;
        }
    }
    if let Some(ref t) = tier {
        if !pack
            .manifest
            .tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(t))
        {
            return false;
        }
    }
    true
}

pub async fn run(args: ListArgs, cli: &CmdCtx) -> Result<()> {
    let mut packs = Vec::new();

    // If root is itself a valid pack, list just that one
    if let Ok(pack) = Pack::open(&args.root) {
        if passes_filters(&pack, &args.domain, &args.tier) {
            packs.push(pack_to_summary(&pack, &args.root));
        }
    } else if args.root.is_dir() {
        for entry in std::fs::read_dir(&args.root)? {
            let path = entry?.path();
            if !path.is_dir() {
                continue;
            }
            // Try direct child first
            if let Ok(pack) = Pack::open(&path) {
                if passes_filters(&pack, &args.domain, &args.tier) {
                    packs.push(pack_to_summary(&pack, &path));
                }
                continue;
            }
            // Otherwise scan one level deeper (domain/pack-name layout)
            if let Ok(children) = std::fs::read_dir(&path) {
                for child in children {
                    let child_path = child?.path();
                    if child_path.is_dir() {
                        if let Ok(pack) = Pack::open(&child_path) {
                            if passes_filters(&pack, &args.domain, &args.tier) {
                                packs.push(pack_to_summary(&pack, &child_path));
                            }
                        }
                    }
                }
            }
        }
    } else {
        anyhow::bail!("{} is not a valid pack or directory", args.root.display());
    }

    packs.sort_by(|a, b| a.domain.cmp(&b.domain).then(a.name.cmp(&b.name)));
    PackList { packs }.print(cli.output);
    Ok(())
}
