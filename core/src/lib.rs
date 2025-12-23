use backend::{LlmClient, MockClient};
use protocol::{AgentEvent, Message};
use tokio::sync::mpsc;

pub struct Session {
    history: Vec<Message>,
    client: Box<dyn LlmClient>,
    event_tx: mpsc::Sender<AgentEvent>,
}

impl Session {
    pub fn new(event_tx: mpsc::Sender<AgentEvent>) -> Self {
        Self {
            history: Vec::new(),
            client: Box::new(MockClient),
            event_tx,
        }
    }

    pub async fn handle_user_input(&mut self, input: String) {
        // 1. Add user message
        self.history.push(Message {
            role: "user".to_string(),
            content: input.clone(),
        });
        
        // 2. Notify UI
        let _ = self.event_tx.send(AgentEvent::UserMessage(input)).await;
        let _ = self.event_tx.send(AgentEvent::AgentThinking("Processing...".to_string())).await;

        // 3. Call Backend
        match self.client.complete(&self.history).await {
            Ok(response) => {
                self.history.push(Message {
                    role: "assistant".to_string(),
                    content: response.clone(),
                });
                let _ = self.event_tx.send(AgentEvent::AgentResponse(response)).await;
            }
            Err(e) => {
                let _ = self.event_tx.send(AgentEvent::SystemError(e.to_string())).await;
            }
        }
    }
}
