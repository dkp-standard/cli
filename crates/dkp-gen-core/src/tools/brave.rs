use async_trait::async_trait;
use serde_json::Value;

use crate::error::{GenError, GenResult};
use crate::tools::search::{SearchProvider, SearchResult};

const DEFAULT_BASE_URL: &str = "https://api.search.brave.com/res/v1";

/// Brave Search API implementation of `SearchProvider` — the default,
/// cost/simplicity-motivated first implementation behind the trait (not a
/// lock-in; see `.docs/GEN_AGENTS.md` §3.2). Users must supply their own key.
pub struct BraveSearchProvider {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl BraveSearchProvider {
    pub fn new(api_key: String) -> GenResult<Self> {
        Self::with_base_url(api_key, DEFAULT_BASE_URL.to_string())
    }

    /// `base_url` is injectable so tests can point at a wiremock server.
    pub fn with_base_url(api_key: String, base_url: String) -> GenResult<Self> {
        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| GenError::ToolFailed {
                name: "web_search".to_string(),
                message: format!("failed to build HTTP client: {e}"),
            })?;
        Ok(Self {
            http,
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }
}

#[async_trait]
impl SearchProvider for BraveSearchProvider {
    async fn search(&self, query: &str, max_results: u32) -> GenResult<Vec<SearchResult>> {
        let url = format!("{}/web/search", self.base_url);

        // Brave's free tier is rate-limited to 1 req/sec; on a 429 back off
        // briefly once and retry, rather than failing the whole tool call.
        let max_attempts = 2u32;
        let mut attempts = 0u32;
        loop {
            attempts += 1;
            let resp = self
                .http
                .get(&url)
                .query(&[("q", query), ("count", &max_results.to_string())])
                .header("X-Subscription-Token", &self.api_key)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| GenError::ToolFailed {
                    name: "web_search".to_string(),
                    message: format!("request failed: {e}"),
                })?;

            let status = resp.status();
            if status.as_u16() == 429 && attempts < max_attempts {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            if status.as_u16() == 401 {
                return Err(GenError::ToolFailed {
                    name: "web_search".to_string(),
                    message: "Brave Search API rejected the key (401) — check search_api_key"
                        .to_string(),
                });
            }
            if !status.is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(GenError::ToolFailed {
                    name: "web_search".to_string(),
                    message: format!("Brave Search API returned HTTP {}: {body}", status.as_u16()),
                });
            }

            let json: Value = resp.json().await.map_err(|e| GenError::ToolFailed {
                name: "web_search".to_string(),
                message: format!("failed to decode Brave Search response: {e}"),
            })?;

            let results = json["web"]["results"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|r| SearchResult {
                    url: r["url"].as_str().unwrap_or_default().to_string(),
                    title: r["title"].as_str().unwrap_or_default().to_string(),
                    snippet: r["description"].as_str().unwrap_or_default().to_string(),
                })
                .collect();

            return Ok(results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn search_parses_web_results() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/web/search"))
            .and(header("X-Subscription-Token", "test-brave-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "web": {
                    "results": [
                        {"url": "https://example.com/a", "title": "A", "description": "desc a"},
                        {"url": "https://example.com/b", "title": "B", "description": "desc b"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let provider =
            BraveSearchProvider::with_base_url("test-brave-key".to_string(), server.uri()).unwrap();
        let results = provider.search("rust async traits", 5).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://example.com/a");
        assert_eq!(results[0].title, "A");
        assert_eq!(results[0].snippet, "desc a");
    }

    #[tokio::test]
    async fn search_401_errors_with_clear_message() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/web/search"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let provider =
            BraveSearchProvider::with_base_url("bad-key".to_string(), server.uri()).unwrap();
        let err = provider.search("query", 5).await.unwrap_err();
        assert!(matches!(err, GenError::ToolFailed { .. }));
    }

    #[tokio::test]
    async fn search_empty_results_returns_empty_vec() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/web/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"web": {"results": []}})),
            )
            .mount(&server)
            .await;

        let provider =
            BraveSearchProvider::with_base_url("test-brave-key".to_string(), server.uri()).unwrap();
        let results = provider.search("query", 5).await.unwrap();
        assert!(results.is_empty());
    }
}
