use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Chat History
    let history_text = app.messages.join("\n");
    let content_height = app.messages.len() as u16;
    let inner_height = chunks[0].height.saturating_sub(2);
    let max_scroll = content_height.saturating_sub(inner_height);

    // Clamp scroll to valid range
    app.scroll = app.scroll.min(max_scroll);

    let history = Paragraph::new(history_text)
        .block(Block::default().title("Chat History").borders(Borders::ALL))
        .scroll((app.scroll, 0));
    f.render_widget(history, chunks[0]);

    // Input Box
    let input = Paragraph::new(app.input.clone())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().title("Input").borders(Borders::ALL));
    f.render_widget(input, chunks[1]);
}
