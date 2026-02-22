use crate::app::{App, Mode, Panel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

// Tokyo Night Theme Colors
pub const BG: Color = Color::Rgb(26, 27, 38);
pub const FG: Color = Color::Rgb(192, 202, 245);
pub const SELECTION: Color = Color::Rgb(51, 70, 124);
pub const BORDER: Color = Color::Rgb(125, 207, 255);
pub const KEYWORD: Color = Color::Rgb(187, 154, 247);
pub const COMMENT: Color = Color::Rgb(86, 95, 137);
pub const STRING: Color = Color::Rgb(158, 206, 106);
pub const NUMBER: Color = Color::Rgb(255, 158, 100);

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Main content
            Constraint::Length(10), // Terminal
            Constraint::Length(3),  // Status
        ])
        .split(f.size());

    render_title(f, chunks[0], app);
    render_main(f, chunks[1], app);
    render_terminal(f, chunks[2], app);
    render_status(f, chunks[3], app);
}

fn render_title(f: &mut Frame, area: Rect, app: &App) {
    let dir_name = app.current_directory
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(app.current_directory.to_str().unwrap_or(""));

    let title = vec![
        Span::styled("Rusty TUI", Style::default().fg(KEYWORD).add_modifier(Modifier::BOLD)),
        Span::raw(" - "),
        Span::styled(dir_name, Style::default().fg(BORDER).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(
            format!("{:?}", app.mode),
            Style::default().fg(STRING),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("{:?}", app.focused_panel),
            Style::default().fg(NUMBER),
        ),
    ];

    let title_block = Paragraph::new(Line::from(title))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(BORDER)))
        .style(Style::default().bg(BG).fg(FG));

    f.render_widget(title_block, area);
}

fn render_main(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // File tree
            Constraint::Percentage(50), // Editor
            Constraint::Percentage(30), // Agent
        ])
        .split(area);

    render_file_tree(f, chunks[0], app);
    render_editor(f, chunks[1], app);
    render_agent(f, chunks[2], app);
}

fn render_file_tree(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .file_tree_items
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == app.file_tree_cursor {
                Style::default().bg(SELECTION).fg(FG)
            } else {
                Style::default().fg(FG)
            };
            ListItem::new(name.clone()).style(style)
        })
        .collect();

    let border_style = if app.focused_panel == Panel::FileTree {
        Style::default().fg(KEYWORD)
    } else {
        Style::default().fg(BORDER)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" Files - {} ", app.current_directory.display())),
        )
        .style(Style::default().bg(BG).fg(FG));

    f.render_widget(list, area);
}

fn render_editor(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if app.focused_panel == Panel::Editor {
        Style::default().fg(KEYWORD)
    } else {
        Style::default().fg(BORDER)
    };

    let title = if let Some(ref path) = app.current_file {
        format!(" Editor - {} ", path.display())
    } else {
        " Editor - No file open ".to_string()
    };

    // Simple syntax highlighting for Rust
    let lines: Vec<Line> = app
        .current_content
        .lines()
        .skip(app.scroll_offset)
        .take(area.height.saturating_sub(2) as usize)
        .map(|line| highlight_rust_line(line))
        .collect();

    let text = Text::from(lines);

    let editor = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: false });

    f.render_widget(editor, area);
}

fn render_agent(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if app.focused_panel == Panel::Agent {
        Style::default().fg(KEYWORD)
    } else {
        Style::default().fg(BORDER)
    };

    // Split agent panel: chat history (top) + input (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // Chat history
            Constraint::Length(3),  // Input box
        ])
        .split(area);

    // Render chat history
    let mut history_text = Vec::new();
    if app.agent_chat_history.is_empty() {
        history_text.push(Line::from(Span::styled(
            "Press 'a' to focus, 'i' to enter insert mode, type query, and press Enter.",
            Style::default().fg(COMMENT),
        )));
        history_text.push(Line::from(""));
        history_text.push(Line::from(Span::styled("Agent Status:", Style::default().fg(KEYWORD))));
        for line in app.agent_manager.get_status().lines() {
            history_text.push(Line::from(line.to_string()));
        }
    } else {
        for (is_user, message) in &app.agent_chat_history {
            let prefix = if *is_user { "You: " } else { "Agent: " };
            let style = if *is_user {
                Style::default().fg(STRING)
            } else {
                Style::default().fg(FG)
            };

            // Split long messages
            for line in message.lines() {
                history_text.push(Line::from(Span::styled(
                    format!("{}{}", prefix, line),
                    style,
                )));
            }
            history_text.push(Line::from("")); // Empty line between messages
        }
    }

    let history = Paragraph::new(history_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" AI Agent "),
        )
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: true })
        .scroll((0, 0));

    f.render_widget(history, chunks[0]);

    // Render input box with cursor
    let input_text = if app.mode == Mode::Insert && app.focused_panel == Panel::Agent {
        format!("> {}█", app.agent_input)
    } else {
        format!("> {}", app.agent_input)
    };

    let input = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().bg(BG).fg(FG));

    f.render_widget(input, chunks[1]);
}

