use anyhow::{bail, Result};
use clap::Args;
use std::path::PathBuf;

use dkp_core::{okf::exporter::export_okf, Pack};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ExportArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Target format: okf | langchain | llamaindex | openai-files | markdown | csv | anki
    pub format: String,

    /// Output directory
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,
}

pub async fn run(args: ExportArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let format = args.format.to_lowercase();

    match format.as_str() {
        "okf" => {
            let out_dir = args.out.unwrap_or_else(|| args.pack.join("okf"));
            eprintln!("Exporting OKF bundle to {} ...", out_dir.display());
            let stats = export_okf(&pack, &out_dir)?;
            println!("OKF export complete → {}", out_dir.display());
            println!("  Terms:       {}", stats.terms_written);
            println!("  Rules:       {}", stats.rules_written);
            println!("  Constraints: {}", stats.constraints_written);
            println!("  Chunks:      {}", stats.chunks_written);
            println!("  Entities:    {}", stats.ontology_written);
        }
        "langchain" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./langchain_export"));
            export_langchain(&pack, &out_dir)?;
            println!("LangChain export complete → {}", out_dir.display());
        }
        "llamaindex" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./llamaindex_export"));
            export_llamaindex(&pack, &out_dir)?;
            println!("LlamaIndex export complete → {}", out_dir.display());
        }
        "openai-files" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./openai_export"));
            export_openai_files(&pack, &out_dir)?;
            println!("OpenAI Files export complete → {}", out_dir.display());
        }
        "markdown" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./markdown_export"));
            export_markdown(&pack, &out_dir)?;
            println!("Markdown export complete → {}", out_dir.display());
        }
        "csv" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./csv_export"));
            export_csv(&pack, &out_dir)?;
            println!("CSV export complete → {}", out_dir.display());
        }
        "anki" => {
            let out_dir = args.out.unwrap_or_else(|| PathBuf::from("./anki_export"));
            export_anki(&pack, &out_dir)?;
            println!("Anki export complete → {}", out_dir.display());
        }
        other => bail!(
            "unknown format '{}'. Valid formats: okf, langchain, llamaindex, openai-files, markdown, csv, anki",
            other
        ),
    }

    Ok(())
}

// ── Format exporters ─────────────────────────────────────────────────────────

