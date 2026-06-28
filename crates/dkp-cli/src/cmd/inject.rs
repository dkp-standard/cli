use anyhow::{bail, Result};
use clap::Args;
use std::path::PathBuf;

use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct InjectArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Content scope: system-prompt | full | minimal | chunks
    #[arg(long, default_value = "system-prompt", value_name = "SCOPE")]
    pub scope: String,

    /// Wrapping format: markdown | xml | json
    #[arg(long, default_value = "markdown", value_name = "FMT")]
    pub format: String,

    /// Print estimated token count before the block
    #[arg(long)]
    pub count_tokens: bool,

    /// Truncate to fit within a token budget, dropping lowest-confidence chunks first
    #[arg(long, value_name = "N")]
    pub max_tokens: Option<u32>,
}

pub fn build_inject_content(pack: &Pack, scope: &str, max_tokens: Option<u32>) -> Result<String> {
    let content = match scope {
        "minimal" => build_minimal(pack)?,
        "system-prompt" => build_system_prompt(pack)?,
        "chunks" => build_chunks(pack, max_tokens)?,
        "full" => build_full(pack, max_tokens)?,
        other => bail!(
            "unknown scope '{}'. Valid scopes: system-prompt, full, minimal, chunks",
            other
        ),
    };
    let budget = max_tokens.unwrap_or(u32::MAX);
    Ok(truncate_to_budget(content, budget))
}

pub fn wrap_inject_output(
    pack: &Pack,
    scope: &str,
    format: &str,
    content: String,
) -> Result<String> {
    match format {
        "markdown" => Ok(wrap_markdown(pack, scope, &content)),
        "xml" => Ok(wrap_xml(pack, scope, &content)),
        "json" => wrap_json(pack, scope, &content),
        other => bail!(
            "unknown format '{}'. Valid formats: markdown, xml, json",
            other
        ),
    }
}

pub async fn run(args: InjectArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    let content = build_inject_content(&pack, &args.scope, args.max_tokens)?;

    if args.count_tokens {
        let estimated = estimate_tokens(&content);
        eprintln!("Estimated tokens: ~{estimated}");
    }

    let output = wrap_inject_output(&pack, &args.scope, &args.format, content)?;

    println!("{output}");
    Ok(())
}

// ── Scope builders ───────────────────────────────────────────────────────────

fn build_minimal(pack: &Pack) -> Result<String> {
    let sp = pack.load_system_prompt()?;
    Ok(sp.unwrap_or_else(|| format!("# {}\n\nNo system prompt present.", pack.manifest.name)))
}

fn build_system_prompt(pack: &Pack) -> Result<String> {
    let mut out = String::new();

    if let Some(sp) = pack.load_system_prompt()? {
        out.push_str(&sp);
        out.push_str("\n\n");
    }

    // Append compressed glossary
    if let Some(gf) = pack.load_glossary()? {
        if !gf.terms.is_empty() {
            out.push_str("## Key Terms\n\n");
            for t in &gf.terms {
                out.push_str(&format!("**{}**: {}\n", t.term, t.definition));
            }
        }
    }

    Ok(out)
}

fn build_chunks(pack: &Pack, max_tokens: Option<u32>) -> Result<String> {
    let mut chunks = pack.load_chunks()?;
    // Sort by confidence descending (highest first), drop lowest when over budget
    chunks.sort_by(|a, b| {
        b.confidence
            .unwrap_or(0.5)
            .partial_cmp(&a.confidence.unwrap_or(0.5))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut out = String::new();
    out.push_str("## Retrieval Chunks\n\n");
    for c in &chunks {
        let entry = format!("### {}\n\n{}\n\n", c.title, c.chunk_text);
        if let Some(budget) = max_tokens {
            let current_tokens = estimate_tokens(&out) as u32;
            let entry_tokens = estimate_tokens(&entry) as u32;
            if current_tokens + entry_tokens > budget {
                break;
            }
        }
        out.push_str(&entry);
    }
    Ok(out)
}

fn build_full(pack: &Pack, max_tokens: Option<u32>) -> Result<String> {
    let mut out = String::new();

    if let Some(sp) = pack.load_system_prompt()? {
        out.push_str(&sp);
        out.push_str("\n\n");
    }

    if let Some(gf) = pack.load_glossary()? {
        if !gf.terms.is_empty() {
            out.push_str("## Glossary\n\n");
            for t in &gf.terms {
                out.push_str(&format!("**{}** ({}): {}\n", t.term, t.id, t.definition));
                if !t.aliases.is_empty() {
                    out.push_str(&format!("  *Aliases: {}*\n", t.aliases.join(", ")));
                }
            }
            out.push('\n');
        }
    }

    if let Some(rf) = pack.load_rules()? {
        if !rf.rules.is_empty() {
            out.push_str("## Domain Rules\n\n");
            for r in &rf.rules {
                out.push_str(&format!("**{}** [{}]: {}\n", r.title, r.id, r.description));
            }
            out.push('\n');
        }
    }

    if let Some(cf) = pack.load_constraints()? {
        let all: Vec<_> = cf.all_constraints().collect();
        if !all.is_empty() {
            out.push_str("## Constraints\n\n");
            for c in all {
                out.push_str(&format!("**{}** [{}]: {}\n", c.title, c.id, c.description));
            }
            out.push('\n');
        }
    }

    // Append chunks with budget awareness
    let chunks_section = build_chunks(pack, max_tokens)?;
    out.push_str(&chunks_section);

    Ok(out)
}

// ── Wrap helpers ─────────────────────────────────────────────────────────────

fn wrap_markdown(pack: &Pack, scope: &str, content: &str) -> String {
    format!(
        "<!-- DKP Context: {} v{} (scope: {}) -->\n\n{}",
        pack.manifest.name, pack.manifest.version, scope, content
    )
}

fn wrap_xml(pack: &Pack, scope: &str, content: &str) -> String {
    format!(
        "<dkp_context pack=\"{}\" version=\"{}\" scope=\"{}\">\n{}\n</dkp_context>",
        pack.manifest.name, pack.manifest.version, scope, content
    )
}

fn wrap_json(pack: &Pack, scope: &str, content: &str) -> Result<String> {
    let v = serde_json::json!({
        "pack": pack.manifest.name,
        "version": pack.manifest.version,
        "scope": scope,
        "content": content,
    });
    Ok(serde_json::to_string_pretty(&v)?)
}

// ── Utilities ────────────────────────────────────────────────────────────────

fn estimate_tokens(text: &str) -> usize {
    text.len().div_ceil(4)
}

fn truncate_to_budget(content: String, max_tokens: u32) -> String {
    if max_tokens == u32::MAX {
        return content;
    }
    let limit_chars = (max_tokens as usize) * 4;
    if content.len() <= limit_chars {
        return content;
    }
    // Truncate at a paragraph boundary near the limit
    let truncated = &content[..limit_chars];
    let cut = truncated.rfind("\n\n").unwrap_or(limit_chars);
    format!(
        "{}\n\n[...truncated to fit token budget...]",
        &content[..cut]
    )
}
