mod common;

use tempfile::TempDir;

use common::{dkp_cmd, scaffold_pack};

#[test]
fn build_default_format_produces_archive_and_checksums() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "build-pack", "testing");
    let build_out = tmp.path().join("out");

    dkp_cmd()
        .arg("build")
        .arg(&pack)
        .arg("--out")
        .arg(&build_out)
        .assert()
        .success();

    assert!(build_out.join("checksums.json").exists());
    let checksums: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(build_out.join("checksums.json")).unwrap())
            .unwrap();
    assert!(checksums.as_object().unwrap().contains_key("manifest.json"));

    let archives: Vec<_> = std::fs::read_dir(&build_out)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "dkp"))
        .collect();
    assert_eq!(archives.len(), 1);
}

#[test]
fn build_zip_format_produces_zip_archive() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "zip-pack", "testing");
    let build_out = tmp.path().join("out");

    dkp_cmd()
        .arg("build")
        .arg(&pack)
        .arg("--format")
        .arg("zip")
        .arg("--out")
        .arg(&build_out)
        .assert()
        .success();

    let zips: Vec<_> = std::fs::read_dir(&build_out)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "zip"))
        .collect();
    assert_eq!(zips.len(), 1);
}

#[test]
fn build_no_human_excludes_human_directory_from_checksums() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "nohuman-pack", "testing");
    // scaffold_pack doesn't pass --extras, so add human/ manually to have
    // something meaningful to exclude.
    std::fs::create_dir_all(pack.join("human")).unwrap();
    std::fs::write(pack.join("human/handbook.md"), "Handbook content.").unwrap();
    let build_out = tmp.path().join("out");

    dkp_cmd()
        .arg("build")
        .arg(&pack)
        .arg("--no-human")
        .arg("--out")
        .arg(&build_out)
        .assert()
        .success();

    let checksums: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(build_out.join("checksums.json")).unwrap())
            .unwrap();
    let keys: Vec<&str> = checksums
        .as_object()
        .unwrap()
        .keys()
        .map(|s| s.as_str())
        .collect();
    assert!(!keys.iter().any(|k| k.starts_with("human/")));
}

#[test]
fn build_nonexistent_pack_fails() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("does-not-exist");

    dkp_cmd().arg("build").arg(&missing).assert().failure();
}
