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

    fn open_pack(tmp: &TempDir) -> Pack {
        std::fs::write(tmp.path().join("manifest.json"), minimal_manifest_json()).unwrap();
        Pack::open(tmp.path()).unwrap()
    }

    #[test]
    fn okf_absent_is_not_applicable() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        let result = run(&pack);
        assert_eq!(result.status, GateStatus::NotApplicable);
        assert_eq!(result.gate, 8);
    }

    #[test]
    fn okf_valid_frontmatter_passes() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::write(
            pack.okf_dir().join("concept.md"),
            "---\ntype: concept\n---\nBody text.\n",
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
    }

    #[test]
    fn okf_missing_type_field_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::write(
            pack.okf_dir().join("concept.md"),
            "---\nname: concept\n---\nBody text.\n",
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
    }

    #[test]
    fn okf_unparseable_file_fails() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::write(pack.okf_dir().join("concept.md"), "no frontmatter here").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Fail);
    }

    #[test]
    fn bundle_sig_absent_is_skipped_not_failed() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::write(
            pack.okf_dir().join("concept.md"),
            "---\ntype: concept\n---\nBody.\n",
        )
        .unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
        assert!(
            result
                .checks
                .iter()
                .any(|c| c.description.contains("bundle.sig") && c.status == GateStatus::Skipped)
        );
    }

    #[test]
    fn bundle_sig_present_passes_that_check() {
        let tmp = TempDir::new().unwrap();
        let pack = open_pack(&tmp);
        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::write(
            pack.okf_dir().join("concept.md"),
            "---\ntype: concept\n---\nBody.\n",
        )
        .unwrap();
        std::fs::write(pack.okf_dir().join("bundle.sig"), "signature-bytes").unwrap();

        let result = run(&pack);
        assert_eq!(result.status, GateStatus::Pass);
        assert!(
            result
                .checks
                .iter()
                .any(|c| c.description == "bundle.sig present" && c.status == GateStatus::Pass)
        );
    }
}
