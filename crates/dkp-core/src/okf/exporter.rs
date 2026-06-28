use std::path::Path;

use crate::{error::DkpResult, pack::loader::Pack};

/// Generate the `okf/` layer from the `machine/` layer assets.
///
/// Each asset type maps to a subdirectory of concept Markdown files with
/// the required DKP OKF frontmatter fields.
pub fn export_okf(pack: &Pack, output_dir: &Path) -> DkpResult<ExportStats> {
    let mut stats = ExportStats::default();

    // terms/ from glossary.json
    if let Some(gf) = pack.load_glossary()? {
        let dir = output_dir.join("terms");
        std::fs::create_dir_all(&dir)?;
        for t in &gf.terms {
            let mut fm = String::from("---\n");
            fm.push_str(&format!("id: {}\n", t.id));
            fm.push_str("type: term\n");
            fm.push_str(&format!("term: \"{}\"\n", escape_yaml(&t.term)));
            if !t.aliases.is_empty() {
                fm.push_str(&format!(
                    "aliases: [{}]\n",
                    t.aliases
                        .iter()
                        .map(|a| format!("\"{}\"", escape_yaml(a)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if !t.tags.is_empty() {
                fm.push_str(&format!(
                    "tags: [{}]\n",
                    t.tags
                        .iter()
                        .map(|t| format!("\"{}\"", escape_yaml(t)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if let Some(s) = &t.stability {
                fm.push_str(&format!("stability: {s}\n"));
            }
            if let Some(sr) = &t.source_ref {
                fm.push_str(&format!("source_ref: {sr}\n"));
            }
            fm.push_str("---\n\n");
            fm.push_str(&format!("# {}\n\n", t.term));
            fm.push_str(&t.definition);
            fm.push('\n');

            std::fs::write(dir.join(format!("{}.md", t.id)), &fm)?;
            stats.terms_written += 1;
        }
    }

    // rules/ from rules.json
    if let Some(rf) = pack.load_rules()? {
        let dir = output_dir.join("rules");
        std::fs::create_dir_all(&dir)?;
        for r in &rf.rules {
            let polarity = format!("{:?}", r.polarity).to_lowercase();
            let mut fm = String::from("---\n");
            fm.push_str(&format!("id: {}\n", r.id));
            fm.push_str("type: rule\n");
            fm.push_str(&format!("title: \"{}\"\n", escape_yaml(&r.title)));
            fm.push_str(&format!("polarity: {polarity}\n"));
            if let Some(c) = r.confidence {
                fm.push_str(&format!("confidence: {c}\n"));
            }
            if let Some(s) = &r.stability {
                fm.push_str(&format!("stability: {s}\n"));
            }
            if let Some(sr) = &r.source_ref {
                fm.push_str(&format!("source_ref: {sr}\n"));
            }
            if !r.tags.is_empty() {
                fm.push_str(&format!(
                    "tags: [{}]\n",
                    r.tags
                        .iter()
                        .map(|t| format!("\"{}\"", escape_yaml(t)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            fm.push_str("---\n\n");
            fm.push_str(&format!("# {}\n\n", r.title));
            fm.push_str(&r.description);
            fm.push('\n');

            std::fs::write(dir.join(format!("{}.md", r.id)), &fm)?;
            stats.rules_written += 1;
        }
    }

    // constraints/ from constraints.json
    if let Some(cf) = pack.load_constraints()? {
        let dir = output_dir.join("constraints");
        std::fs::create_dir_all(&dir)?;
        for c in cf.all_constraints() {
            let mut fm = String::from("---\n");
            fm.push_str(&format!("id: {}\n", c.id));
            fm.push_str("type: constraint\n");
            fm.push_str(&format!("title: \"{}\"\n", escape_yaml(&c.title)));
            if let Some(s) = &c.stability {
                fm.push_str(&format!("stability: {s}\n"));
            }
            if let Some(sr) = &c.source_ref {
                fm.push_str(&format!("source_ref: {sr}\n"));
            }
            if !c.tags.is_empty() {
                fm.push_str(&format!(
                    "tags: [{}]\n",
                    c.tags
                        .iter()
                        .map(|t| format!("\"{}\"", escape_yaml(t)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            fm.push_str("---\n\n");
            fm.push_str(&format!("# {}\n\n", c.title));
            fm.push_str(&c.description);
            fm.push('\n');

            std::fs::write(dir.join(format!("{}.md", c.id)), &fm)?;
            stats.constraints_written += 1;
        }
    }

    // chunks/ from retrieval_chunks.jsonl
    {
        let chunks = pack.load_chunks()?;
        if !chunks.is_empty() {
            let dir = output_dir.join("chunks");
            std::fs::create_dir_all(&dir)?;
            for c in &chunks {
                let mut fm = String::from("---\n");
                fm.push_str(&format!("id: {}\n", c.id));
                fm.push_str("type: chunk\n");
                fm.push_str(&format!("title: \"{}\"\n", escape_yaml(&c.title)));
                fm.push_str(&format!("source_ref: {}\n", c.source_ref));
                if let Some(conf) = c.confidence {
                    fm.push_str(&format!("confidence: {conf}\n"));
                }
                if !c.tags.is_empty() {
                    fm.push_str(&format!(
                        "tags: [{}]\n",
                        c.tags
                            .iter()
                            .map(|t| format!("\"{}\"", escape_yaml(t)))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                fm.push_str("---\n\n");
                fm.push_str(&format!("# {}\n\n", c.title));
                fm.push_str(&c.chunk_text);
                fm.push('\n');

                std::fs::write(dir.join(format!("{}.md", c.id)), &fm)?;
                stats.chunks_written += 1;
            }
        }
    }

    // ontology/ from ontology.json
    if let Some(of) = pack.load_ontology()? {
        let dir = output_dir.join("ontology");
        std::fs::create_dir_all(&dir)?;
        for e in &of.entity_types {
            let mut fm = String::from("---\n");
            fm.push_str(&format!("id: {}\n", e.id));
            fm.push_str("type: entity\n");
            fm.push_str(&format!("name: \"{}\"\n", escape_yaml(&e.name)));
            if let Some(st) = &e.schema_org_type {
                fm.push_str(&format!("schema_org_type: {st}\n"));
            }
            if !e.attributes.is_empty() {
                fm.push_str(&format!(
                    "attributes: [{}]\n",
                    e.attributes
                        .iter()
                        .map(|a| format!("\"{}\"", escape_yaml(a)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            fm.push_str("---\n\n");
            fm.push_str(&format!("# {}\n\n", e.name));
            fm.push_str(&e.description);
            fm.push('\n');

            std::fs::write(dir.join(format!("{}.md", e.id)), &fm)?;
            stats.ontology_written += 1;
        }
    }

    Ok(stats)
}

fn escape_yaml(s: &str) -> String {
    s.replace('"', "\\\"")
}

#[derive(Debug, Default)]
pub struct ExportStats {
    pub terms_written: usize,
    pub rules_written: usize,
    pub constraints_written: usize,
    pub chunks_written: usize,
    pub ontology_written: usize,
}
