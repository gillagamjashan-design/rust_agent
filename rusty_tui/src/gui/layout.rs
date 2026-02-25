use super::app::RustyApp;
use super::messages::{Message, Role};
use super::theme;
use egui::{RichText, ScrollArea};

pub fn render_ui(ctx: &egui::Context, app: &mut RustyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            // Header
            render_header(ui, app);
            ui.separator();

            // Chat area (expanding)
            render_chat_area(ui, app);
            ui.separator();

            // Input area
            render_input_area(ui, app);
        });
    });
}

fn render_header(ui: &mut egui::Ui, app: &RustyApp) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("🦀 Rusty").color(theme::BRIGHT_CYAN).strong());
        ui.label(RichText::new("Rust Learning Agent").color(theme::GRAY));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !app.knowledge_stats.is_empty() {
                ui.label(RichText::new(&app.knowledge_stats).color(theme::YELLOW).small());
            }
        });
    });
    ui.add_space(5.0);
}

fn render_chat_area(ui: &mut egui::Ui, app: &mut RustyApp) {
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(app.scroll_to_bottom)
        .show(ui, |ui| {
            for message in &app.messages {
                render_message(ui, message);
            }

            if app.waiting_for_response {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(RichText::new("Agent is thinking...").color(theme::YELLOW));
                });
            }
        });

    // Reset scroll flag after rendering
    if app.scroll_to_bottom {
        app.scroll_to_bottom = false;
    }
}

fn render_message(ui: &mut egui::Ui, message: &Message) {
    let (prefix, color) = match message.role {
        Role::User => ("You: ", theme::CYAN),
        Role::Assistant => ("Agent: ", theme::GREEN),
        Role::System => ("System: ", theme::YELLOW),
    };

    let timestamp = message.timestamp.format("%H:%M:%S");

    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("[{}]", timestamp))
                .color(theme::GRAY)
                .small()
        );
        ui.label(
            RichText::new(prefix)
                .color(color)
                .strong()
        );
    });

    // Format message content
    let lines: Vec<&str> = message.content.lines().collect();
    let mut in_code_block = false;
    let mut code_lines = Vec::new();

    for line in lines {
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                let code_text = code_lines.join("\n");
                ui.add(
                    egui::TextEdit::multiline(&mut code_text.as_str())
                        .code_editor()
                        .desired_width(f32::INFINITY)
                );
                code_lines.clear();
                in_code_block = false;
            } else {
                // Start of code block
                in_code_block = true;
            }
        } else if in_code_block {
            code_lines.push(line);
        } else {
            ui.label(RichText::new(line).color(theme::FG));
        }
    }

    ui.add_space(10.0);
}

fn render_input_area(ui: &mut egui::Ui, app: &mut RustyApp) {
    ui.horizontal(|ui| {
        let response = ui.add_sized(
            [ui.available_width() - 80.0, 30.0],
            egui::TextEdit::singleline(&mut app.input)
                .hint_text("Type your message or /help for commands...")
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

        if ui.button(RichText::new("Send").color(theme::BRIGHT_CYAN).strong()).clicked()
            && !app.input.is_empty() {
            let input = app.input.clone();
            app.input.clear();
            app.send_message(input);
            app.scroll_to_bottom = true;
        }
    });
}
