mod common;

use predicates::prelude::*;
use tempfile::TempDir;

use common::{dkp_cmd, scaffold_pack};

#[test]
fn list_shows_all_packs_under_root() {
    let tmp = TempDir::new().unwrap();
    scaffold_pack(tmp.path(), "pack-one", "testing");
    scaffold_pack(tmp.path(), "pack-two", "testing");

    dkp_cmd()
        .arg("list")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("pack-one"))
        .stdout(predicate::str::contains("pack-two"));
}

#[test]
fn list_domain_filter_narrows_results() {
    let tmp = TempDir::new().unwrap();
    scaffold_pack(tmp.path(), "alpha-pack", "alpha-domain");
    scaffold_pack(tmp.path(), "beta-pack", "beta-domain");

    dkp_cmd()
        .arg("list")
        .arg(tmp.path())
        .arg("--domain")
        .arg("alpha-domain")
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha-pack"))
        .stdout(predicate::str::contains("beta-pack").not());
}

#[test]
fn list_empty_directory_reports_no_packs_without_erroring() {
    let tmp = TempDir::new().unwrap();
    let empty = tmp.path().join("empty");
    std::fs::create_dir_all(&empty).unwrap();

    dkp_cmd()
        .arg("list")
        .arg(&empty)
        .assert()
        .success()
        .stdout(predicate::str::contains("No packs found"));
}
