pub mod app;
pub mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use protocol::AgentEvent;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use ui::ui;

pub async fn run(mut app: App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle Events
        tokio::select! {
            // Agent Events
            Some(event) = app.event_rx.recv() => {
                match event {
                    AgentEvent::UserMessage(msg) => app.messages.push(format!("You: {}", msg)),
                    AgentEvent::AgentResponse(msg) => app.messages.push(format!("Agent: {}", msg)),
                    AgentEvent::AgentThinking(msg) => app.messages.push(format!("(Thinking: {})", msg)),
                    AgentEvent::SystemError(err) => app.messages.push(format!("Error: {}", err)),
                    AgentEvent::Shutdown => break,
                }
                // Auto-scroll to bottom on new events
                app.scroll = u16::MAX;
            }
            
            // User Input
            _ = tokio::time::sleep(std::time::Duration::from_millis(16)) => {
               if event::poll(std::time::Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == event::KeyEventKind::Press {
                            if app.show_exit_popup {
                                match key.code {
                                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => break,
                                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.show_exit_popup = false,
                                    _ => {}
                                }
                            } else {
                                match key.code {
                                    KeyCode::Enter => {
                                        let msg = app.input.drain(..).collect();
                                        app.input_tx.send(msg).await.unwrap();
                                    }
                                    KeyCode::Char(c) => app.input.push(c),
                                    KeyCode::Backspace => { app.input.pop(); },
                                    KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
                                    KeyCode::Down => app.scroll = app.scroll.saturating_add(1),
                                    KeyCode::Esc => app.show_exit_popup = true,
                                    _ => {}
                                }
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
