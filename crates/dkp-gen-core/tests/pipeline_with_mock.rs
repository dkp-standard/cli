use std::collections::HashMap;
use std::sync::Arc;

use dkp_core::pack::loader::Pack;
use dkp_core::validate::gate4;
use dkp_core::validate::gates::GateStatus;
use dkp_gen_core::llm::mock::MockClient;
use dkp_gen_core::{GenConfig, PipelineContext};
use tempfile::TempDir;

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
        r#"{"entity_types": [{"id": "e1", "name": "Entity One", "description": "desc", "attributes": [], "relationships": []}]}"#.to_string(),
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
        "## Chunk One\nSome useful content for chunk one that is long enough to be kept.\n"
            .to_string(),
    );
    m.insert(
        "evaluation entries".to_string(),
        r#"{"query": "q1", "expected_dimensions": [], "critical_must_include": [], "scoring_rubric": "r"}"#.to_string(),
    );
    m
}

/// Drives the full generation pipeline (machine assets) against a MockClient
/// and asserts the resulting pack is gate4-conformant — the seam most likely
/// to break silently even when each stage's own unit tests pass.
#[tokio::test]
async fn generated_pack_passes_gate4() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("manifest.json"),
        r#"{
            "spec": "1.0.0",
            "name": "generated-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "integration test",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        }"#,
    )
    .unwrap();

    let ctx = PipelineContext {
        pack_dir: tmp.path().to_path_buf(),
        domain: "testing".to_string(),
        pack_name: "generated-pack".to_string(),
        config: GenConfig::default(),
        client: Arc::new(MockClient::new(happy_path_fixtures(), "")),
        progress: None,
        verbose: false,
    };

    dkp_gen_core::pipeline::machine::run(&ctx)
        .await
        .expect("machine pipeline should succeed with mocked LLM");

    let pack = Pack::open(tmp.path()).expect("generated pack should open");
    let result = gate4::run(&pack);
    assert_eq!(
        result.status,
        GateStatus::Pass,
        "gate4 checks: {:#?}",
        result.checks
    );
}
