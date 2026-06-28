use axum::{extract::State, response::Json};
use serde::Serialize;
use std::sync::Arc;

use crate::cmd::webui::server::AppState;

#[derive(Serialize)]
pub struct PackSummary {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub update_date: String,
    pub description: Option<String>,
    pub counts: AssetCounts,
}

#[derive(Serialize)]
pub struct AssetCounts {
    pub glossary: usize,
    pub rules: usize,
    pub chunks: usize,
    pub eval_cases: usize,
    pub has_graph: bool,
    pub has_skills: bool,
    pub has_l10n: bool,
    pub has_mcp_manifest: bool,
    pub has_cross_refs: bool,
    pub has_ontology: bool,
    pub has_constraints: bool,
    pub has_system_prompt: bool,
}

pub async fn get_pack(State(state): State<Arc<AppState>>) -> Json<PackSummary> {
    let pack = &state.pack;
    let m = &pack.manifest;

    let glossary = pack
        .load_glossary()
        .ok()
        .flatten()
        .map(|g| g.terms.len())
        .unwrap_or(0);
    let rules = pack
        .load_rules()
        .ok()
        .flatten()
        .map(|r| r.rules.len())
        .unwrap_or(0);
    let chunks = pack.load_chunks().ok().map(|c| c.len()).unwrap_or(0);
    let eval_cases = pack.load_eval_set().ok().map(|e| e.len()).unwrap_or(0);

    Json(PackSummary {
        name: m.name.clone(),
        version: m.version.clone(),
        domain: m.domain.clone(),
        update_date: m.update_date.clone(),
        description: m.description.clone(),
        counts: AssetCounts {
            glossary,
            rules,
            chunks,
            eval_cases,
            has_graph: pack.has_knowledge_graph(),
            has_skills: pack.has_skills(),
            has_l10n: pack.has_l10n(),
            has_mcp_manifest: pack.has_mcp_manifest(),
            has_cross_refs: pack.has_cross_refs(),
            has_ontology: pack.machine_dir().join("ontology.json").exists(),
            has_constraints: pack.machine_dir().join("constraints.json").exists(),
            has_system_prompt: pack.machine_dir().join("system_prompt.md").exists(),
        },
    })
}