fn export_langchain(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let mut docs: Vec<serde_json::Value> = Vec::new();

    if let Some(gf) = pack.load_glossary()? {
        for t in &gf.terms {
            docs.push(serde_json::json!({
                "page_content": format!("{}: {}", t.term, t.definition),
                "metadata": { "id": t.id, "type": "term", "source": "machine/glossary.json" }
            }));
        }
    }
    for c in &pack.load_chunks()? {
        docs.push(serde_json::json!({
            "page_content": c.chunk_text,
            "metadata": { "id": c.id, "title": c.title, "type": "chunk", "source": "machine/retrieval_chunks.jsonl" }
        }));
    }
    if let Some(rf) = pack.load_rules()? {
        for r in &rf.rules {
            docs.push(serde_json::json!({
                "page_content": format!("{}: {}", r.title, r.description),
                "metadata": { "id": r.id, "type": "rule", "source": "machine/rules.json" }
            }));
        }
    }

    let out: String = docs
        .iter()
        .map(|d| serde_json::to_string(d).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(out_dir.join("documents.jsonl"), out)?;
    Ok(())
}

fn export_llamaindex(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let mut nodes: Vec<serde_json::Value> = Vec::new();

    if let Some(gf) = pack.load_glossary()? {
        for t in &gf.terms {
            nodes.push(serde_json::json!({
                "id_": t.id,
                "text": format!("{}: {}", t.term, t.definition),
                "metadata": { "type": "term" },
                "excluded_embed_metadata_keys": [],
                "excluded_llm_metadata_keys": []
            }));
        }
    }
    for c in &pack.load_chunks()? {
        nodes.push(serde_json::json!({
            "id_": c.id,
            "text": c.chunk_text,
            "metadata": { "title": c.title, "type": "chunk", "source_ref": c.source_ref },
            "excluded_embed_metadata_keys": [],
            "excluded_llm_metadata_keys": []
        }));
    }

    let out: String = nodes
        .iter()
        .map(|n| serde_json::to_string(n).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(out_dir.join("nodes.jsonl"), out)?;
    Ok(())
}

fn export_openai_files(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    if let Some(gf) = pack.load_glossary()? {
        let mut content = String::from("# Glossary\n\n");
        for t in &gf.terms {
            content.push_str(&format!("## {}\n{}\n\n", t.term, t.definition));
        }
        std::fs::write(out_dir.join("glossary.txt"), content)?;
    }

    for c in &pack.load_chunks()? {
        let content = format!("# {}\n\n{}", c.title, c.chunk_text);
        std::fs::write(out_dir.join(format!("{}.txt", c.id)), content)?;
    }

    if let Some(rf) = pack.load_rules()? {
        let mut content = String::from("# Domain Rules\n\n");
        for r in &rf.rules {
            content.push_str(&format!("## {}\n{}\n\n", r.title, r.description));
        }
        std::fs::write(out_dir.join("rules.txt"), content)?;
    }

    Ok(())
}

fn export_markdown(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    if let Some(gf) = pack.load_glossary()? {
        let mut content = format!("# {} — Glossary\n\n", pack.manifest.name);
        for t in &gf.terms {
            content.push_str(&format!("## {}\n\n{}\n\n", t.term, t.definition));
        }
        std::fs::write(out_dir.join("glossary.md"), content)?;
    }

    let chunks = pack.load_chunks()?;
    if !chunks.is_empty() {
        let mut content = format!("# {} — Retrieval Chunks\n\n", pack.manifest.name);
        for c in &chunks {
            content.push_str(&format!("## {}\n\n{}\n\n", c.title, c.chunk_text));
        }
        std::fs::write(out_dir.join("chunks.md"), content)?;
    }

    if let Some(rf) = pack.load_rules()? {
        let mut content = format!("# {} — Domain Rules\n\n", pack.manifest.name);
        for r in &rf.rules {
            content.push_str(&format!("## {}\n\n{}\n\n", r.title, r.description));
        }
        std::fs::write(out_dir.join("rules.md"), content)?;
    }

    Ok(())
}

fn export_csv(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    if let Some(gf) = pack.load_glossary()? {
        let mut wtr = csv::Writer::from_path(out_dir.join("glossary.csv"))?;
        wtr.write_record(["id", "term", "definition", "aliases", "tags"])?;
        for t in &gf.terms {
            wtr.write_record([
                &t.id,
                &t.term,
                &t.definition,
                &t.aliases.join("; "),
                &t.tags.join("; "),
            ])?;
        }
        wtr.flush()?;
    }

    let chunks = pack.load_chunks()?;
    if !chunks.is_empty() {
        let mut wtr = csv::Writer::from_path(out_dir.join("chunks.csv"))?;
        wtr.write_record(["id", "title", "chunk_text", "confidence", "tags"])?;
        for c in &chunks {
            wtr.write_record([
                &c.id,
                &c.title,
                &c.chunk_text,
                &c.confidence.map(|f| f.to_string()).unwrap_or_default(),
                &c.tags.join("; "),
            ])?;
        }
        wtr.flush()?;
    }

    if let Some(rf) = pack.load_rules()? {
        let mut wtr = csv::Writer::from_path(out_dir.join("rules.csv"))?;
        wtr.write_record(["id", "title", "description", "polarity"])?;
        for r in &rf.rules {
            let polarity = format!("{:?}", r.polarity).to_lowercase();
            wtr.write_record([&r.id, &r.title, &r.description, &polarity])?;
        }
        wtr.flush()?;
    }

    Ok(())
}

fn export_anki(pack: &Pack, out_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    // Glossary terms as front/back flashcards
    if let Some(gf) = pack.load_glossary()? {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path(out_dir.join("glossary_anki.tsv"))?;
        for t in &gf.terms {
            wtr.write_record([&t.term, &t.definition])?;
        }
        wtr.flush()?;
    }

    // Eval Q/A pairs
    let evals = pack.load_eval_set()?;
    if !evals.is_empty() {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path(out_dir.join("eval_anki.tsv"))?;
        for e in &evals {
            let answer = e.expected_dimensions.join("; ");
            wtr.write_record([&e.query, &answer])?;
        }
        wtr.flush()?;
    }

    Ok(())
}
