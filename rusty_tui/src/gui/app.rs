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
    pub created_files: Vec<String>,

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

        // Create welcome message
        let welcome = Message::new(
            Role::System,
            "🦀 Welcome to Rusty - Your Rust Learning Agent!\n\n\
             I'm here to help you learn Rust programming. I have instant access to:\n\
             • Rust core concepts (ownership, lifetimes, traits)\n\
             • Design patterns and idioms\n\
             • Cargo commands and toolchain usage\n\
             • File templates for automatic code generation\n\
             • Async/concurrency examples\n\n\
             💬 Ask me questions or request code generation:\n\
             • \"What is ownership?\" - I'll search the knowledge base\n\
             • \"Create a hello world program\" - I'll detect this is code generation,\n\
               search for which files to create (src/main.rs, Cargo.toml), and\n\
               automatically create them in your working directory\n\
             • \"Build a web server\" - I'll generate code AND create the files\n\
             • \"Show me the builder pattern\" - I'll explain with examples\n\n\
             📋 Type /help to see available commands\n\
             🔍 Type /search <topic> to search the knowledge base\n\
             📁 Generated files appear in your current directory".to_string(),
        );

        Self {
            messages: vec![welcome],
            input: String::new(),
            waiting_for_response: false,
            knowledge_stats: String::new(),
            scroll_to_bottom: false,
            first_render: true,
            created_files: Vec::new(),
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
            // Worker will:
            // 1. Detect if this is a code generation request
            // 2. Search knowledge database for relevant concepts/patterns
            // 3. If code generation: also search for which files to create
            // 4. Send all context to Claude API
            // 5. Auto-create files from Claude's response
            self.waiting_for_response = true;
            self.command_tx.send(UserCommand::Query(input)).ok();
        }
    }

    /// Handle messages from worker thread
    /// Worker processes queries by:
    /// 1. Detecting if query is a code generation request (create/make/generate keywords)
    /// 2. Searching knowledge DB for concepts AND file templates
    /// 3. Sending full context to Claude (knowledge + file creation guide)
    /// 4. Auto-creating files from Claude's response using database templates
    pub fn handle_worker_message(&mut self, msg: WorkerMessage) {
        match msg {
            WorkerMessage::Response(text) => {
                // Contains Claude's answer + file creation summary if files were made
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
            WorkerMessage::FilesCreated(files) => {
                // Track files created this session
                for file in files {
                    if file.success {
                        self.created_files.push(file.path.clone());
                    }
                }
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
