use async_trait::async_trait;

use crate::error::GenResult;
use crate::tools::ToolExecutor;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, system: &str, user: &str) -> GenResult<String>;

    /// Bounded tool-use loop: the model may call tools from `executor` up to
    /// `max_turns` times before a final text response is required.
    ///
    /// Default implementation ignores tools and falls back to `complete`, so
    /// existing `LlmClient` implementors (and tests built against them) keep
    /// compiling and behaving the same until they opt into a real tool loop.
    async fn complete_with_tools(
        &self,
        system: &str,
        user: &str,
        _tools: &dyn ToolExecutor,
        _max_turns: u32,
    ) -> GenResult<String> {
        self.complete(system, user).await
    }
}