fn render_terminal(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if app.focused_panel == Panel::Terminal {
        Style::default().fg(KEYWORD)
    } else {
        Style::default().fg(BORDER)
    };

    // Split terminal: output (top) + input (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // Output
            Constraint::Length(3),  // Input line
        ])
        .split(area);

    // Render output
    let output = app.terminal_output.join("");

    let terminal_output = Paragraph::new(output)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" Terminal "),
        )
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: false });

    f.render_widget(terminal_output, chunks[0]);

    // Render input line with cursor
    let input_text = if app.mode == Mode::Insert && app.focused_panel == Panel::Terminal {
        format!("$ {}█", app.terminal_input)
    } else {
        format!("$ {}", app.terminal_input)
    };

    let terminal_input = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().bg(BG).fg(FG));

    f.render_widget(terminal_input, chunks[1]);
}

fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let status_text = if app.mode == Mode::Command {
        format!(":{}", app.command_buffer)
    } else {
        app.status_message.clone()
    };

    let help = match app.mode {
        Mode::Normal => {
            " [h/l/a/t] Switch Panel | [j/k] Navigate Tree | [i] Insert Mode | [Enter] Open File | [:] Command | [q] Quit "
        }
        Mode::Insert => {
            " [Esc] Normal Mode | [Enter] Send/Execute | Type to input... "
        }
        Mode::Command => {
            " [Esc] Cancel | [Enter] Execute | Type command... "
        }
    };

    let status = Paragraph::new(vec![
        Line::from(status_text),
        Line::from(Span::styled(help, Style::default().fg(COMMENT))),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(BORDER)))
    .style(Style::default().bg(BG).fg(FG));

    f.render_widget(status, area);
}

fn highlight_rust_line(line: &str) -> Line {
    let mut spans = Vec::new();
    let keywords = [
        "fn", "let", "mut", "pub", "struct", "enum", "impl", "use", "mod", "crate",
        "self", "Self", "if", "else", "match", "for", "while", "loop", "return",
        "async", "await", "const", "static", "trait", "type", "where",
    ];

    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    if !indent.is_empty() {
        spans.push(Span::raw(indent.to_string()));
    }

    if trimmed.starts_with("//") {
        spans.push(Span::styled(trimmed.to_string(), Style::default().fg(COMMENT)));
        return Line::from(spans);
    }

    let mut current = String::new();
    let mut in_string = false;
    let chars: Vec<char> = trimmed.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '"' {
            if !current.is_empty() {
                let is_keyword = keywords.contains(&current.as_str());
                let style = if is_keyword {
                    Style::default().fg(KEYWORD)
                } else {
                    Style::default().fg(FG)
                };
                spans.push(Span::styled(current.clone(), style));
                current.clear();
            }

            in_string = !in_string;
            let mut string_content = String::from('"');
            i += 1;

            while i < chars.len() {
                let c = chars[i];
                string_content.push(c);
                if c == '"' && chars.get(i.saturating_sub(1)) != Some(&'\\') {
                    break;
                }
                i += 1;
            }

            spans.push(Span::styled(string_content, Style::default().fg(STRING)));
        } else if ch.is_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            if !current.is_empty() {
                let is_keyword = keywords.contains(&current.as_str());
                let is_number = current.chars().all(|c| c.is_numeric());
                let style = if is_keyword {
                    Style::default().fg(KEYWORD)
                } else if is_number {
                    Style::default().fg(NUMBER)
                } else {
                    Style::default().fg(FG)
                };
                spans.push(Span::styled(current.clone(), style));
                current.clear();
            }
            spans.push(Span::raw(ch.to_string()));
        }

        i += 1;
    }

    if !current.is_empty() {
        let is_keyword = keywords.contains(&current.as_str());
        let style = if is_keyword {
            Style::default().fg(KEYWORD)
        } else {
            Style::default().fg(FG)
        };
        spans.push(Span::styled(current, style));
    }

    Line::from(spans)
}
