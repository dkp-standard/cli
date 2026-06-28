use crate::{
    pack::loader::Pack,
    validate::gates::{CheckResult, GateResult, GateStatus},
};

/// Gate 7: Evaluation — eval_set.jsonl present and eval_report.json shows no failures.
pub fn run(pack: &Pack) -> GateResult {
    if !pack.has_eval_set() {
        return GateResult {
            gate: 7,
            status: GateStatus::Skipped,
            checks: vec![CheckResult::skip("eval_set.jsonl (optional)")],
            message: Some("eval_set.jsonl not present; gate 7 skipped".to_string()),
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

    // Check eval_report.json in build/
    let report_path = pack.root.join("build").join("eval_report.json");
    if report_path.exists() {
        match std::fs::read_to_string(&report_path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        {
            Some(v) => {
                let total = v["summary"]["total"].as_u64().unwrap_or(0);
                let failed = v["summary"]["failed"].as_u64().unwrap_or(0);
                if failed == 0 {
                    checks.push(CheckResult::pass(format!(
                        "eval_report.json: all {total} cases passed"
                    )));
                } else {
                    checks.push(CheckResult::fail(
                        "eval_report.json",
                        format!("{failed}/{total} cases failed"),
                    ));
                }
            }
            None => checks.push(CheckResult::fail(
                "eval_report.json parses",
                "could not parse eval_report.json",
            )),
        }
    } else {
        checks.push(CheckResult::skip(
            "eval_report.json (run `dkp eval` to generate)",
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
