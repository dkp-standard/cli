mod common;

use common::{dkp_cmd, scaffold_pack};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn add_source_from_discovered_promotes_entry_interactively() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "rights-pack", "testing");

    std::fs::create_dir_all(pack.join("build")).unwrap();
    std::fs::write(
        pack.join("build/sources_discovered.jsonl"),
        r#"{"url":"https://example.com/doc","title":"Example Doc","via":"web_fetch","retrieved_at":"2026-01-01T00:00:00Z","command":"generate"}"#,
    )
    .unwrap();

    dkp_cmd()
        .arg("rights")
        .arg(&pack)
        .arg("add-source")
        .arg("--from-discovered")
        .write_stdin("y\nCC-BY-4.0\nsome notes\n")
        .assert()
        .success();

    let csv = std::fs::read_to_string(pack.join("evidence/sources.csv")).unwrap();
    assert!(csv.contains("https://example.com/doc"));
    assert!(csv.contains("CC-BY-4.0"));
    assert!(csv.contains("Example Doc"));
}

#[test]
fn add_source_from_discovered_skips_urls_already_in_sources_csv() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "rights-dedup-pack", "testing");

    std::fs::create_dir_all(pack.join("build")).unwrap();
    std::fs::create_dir_all(pack.join("evidence")).unwrap();
    std::fs::write(
        pack.join("build/sources_discovered.jsonl"),
        r#"{"url":"https://example.com/already","title":"Already There","via":"web_fetch","retrieved_at":"2026-01-01T00:00:00Z","command":"generate"}"#,
    )
    .unwrap();
    std::fs::write(
        pack.join("evidence/sources.csv"),
        "id,title,url,retrieved_date,license,notes\nsrc-001,Already There,https://example.com/already,2026-01-01,MIT,\n",
    )
    .unwrap();

    dkp_cmd()
        .arg("rights")
        .arg(&pack)
        .arg("add-source")
        .arg("--from-discovered")
        .assert()
        .success()
        .stdout(predicate::str::contains("No new discovered sources"));
}

#[test]
fn add_source_from_discovered_reports_missing_jsonl_clearly() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "rights-missing-pack", "testing");

    dkp_cmd()
        .arg("rights")
        .arg(&pack)
        .arg("add-source")
        .arg("--from-discovered")
        .assert()
        .success()
        .stdout(predicate::str::contains("No discovered sources found"));
}
