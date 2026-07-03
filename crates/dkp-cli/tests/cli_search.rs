mod common;

use predicates::prelude::*;
use tempfile::TempDir;

use common::dkp_cmd;

/// Builds a small, explicit pack with known searchable content rather than
/// relying on `dkp init`'s stub text, which could change independently.
fn build_searchable_pack(tmp: &TempDir) -> std::path::PathBuf {
    let pack = tmp.path().join("search-pack");
    std::fs::create_dir_all(pack.join("machine")).unwrap();
    std::fs::write(
        pack.join("manifest.json"),
        r#"{
            "spec": "1.0.0",
            "name": "search-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "search test",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        }"#,
    )
    .unwrap();
    std::fs::write(pack.join("machine/glossary.json"), r#"{"terms": []}"#).unwrap();
    std::fs::write(
        pack.join("machine/retrieval_chunks.jsonl"),
        concat!(
            r#"{"id":"chunk-onboard","title":"Onboarding Guide","chunk_text":"How to onboard a new hire into the widgetronic platform.","source_ref":"generated"}"#,
            "\n",
            r#"{"id":"chunk-offboard","title":"Offboarding Guide","chunk_text":"How to offboard a departing employee.","source_ref":"generated"}"#,
            "\n",
        ),
    )
    .unwrap();
    pack
}

#[test]
fn search_returns_matching_chunk() {
    let tmp = TempDir::new().unwrap();
    let pack = build_searchable_pack(&tmp);

    dkp_cmd()
        .arg("search")
        .arg(&pack)
        .arg("widgetronic")
        .assert()
        .success()
        .stdout(predicate::str::contains("chunk-onboard"));
}

#[test]
fn search_no_match_returns_no_results_message() {
    let tmp = TempDir::new().unwrap();
    let pack = build_searchable_pack(&tmp);

    dkp_cmd()
        .arg("search")
        .arg(&pack)
        .arg("zzznonexistentzzz")
        .assert()
        .success()
        .stdout(predicate::str::contains("No results"));
}

#[test]
fn search_limit_flag_is_accepted() {
    let tmp = TempDir::new().unwrap();
    let pack = build_searchable_pack(&tmp);

    dkp_cmd()
        .arg("search")
        .arg(&pack)
        .arg("onboard")
        .arg("--limit")
        .arg("1")
        .assert()
        .success();
}

#[test]
fn search_missing_query_arg_fails() {
    let tmp = TempDir::new().unwrap();
    let pack = build_searchable_pack(&tmp);

    dkp_cmd().arg("search").arg(&pack).assert().failure();
}
