mod common;

use predicates::prelude::*;
use tempfile::TempDir;

use common::{dkp_cmd, scaffold_pack};

#[test]
fn validate_conformant_pack_succeeds() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "valid-pack", "testing");

    dkp_cmd()
        .arg("validate")
        .arg(&pack)
        .assert()
        .success()
        .stdout(predicate::str::contains("Conformance:"));
}

#[test]
fn validate_broken_pack_fails_with_nonzero_exit() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "broken-pack", "testing");
    std::fs::remove_file(pack.join("machine/glossary.json")).unwrap();

    dkp_cmd()
        .arg("validate")
        .arg(&pack)
        .assert()
        .failure()
        .stdout(predicate::str::contains("FAIL"));
}

#[test]
fn validate_gate_flag_scopes_to_single_gate() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "gate-pack", "testing");

    dkp_cmd()
        .arg("validate")
        .arg(&pack)
        .arg("--gate")
        .arg("4")
        .assert()
        .success()
        .stdout(predicate::str::contains("[Gate 4]"))
        .stdout(predicate::str::contains("[Gate 7]").not())
        .stdout(predicate::str::contains("[Gate 8]").not());
}

#[test]
fn validate_json_output_is_valid_json() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "json-pack", "testing");

    let output = dkp_cmd()
        .arg("--output")
        .arg("json")
        .arg("validate")
        .arg(&pack)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let report: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(report.get("conformance").is_some());
    assert!(report.get("gates").is_some());
}
