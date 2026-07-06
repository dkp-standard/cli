pub mod brave;
pub mod discovered;
pub mod search;
pub mod web_fetch;

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use crate::config::GenConfig;
use crate::error::{GenError, GenResult};
use brave::BraveSearchProvider;
use discovered::{DiscoveredLog, DiscoveredSource};
use search::SearchProvider;
use web_fetch::WebFetchTool;

/// Construct the configured `SearchProvider`, or `None` if no search API key
/// is set (tool use may still offer `web_fetch` alone in that case).
pub fn make_search_provider(config: &GenConfig) -> GenResult<Option<Arc<dyn SearchProvider>>> {
    if config.search_api_key.is_empty() {
        return Ok(None);
    }
    match config.search_provider.as_str() {
        "brave" => {
            let provider = BraveSearchProvider::new(config.search_api_key.clone())?;
            Ok(Some(Arc::new(provider)))
        }
        other => Err(GenError::ToolFailed {
            name: "web_search".to_string(),
            message: format!("unknown search_provider '{other}' — supported providers: brave"),
        }),
    }
}

/// A tool advertised to the model, in a backend-agnostic shape.
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    /// JSON Schema for the tool's arguments.
    pub parameters: Value,
}

impl ToolSpec {
    /// Render as an OpenAI-compatible `tools` array entry.
    pub fn to_openai(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": self.parameters,
            }
        })
    }
}

/// Executes tool calls on behalf of an `LlmClient` tool-use loop.
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    fn specs(&self) -> Vec<ToolSpec>;
    async fn execute(&self, name: &str, arguments: &Value) -> GenResult<String>;
}

/// The concrete tool set `dkp-gen-core` offers to generation steps:
/// always `web_fetch`, plus `web_search` when a search provider is configured.
/// Every fetched/discovered URL is appended to a staging provenance log
/// (`build/sources_discovered.jsonl`) — never written directly to `evidence/sources.csv`.
pub struct GenToolExecutor {
    web_fetch: WebFetchTool,
    search: Option<Arc<dyn SearchProvider>>,
    discovered: Arc<DiscoveredLog>,
    command: String,
    verbose: bool,
}

impl GenToolExecutor {
    pub fn new(
        http_timeout_secs: u64,
        discovered: Arc<DiscoveredLog>,
        command: impl Into<String>,
        search: Option<Arc<dyn SearchProvider>>,
        verbose: bool,
    ) -> GenResult<Self> {
        Ok(Self {
            web_fetch: WebFetchTool::new(http_timeout_secs)?,
            search,
            discovered,
            command: command.into(),
            verbose,
        })
    }
}

#[async_trait]
impl ToolExecutor for GenToolExecutor {
    fn specs(&self) -> Vec<ToolSpec> {
        let mut specs = vec![ToolSpec {
            name: "web_fetch".to_string(),
            description: "Fetch a URL and return its content as plain text.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {"type": "string", "description": "The URL to fetch."}
                },
                "required": ["url"]
            }),
        }];
        if self.search.is_some() {
            specs.push(ToolSpec {
                name: "web_search".to_string(),
                description: "Search the web and return matching page titles, URLs, and snippets."
                    .to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "The search query."}
                    },
                    "required": ["query"]
                }),
            });
        }
        specs
    }

    async fn execute(&self, name: &str, arguments: &Value) -> GenResult<String> {
        match name {
            "web_fetch" => {
                let url = arguments["url"]
                    .as_str()
                    .ok_or_else(|| GenError::ToolFailed {
                        name: name.to_string(),
                        message: "missing required argument 'url'".to_string(),
                    })?;
                if self.verbose {
                    eprintln!("    🔧 web_fetch({url})");
                }
                let result = self.web_fetch.fetch(url).await;
                if self.verbose {
                    match &result {
                        Ok(text) => eprintln!("       ✓ fetched {} chars", text.len()),
                        Err(e) => eprintln!("       ✗ {e}"),
                    }
                }
                let text = result?;
                self.discovered.append(&DiscoveredSource::now(
                    url.to_string(),
                    None,
                    "web_fetch",
                    None,
                    self.command.clone(),
                ))?;
                Ok(text)
            }
            "web_search" => {
                let provider = self.search.as_ref().ok_or_else(|| GenError::ToolFailed {
                    name: name.to_string(),
                    message: "web_search is not available: no search provider configured"
                        .to_string(),
                })?;
                let query = arguments["query"]
                    .as_str()
                    .ok_or_else(|| GenError::ToolFailed {
                        name: name.to_string(),
                        message: "missing required argument 'query'".to_string(),
                    })?;
                if self.verbose {
                    eprintln!("    🔧 web_search({query:?})");
                }
                let results = provider.search(query, 5).await?;
                if self.verbose {
                    if results.is_empty() {
                        eprintln!("       ✓ 0 results");
                    } else {
                        eprintln!("       ✓ {} result(s):", results.len());
                        for r in &results {
                            eprintln!("         - {} ({})", r.title, r.url);
                        }
                    }
                }
                for r in &results {
                    self.discovered.append(&DiscoveredSource::now(
                        r.url.clone(),
                        Some(r.title.clone()),
                        "web_search",
                        Some(query.to_string()),
                        self.command.clone(),
                    ))?;
                }
                Ok(format_search_results(&results))
            }
            other => Err(GenError::ToolFailed {
                name: other.to_string(),
                message: "unknown tool".to_string(),
            }),
        }
    }
}

fn format_search_results(results: &[search::SearchResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }
    results
        .iter()
        .enumerate()
        .map(|(i, r)| format!("{}. {} — {}\n{}", i + 1, r.title, r.url, r.snippet))
        .collect::<Vec<_>>()
        .join("\n\n")
}
