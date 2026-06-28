use async_trait::async_trait;

use crate::error::GenResult;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, system: &str, user: &str) -> GenResult<String>;
}
