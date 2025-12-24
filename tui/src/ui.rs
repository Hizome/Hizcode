use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
use crate::app::App;
#[allow(unused_imports)]
use ratatui::layout::Rect;

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

    // Cleanup Popup
    if app.show_exit_popup {
        let block = Block::default().title("Quit").borders(Borders::ALL).style(Style::default().bg(Color::Red).fg(Color::White));
        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(
            Paragraph::new("Are you sure you want to quit? (y/n)")
                .block(block)
                .alignment(ratatui::layout::Alignment::Center),
            area,
        );
    }
}

// Helper function to center the popup
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
