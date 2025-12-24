use protocol::AgentEvent;
use tokio::sync::mpsc;

pub struct App {
    pub messages: Vec<String>,
    pub input: String,
    pub event_rx: mpsc::Receiver<AgentEvent>,
    pub input_tx: mpsc::Sender<String>,
    pub scroll: u16,
}

impl App {
    pub fn new(event_rx: mpsc::Receiver<AgentEvent>, input_tx: mpsc::Sender<String>) -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            event_rx,
            input_tx,
            scroll: 0,
        }
    }
}
