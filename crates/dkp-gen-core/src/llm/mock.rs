use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::{GenError, GenResult};
use crate::llm::client::LlmClient;

/// Test double: returns a fixed response keyed by substring of the user prompt.
pub struct MockClient {
    /// Map from user-prompt substring to canned response.
    fixtures: HashMap<String, String>,
    /// Fallback response when no fixture matches.
    fallback: String,
}

impl MockClient {
    pub fn new(fixtures: HashMap<String, String>, fallback: impl Into<String>) -> Self {
        Self {
            fixtures,
            fallback: fallback.into(),
        }
    }

    pub fn with_fallback(fallback: impl Into<String>) -> Self {
        Self::new(HashMap::new(), fallback)
    }
}

#[async_trait]
impl LlmClient for MockClient {
    async fn complete(&self, _system: &str, user: &str) -> GenResult<String> {
        for (key, val) in &self.fixtures {
            if user.contains(key.as_str()) {
                return Ok(val.clone());
            }
        }
        if self.fallback.is_empty() {
            return Err(GenError::LlmRefusal(
                "mock: no fixture matched and no fallback".into(),
            ));
        }
        Ok(self.fallback.clone())
    }
}
