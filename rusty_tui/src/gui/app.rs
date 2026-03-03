use super::messages::{Message, Role, UserCommand, WorkerMessage, PendingFileCreation};
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

    // File confirmation dialog state
    pub pending_file_confirmation: Option<PendingFileCreation>,

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

        eprintln!("🚀 Rusty GUI starting...");
        eprintln!("📂 Database path: {:?}", db_path);

        // Spawn worker thread
        worker::spawn_worker(command_rx, message_tx, db_path);

        eprintln!("✅ Worker thread spawned");
        eprintln!("🎨 Initializing GUI...");

        // Get workspace for welcome message
        let workspace = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "current directory".to_string());

        // Create welcome message
        let welcome = Message::new(
            Role::System,
            format!(
                "🦀 Welcome to Rusty - Your Rust Learning Agent!\n\n\
                 I'm here to help you learn Rust programming. I have instant access to:\n\
                 • Rust core concepts (ownership, lifetimes, traits)\n\
                 • Design patterns and idioms\n\
                 • Cargo commands and toolchain usage\n\
                 • Async/concurrency examples\n\n\
                 💬 Ask me anything! For example:\n\
                 • \"What is ownership?\"\n\
                 • \"How do I use cargo test?\"\n\
                 • \"Show me the builder pattern\"\n\n\
                 📋 Type /help to see available commands\n\
                 🔍 Type /search <topic> to search the knowledge base\n\n\
                 📂 Files will be created in: {}",
                workspace
            ),
        );

        Self {
            messages: vec![welcome],
            input: String::new(),
            waiting_for_response: false,
            knowledge_stats: String::new(),
            scroll_to_bottom: false,
            first_render: true,
            pending_file_confirmation: None,
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
            WorkerMessage::FileCreated { path, success, message } => {
                let content = if success {
                    format!("📄 Created file: {}\n{}", path, message)
                } else {
                    format!("❌ Failed to create {}: {}", path, message)
                };
                self.messages.push(Message::new(Role::FileOperation, content));
                self.scroll_to_bottom = true;
            }
            WorkerMessage::FileModified { path, success, message } => {
                let content = if success {
                    format!("✏️  Modified file: {}\n{}", path, message)
                } else {
                    format!("❌ Failed to modify {}: {}", path, message)
                };
                self.messages.push(Message::new(Role::FileOperation, content));
                self.scroll_to_bottom = true;
            }
            WorkerMessage::FileOperationError { path, error } => {
                self.messages.push(Message::new(
                    Role::FileOperation,
                    format!("❌ File operation failed on {}:\n{}", path, error),
                ));
                self.scroll_to_bottom = true;
            }
            WorkerMessage::RequestFileConfirmation(pending) => {
                self.pending_file_confirmation = Some(pending);
                self.scroll_to_bottom = true;
            }
        }
    }

    // Approve file creation
    pub fn approve_file_creation(&mut self) {
        if let Some(pending) = self.pending_file_confirmation.take() {
            self.command_tx.send(UserCommand::ConfirmFileCreation {
                approved: true,
                operations: pending.operations,
            }).ok();
        }
    }

    // Cancel file creation
    pub fn cancel_file_creation(&mut self) {
        if let Some(pending) = self.pending_file_confirmation.take() {
            self.command_tx.send(UserCommand::ConfirmFileCreation {
                approved: false,
                operations: pending.operations,
            }).ok();
            self.messages.push(Message::new(
                Role::System,
                "File creation cancelled.".to_string(),
            ));
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
