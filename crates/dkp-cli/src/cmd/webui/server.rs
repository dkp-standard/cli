use anyhow::Result;
use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use dkp_core::{search::SearchIndex, Pack};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

use super::{embed::Assets, routes, WebuiArgs};

pub struct AppState {
    pub pack: Pack,
    pub search_index: SearchIndex,
}

pub async fn serve(args: WebuiArgs) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let search_index = SearchIndex::build(&pack)?;

    let state = Arc::new(AppState { pack, search_index });

    let api = Router::new()
        .route("/api/pack", get(routes::pack::get_pack))
        .route("/api/assets/{asset_type}", get(routes::assets::get_assets))
        .route("/api/graph", get(routes::graph::get_graph))
        .route("/api/search", get(routes::search::search))
        .with_state(state);

    let app = api.fallback(spa_handler).layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;
    let url = format!("http://{bound}");

    println!("Serving DKP web UI at {url}");
    println!("Press Ctrl+C to stop.");

    if !args.no_open {
        let _ = open::that(&url);
    }

    axum::serve(listener, app).await?;
    Ok(())
}

async fn spa_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // Try to serve the exact asset first
    if let Some(asset) = Assets::get(path) {
        let content_type = content_type_for(path);
        return ([(header::CONTENT_TYPE, content_type)], asset.data).into_response();
    }

    // Fall back to index.html for SPA client-side routing
    match Assets::get("index.html") {
        Some(index) => (
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            index.data,
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            "Web UI not built. Run `npm run build` in the web/ directory.",
        )
            .into_response(),
    }
}

fn content_type_for(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
}
