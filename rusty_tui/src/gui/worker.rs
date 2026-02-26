use super::messages::{UserCommand, WorkerMessage};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeQuery};
use rust_agent::tools::{KnowledgeFetcher, ToolExecutor, get_tools};
use rust_agent::claude_proxy::ClaudeProxy;

pub fn spawn_worker(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
    launch_dir: PathBuf,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = worker_loop(command_rx, message_tx, db_path, launch_dir).await {
                eprintln!("Worker error: {}", e);
            }
        })
    })
}

async fn worker_loop(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
    launch_dir: PathBuf,
) -> Result<()> {
    // Initialize knowledge database
    let db = KnowledgeDatabase::new(&db_path)?;
    let query = KnowledgeQuery::new(db.clone());
    let knowledge_fetcher = KnowledgeFetcher::new(query);
    let claude = ClaudeProxy::new();

    // Initialize autonomous tool executor
    let tool_executor = ToolExecutor::new(launch_dir.clone());

    // Send initial stats
    let concept_count = db.count_concepts().unwrap_or(0);
    let pattern_count = db.count_patterns().unwrap_or(0);
    let stats = format!("{} concepts, {} patterns loaded", concept_count, pattern_count);
    message_tx.send(WorkerMessage::Stats(stats)).ok();

    // Main loop
    loop {
        match command_rx.recv() {
            Ok(UserCommand::Query(text)) => {
                // Search knowledge database
                let context = match knowledge_fetcher.search(&text) {
                    Ok(knowledge) if knowledge.has_results() => {
                        format!("Knowledge Context:\n{}\n\n", knowledge.formatted)
                    }
                    _ => String::new(),
                };

                // Enhanced system prompt for autonomous agent
                let system_prompt = format!(
                    "You are Rusty, a Rust programming assistant with autonomous capabilities.\n\n\
                    You have access to these tools:\n\
                    - write_file: Create or overwrite files in the project\n\
                    - read_file: Read file contents\n\
                    - run_command: Execute bash commands (cargo, rustc, tests, etc.)\n\
                    - list_files: List directory contents\n\n\
                    When the user asks you to create, modify, or run code:\n\
                    1. Use list_files to understand the project structure\n\
                    2. Use read_file to examine existing code\n\
                    3. Use write_file to create/modify files\n\
                    4. Use run_command to compile and run code\n\n\
                    Always explain what you're doing before using tools.\n\
                    All file paths are relative to the project directory: {}\n\
                    Be proactive and complete tasks fully.",
                    launch_dir.display()
                );

                let prompt = format!("{}User: {}", context, text);

                // Get available tools
                let tools = get_tools();

                // Create progress callback
                let message_tx_clone = message_tx.clone();
                let progress_callback = |msg: String| {
                    message_tx_clone.send(WorkerMessage::ToolProgress(msg)).ok();
                };

                // Query Claude with tools (async)
                match claude.run_with_tools(
                    prompt,
                    Some(system_prompt),
                    tools,
                    &tool_executor,
                    Some(&progress_callback),
                ).await {
                    Ok(response) => {
                        message_tx.send(WorkerMessage::Response(response)).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::Error(format!("API Error: {}", e))).ok();
                    }
                }
            }
            Ok(UserCommand::Command(cmd)) => {
                // Handle slash commands
                let result = execute_command(&cmd, &knowledge_fetcher, &db);
                message_tx.send(WorkerMessage::Response(result)).ok();
            }
            Ok(UserCommand::Quit) => break,
            Err(_) => break,
        }
    }

    Ok(())
}

fn execute_command(cmd: &str, knowledge_fetcher: &KnowledgeFetcher, db: &KnowledgeDatabase) -> String {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let command = parts[0];
    let args = parts.get(1).copied();

    match command {
        "/help" => help_text(),
        "/search" => {
            if let Some(query) = args {
                search_command(knowledge_fetcher, query)
            } else {
                "Usage: /search <query>".to_string()
            }
        }
        "/stats" => stats_command(db),
        "/web" => {
            if args.is_some() {
                "Web search not yet implemented. Coming soon!".to_string()
            } else {
                "Usage: /web <query>".to_string()
            }
        }
        "/clear" => "Chat history cleared.".to_string(),
        "/quit" | "/exit" | "/q" => "Goodbye! 👋".to_string(),
        _ => format!("Unknown command: {}. Type /help for available commands.", command),
    }
}

fn help_text() -> String {
    r#"Available Commands:

/help              - Show this help message
/search <query>    - Search knowledge database
/stats             - Show database statistics
/web <query>       - Force web search (bypasses database)
/clear             - Clear chat history
/quit              - Exit application

Keyboard Shortcuts:
Enter              - Send message
Ctrl+C             - Quit application

Autonomous Capabilities:
I can now autonomously:
✓ Write and read files in your project
✓ Run terminal commands (cargo, rustc, tests, etc.)
✓ Complete full programming tasks independently

Example prompts:
- "Create a hello world program and run it"
- "Write a function to calculate fibonacci numbers"
- "Run cargo build and show me the errors"
- "Create a new Rust project structure"

Tips:
- Just type your question to get help with Rust
- Ask agent to write code: "Write a TCP server"
- Ask about concepts: "What is ownership?"
- Ask about patterns: "Show me the builder pattern"
- If response isn't helpful, say "no" and explain the problem
"#.to_string()
}

fn search_command(knowledge_fetcher: &KnowledgeFetcher, query: &str) -> String {
    match knowledge_fetcher.search(query) {
        Ok(results) => {
            let mut output = format!("Search results for '{}':\n\n", query);

            if results.results.concepts.is_empty()
                && results.results.patterns.is_empty()
                && results.results.commands.is_empty() {
                output.push_str("No results found in knowledge database.\n");
                output.push_str("Try a web search with: /web ");
                output.push_str(query);
                return output;
            }

            if !results.results.concepts.is_empty() {
                output.push_str("📚 Concepts:\n");
                for concept in &results.results.concepts {
                    output.push_str(&format!("  • {}\n", concept.title));
                }
                output.push('\n');
            }

            if !results.results.patterns.is_empty() {
                output.push_str("🔧 Patterns:\n");
                for pattern in &results.results.patterns {
                    output.push_str(&format!("  • {}\n", pattern.name));
                }
                output.push('\n');
            }

            if !results.results.commands.is_empty() {
                output.push_str("⚙️  Commands:\n");
                for cmd in &results.results.commands {
                    output.push_str(&format!("  • {} {}\n", cmd.tool, cmd.command));
                }
            }

            output
        }
        Err(e) => format!("Search error: {}", e),
    }
}

fn stats_command(db: &KnowledgeDatabase) -> String {
    let concept_count = db.count_concepts().unwrap_or(0);
    let pattern_count = db.count_patterns().unwrap_or(0);
    let command_count = db.count_commands().unwrap_or(0);

    format!(
        "Knowledge Database Statistics:\n\n\
         📚 Concepts: {}\n\
         🔧 Patterns: {}\n\
         ⚙️  Commands: {}\n\n\
         Database location: ~/.agent/data/knowledge.db\n\
         Query performance: <50ms average",
        concept_count, pattern_count, command_count
    )
}
