use protocol::AgentEvent;
use tokio::sync::mpsc;

pub struct App {
    pub messages: Vec<String>,
    pub input: String,
    pub event_rx: mpsc::Receiver<AgentEvent>,
    pub input_tx: mpsc::Sender<String>,
    pub scroll: u16,
    pub show_exit_popup: bool,
}

impl App {
    pub fn new(event_rx: mpsc::Receiver<AgentEvent>, input_tx: mpsc::Sender<String>) -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            event_rx,
            input_tx,
            scroll: 0,
            show_exit_popup: false,
        }
    }
}
