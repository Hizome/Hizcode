use async_trait::async_trait;
use protocol::Message;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String>;
}

pub struct MockClient;

#[async_trait]
impl LlmClient for MockClient {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        // Simulate network latency
        sleep(Duration::from_millis(1000)).await;
        
        let last_msg = messages.last().map(|m| m.content.as_str()).unwrap_or("");
        
        Ok(format!("Mock Response to: '{}'. I am a simple agent based on Codex architecture.", last_msg))
    }
}
