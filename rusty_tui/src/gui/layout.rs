use super::app::RustyApp;
use super::messages::{Message, Role, PendingFileCreation};
use super::theme;
use egui::{RichText, ScrollArea};

pub fn render_ui(ctx: &egui::Context, app: &mut RustyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            // Header (fixed height ~40px)
            render_header(ui, app);
            ui.separator();

            // Chat area - allocate all remaining space
            let available_height = ui.available_height() - 50.0; // Reserve 50px for input + separator
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

    // Render file confirmation dialog (overlays chat)
    if let Some(ref pending) = app.pending_file_confirmation.clone() {
        render_file_confirmation_dialog(ctx, app, &pending);
    }
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
        .min_scrolled_height(200.0)  // CRITICAL FIX: Minimum 200px height
        .max_height(ui.available_height())  // Use all available height
        .stick_to_bottom(app.scroll_to_bottom)
        .show(ui, |ui| {
            // Add padding
            ui.add_space(10.0);

            for message in &app.messages {
                render_message(ui, message);
            }

            if app.waiting_for_response {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(RichText::new("Agent is thinking...").color(theme::YELLOW));
                });
            }

            // Bottom padding
            ui.add_space(10.0);
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
        Role::FileOperation => ("📁 File: ", theme::BRIGHT_CYAN),
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

    ui.add_space(15.0);  // ADD SPACE BETWEEN MESSAGES
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

pub fn render_file_confirmation_dialog(
    ctx: &egui::Context,
    app: &mut RustyApp,
    pending: &PendingFileCreation,
) {
    // Keyboard shortcuts
    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        app.approve_file_creation();
        return;
    }

    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.cancel_file_creation();
        return;
    }

    egui::Window::new("📝 Confirm File Creation")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.heading("The agent wants to create the following files:");
            ui.add_space(10.0);

            // File list
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    for op in &pending.operations {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("📄");
                                ui.strong(&op.path);
                            });

                            // Content preview (first 3 lines)
                            let preview: Vec<&str> = op.content.lines().take(3).collect();
                            ui.label(format!("Preview:\n{}", preview.join("\n")));

                            if op.content.lines().count() > 3 {
                                ui.label(format!("... ({} more lines)", op.content.lines().count() - 3));
                            }
                        });
                        ui.add_space(5.0);
                    }
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Buttons
            ui.horizontal(|ui| {
                if ui.button("✅ Create Files").clicked() {
                    app.approve_file_creation();
                }

                ui.add_space(10.0);

                if ui.button("❌ Cancel").clicked() {
                    app.cancel_file_creation();
                }
            });

            ui.add_space(5.0);
            ui.label("💡 Tip: Press Enter to create, Esc to cancel");
        });
}
