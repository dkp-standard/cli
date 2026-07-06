use serde_json::Value;

use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::templates;

/// Regenerates the "## Contents" section of `README.md` with a description
/// grounded in the pack's actual generated content, replacing the "TODO:
/// describe what's in this pack." placeholder `dkp init` scaffolds. Runs
/// after the machine layer exists so there's real content to summarize.
pub async fn update_readme(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.pack_dir.join("README.md");
    let existing = std::fs::read_to_string(&path).unwrap_or_default();

    let needs_fill = existing.is_empty() || existing.contains("TODO") || ctx.config.overwrite;
    if !needs_fill {
        return Ok(());
    }

    let summary = build_pack_summary(ctx);
    let (sys, user) = templates::prompt_readme_contents(&ctx.domain, &ctx.pack_name, &summary);
    let contents = ctx.generate("readme_contents", &sys, &user).await?;

    let updated = splice_contents_section(&existing, &ctx.pack_name, &ctx.domain, contents.trim());
    ctx.write_text(&path, &updated)
}

/// Replaces everything from the first "## Contents" heading onward with the
/// generated section, preserving the title/tagline/Quick Start block that
/// `dkp init` scaffolds above it. Falls back to a minimal README shell if no
/// "## Contents" heading is found (e.g. a hand-edited or missing README).
fn splice_contents_section(
    existing: &str,
    pack_name: &str,
    domain: &str,
    contents: &str,
) -> String {
    const HEADING: &str = "## Contents";
    match existing.find(HEADING) {
        Some(idx) => {
            let mut updated = existing[..idx].to_string();
            updated.push_str(HEADING);
            updated.push_str("\n\n");
            updated.push_str(contents);
            updated.push('\n');
            updated
        }
        None => format!(
            "# {pack_name}\n\nA Domain Knowledge Pack for the **{domain}** domain.\n\n{HEADING}\n\n{contents}\n"
        ),
    }
}

/// Summarizes the pack's actual generated content (counts and a sample of
/// names/titles from each machine-layer asset) for the README prompt, so the
/// generated Contents section describes this pack, not generic boilerplate.
fn build_pack_summary(ctx: &PipelineContext) -> String {
    let machine = ctx.machine_path();

    let rule_titles = json_array_field(&machine.join("rules.json"), "rules", "title", 8);
    let term_names = json_array_field(&machine.join("glossary.json"), "terms", "term", 12);
    let entity_names = json_array_field(&machine.join("ontology.json"), "entity_types", "name", 8);
    let chunk_count = jsonl_line_count(&machine.join("retrieval_chunks.jsonl"));

    serde_json::json!({
        "domain": ctx.domain,
        "pack_name": ctx.pack_name,
        "rule_count": rule_titles.len(),
        "sample_rules": rule_titles,
        "term_count": term_names.len(),
        "sample_terms": term_names,
        "entity_types": entity_names,
        "retrieval_chunk_count": chunk_count,
    })
    .to_string()
}

fn json_array_field(
    path: &std::path::Path,
    array_key: &str,
    item_key: &str,
    take: usize,
) -> Vec<String> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<Value>(&content) else {
        return Vec::new();
    };
    value[array_key]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item[item_key].as_str().map(String::from))
                .take(take)
                .collect()
        })
        .unwrap_or_default()
}

fn jsonl_line_count(path: &std::path::Path) -> usize {
    std::fs::read_to_string(path)
        .map(|content| content.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GenConfig;
    use crate::llm::mock::MockClient;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn test_ctx(tmp: &TempDir, fixtures: HashMap<String, String>) -> PipelineContext {
        PipelineContext {
            pack_dir: tmp.path().to_path_buf(),
            domain: "testing".to_string(),
            pack_name: "test-pack".to_string(),
            config: GenConfig::default(),
            client: Arc::new(MockClient::new(fixtures, "")),
            progress: None,
            verbose: false,
        }
    }

    #[tokio::test]
    async fn fills_todo_placeholder_and_preserves_quick_start() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("README.md"),
            "# test-pack\n\nA Domain Knowledge Pack for the **testing** domain.\n\n## Quick Start\n\n```bash\ndkp info .\n```\n\n## Contents\n\nTODO: describe what's in this pack.\n",
        )
        .unwrap();

        let mut fixtures = HashMap::new();
        fixtures.insert(
            "Contents".to_string(),
            "This pack covers widget troubleshooting rules and terminology.".to_string(),
        );
        let ctx = test_ctx(&tmp, fixtures);

        update_readme(&ctx).await.unwrap();

        let content = std::fs::read_to_string(tmp.path().join("README.md")).unwrap();
        assert!(
            content.contains("dkp info ."),
            "Quick Start block should survive"
        );
        assert!(content.contains("widget troubleshooting rules"));
        assert!(!content.contains("TODO"));
    }

    #[tokio::test]
    async fn skips_when_no_todo_and_not_overwriting() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("README.md"),
            "# test-pack\n\n## Contents\n\nAlready has real content, no placeholder here.\n",
        )
        .unwrap();
        // No fixtures registered: if the LLM were called, this would error.
        let ctx = test_ctx(&tmp, HashMap::new());

        update_readme(&ctx).await.unwrap();

        let content = std::fs::read_to_string(tmp.path().join("README.md")).unwrap();
        assert!(content.contains("Already has real content"));
    }

    #[tokio::test]
    async fn missing_readme_gets_a_minimal_shell() {
        let tmp = TempDir::new().unwrap();
        let mut fixtures = HashMap::new();
        fixtures.insert(
            "Contents".to_string(),
            "Generated contents section.".to_string(),
        );
        let ctx = test_ctx(&tmp, fixtures);

        update_readme(&ctx).await.unwrap();

        let content = std::fs::read_to_string(tmp.path().join("README.md")).unwrap();
        assert!(content.contains("# test-pack"));
        assert!(content.contains("Generated contents section."));
    }

    #[test]
    fn build_pack_summary_includes_generated_asset_counts() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("machine")).unwrap();
        std::fs::write(
            tmp.path().join("machine/rules.json"),
            r#"{"rules": [{"title": "Rule A"}, {"title": "Rule B"}]}"#,
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("machine/retrieval_chunks.jsonl"),
            "{\"id\":\"1\"}\n{\"id\":\"2\"}\n{\"id\":\"3\"}\n",
        )
        .unwrap();
        let ctx = test_ctx(&tmp, HashMap::new());

        let summary = build_pack_summary(&ctx);
        assert!(summary.contains("\"rule_count\":2"));
        assert!(summary.contains("Rule A"));
        assert!(summary.contains("\"retrieval_chunk_count\":3"));
    }
}
