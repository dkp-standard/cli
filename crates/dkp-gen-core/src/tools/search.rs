use async_trait::async_trait;

use crate::error::GenResult;

/// Provider-agnostic web search, mirroring the `LlmClient` pattern: `dkp-gen-core`
/// depends on this trait, not on any one vendor's SDK.
#[async_trait]
pub trait SearchProvider: Send + Sync {
    async fn search(&self, query: &str, max_results: u32) -> GenResult<Vec<SearchResult>>;
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String,
}

/// Test double: returns scripted results keyed by substring of the query,
/// same pattern as `llm::mock::MockClient`.
pub struct MockSearchProvider {
    fixtures: std::collections::HashMap<String, Vec<SearchResult>>,
}

impl MockSearchProvider {
    pub fn new(fixtures: std::collections::HashMap<String, Vec<SearchResult>>) -> Self {
        Self { fixtures }
    }
}

#[async_trait]
impl SearchProvider for MockSearchProvider {
    async fn search(&self, query: &str, max_results: u32) -> GenResult<Vec<SearchResult>> {
        for (key, results) in &self.fixtures {
            if query.contains(key.as_str()) {
                return Ok(results.iter().take(max_results as usize).cloned().collect());
            }
        }
        Ok(vec![])
    }
}
