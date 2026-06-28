use anyhow::{Context, Result};
use clap::Args;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Pack name (e.g. "Nutrition for Men")
    pub name: String,

    /// Top-level domain category (e.g. "Health", "Finance")
    #[arg(long)]
    pub domain: String,

    /// Output directory (default: ./<name-slug>/)
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Also scaffold optional recommended assets: eval_set.jsonl,
    /// knowledge_graph.json, human/handbook.md, README.md, CHANGELOG.md
    #[arg(long)]
    pub extras: bool,

    /// Overwrite if directory already exists
    #[arg(long)]
    pub force: bool,

    /// Write placeholder stub content for machine/human assets (default: true).
    /// Set to false when called programmatically so LLM generation starts clean.
    #[arg(skip)]
    pub stubs: bool,
}

pub async fn run(args: InitArgs, ctx: &CmdCtx) -> Result<()> {
    let slug = slugify(&args.name);
    let out = args.out.unwrap_or_else(|| PathBuf::from(&slug));

    if out.exists() && !args.force {
        anyhow::bail!(
            "directory '{}' already exists; use --force to overwrite",
            out.display()
        );
    }

    let today = today_iso8601();
    let name = &args.name;
    let domain = &args.domain;

    write_file(
        &out.join("manifest.json"),
        &manifest_json(name, domain, &today),
    )?;

    let machine = out.join("machine");
    if args.stubs {
        write_file(
            &machine.join("system_prompt.md"),
            &system_prompt_md(name, domain),
        )?;
        write_file(&machine.join("glossary.json"), GLOSSARY_JSON)?;
        write_file(&machine.join("rules.json"), RULES_JSON)?;
        write_file(&machine.join("ontology.json"), ONTOLOGY_JSON)?;
        write_file(&machine.join("constraints.json"), CONSTRAINTS_JSON)?;
        write_file(&machine.join("decision_trees.json"), DECISION_TREES_JSON)?;
        write_file(
            &machine.join("retrieval_chunks.jsonl"),
            RETRIEVAL_CHUNKS_JSONL,
        )?;
    } else {
        // Create the directory without stubs so LLM generation starts clean
        std::fs::create_dir_all(&machine)
            .with_context(|| format!("creating directory '{}'", machine.display()))?;
    }

    let evidence = out.join("evidence");
    write_file(&evidence.join("sources.csv"), SOURCES_CSV)?;
    write_file(&evidence.join("rights_log.csv"), RIGHTS_LOG_CSV)?;
    write_file(&evidence.join("review_notes.md"), REVIEW_NOTES_MD)?;

    let okf = out.join("okf");
    write_file(&okf.join("index.md"), &okf_index_md(name, &today))?;
    for subdir in &[
        "terms",
        "rules",
        "constraints",
        "procedures",
        "chunks",
        "ontology",
    ] {
        write_file(&okf.join(subdir).join(".gitkeep"), "")?;
    }

    write_file(&out.join("README.md"), &readme_md(name, domain))?;

    if args.extras {
        write_file(&machine.join("eval_set.jsonl"), "")?;
        write_file(&machine.join("knowledge_graph.json"), KNOWLEDGE_GRAPH_JSON)?;
        if args.stubs {
            write_file(
                &out.join("human").join("handbook.md"),
                &handbook_md(name, domain),
            )?;
        } else {
            std::fs::create_dir_all(out.join("human"))
                .with_context(|| "creating human/ directory".to_string())?;
        }
        write_file(&out.join("CHANGELOG.md"), &changelog_md(&today))?;
    }

    if !ctx.quiet {
        println!("Initialized DKP pack at '{}'", out.display());
        println!();
        println!("Next steps:");
        println!("  1. Fill in the placeholder content in machine/ and evidence/");
        println!("  2. Run: dkp validate {}", out.display());
    }

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating directory '{}'", parent.display()))?;
    }
    std::fs::write(path, content).with_context(|| format!("writing '{}'", path.display()))?;
    Ok(())
}

fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn today_iso8601() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    // Gregorian calendar calculation from days since 1970-01-01
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

// ── Templates ────────────────────────────────────────────────────────────────

fn manifest_json(name: &str, domain: &str, today: &str) -> String {
    format!(
        r#"{{
  "spec": "1.0.0",
  "name": "{name}",
  "version": "0.1.0",
  "domain": "{domain}",
  "audience": "TODO: describe the target user or agent type",
  "intended_use": "TODO: describe authorized use cases (e.g. LLM context injection, RAG retrieval)",
  "known_limitations": "TODO: describe scope gaps and what this pack does not cover",
  "update_date": "{today}",
  "compatibility": []
}}
"#
    )
}

fn system_prompt_md(name: &str, domain: &str) -> String {
    format!(
        "# {name}\n\
         \n\
         You are a knowledgeable assistant specialized in the **{domain}** domain.\n\
         This Domain Knowledge Pack provides authoritative guidance on {domain} topics.\n\
         \n\
         ## Domain Scope\n\
         \n\
         TODO: describe what topics this pack covers.\n\
         \n\
         ## Behavioral Guidelines\n\
         \n\
         - Apply the domain rules and constraints provided in this pack.\n\
         - Cite glossary terms when using domain-specific language.\n\
         - Refer to retrieval chunks for specific facts and evidence.\n\
         - Acknowledge the boundaries of this pack's scope when asked out-of-scope questions.\n\
         \n\
         ## Known Limitations\n\
         \n\
         TODO: list what this pack does not cover.\n"
    )
}

