use serde_json::Value;
use sha2::{Digest, Sha256};

use dkp_core::types::chunks::RetrievalChunk;
use dkp_core::types::eval::{EvalCase, EvalVersionMeta};
use dkp_core::types::graph::{KgEdge, KgNode, KgRelation, KnowledgeGraph};

use crate::chunk;
use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::{extract_json, extract_jsonl, templates};
use crate::tools::GenToolExecutor;

pub async fn run(ctx: &PipelineContext) -> GenResult<()> {
    // Built once per run; `None` when tool use is disabled (`--no-tools`).
    let tools = ctx.build_tool_executor("generate")?;

    generate_system_prompt(ctx).await?;
    let rules = generate_rules(ctx, tools.as_ref()).await?;
    let ontology = generate_ontology(ctx, tools.as_ref()).await?;
    generate_knowledge_graph(ctx, &ontology)?;
    let glossary = generate_glossary(ctx, tools.as_ref()).await?;
    let constraints = generate_constraints(ctx).await?;
    generate_decision_trees(ctx).await?;
    let chunks = generate_chunks(ctx, tools.as_ref(), &ontology, &glossary).await?;
    generate_eval_set(ctx, &chunks, &rules).await?;
    run_consistency_check(ctx, &rules, &constraints, &glossary, &chunks).await?;
    Ok(())
}

async fn generate_system_prompt(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.machine_path().join("system_prompt.md");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_system_prompt(&ctx.domain, &ctx.pack_name);
    let text = ctx.generate("system_prompt", &sys, &user).await?;
    ctx.write_text(&path, text.trim())
}

