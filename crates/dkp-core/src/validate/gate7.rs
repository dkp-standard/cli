use crate::{
    pack::loader::Pack,
    validate::gates::{CheckResult, GateResult, GateStatus},
};

/// Gate 7: Evaluation — eval_set.jsonl present and evidence/eval_results/eval_summary.json
/// reports gate7_pass=true (SPEC.md §12.4, §16).
pub fn run(pack: &Pack) -> GateResult {
    let has_cases = pack.has_eval_set()
        && pack
            .load_eval_set()
            .map(|cases| !cases.is_empty())
            .unwrap_or(false);

    if !has_cases {
        return GateResult {
            gate: 7,
            status: GateStatus::Skipped,
            checks: vec![CheckResult::skip("eval_set.jsonl (optional, or empty)")],
            message: Some("eval_set.jsonl not present or empty; gate 7 skipped".to_string()),
        };
    }

    let mut checks = Vec::new();

    // Count eval cases
    match pack.load_eval_set() {
        Ok(cases) => checks.push(CheckResult::pass(format!(
            "eval_set.jsonl: {} cases loaded",
            cases.len()
        ))),
        Err(e) => checks.push(CheckResult::fail("eval_set.jsonl parses", e.to_string())),
    }

    // Check evidence/eval_results/eval_summary.json (SPEC.md §12.4, Appendix B §B.13)
    let summary_path = pack
        .evidence_dir()
        .join("eval_results")
        .join("eval_summary.json");
    if summary_path.exists() {
        match std::fs::read_to_string(&summary_path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        {
            Some(v) => {
                let gate7_pass = v["gate7_pass"].as_bool().unwrap_or(false);
                let mean_delta = v["mean_delta"].as_f64().unwrap_or(0.0);
                if gate7_pass {
                    checks.push(CheckResult::pass(format!(
                        "eval_summary.json: gate7_pass=true (mean_delta={mean_delta:.3})"
                    )));
                } else {
                    let min_delta = pack.manifest.min_eval_delta.unwrap_or(0.0);
                    checks.push(CheckResult::fail(
                        "eval_summary.json",
                        format!("gate7_pass=false (mean_delta={mean_delta:.3}, required >= {min_delta:.3})"),
                    ));
                }
            }
            None => checks.push(CheckResult::fail(
                "eval_summary.json parses",
                "could not parse evidence/eval_results/eval_summary.json",
            )),
        }
    } else {
        checks.push(CheckResult::fail(
            "evidence/eval_results/eval_summary.json",
            "not found; run `dkp eval` to generate before publishing",
        ));
    }

    let failed = checks.iter().any(|c| c.status == GateStatus::Fail);
    GateResult {
        gate: 7,
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
            "spec": "dkp/0.2",
            "name": "test-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "unit tests",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        }"#
    }

    fn eval_case_line() -> &'static str {
        r#"{"query": "q", "expected_dimensions": [], "critical_must_include": [], "scoring_rubric": "r", "version_meta": {"prompt_hash": "h", "model_version": "m", "dataset_version": "d"}}"#
    }

    fn open_pack(tmp: &TempDir) -> Pack {
        std::fs::write(tmp.path().join("manifest.json"), minimal_manifest_json()).unwrap();
        Pack::open(tmp.path()).unwrap()
    }

    #[test]
    fn eval_set_absent_is_skipped() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Skipped);
        assert_eq!(result.gate, 7);
    }

    #[test]
    fn eval_set_present_but_empty_is_skipped() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(pack.machine_file("eval_set.jsonl"), "").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Skipped);
    }

    #[test]
    fn eval_set_present_but_no_summary_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(
            pack.machine_file("eval_set.jsonl"),
            format!("{}\n", eval_case_line()),
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
        assert!(result.checks.iter().any(|c| c
            .description
            .contains("evidence/eval_results/eval_summary.json")
            && c.status == GateStatus::Fail));
    }

    #[test]
    fn eval_summary_gate7_pass_true_passes() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(
            pack.machine_file("eval_set.jsonl"),
            format!("{}\n", eval_case_line()),
        )
        .unwrap();
        let eval_results_dir = pack.evidence_dir().join("eval_results");
        std::fs::create_dir_all(&eval_results_dir).unwrap();
        std::fs::write(
            eval_results_dir.join("eval_summary.json"),
            r#"{"gate7_pass": true, "mean_delta": 0.42}"#,
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
    }

    #[test]
    fn eval_summary_gate7_pass_false_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(
            pack.machine_file("eval_set.jsonl"),
            format!("{}\n", eval_case_line()),
        )
        .unwrap();
        let eval_results_dir = pack.evidence_dir().join("eval_results");
        std::fs::create_dir_all(&eval_results_dir).unwrap();
        std::fs::write(
            eval_results_dir.join("eval_summary.json"),
            r#"{"gate7_pass": false, "mean_delta": 0.01}"#,
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
    }

    #[test]
    fn eval_summary_unparseable_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(
            pack.machine_file("eval_set.jsonl"),
            format!("{}\n", eval_case_line()),
        )
        .unwrap();
        let eval_results_dir = pack.evidence_dir().join("eval_results");
        std::fs::create_dir_all(&eval_results_dir).unwrap();
        std::fs::write(eval_results_dir.join("eval_summary.json"), "not json").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
    }
}