const GLOSSARY_JSON: &str = r#"{
  "terms": [
    {
      "id": "term-001",
      "term": "Example Term",
      "definition": "Replace with your first domain-specific term and its authoritative definition.",
      "stability": "stable",
      "source_ref": "src-001"
    }
  ]
}
"#;

const RULES_JSON: &str = r#"{
  "rules": [
    {
      "id": "rule-001",
      "title": "Example Affirmative Rule",
      "description": "Replace with a rule describing what agents SHOULD or MUST do in this domain.",
      "polarity": "affirmative",
      "stability": "stable",
      "source_ref": "src-001"
    },
    {
      "id": "rule-002",
      "title": "Example Prohibitive Rule",
      "description": "Replace with a rule describing what agents MUST NOT do in this domain.",
      "polarity": "prohibitive",
      "stability": "stable",
      "source_ref": "src-001"
    }
  ]
}
"#;

const ONTOLOGY_JSON: &str = r#"{
  "entity_types": [
    {
      "id": "example-entity",
      "name": "Example Entity",
      "description": "Replace with a domain entity type and its relevant attributes.",
      "attributes": ["attribute_one", "attribute_two"]
    }
  ]
}
"#;

const CONSTRAINTS_JSON: &str = r#"{
  "edge_cases": [
    {
      "id": "edge-001",
      "title": "Example Edge Case",
      "description": "Replace with a situation where standard rules require special handling."
    }
  ],
  "anti_patterns": [
    {
      "id": "anti-001",
      "title": "Example Anti-Pattern",
      "description": "Replace with a common mistake or misconception to avoid in this domain."
    }
  ],
  "hard_limits": [
    {
      "id": "hard-001",
      "title": "Example Hard Limit",
      "description": "Replace with an absolute boundary that must never be crossed in this domain."
    }
  ]
}
"#;

const DECISION_TREES_JSON: &str = r#"{
  "trees": [
    {
      "id": "tree-001",
      "title": "Example Decision Procedure",
      "description": "Replace with a traversable decision procedure for a common domain question.",
      "root": {
        "question": "Replace with the first branching question.",
        "branches": [
          {
            "condition": "Yes",
            "next": { "answer": "Replace with the recommendation when the condition is true." }
          },
          {
            "condition": "No",
            "next": { "answer": "Replace with the recommendation when the condition is false." }
          }
        ]
      }
    }
  ]
}
"#;

const RETRIEVAL_CHUNKS_JSONL: &str = "{\"id\":\"chunk-001\",\"title\":\"Example Retrieval Chunk\",\"chunk_text\":\"Replace with a self-contained, standalone piece of domain knowledge. Each chunk should be independently useful when retrieved by a RAG pipeline without additional context.\",\"tags\":[\"example\",\"placeholder\"],\"source_ref\":\"src-001\",\"confidence\":0.9,\"stability\":\"stable\"}\n";

const SOURCES_CSV: &str = "id,title,url,retrieved_date,license,notes\n\
src-001,Example Source,https://example.com,2024-01-01,TODO,Replace with your first authoritative source\n";

const RIGHTS_LOG_CSV: &str = "id,source_id,rights_holder,license_type,granted_date,expiry_date,notes\n\
rr-001,src-001,TODO Rights Holder,TODO License,2024-01-01,perpetual,Replace with actual rights information\n";

const REVIEW_NOTES_MD: &str = "# Review Notes\n\
\n\
TODO: Document editorial decisions, source quality assessments, and content review observations here.\n\
\n\
When the pack has passed editorial review (Gates 1–3, 5, 6), add a completed sign-off\n\
section with the reviewer's name, review date, gates attested, and status.\n\
Run `dkp validate` to confirm the conformance level after adding the sign-off.\n";

fn okf_index_md(name: &str, today: &str) -> String {
    format!(
        "---\n\
         type: bundle-index\n\
         pack: \"{name}\"\n\
         generated: \"{today}\"\n\
         ---\n\
         \n\
         # {name} — OKF Bundle Index\n\
         \n\
         This file is the OKF layer index for the **{name}** Domain Knowledge Pack.\n\
         Run `dkp okf export` to populate the concept files in subdirectories.\n"
    )
}

const KNOWLEDGE_GRAPH_JSON: &str = r#"{
  "nodes": [],
  "edges": []
}
"#;

fn handbook_md(name: &str, domain: &str) -> String {
    format!(
        "# {name} Handbook\n\
         \n\
         A comprehensive reference for the **{domain}** domain.\n\
         \n\
         ## Introduction\n\
         \n\
         TODO: Introduce the domain and the purpose of this handbook.\n\
         \n\
         ## Key Concepts\n\
         \n\
         TODO: Explain the foundational concepts a reader needs to understand.\n\
         \n\
         ## Domain Rules\n\
         \n\
         TODO: Summarize the operational rules in human-readable prose.\n\
         \n\
         ## Common Questions\n\
         \n\
         TODO: Address frequently asked questions in this domain.\n"
    )
}

fn readme_md(name: &str, domain: &str) -> String {
    format!(
        "# {name}\n\
         \n\
         A Domain Knowledge Pack for the **{domain}** domain.\n\
         \n\
         ## Quick Start\n\
         \n\
         ```bash\n\
         dkp info .\n\
         dkp validate .\n\
         dkp inject . --scope system-prompt\n\
         ```\n\
         \n\
         ## Contents\n\
         \n\
         TODO: describe what's in this pack.\n"
    )
}

fn changelog_md(today: &str) -> String {
    format!(
        "# Changelog\n\
         \n\
         ## [0.1.0] - {today}\n\
         \n\
         - Initial release.\n"
    )
}
