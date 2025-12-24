use async_trait::async_trait;
use protocol::Message;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String>;
}
