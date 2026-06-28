use crate::{
    okf::parser::parse_okf_dir,
    pack::loader::Pack,
    validate::gates::{CheckResult, GateResult, GateStatus},
};

/// Gate 8: OKF Conformance — frontmatter valid, type field present, bundle.sig present.
pub fn run(pack: &Pack) -> GateResult {
    if !pack.has_okf() {
        return GateResult {
            gate: 8,
            status: GateStatus::NotApplicable,
            checks: vec![CheckResult::skip("okf/ layer (not present)")],
            message: Some("OKF layer absent; gate 8 not applicable".to_string()),
        };
    }

    let mut checks = Vec::new();

    // Walk and parse all OKF concept files
    match parse_okf_dir(&pack.okf_dir()) {
        Ok(concepts) => {
            let n = concepts.len();
            checks.push(CheckResult::pass(format!("OKF concept files: {n} parsed")));

            // Every concept must have a 'type' key in frontmatter
            let missing_type: Vec<String> = concepts
                .iter()
                .filter(|c| c.frontmatter.get("type").is_none())
                .map(|c| {
                    c.path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect();

            if missing_type.is_empty() {
                checks.push(CheckResult::pass(format!(
                    "OKF frontmatter: all {n} files have 'type' field"
                )));
            } else {
                checks.push(CheckResult::fail(
                    "OKF frontmatter: 'type' field required",
                    format!("missing 'type': {}", missing_type.join(", ")),
                ));
            }
        }
        Err(e) => {
            checks.push(CheckResult::fail("OKF concept files parse", e.to_string()));
        }
    }

    // bundle.sig presence
    let bundle_sig = pack.okf_dir().join("bundle.sig");
    if bundle_sig.exists() {
        checks.push(CheckResult::pass("bundle.sig present"));
    } else {
        checks.push(CheckResult::skip("bundle.sig (not present)"));
    }

    let failed = checks.iter().any(|c| c.status == GateStatus::Fail);
    GateResult {
        gate: 8,
        status: if failed {
            GateStatus::Fail
        } else {
            GateStatus::Pass
        },
        checks,
        message: None,
    }
}
