use std::collections::HashSet;

use crate::{
    pack::loader::Pack,
    procedures,
    validate::gates::{CheckResult, GateResult, GateStatus},
};

const REQUIRED_MACHINE_FILES: &[&str] = &[
    "system_prompt.md",
    "glossary.json",
    "ontology.json",
    "rules.json",
    "constraints.json",
    "retrieval_chunks.jsonl",
];

/// Gate 4: Machine Usability — required files present, schemas valid, refs resolve.
pub fn run(pack: &Pack) -> GateResult {
    let mut checks = Vec::new();

    // Required file presence
    for file in REQUIRED_MACHINE_FILES {
        if pack.machine_file(file).exists() {
            checks.push(CheckResult::pass(format!("machine/{file} present")));
        } else {
            checks.push(CheckResult::fail(
                format!("machine/{file} present"),
                format!("machine/{file} is required but not found"),
            ));
        }
    }

    // manifest.json present (already opened, so always true if we got here)
    checks.push(CheckResult::pass("manifest.json present"));

    // Parse each required JSON/JSONL asset — deserialization failure = schema violation
    for (name, result) in [
        ("glossary.json", pack.load_glossary().map(|_| ())),
        ("ontology.json", pack.load_ontology().map(|_| ())),
        ("rules.json", pack.load_rules().map(|_| ())),
        ("constraints.json", pack.load_constraints().map(|_| ())),
        ("retrieval_chunks.jsonl", pack.load_chunks().map(|_| ())),
    ] {
        match result {
            Ok(_) => checks.push(CheckResult::pass(format!("machine/{name} parses"))),
            Err(e) => checks.push(CheckResult::fail(
                format!("machine/{name} parses"),
                e.to_string(),
            )),
        }
    }

    // source_ref resolution against evidence/sources.csv
    let sources_csv = pack.evidence_file("sources.csv");
    if sources_csv.exists() {
        let known_ids: HashSet<String> = std::fs::read_to_string(&sources_csv)
            .unwrap_or_default()
            .lines()
            .skip(1) // header row
            .filter_map(|line| {
                line.split(',')
                    .next()
                    .map(|s| s.trim().trim_matches('"').to_string())
            })
            .filter(|s| !s.is_empty())
            .collect();

        let mut unresolved: Vec<String> = Vec::new();

        if let Ok(Some(gf)) = pack.load_glossary() {
            for t in &gf.terms {
                if let Some(ref sr) = t.source_ref {
                    if sr != "generated" && !known_ids.contains(sr.as_str()) {
                        unresolved.push(format!("term/{}: {sr}", t.id));
                    }
                }
            }
        }
        if let Ok(Some(rf)) = pack.load_rules() {
            for r in &rf.rules {
                if let Some(ref sr) = r.source_ref {
                    if sr != "generated" && !known_ids.contains(sr.as_str()) {
                        unresolved.push(format!("rule/{}: {sr}", r.id));
                    }
                }
            }
        }
        if let Ok(Some(cf)) = pack.load_constraints() {
            for c in cf.all_constraints() {
                if let Some(ref sr) = c.source_ref {
                    if sr != "generated" && !known_ids.contains(sr.as_str()) {
                        unresolved.push(format!("constraint/{}: {sr}", c.id));
                    }
                }
            }
        }

        if unresolved.is_empty() {
            checks.push(CheckResult::pass("source_ref resolution"));
        } else {
            checks.push(CheckResult::fail(
                "source_ref resolution",
                format!("unresolved refs: {}", unresolved.join(", ")),
            ));
        }
    }

    // knowledge_graph edge resolution
    if pack.has_knowledge_graph() {
        match pack.load_graph() {
            Ok(Some(graph)) => {
                let node_ids: HashSet<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
                let broken: Vec<String> = graph
                    .edges
                    .iter()
                    .filter_map(|e| {
                        let src_ok = node_ids.contains(e.source.as_str());
                        let tgt_ok = node_ids.contains(e.target.as_str());
                        if !src_ok || !tgt_ok {
                            Some(format!("{}->{}", e.source, e.target))
                        } else {
                            None
                        }
                    })
                    .collect();
                if broken.is_empty() {
                    checks.push(CheckResult::pass("knowledge_graph edge resolution"));
                } else {
                    checks.push(CheckResult::fail(
                        "knowledge_graph edge resolution",
                        format!("broken edges: {}", broken.join(", ")),
                    ));
                }
            }
            Ok(None) => checks.push(CheckResult::skip("knowledge_graph.json (not present)")),
            Err(e) => checks.push(CheckResult::fail(
                "knowledge_graph.json parses",
                e.to_string(),
            )),
        }
    }

    // Procedure completeness when machine/procedures/ is non-empty
    if pack.has_procedures() {
        match procedures::validate_all(pack) {
            Ok(errors) if errors.is_empty() => {
                checks.push(CheckResult::pass("machine/procedures/ completeness"));
            }
            Ok(errors) => {
                for e in &errors {
                    checks.push(CheckResult::fail(
                        "machine/procedures/ completeness",
                        e.clone(),
                    ));
                }
            }
            Err(e) => {
                checks.push(CheckResult::fail(
                    "machine/procedures/ completeness",
                    e.to_string(),
                ));
            }
        }
    }

    // MCP manifest when mcp block is present
    if pack.manifest.mcp.is_some() {
        if pack.machine_file("mcp_manifest.json").exists() {
            checks.push(CheckResult::pass(
                "machine/mcp_manifest.json present (mcp configured)",
            ));
        } else {
            checks.push(CheckResult::fail(
                "machine/mcp_manifest.json present (mcp configured)",
                "manifest.mcp is set but machine/mcp_manifest.json is missing",
            ));
        }
    }

    let failed = checks.iter().any(|c| c.status == GateStatus::Fail);
    GateResult {
        gate: 4,
        status: if failed {
            GateStatus::Fail
        } else {
            GateStatus::Pass
        },
        checks,
        message: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn minimal_manifest_json() -> &'static str {
        r#"{
            "spec": "1.0.0",
            "name": "test-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "unit tests",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        }"#
    }

    /// Builds a pack directory with all gate 4 required machine files present
    /// and valid, returning the opened `Pack`.
    fn complete_pack(tmp: &TempDir) -> Pack {
        std::fs::write(tmp.path().join("manifest.json"), minimal_manifest_json()).unwrap();
        let machine = tmp.path().join("machine");
        std::fs::create_dir_all(&machine).unwrap();
        std::fs::write(machine.join("system_prompt.md"), "Be helpful.").unwrap();
        std::fs::write(machine.join("glossary.json"), r#"{"terms": []}"#).unwrap();
        std::fs::write(machine.join("ontology.json"), r#"{"entity_types": []}"#).unwrap();
        std::fs::write(machine.join("rules.json"), r#"{"rules": []}"#).unwrap();
        std::fs::write(
            machine.join("constraints.json"),
            r#"{"edge_cases": [], "anti_patterns": [], "hard_limits": []}"#,
        )
        .unwrap();
        std::fs::write(machine.join("retrieval_chunks.jsonl"), "").unwrap();
        Pack::open(tmp.path()).unwrap()
    }

    #[test]
    fn all_required_files_present_and_valid_passes() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
        assert_eq!(result.gate, 4);
    }

    #[test]
    fn missing_required_file_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::remove_file(pack.machine_file("glossary.json")).unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
        assert!(
            result
                .checks
                .iter()
                .any(|c| c.description.contains("glossary.json") && c.status == GateStatus::Fail)
        );
    }

    #[test]
    fn invalid_json_in_machine_file_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(pack.machine_file("rules.json"), "{ not valid").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
        assert!(
            result
                .checks
                .iter()
                .any(|c| c.description.contains("rules.json") && c.status == GateStatus::Fail)
        );
    }

    #[test]
    fn sources_csv_absent_skips_source_ref_check() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        let result = run(&pack);
        assert!(
            !result
                .checks
                .iter()
                .any(|c| c.description == "source_ref resolution")
        );
    }

    #[test]
    fn unresolved_source_ref_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(
            pack.machine_file("glossary.json"),
            r#"{"terms": [{"id": "t1", "term": "Term", "definition": "def", "source_ref": "src-1"}]}"#,
        )
        .unwrap();
        std::fs::create_dir_all(pack.evidence_dir()).unwrap();
        std::fs::write(pack.evidence_file("sources.csv"), "id,title\n").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
        assert!(
            result
                .checks
                .iter()
                .any(|c| c.description == "source_ref resolution" && c.status == GateStatus::Fail)
        );
    }

    #[test]
    fn source_ref_generated_is_always_allowed() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(
            pack.machine_file("glossary.json"),
            r#"{"terms": [{"id": "t1", "term": "Term", "definition": "def", "source_ref": "generated"}]}"#,
        )
        .unwrap();
        std::fs::create_dir_all(pack.evidence_dir()).unwrap();
        std::fs::write(pack.evidence_file("sources.csv"), "id,title\n").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
    }

    #[test]
    fn resolved_source_ref_passes() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(
            pack.machine_file("glossary.json"),
            r#"{"terms": [{"id": "t1", "term": "Term", "definition": "def", "source_ref": "src-1"}]}"#,
        )
        .unwrap();
        std::fs::create_dir_all(pack.evidence_dir()).unwrap();
        std::fs::write(
            pack.evidence_file("sources.csv"),
            "id,title\nsrc-1,Some Source\n",
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
    }

    #[test]
    fn knowledge_graph_broken_edge_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(
            pack.machine_file("knowledge_graph.json"),
            r#"{"nodes": [{"id": "n1", "node_type": "concept", "label": "N1"}], "edges": [{"source": "n1", "relation": "see-also", "target": "missing"}]}"#,
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
        assert!(result.checks.iter().any(|c| {
            c.description.contains("knowledge_graph edge resolution")
                && c.status == GateStatus::Fail
        }));
    }

    #[test]
    fn knowledge_graph_resolved_edges_pass() {
        let tmp = TempDir::new().unwrap();
        let pack = complete_pack(&tmp);
        std::fs::write(
            pack.machine_file("knowledge_graph.json"),
            r#"{"nodes": [{"id": "n1", "node_type": "concept", "label": "N1"}, {"id": "n2", "node_type": "concept", "label": "N2"}], "edges": [{"source": "n1", "relation": "see-also", "target": "n2"}]}"#,
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
    }
}
