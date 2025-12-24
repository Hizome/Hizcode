use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentEvent {
    UserMessage(String),
    AgentThinking(String),
    AgentResponse(String),
    SystemError(String),
    Shutdown,
}
