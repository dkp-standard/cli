use serde_json::Value;
use sha2::{Digest, Sha256};

use dkp_core::types::chunks::RetrievalChunk;
use dkp_core::types::eval::{EvalCase, EvalVersionMeta};
use dkp_core::types::graph::{KgEdge, KgNode, KgRelation, KnowledgeGraph};

use crate::chunk;
use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::{extract_json, extract_jsonl, templates};

pub async fn run(ctx: &PipelineContext) -> GenResult<()> {
    generate_system_prompt(ctx).await?;
    let rules = generate_rules(ctx).await?;
    let ontology = generate_ontology(ctx).await?;
    generate_knowledge_graph(ctx, &ontology)?;
    let glossary = generate_glossary(ctx).await?;
    generate_constraints(ctx).await?;
    generate_decision_trees(ctx).await?;
    let chunks = generate_chunks(ctx, &ontology, &glossary).await?;
    generate_eval_set(ctx, &chunks, &rules).await?;
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

async fn generate_rules(ctx: &PipelineContext) -> GenResult<Value> {
    let path = ctx.machine_path().join("rules.json");
    if !ctx.should_generate(&path) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str(&content) {
                return Ok(v);
            }
        }
    }
    let (sys, user) = templates::prompt_rules(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("rules", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_ontology(ctx: &PipelineContext) -> GenResult<Value> {
    let path = ctx.machine_path().join("ontology.json");
    if !ctx.should_generate(&path) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str(&content) {
                return Ok(v);
            }
        }
    }
    let (sys, user) = templates::prompt_ontology(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("ontology", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_glossary(ctx: &PipelineContext) -> GenResult<Value> {
    let path = ctx.machine_path().join("glossary.json");
    if !ctx.should_generate(&path) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str(&content) {
                return Ok(v);
            }
        }
    }
    let (sys, user) = templates::prompt_glossary(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("glossary", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)?;
    Ok(value)
}

async fn generate_constraints(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.machine_path().join("constraints.json");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_constraints(&ctx.domain, &ctx.pack_name);
    let raw = ctx.generate("constraints", &sys, &user).await?;
    let value = extract_json(&raw)?;
    ctx.write_json(&path, &value)
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
    ontology: &Value,
    glossary: &Value,
) -> GenResult<Vec<RetrievalChunk>> {
    let path = ctx.machine_path().join("retrieval_chunks.jsonl");
    if !ctx.should_generate(&path) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let chunks: Vec<RetrievalChunk> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect();
            if !chunks.is_empty() {
                return Ok(chunks);
            }
        }
    }
    let context_bundle = build_context_bundle(&ctx.domain, ontology, glossary);
    let (sys, user) = templates::prompt_chunks_raw(&ctx.domain, &ctx.pack_name, &context_bundle);
    let raw = ctx.generate("retrieval_chunks", &sys, &user).await?;
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
