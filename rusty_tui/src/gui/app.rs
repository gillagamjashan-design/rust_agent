use super::messages::{Message, Role, UserCommand, WorkerMessage};
use super::theme;
use super::worker;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

pub struct RustyApp {
    pub messages: Vec<Message>,
    pub input: String,
    pub waiting_for_response: bool,
    pub knowledge_stats: String,
    pub scroll_to_bottom: bool,
    pub first_render: bool,

    // Channels for async communication
    message_rx: Receiver<WorkerMessage>,
    command_tx: Sender<UserCommand>,
}

impl RustyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Create channels
        let (command_tx, command_rx) = channel();
        let (message_tx, message_rx) = channel();

        // Get database path
        let db_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".agent")
            .join("data")
            .join("knowledge.db");

        // Spawn worker thread
        worker::spawn_worker(command_rx, message_tx, db_path);

        // Create welcome message
        let welcome = Message::new(
            Role::System,
            "Welcome to Rusty! 🦀\n\nI'm your Rust learning agent. Ask me anything about Rust programming!\n\nType /help to see available commands.".to_string(),
        );

        Self {
            messages: vec![welcome],
            input: String::new(),
            waiting_for_response: false,
            knowledge_stats: String::new(),
            scroll_to_bottom: false,
            first_render: true,
            message_rx,
            command_tx,
        }
    }

    pub fn send_message(&mut self, input: String) {
        // Add user message to chat
        self.messages.push(Message::new(Role::User, input.clone()));

        // Determine if it's a command or query
        if input.starts_with('/') {
            // Handle special case for /clear
            if input.trim() == "/clear" {
                self.messages.clear();
                self.messages.push(Message::new(
                    Role::System,
                    "Chat history cleared.".to_string(),
                ));
                return;
            }

            // Handle /quit
            if input.trim() == "/quit" || input.trim() == "/exit" || input.trim() == "/q" {
                self.messages.push(Message::new(
                    Role::System,
                    "Goodbye! 👋\n\nClose the window to exit.".to_string(),
                ));
                return;
            }

            // Send command to worker
            self.command_tx.send(UserCommand::Command(input)).ok();
        } else {
            // Send query to worker
            self.waiting_for_response = true;
            self.command_tx.send(UserCommand::Query(input)).ok();
        }
    }

    pub fn handle_worker_message(&mut self, msg: WorkerMessage) {
        match msg {
            WorkerMessage::Response(text) => {
                self.messages.push(Message::new(Role::Assistant, text));
                self.waiting_for_response = false;
                self.scroll_to_bottom = true;
            }
            WorkerMessage::SystemMessage(text) => {
                self.messages.push(Message::new(Role::System, text));
                self.scroll_to_bottom = true;
            }
            WorkerMessage::Error(text) => {
                self.messages.push(Message::new(
                    Role::System,
                    format!("❌ Error: {}", text),
                ));
                self.waiting_for_response = false;
                self.scroll_to_bottom = true;
            }
            WorkerMessage::Stats(text) => {
                self.knowledge_stats = text;
            }
        }
    }
}

impl eframe::App for RustyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll for worker messages (non-blocking)
        while let Ok(msg) = self.message_rx.try_recv() {
            self.handle_worker_message(msg);
        }

        // Apply theme
        theme::apply_theme(ctx);

        // Render UI
        super::layout::render_ui(ctx, self);

        // Request repaint for smooth animations
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
