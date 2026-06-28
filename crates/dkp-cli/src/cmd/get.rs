use anyhow::{bail, Result};
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use dkp_core::Pack;

use crate::cli::CmdCtx;
use crate::output::{OutputFormat, Render};

#[derive(Args, Debug)]
pub struct GetArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Asset type: term | rule | chunk | constraint | entity | eval | graph | cross-ref | system-prompt
    #[arg(value_name = "TYPE")]
    pub asset_type: String,

    /// Asset ID or title to retrieve (omit for all assets of this type)
    pub id: Option<String>,

    /// Fetch by ID field instead of title search
    #[arg(long)]
    pub by_id: bool,
}

// ── Output wrappers ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct GetOutput {
    asset_type: String,
    items: Vec<serde_json::Value>,
}

impl Render for GetOutput {
    fn render_plain(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            out.push_str(&render_item_plain(item));
            out.push('\n');
        }
        out
    }
}

fn render_item_plain(v: &serde_json::Value) -> String {
    let mut out = String::new();
    // Print id + title/term/name on first line, then body fields
    let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("?");
    let title = v
        .get("title")
        .or_else(|| v.get("term"))
        .or_else(|| v.get("name"))
        .and_then(|x| x.as_str())
        .unwrap_or("");
    out.push_str(&format!("[{id}] {title}\n"));

    for key in ["definition", "description", "chunk_text"] {
        if let Some(body) = v.get(key).and_then(|x| x.as_str()) {
            out.push_str(&format!("  {body}\n"));
            break;
        }
    }
    if let Some(tags) = v.get("tags").and_then(|x| x.as_array()) {
        if !tags.is_empty() {
            let t: Vec<&str> = tags.iter().filter_map(|x| x.as_str()).collect();
            out.push_str(&format!("  tags: {}\n", t.join(", ")));
        }
    }
    out
}

// ── Entry point ──────────────────────────────────────────────────────────────

pub fn get_assets(
    pack: &Pack,
    asset_type: &str,
    id: Option<&str>,
    by_id: bool,
) -> Result<Vec<serde_json::Value>> {
    let asset_type = asset_type.to_lowercase();
    let id_owned = id.map(String::from);
    let items: Vec<serde_json::Value> = match asset_type.as_str() {
        "term" => {
            let gf = pack.load_glossary()?.unwrap_or_else(|| dkp_core::types::glossary::GlossaryFile { terms: vec![] });
            filter_items(gf.terms, &id_owned, by_id, |t| t.id.clone(), |t| t.term.clone())
        }
        "rule" => {
            let rf = pack.load_rules()?.unwrap_or_else(|| dkp_core::types::rules::RulesFile { rules: vec![] });
            filter_items(rf.rules, &id_owned, by_id, |r| r.id.clone(), |r| r.title.clone())
        }
        "chunk" => {
            let chunks = pack.load_chunks()?;
            filter_items(chunks, &id_owned, by_id, |c| c.id.clone(), |c| c.title.clone())
        }
        "constraint" => {
            let cf = pack.load_constraints()?;
            let all: Vec<_> = cf.map(|c| c.all_constraints().cloned().collect()).unwrap_or_default();
            filter_items(all, &id_owned, by_id, |c| c.id.clone(), |c| c.title.clone())
        }
        "entity" => {
            let of = pack.load_ontology()?.unwrap_or_else(|| dkp_core::types::ontology::OntologyFile { entity_types: vec![] });
            filter_items(of.entity_types, &id_owned, by_id, |e| e.id.clone(), |e| e.name.clone())
        }
        "eval" => {
            let evals = pack.load_eval_set()?;
            let filtered: Vec<_> = match &id_owned {
                None => evals,
                Some(q) => {
                    let q = q.to_lowercase();
                    evals.into_iter().filter(|e| e.query.to_lowercase().contains(&q)).collect()
                }
            };
            filtered.into_iter().map(|e| serde_json::to_value(e).unwrap_or(serde_json::Value::Null)).collect()
        }
        "graph" => {
            match pack.load_graph()? {
                None => vec![],
                Some(g) => vec![serde_json::to_value(&g)?],
            }
        }
        "cross-ref" => {
            use dkp_core::types::cross_refs::CrossRefsFile;
            let path = pack.machine_file("cross_refs.json");
            if path.exists() {
                let bytes = std::fs::read(&path)?;
                let crf: CrossRefsFile = serde_json::from_slice(&bytes)?;
                filter_items(crf.cross_refs, &id_owned, by_id, |c| c.pack_name.clone(), |c| c.pack_name.clone())
            } else {
                vec![]
            }
        }
        "system-prompt" => {
            match pack.load_system_prompt()? {
                None => vec![],
                Some(text) => vec![serde_json::json!({ "id": "system_prompt", "content": text })],
            }
        }
        other => bail!("unknown asset type '{}'. Valid types: term, rule, chunk, constraint, entity, eval, graph, cross-ref, system-prompt", other),
    };
    Ok(items)
}

