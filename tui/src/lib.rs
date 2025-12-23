use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use protocol::AgentEvent;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use tokio::sync::mpsc;

pub struct App {
    messages: Vec<String>,
    input: String,
    event_rx: mpsc::Receiver<AgentEvent>,
    input_tx: mpsc::Sender<String>,
    scroll: u16,
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

    pub async fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // Draw UI
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                    .split(f.area());

                // Chat History
                let history_text = self.messages.join("\n");
                let content_height = self.messages.len() as u16;
                let inner_height = chunks[0].height.saturating_sub(2);
                let max_scroll = content_height.saturating_sub(inner_height);
                
                // Clamp scroll to valid range
                self.scroll = self.scroll.min(max_scroll);

                let history = Paragraph::new(history_text)
                    .block(Block::default().title("Chat History").borders(Borders::ALL))
                    .scroll((self.scroll, 0));
                f.render_widget(history, chunks[0]);

                // Input Box
                let input = Paragraph::new(self.input.clone())
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().title("Input").borders(Borders::ALL));
                f.render_widget(input, chunks[1]);
            })?;

            // Handle Events
            tokio::select! {
                // Agent Events
                Some(event) = self.event_rx.recv() => {
                    match event {
                        AgentEvent::UserMessage(msg) => self.messages.push(format!("You: {}", msg)),
                        AgentEvent::AgentResponse(msg) => self.messages.push(format!("Agent: {}", msg)),
                        AgentEvent::AgentThinking(msg) => self.messages.push(format!("(Thinking: {})", msg)),
                        AgentEvent::SystemError(err) => self.messages.push(format!("Error: {}", err)),
                        AgentEvent::Shutdown => break,
                    }
                    // Auto-scroll to bottom on new events
                    self.scroll = u16::MAX;
                }
                
                // User Input
                _ = tokio::time::sleep(std::time::Duration::from_millis(16)) => {
                   if event::poll(std::time::Duration::from_millis(100))? {
                        if let Event::Key(key) = event::read()? {
                            if key.kind == event::KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Enter => {
                                        let msg = self.input.drain(..).collect();
                                        self.input_tx.send(msg).await.unwrap();
                                    }
                                    KeyCode::Char(c) => self.input.push(c),
                                    KeyCode::Backspace => { self.input.pop(); },
                                    KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                                    KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                                    KeyCode::Esc => return Ok(()),
                                    _ => {}
                                }
                            }
                        }
                   }
                } 
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
