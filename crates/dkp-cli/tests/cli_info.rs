mod common;

use predicates::prelude::*;
use tempfile::TempDir;

use common::{dkp_cmd, scaffold_pack};

#[test]
fn info_prints_pack_summary() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "info-pack", "testing");

    dkp_cmd()
        .arg("info")
        .arg(&pack)
        .assert()
        .success()
        .stdout(predicate::str::contains("info-pack"))
        .stdout(predicate::str::contains("testing"));
}

#[test]
fn info_json_output_contains_expected_fields() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "info-json-pack", "testing");

    let output = dkp_cmd()
        .arg("--output")
        .arg("json")
        .arg("info")
        .arg(&pack)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let info: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(info["name"], "info-json-pack");
    assert_eq!(info["domain"], "testing");
    assert!(info.get("asset_counts").is_some());
}

#[test]
fn info_nonexistent_pack_fails() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("nope");

    dkp_cmd().arg("info").arg(&missing).assert().failure();
}
