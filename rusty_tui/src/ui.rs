//! UI rendering with ratatui

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Role};
use crate::chat::render_messages;

/// Tokyo Night theme colors
pub mod colors {
    use ratatui::style::Color;
    
    pub const BG: Color = Color::Rgb(26, 27, 38);           // #1a1b26
    pub const FG: Color = Color::Rgb(192, 202, 245);        // #c0caf5
    pub const CYAN: Color = Color::Rgb(122, 162, 247);      // #7aa2f7 (User)
    pub const GREEN: Color = Color::Rgb(158, 206, 106);     // #9ece6a (Agent)
    pub const YELLOW: Color = Color::Rgb(224, 175, 104);    // #e0af68 (System)
    pub const GRAY: Color = Color::Rgb(65, 72, 104);        // #414868
    pub const BRIGHT_CYAN: Color = Color::Rgb(125, 207, 255); // #7dcfff
}

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Header
            Constraint::Min(0),         // Chat area
            Constraint::Length(3),      // Input
        ])
        .split(f.size());
    
    // Render header
    render_header(f, chunks[0], app);
    
    // Render chat messages
    render_messages(f, chunks[1], app);
    
    // Render input box
    render_input(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let time = chrono::Local::now().format("%H:%M:%S").to_string();
    
    let header_text = vec![
        Line::from(vec![
            Span::styled("🦀 ", Style::default().fg(colors::YELLOW)),
            Span::styled("Rusty", Style::default().fg(colors::BRIGHT_CYAN).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(
                format!("[{}] ", app.knowledge_stats),
                Style::default().fg(colors::GRAY)
            ),
            Span::raw(" ".repeat(area.width.saturating_sub(50) as usize)),
            Span::styled(time, Style::default().fg(colors::GRAY)),
        ]),
    ];
    
    let header = Paragraph::new(header_text)
        .block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(colors::GRAY))
        )
        .alignment(Alignment::Left);
    
    f.render_widget(header, area);
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    let input_text = vec![
        Line::from(vec![
            Span::styled("> ", Style::default().fg(colors::BRIGHT_CYAN)),
            Span::raw(&app.input),
            Span::styled("█", Style::default().fg(colors::BRIGHT_CYAN)), // Cursor
        ]),
    ];
    
    let input = Paragraph::new(input_text)
        .block(Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(colors::GRAY))
            .title(" Type your question or /help for commands ")
            .title_style(Style::default().fg(colors::GRAY))
        );
    
    f.render_widget(input, area);
}