pub async fn run(args: GetArgs, cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let asset_type = args.asset_type.to_lowercase();

    let items: Vec<serde_json::Value> = match asset_type.as_str() {
        "term" => {
            let gf = pack.load_glossary()?.unwrap_or_else(|| dkp_core::types::glossary::GlossaryFile { terms: vec![] });
            filter_items(gf.terms, &args.id, args.by_id, |t| t.id.clone(), |t| t.term.clone())
        }
        "rule" => {
            let rf = pack.load_rules()?.unwrap_or_else(|| dkp_core::types::rules::RulesFile { rules: vec![] });
            filter_items(rf.rules, &args.id, args.by_id, |r| r.id.clone(), |r| r.title.clone())
        }
        "chunk" => {
            let chunks = pack.load_chunks()?;
            filter_items(chunks, &args.id, args.by_id, |c| c.id.clone(), |c| c.title.clone())
        }
        "constraint" => {
            let cf = pack.load_constraints()?;
            let all: Vec<_> = cf.map(|c| c.all_constraints().cloned().collect()).unwrap_or_default();
            filter_items(all, &args.id, args.by_id, |c| c.id.clone(), |c| c.title.clone())
        }
        "entity" => {
            let of = pack.load_ontology()?.unwrap_or_else(|| dkp_core::types::ontology::OntologyFile { entity_types: vec![] });
            filter_items(of.entity_types, &args.id, args.by_id, |e| e.id.clone(), |e| e.name.clone())
        }
        "eval" => {
            let evals = pack.load_eval_set()?;
            // EvalCase has no id field — filter by query substring
            let filtered: Vec<_> = match &args.id {
                None => evals,
                Some(q) => {
                    let q = q.to_lowercase();
                    evals.into_iter().filter(|e| e.query.to_lowercase().contains(&q)).collect()
                }
            };
            filtered.into_iter().map(|e| serde_json::to_value(e).unwrap_or(serde_json::Value::Null)).collect()
        }
        "graph" => {
            match pack.load_graph()? {
                None => vec![],
                Some(g) => {
                    let v = serde_json::to_value(&g)?;
                    vec![v]
                }
            }
        }
        "cross-ref" => {
            use dkp_core::types::cross_refs::CrossRefsFile;
            let path = pack.machine_file("cross_refs.json");
            if path.exists() {
                let bytes = std::fs::read(&path)?;
                let crf: CrossRefsFile = serde_json::from_slice(&bytes)?;
                filter_items(crf.cross_refs, &args.id, args.by_id, |c| c.pack_name.clone(), |c| c.pack_name.clone())
            } else {
                vec![]
            }
        }
        "system-prompt" => {
            match pack.load_system_prompt()? {
                None => vec![],
                Some(text) => vec![serde_json::json!({ "id": "system_prompt", "content": text })],
            }
        }
        other => bail!("unknown asset type '{}'. Valid types: term, rule, chunk, constraint, entity, eval, graph, cross-ref, system-prompt", other),
    };

    if items.is_empty() {
        if args.id.is_some() {
            eprintln!(
                "no {} found matching '{}'",
                asset_type,
                args.id.as_deref().unwrap_or("")
            );
        } else {
            eprintln!("no {} assets found in pack", asset_type);
        }
        return Ok(());
    }

    // system-prompt plain output is just the raw text
    if asset_type == "system-prompt" && cli.output == OutputFormat::Plain {
        if let Some(v) = items.first() {
            println!("{}", v["content"].as_str().unwrap_or(""));
            return Ok(());
        }
    }

    GetOutput { asset_type, items }.print(cli.output);
    Ok(())
}

fn filter_items<T: serde::Serialize>(
    items: Vec<T>,
    id_filter: &Option<String>,
    by_id: bool,
    get_id: impl Fn(&T) -> String,
    get_title: impl Fn(&T) -> String,
) -> Vec<serde_json::Value> {
    let filtered: Vec<T> = match id_filter {
        None => items,
        Some(query) => {
            let q = query.to_lowercase();
            items
                .into_iter()
                .filter(|item| {
                    if by_id {
                        get_id(item).to_lowercase() == q
                    } else {
                        get_title(item).to_lowercase().contains(&q)
                            || get_id(item).to_lowercase().contains(&q)
                    }
                })
                .collect()
        }
    };
    filtered
        .into_iter()
        .map(|item| serde_json::to_value(item).unwrap_or(serde_json::Value::Null))
        .collect()
}