async fn generate_rules(
    ctx: &PipelineContext,
    tools: Option<&GenToolExecutor>,
) -> GenResult<Value> {
    let path = ctx.machine_path().join("rules.json");
    if !ctx.should_generate(&path)
        && let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(v) = serde_json::from_str(&content)
    {
        return Ok(v);
    }
    let (sys, user) = templates::prompt_rules(&ctx.domain, &ctx.pack_name);
    let sys = grounded_system(sys, tools);
    let raw = ctx
        .generate_maybe_tools("rules", &sys, &user, tools)
        .await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_ontology(
    ctx: &PipelineContext,
    tools: Option<&GenToolExecutor>,
) -> GenResult<Value> {
    let path = ctx.machine_path().join("ontology.json");
    if !ctx.should_generate(&path)
        && let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(v) = serde_json::from_str(&content)
    {
        return Ok(v);
    }
    let (sys, user) = templates::prompt_ontology(&ctx.domain, &ctx.pack_name);
    let sys = grounded_system(sys, tools);
    let raw = ctx
        .generate_maybe_tools("ontology", &sys, &user, tools)
        .await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_glossary(
    ctx: &PipelineContext,
    tools: Option<&GenToolExecutor>,
) -> GenResult<Value> {
    let path = ctx.machine_path().join("glossary.json");
    if !ctx.should_generate(&path)
        && let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(v) = serde_json::from_str(&content)
    {
        return Ok(v);
    }
    let (sys, user) = templates::prompt_glossary(&ctx.domain, &ctx.pack_name);
    let sys = grounded_system(sys, tools);
    let raw = ctx
        .generate_maybe_tools("glossary", &sys, &user, tools)
        .await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

/// Appends the grounding preamble to `sys` when tool use is available for
/// this step, so grounded steps are told to actually call the tools.
fn grounded_system(sys: String, tools: Option<&GenToolExecutor>) -> String {
    if tools.is_some() {
        sys + templates::grounding_preamble()
    } else {
        sys
    }
}

async fn generate_constraints(ctx: &PipelineContext) -> GenResult<Value> {
    let path = ctx.machine_path().join("constraints.json");
    if !ctx.should_generate(&path)
        && let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(v) = serde_json::from_str(&content)
    {
        return Ok(v);
    }
    let (sys, user) = templates::prompt_constraints(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("constraints", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_decision_trees(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.machine_path().join("decision_trees.json");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_decision_trees(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("decision_trees", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)
}

async fn generate_chunks(
    ctx: &PipelineContext,
    tools: Option<&GenToolExecutor>,
    ontology: &Value,
    glossary: &Value,
) -> GenResult<Vec<RetrievalChunk>> {
    let path = ctx.machine_path().join("retrieval_chunks.jsonl");
    if !ctx.should_generate(&path)
        && let Ok(content) = std::fs::read_to_string(&path)
    {
        let chunks: Vec<RetrievalChunk> = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        if !chunks.is_empty() {
            return Ok(chunks);
        }
    }
    let context_bundle = build_context_bundle(&ctx.domain, ontology, glossary);
    let discovered_source_count = count_discovered_sources(ctx);
    let (sys, user) = templates::prompt_chunks_raw(
        &ctx.domain,
        &ctx.pack_name,
        &context_bundle,
        discovered_source_count,
    );
    let sys = grounded_system(sys, tools);
    let raw = ctx
        .generate_maybe_tools("retrieval_chunks", &sys, &user, tools)
        .await?;
    let chunks = chunk::split(&raw, &ctx.domain, &ctx.pack_name);
    ctx.write_jsonl(&path, &chunks)?;
    Ok(chunks)
}

async fn generate_eval_set(
    ctx: &PipelineContext,
    chunks: &[RetrievalChunk],
    _rules: &Value,
) -> GenResult<()> {
    let path = ctx.machine_path().join("eval_set.jsonl");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let corpus_excerpt: String = chunks
        .iter()
        .map(|c| c.chunk_text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
        .chars()
        .take(4000)
        .collect();

    let (sys, user) = templates::prompt_eval_set(&ctx.domain, &ctx.pack_name, &corpus_excerpt);
    let raw = ctx.generate("eval_set", &sys, &user).await?;
    let rows = extract_jsonl(&raw)?;

    let prompt_hash = hex_hash(&format!("{sys}\n---\n{user}"));
    let dataset_version = read_manifest_version(&ctx.pack_dir);

    let mut eval_cases: Vec<EvalCase> = Vec::new();
    for row in rows {
        let query = row["query"].as_str().unwrap_or("").to_string();
        let expected_dimensions = row["expected_dimensions"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let critical_must_include = row["critical_must_include"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let scoring_rubric = row["scoring_rubric"].as_str().unwrap_or("").to_string();
        eval_cases.push(EvalCase {
            query,
            expected_dimensions,
            critical_must_include,
            scoring_rubric,
            version_meta: EvalVersionMeta {
                prompt_hash: prompt_hash.clone(),
                model_version: ctx.config.model.clone(),
                dataset_version: dataset_version.clone(),
            },
            tags: vec![],
            audience: vec![],
        });
    }

    ctx.write_jsonl(&path, &eval_cases)
}

async fn run_consistency_check(
    ctx: &PipelineContext,
    rules: &Value,
    constraints: &Value,
    glossary: &Value,
    chunks: &[RetrievalChunk],
) -> GenResult<()> {
    let chunk_excerpt: String = chunks
        .iter()
        .map(|c| c.chunk_text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
        .chars()
        .take(4000)
        .collect();
    let bundle = serde_json::json!({
        "rules": rules,
        "constraints": constraints,
        "glossary": glossary,
        "chunks_excerpt": chunk_excerpt,
    })
    .to_string();
    let bundle: String = bundle.chars().take(12_000).collect();

    let (sys, user) = templates::prompt_consistency_check(&ctx.domain, &ctx.pack_name, &bundle);
    let raw = ctx.generate("consistency_check", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&ctx.build_path().join("consistency_report.json"), &value)
}

fn build_context_bundle(domain: &str, ontology: &Value, glossary: &Value) -> String {
    let entity_names: Vec<&str> = ontology["entity_types"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|e| e["name"].as_str())
                .take(10)
                .collect()
        })
        .unwrap_or_default();
    let top_terms: Vec<&str> = glossary["terms"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|t| t["term"].as_str())
                .take(10)
                .collect()
        })
        .unwrap_or_default();
    serde_json::json!({
        "domain": domain,
        "key_entities": entity_names,
        "key_terms": top_terms,
    })
    .to_string()
}

/// Number of sources logged to `build/sources_discovered.jsonl` so far this
/// run — a proxy for "how much grounding material has this pipeline already
/// gathered," used to scale the chunk-count guidance in `prompt_chunks_raw`.
/// Earlier grounded steps (rules/ontology/glossary) run before chunks and may
/// have already populated this file via `web_fetch`/`web_search`.
fn count_discovered_sources(ctx: &PipelineContext) -> usize {
    std::fs::read_to_string(ctx.build_path().join("sources_discovered.jsonl"))
        .map(|content| content.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0)
}

fn hex_hash(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    format!("{:x}", h.finalize())[..16].to_string()
}

fn read_manifest_version(pack_dir: &std::path::Path) -> String {
    let path = pack_dir.join("manifest.json");
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["version"].as_str().map(String::from))
        .unwrap_or_else(|| "0.1.0".into())
}

fn generate_knowledge_graph(ctx: &PipelineContext, ontology: &Value) -> GenResult<()> {
    let path = ctx.machine_path().join("knowledge_graph.json");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let graph = derive_knowledge_graph(ontology);
    ctx.write_json(&path, &graph)
}

fn derive_knowledge_graph(ontology: &Value) -> KnowledgeGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    if let Some(entity_types) = ontology["entity_types"].as_array() {
        for entity in entity_types {
            let id = entity["id"].as_str().unwrap_or("").to_string();
            let label = entity["name"].as_str().unwrap_or("").to_string();
            let description = entity["description"].as_str().map(String::from);
            nodes.push(KgNode {
                id: id.clone(),
                node_type: "entity-type".into(),
                label,
                description,
            });
            if let Some(rels) = entity["relationships"].as_array() {
                for rel in rels {
                    let rel_name = rel["name"].as_str().unwrap_or("").to_string();
                    let target = rel["target_type"].as_str().unwrap_or("").to_string();
                    if !target.is_empty() {
                        edges.push(KgEdge {
                            source: id.clone(),
                            relation: relation_from_name(&rel_name),
                            target,
                            weight: None,
                            description: Some(rel_name),
                        });
                    }
                }
            }
        }
    }
    KnowledgeGraph { nodes, edges }
}

fn relation_from_name(name: &str) -> KgRelation {
    let n = name.to_lowercase();
    if n.contains("requires") || n.contains("needs") || n.contains("must") {
        KgRelation::Requires
    } else if n.contains("part")
        || n.contains("belongs")
        || n.contains("within")
        || n.contains("contains")
    {
        KgRelation::PartOf
    } else if n.contains("depends") || n.contains("based") {
        KgRelation::DependsOn
    } else if n.contains("defines") || n.contains("specifies") {
        KgRelation::DefinedBy
    } else if n.contains("specializes") || n.contains("extends") {
        KgRelation::Specializes
    } else if n.contains("measures") || n.contains("tracks") || n.contains("quantifies") {
        KgRelation::MeasuredBy
    } else if n.contains("elaborates") || n.contains("describes") || n.contains("details") {
        KgRelation::Elaborates
    } else if n.contains("contradicts") || n.contains("conflicts") || n.contains("prevents") {
        KgRelation::Contradicts
    } else if n.contains("supersedes") || n.contains("replaces") {
        KgRelation::Supersedes
    } else {
        KgRelation::SeeAlso
    }
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

    fn happy_path_fixtures() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert(
            "Write a concise LLM system prompt".to_string(),
            "You are a testing expert.".to_string(),
        );
        m.insert(
            "core operational rules".to_string(),
            r#"{"rules": [{"id": "r1", "title": "Rule 1", "description": "Do X", "polarity": "affirmative", "stability": "stable", "source_ref": "generated"}]}"#.to_string(),
        );
        m.insert(
            "domain ontology".to_string(),
            r#"{"entity_types": [{"id": "e1", "name": "Entity One", "description": "desc", "attributes": [], "relationships": [{"name": "requires", "target_type": "e2", "cardinality": "one-to-many"}]}]}"#.to_string(),
        );
        m.insert(
            "domain glossary".to_string(),
            r#"{"terms": [{"id": "t1", "term": "Term One", "definition": "def", "stability": "stable", "source_ref": "generated"}]}"#.to_string(),
        );
        m.insert(
            "domain constraints".to_string(),
            r#"{"edge_cases": [], "anti_patterns": [], "hard_limits": []}"#.to_string(),
        );
        m.insert(
            "decision trees for common".to_string(),
            r#"{"trees": []}"#.to_string(),
        );
        m.insert(
            "comprehensive domain knowledge".to_string(),
            "## Chunk One\nSome useful content for chunk one.\n".to_string(),
        );
        m.insert(
            "evaluation entries".to_string(),
            r#"{"query": "q1", "expected_dimensions": [], "critical_must_include": [], "scoring_rubric": "r"}"#.to_string(),
        );
        m.insert(
            "internal contradictions".to_string(),
            r#"{"consistent": true, "issues": []}"#.to_string(),
        );
        m
    }

    #[tokio::test]
    async fn full_pipeline_run_writes_expected_machine_files() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("manifest.json"), r#"{"version": "1.0.0"}"#).unwrap();
        let ctx = test_ctx(&tmp, happy_path_fixtures());

        run(&ctx).await.unwrap();

        assert!(ctx.machine_path().join("system_prompt.md").exists());
        assert!(ctx.machine_path().join("rules.json").exists());
        assert!(ctx.machine_path().join("ontology.json").exists());
        assert!(ctx.machine_path().join("glossary.json").exists());
        assert!(ctx.machine_path().join("constraints.json").exists());
        assert!(ctx.machine_path().join("decision_trees.json").exists());
        assert!(ctx.machine_path().join("retrieval_chunks.jsonl").exists());
        assert!(ctx.machine_path().join("knowledge_graph.json").exists());
        assert!(ctx.build_path().join("consistency_report.json").exists());
    }

    #[tokio::test]
    async fn knowledge_graph_derived_from_ontology_relationships() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("manifest.json"), r#"{"version": "1.0.0"}"#).unwrap();
        let ctx = test_ctx(&tmp, happy_path_fixtures());

        run(&ctx).await.unwrap();

        let graph_json =
            std::fs::read_to_string(ctx.machine_path().join("knowledge_graph.json")).unwrap();
        let graph: KnowledgeGraph = serde_json::from_str(&graph_json).unwrap();
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].id, "e1");
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].source, "e1");
        assert_eq!(graph.edges[0].target, "e2");
        assert!(matches!(graph.edges[0].relation, KgRelation::Requires));
    }

    #[tokio::test]
    async fn malformed_json_response_errors_instead_of_writing() {
        let tmp = TempDir::new().unwrap();
        let mut fixtures = HashMap::new();
        fixtures.insert(
            "core operational rules".to_string(),
            "not valid json at all".to_string(),
        );
        let ctx = test_ctx(&tmp, fixtures);

        let result = generate_rules(&ctx, None).await;
        assert!(result.is_err());
        assert!(!ctx.machine_path().join("rules.json").exists());
    }

    #[test]
    fn relation_from_name_maps_keywords_to_relations() {
        assert!(matches!(
            relation_from_name("requires funding"),
            KgRelation::Requires
        ));
        assert!(matches!(
            relation_from_name("is part of"),
            KgRelation::PartOf
        ));
        assert!(matches!(
            relation_from_name("depends on"),
            KgRelation::DependsOn
        ));
        assert!(matches!(
            relation_from_name("some unrelated phrase"),
            KgRelation::SeeAlso
        ));
    }

    #[test]
    fn count_discovered_sources_is_zero_when_file_absent() {
        let tmp = TempDir::new().unwrap();
        let ctx = test_ctx(&tmp, HashMap::new());
        assert_eq!(count_discovered_sources(&ctx), 0);
    }

    #[test]
    fn count_discovered_sources_counts_jsonl_lines() {
        let tmp = TempDir::new().unwrap();
        let ctx = test_ctx(&tmp, HashMap::new());
        std::fs::create_dir_all(ctx.build_path()).unwrap();
        std::fs::write(
            ctx.build_path().join("sources_discovered.jsonl"),
            "{\"url\":\"a\"}\n{\"url\":\"b\"}\n\n",
        )
        .unwrap();
        assert_eq!(count_discovered_sources(&ctx), 2);
    }
}
