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
