mod common;

use predicates::prelude::*;
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use common::{dkp_cmd, scaffold_pack};

#[tokio::test]
async fn generate_missing_api_key_fails_without_any_network_call() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "gen-pack", "testing");

    dkp_cmd()
        .arg("generate")
        .arg(&pack)
        .env_remove("DKP_GEN_API_KEY")
        .env("HOME", tmp.path()) // avoid picking up a real ~/.dkp/gen.toml
        .assert()
        .failure()
        .stderr(predicate::str::contains("API key required"));
}

#[tokio::test]
async fn generate_against_mock_server_writes_generated_assets() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "gen-mock-pack", "testing");

    // `dkp generate` regenerates the whole machine + human layer in one run
    // (the `assets` filter arg isn't wired up yet), so the mock must return a
    // response valid for every asset kind requested: plain text for
    // text-based assets, JSON for the *.json assets, markdown-ish text for
    // human assets. A single JSON-object response satisfies both text and
    // JSON consumers well enough to drive the pipeline to completion without
    // per-prompt branching in the mock.
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{"message": {"content": "{\"terms\":[],\"rules\":[],\"entity_types\":[],\"edge_cases\":[],\"anti_patterns\":[],\"hard_limits\":[],\"trees\":[]}"}}]
        })))
        .mount(&server)
        .await;

    dkp_cmd()
        .arg("generate")
        .arg(&pack)
        .arg("--api-key")
        .arg("test-key")
        .arg("--base-url")
        .arg(server.uri())
        .arg("--overwrite")
        .arg("--no-tools")
        .assert()
        .success();

    let content = std::fs::read_to_string(pack.join("machine/rules.json")).unwrap();
    let rules: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(rules["rules"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn generate_with_tools_enabled_but_no_search_key_fails_fast() {
    let tmp = TempDir::new().unwrap();
    let pack = scaffold_pack(tmp.path(), "gen-no-search-key-pack", "testing");

    dkp_cmd()
        .arg("generate")
        .arg(&pack)
        .arg("--api-key")
        .arg("test-key")
        .env_remove("DKP_GEN_SEARCH_API_KEY")
        .env("HOME", tmp.path()) // avoid picking up a real ~/.dkp/gen.toml
        .assert()
        .failure()
        .stderr(predicate::str::contains("no search API key is set"));
}
