use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentEvent {
    UserMessage(String),
    AgentThinking(String),
    AgentResponse(String),
    SystemError(String),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub trait AgentProtocol: Send + Sync {
    fn send_event(&self, event: AgentEvent);
}
