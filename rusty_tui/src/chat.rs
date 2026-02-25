//! Chat message rendering with syntax highlighting

use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Role};
use crate::ui::colors;

pub fn render_messages(f: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();
    
    for message in &app.messages {
        // Add timestamp
        let timestamp = message.timestamp.format("%H:%M:%S").to_string();
        lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", timestamp),
                Style::default().fg(colors::GRAY)
            ),
        ]));
        
        // Add message with role-based coloring
        let (prefix, color) = match message.role {
            Role::User => ("You: ", colors::CYAN),
            Role::Assistant => ("Agent: ", colors::GREEN),
            Role::System => ("System: ", colors::YELLOW),
        };
        
        // Split message into lines and handle code blocks
        let content_lines = format_message(&message.content);
        for (i, line) in content_lines.into_iter().enumerate() {
            if i == 0 {
                lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    Span::raw(line.clone()),
                ]));
            } else {
                lines.push(Line::from(Span::raw(line.clone())));
            }
        }
        
        // Add blank line between messages
        lines.push(Line::from(""));
    }
    
    let messages_widget = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::NONE)
        )
        .wrap(Wrap { trim: false })
        .scroll((app.scroll as u16, 0));
    
    f.render_widget(messages_widget, area);
}

fn format_message(content: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_code_block = false;
    
    for line in content.lines() {
        if line.starts_with("```") {
            in_code_block = !in_code_block;
            result.push(line.to_string());
        } else if in_code_block {
            // Code block line - could add syntax highlighting here
            result.push(format!("  {}", line));
        } else {
            result.push(line.to_string());
        }
    }
    
    result
}
