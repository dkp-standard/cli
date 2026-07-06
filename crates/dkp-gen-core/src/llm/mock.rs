use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

use crate::error::{GenError, GenResult};
use crate::llm::client::LlmClient;
use crate::tools::ToolExecutor;

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

/// Test double for tool-use loops: executes a scripted sequence of tool
/// calls against whatever `ToolExecutor` is passed in (so tests exercise
/// real tool logic, e.g. `WebFetchTool` against a wiremock server), then
/// returns a fixed final response.
pub struct MockToolClient {
    pub scripted_calls: Vec<(String, Value)>,
    pub final_response: String,
}

impl MockToolClient {
    pub fn new(scripted_calls: Vec<(String, Value)>, final_response: impl Into<String>) -> Self {
        Self {
            scripted_calls,
            final_response: final_response.into(),
        }
    }
}

#[async_trait]
impl LlmClient for MockToolClient {
    async fn complete(&self, _system: &str, _user: &str) -> GenResult<String> {
        Ok(self.final_response.clone())
    }

    async fn complete_with_tools(
        &self,
        _system: &str,
        _user: &str,
        tools: &dyn ToolExecutor,
        _max_turns: u32,
    ) -> GenResult<String> {
        for (name, args) in &self.scripted_calls {
            tools.execute(name, args).await?;
        }
        Ok(self.final_response.clone())
    }
}
