pub mod events;
pub mod messages;

pub use events::AgentEvent;
pub use messages::Message;

pub trait AgentProtocol: Send + Sync {
    fn send_event(&self, event: AgentEvent);
}
