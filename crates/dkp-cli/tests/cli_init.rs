mod common;

use predicates::prelude::*;
use tempfile::TempDir;

use common::dkp_cmd;

#[test]
fn init_creates_expected_files_with_correct_manifest_fields() {
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("my-pack");

    dkp_cmd()
        .arg("init")
        .arg("my-pack")
        .arg("--domain")
        .arg("testing")
        .arg("--out")
        .arg(&out)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized DKP pack"));

    assert!(out.join("manifest.json").exists());
    assert!(out.join("machine/system_prompt.md").exists());
    assert!(out.join("machine/glossary.json").exists());
    assert!(out.join("evidence/sources.csv").exists());

    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["name"], "my-pack");
    assert_eq!(manifest["domain"], "testing");
}

#[test]
fn init_without_force_fails_on_existing_directory() {
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("dup-pack");
    std::fs::create_dir_all(&out).unwrap();

    dkp_cmd()
        .arg("init")
        .arg("dup-pack")
        .arg("--domain")
        .arg("testing")
        .arg("--out")
        .arg(&out)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn init_with_force_overwrites_existing_directory() {
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("dup-pack");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("stale.txt"), "old").unwrap();

    dkp_cmd()
        .arg("init")
        .arg("dup-pack")
        .arg("--domain")
        .arg("testing")
        .arg("--out")
        .arg(&out)
        .arg("--force")
        .assert()
        .success();

    assert!(out.join("manifest.json").exists());
}

#[test]
fn init_extras_scaffolds_optional_assets() {
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("extra-pack");

    dkp_cmd()
        .arg("init")
        .arg("extra-pack")
        .arg("--domain")
        .arg("testing")
        .arg("--out")
        .arg(&out)
        .arg("--extras")
        .assert()
        .success();

    assert!(out.join("machine/eval_set.jsonl").exists());
    assert!(out.join("machine/knowledge_graph.json").exists());
    assert!(out.join("human/handbook.md").exists());
    assert!(out.join("CHANGELOG.md").exists());
}
