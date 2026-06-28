use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

use crate::cmd::webui::server::AppState;

pub async fn get_assets(
    Path(asset_type): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response {
    let pack = &state.pack;

    match asset_type.as_str() {
        "glossary" => match pack.load_glossary() {
            Ok(Some(g)) => Json(g.terms).into_response(),
            Ok(None) => Json(Vec::<Value>::new()).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "rules" => match pack.load_rules() {
            Ok(Some(r)) => Json(r.rules).into_response(),
            Ok(None) => Json(Vec::<Value>::new()).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "chunks" => match pack.load_chunks() {
            Ok(chunks) => Json(chunks).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "constraints" => match pack.load_constraints() {
            Ok(Some(c)) => Json(c).into_response(),
            Ok(None) => Json(Value::Null).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "ontology" => match pack.load_ontology() {
            Ok(Some(o)) => Json(o.entity_types).into_response(),
            Ok(None) => Json(Vec::<Value>::new()).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "eval" => match pack.load_eval_set() {
            Ok(cases) => Json(cases).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        "cross-refs" => {
            use std::fs;
            let path = pack.machine_dir().join("cross_refs.json");
            match fs::read_to_string(&path) {
                Ok(s) => match serde_json::from_str::<Value>(&s) {
                    Ok(v) => Json(v).into_response(),
                    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                },
                Err(_) => Json(Value::Array(vec![])).into_response(),
            }
        }
        "system-prompt" => match pack.load_system_prompt() {
            Ok(Some(content)) => Json(SystemPromptResponse { content }).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "No system prompt").into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        _ => (
            StatusCode::NOT_FOUND,
            format!("Unknown asset type: {asset_type}"),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct SystemPromptResponse {
    content: String,
}
