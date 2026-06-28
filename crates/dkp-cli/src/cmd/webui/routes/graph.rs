use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::Value;
use std::sync::Arc;

use crate::cmd::webui::server::AppState;

pub async fn get_graph(State(state): State<Arc<AppState>>) -> Response {
    match state.pack.load_graph() {
        Ok(Some(graph)) => Json(graph).into_response(),
        Ok(None) => Json(Value::Null).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
