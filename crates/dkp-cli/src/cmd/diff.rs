use anyhow::Result;
use clap::Args;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// First pack directory or archive
    pub pack_a: PathBuf,

    /// Second pack directory or archive
    pub pack_b: PathBuf,

    /// Diff only one asset type (term, chunk, rule, constraint)
    #[arg(long, value_name = "TYPE")]
    pub r#type: Option<String>,

    /// Content-drift percentage threshold for "modified" classification
    #[arg(long, default_value = "20")]
    pub threshold: u32,
}

/// A flat concept record: id + type + a content string for drift detection.
struct Concept {
    id: String,
    asset_type: String,
    content: String,
}

fn load_concepts(pack: &Pack, filter: Option<&str>) -> anyhow::Result<Vec<Concept>> {
    let mut concepts: Vec<Concept> = Vec::new();

    let want = |t: &str| filter.is_none_or(|f| f.eq_ignore_ascii_case(t));

    if want("term") {
        if let Ok(Some(gf)) = pack.load_glossary() {
            for t in gf.terms {
                concepts.push(Concept {
                    id: t.id.clone(),
                    asset_type: "term".into(),
                    content: format!("{} {}", t.term, t.definition),
                });
            }
        }
    }

    if want("rule") {
        if let Ok(Some(rf)) = pack.load_rules() {
            for r in rf.rules {
                concepts.push(Concept {
                    id: r.id.clone(),
                    asset_type: "rule".into(),
                    content: format!("{} {}", r.title, r.description),
                });
            }
        }
    }

    if want("constraint") {
        if let Ok(Some(cf)) = pack.load_constraints() {
            for c in cf.all_constraints() {
                concepts.push(Concept {
                    id: c.id.clone(),
                    asset_type: "constraint".into(),
                    content: format!("{} {}", c.title, c.description),
                });
            }
        }
    }

    if want("chunk") {
        if let Ok(chunks) = pack.load_chunks() {
            for c in chunks {
                concepts.push(Concept {
                    id: c.id.clone(),
                    asset_type: "chunk".into(),
                    content: format!("{} {}", c.title, c.chunk_text),
                });
            }
        }
    }

    Ok(concepts)
}

/// Returns a rough drift percentage between two strings (Jaccard on word sets).
fn drift_percent(a: &str, b: &str) -> u32 {
    if a == b {
        return 0;
    }
    let words_a: HashSet<&str> = a.split_whitespace().collect();
    let words_b: HashSet<&str> = b.split_whitespace().collect();
    if words_a.is_empty() && words_b.is_empty() {
        return 0;
    }
    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();
    let similarity = intersection as f64 / union as f64;
    ((1.0 - similarity) * 100.0).round() as u32
}

pub async fn run(args: DiffArgs, _cli: &CmdCtx) -> Result<()> {
    let pack_a = Pack::open(&args.pack_a)?;
    let pack_b = Pack::open(&args.pack_b)?;

    let concepts_a: BTreeMap<String, Concept> = load_concepts(&pack_a, args.r#type.as_deref())?
        .into_iter()
        .map(|c| (format!("{}:{}", c.asset_type, c.id), c))
        .collect();

    let concepts_b: BTreeMap<String, Concept> = load_concepts(&pack_b, args.r#type.as_deref())?
        .into_iter()
        .map(|c| (format!("{}:{}", c.asset_type, c.id), c))
        .collect();

    let keys_a: HashSet<&str> = concepts_a.keys().map(|s| s.as_str()).collect();
    let keys_b: HashSet<&str> = concepts_b.keys().map(|s| s.as_str()).collect();

    let mut added: Vec<&str> = keys_b.difference(&keys_a).copied().collect();
    let mut removed: Vec<&str> = keys_a.difference(&keys_b).copied().collect();
    let mut modified: Vec<(&str, u32)> = Vec::new();
    let mut unchanged: usize = 0;

    for key in keys_a.intersection(&keys_b) {
        let ca = &concepts_a[*key];
        let cb = &concepts_b[*key];
        let drift = drift_percent(&ca.content, &cb.content);
        if drift >= args.threshold {
            modified.push((key, drift));
        } else {
            unchanged += 1;
        }
    }

    added.sort_unstable();
    removed.sort_unstable();
    modified.sort_by_key(|b| std::cmp::Reverse(b.1));

    println!(
        "Diff: {} v{}  →  {} v{}\n",
        pack_a.manifest.name,
        pack_a.manifest.version,
        pack_b.manifest.name,
        pack_b.manifest.version,
    );
    println!("  Added:     {}", added.len());
    println!("  Removed:   {}", removed.len());
    println!(
        "  Modified:  {} (drift ≥ {}%)",
        modified.len(),
        args.threshold
    );
    println!("  Unchanged: {unchanged}");

    if !added.is_empty() {
        println!("\n+ Added");
        for key in &added {
            println!("  + {key}");
        }
    }

    if !removed.is_empty() {
        println!("\n- Removed");
        for key in &removed {
            println!("  - {key}");
        }
    }

    if !modified.is_empty() {
        println!("\n~ Modified");
        for (key, drift) in &modified {
            println!("  ~ {key}  ({drift}% drift)");
        }
    }

    // Eval delta warning
    let min_delta = pack_b.manifest.min_eval_delta.unwrap_or(0.0);
    if min_delta > 0.0 && !modified.is_empty() {
        println!(
            "\nNote: manifest.min_eval_delta is {min_delta:.2} — run 'dkp eval' to confirm score improvement."
        );
    }

    Ok(())
}
