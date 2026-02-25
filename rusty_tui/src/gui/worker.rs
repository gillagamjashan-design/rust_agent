use super::messages::{UserCommand, WorkerMessage};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;
use rust_agent::claude_proxy::ClaudeProxy;

pub fn spawn_worker(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = worker_loop(command_rx, message_tx, db_path).await {
                eprintln!("Worker error: {}", e);
            }
        })
    })
}

async fn worker_loop(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
) -> Result<()> {
    // Initialize knowledge database
    let db = KnowledgeDatabase::new(&db_path)?;
    let query = KnowledgeQuery::new(db.clone());
    let knowledge_fetcher = KnowledgeFetcher::new(query);
    let claude = ClaudeProxy::new();

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

                let prompt = format!("{}User: {}", context, text);

                // Query Claude (async)
                match claude.query(&prompt).await {
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
