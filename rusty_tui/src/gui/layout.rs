use super::app::RustyApp;
use super::messages::{Message, Role};
use super::theme;
use egui::{RichText, ScrollArea};

pub fn render_ui(ctx: &egui::Context, app: &mut RustyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            // Header (fixed height ~40px)
            render_header(ui, app);
            ui.separator();

            // Chat area - allocate all remaining space
            let available_height = ui.available_height() - 80.0; // Reserve 80px for input frame + separator
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    render_chat_area(ui, app);
                }
            );

            ui.separator();

            // Input area (fixed height ~40px)
            render_input_area(ui, app);
        });
    });
}

fn render_header(ui: &mut egui::Ui, app: &RustyApp) {
    theme::header_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Logo and title
            ui.label(RichText::new("🦀").size(24.0));
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Rusty")
                        .color(theme::BRIGHT_CYAN)
                        .strong()
                        .size(18.0)
                );
                ui.label(
                    RichText::new("Rust Learning Agent")
                        .color(theme::GRAY)
                        .size(11.0)
                );
            });

            // Stats badge on right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !app.knowledge_stats.is_empty() {
                    egui::Frame::none()
                        .fill(theme::GRAY.linear_multiply(0.3))
                        .rounding(egui::Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(&app.knowledge_stats)
                                    .color(theme::YELLOW)
                                    .size(11.0)
                            );
                        });
                }
            });
        });
    });
}

fn render_chat_area(ui: &mut egui::Ui, app: &mut RustyApp) {
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .min_scrolled_height(200.0)  // CRITICAL FIX: Minimum 200px height
        .max_height(ui.available_height())  // Use all available height
        .stick_to_bottom(app.scroll_to_bottom)
        .show(ui, |ui| {
            // Add padding
            ui.add_space(10.0);

            for (index, message) in app.messages.iter().enumerate() {
                let alpha = app.get_message_alpha(index);
                render_message(ui, message, alpha);
            }

            if app.waiting_for_response {
                // Polished waiting indicator with bubble frame
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width() * 0.85, 0.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        theme::message_bubble_frame(&Role::Assistant).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🦀").size(18.0));
                                ui.spinner();
                                ui.label(
                                    RichText::new("Thinking...")
                                        .color(theme::YELLOW)
                                        .size(13.5)
                                );
                            });
                        });
                    },
                );
                ui.add_space(12.0);
            }

            // Bottom padding
            ui.add_space(10.0);
        });

    // Reset scroll flag after rendering
    if app.scroll_to_bottom {
        app.scroll_to_bottom = false;
    }
}

fn render_message(ui: &mut egui::Ui, message: &Message, alpha: f32) {
    let (avatar, role_name, color) = match message.role {
        Role::User => ("👤", "You", theme::CYAN),
        Role::Assistant => ("🦀", "Rusty", theme::GREEN),
        Role::System => ("ℹ️", "System", theme::YELLOW),
    };

    let max_width = match message.role {
        Role::User => ui.available_width() * 0.75,
        _ => ui.available_width() * 0.85,
    };

    // User messages are right-aligned
    if matches!(message.role, Role::User) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(max_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    render_message_bubble(ui, message, avatar, role_name, color, alpha);
                },
            );
        });
    } else {
        // Assistant and System messages are left-aligned
        ui.allocate_ui_with_layout(
            egui::vec2(max_width, 0.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                render_message_bubble(ui, message, avatar, role_name, color, alpha);
            },
        );
    }

    ui.add_space(12.0);  // Spacing between messages
}

fn render_message_bubble(
    ui: &mut egui::Ui,
    message: &Message,
    avatar: &str,
    role_name: &str,
    color: egui::Color32,
    alpha: f32,
) {
    let frame = if alpha < 1.0 {
        theme::message_bubble_frame_alpha(&message.role, alpha)
    } else {
        theme::message_bubble_frame(&message.role)
    };

    frame.show(ui, |ui| {
        // Header: avatar + role name + timestamp
        ui.horizontal(|ui| {
            ui.label(RichText::new(avatar).size(18.0));
            ui.label(
                RichText::new(role_name)
                    .color(color)
                    .strong()
                    .size(14.0)
            );

            // Timestamp on right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let timestamp = message.timestamp.format("%H:%M:%S");
                ui.label(
                    RichText::new(format!("[{}]", timestamp))
                        .color(theme::GRAY)
                        .size(11.0)
                );
            });
        });

        ui.add_space(6.0);
        render_message_content(ui, &message.content);
    });
}

fn render_message_content(ui: &mut egui::Ui, content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut code_lines = Vec::new();
    let mut language = String::new();

    for line in lines {
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                render_code_block(ui, &code_lines.join("\n"), &language);
                code_lines.clear();
                language.clear();
                in_code_block = false;
            } else {
                // Start of code block - extract language
                language = line.trim().trim_start_matches("```").to_string();
                in_code_block = true;
            }
        } else if in_code_block {
            code_lines.push(line);
        } else {
            // Regular text with enhanced typography
            render_text_line(ui, line);
        }
    }
}

fn render_code_block(ui: &mut egui::Ui, code: &str, language: &str) {
    theme::code_block_frame().show(ui, |ui| {
        // Show language tag if present
        if !language.is_empty() {
            ui.label(
                RichText::new(language)
                    .color(theme::YELLOW)
                    .monospace()
                    .size(11.0)
            );
            ui.add_space(4.0);
        }

        // Code content (create mutable copy for TextEdit)
        let mut code_copy = code.to_string();
        ui.add(
            egui::TextEdit::multiline(&mut code_copy)
                .code_editor()
                .desired_width(f32::INFINITY)
                .interactive(false)  // Make it read-only
        );
    });
}

fn render_text_line(ui: &mut egui::Ui, line: &str) {
    // Enhanced typography with better line spacing
    if line.trim().starts_with("•") || line.trim().starts_with("-") {
        // Bullet points
        ui.label(RichText::new(line).color(theme::FG).size(13.5));
    } else if line.trim().starts_with("##") {
        // Bold headings
        let text = line.trim().trim_start_matches('#').trim();
        ui.label(RichText::new(text).color(theme::BRIGHT_CYAN).strong().size(15.0));
    } else {
        // Regular text
        ui.label(RichText::new(line).color(theme::FG).size(13.5));
    }
}

fn render_input_area(ui: &mut egui::Ui, app: &mut RustyApp) {
    theme::input_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Emoji indicator
            ui.label(RichText::new("💬").size(18.0));

            let response = ui.add_sized(
                [ui.available_width() - 90.0, 30.0],
                egui::TextEdit::singleline(&mut app.input)
                    .hint_text("Ask me anything about Rust...")
                    .desired_width(f32::INFINITY)
            );

            // THIS FIXES THE ENTER KEY ISSUE
            // Check if Enter was pressed while the text field has focus
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !app.input.is_empty() {
                    let input = app.input.clone();
                    app.input.clear();
                    app.send_message(input);
                    app.scroll_to_bottom = true;
                }
                response.request_focus();
            }

            // Auto-focus on startup
            if app.first_render {
                response.request_focus();
                app.first_render = false;
            }

            // Enhanced send button
            if ui.button(RichText::new("Send ↗").color(theme::BRIGHT_CYAN).strong()).clicked()
                && !app.input.is_empty() {
                let input = app.input.clone();
                app.input.clear();
                app.send_message(input);
                app.scroll_to_bottom = true;
            }
        });
    });
}
