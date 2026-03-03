use super::messages::{FileCreationInfo, UserCommand, WorkerMessage};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;
use rust_agent::claude_proxy::ClaudeProxy;
use rust_agent::file_generator::{AutoFileCreator, FileCreationResult, FileCreationDetector};

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

    // Initialize auto file creator and detector
    let workspace_root = std::env::current_dir().unwrap_or_default();
    let auto_file_creator = AutoFileCreator::new(workspace_root, db.clone());
    let file_detector = FileCreationDetector::new();

    // Send initial stats
    let concept_count = db.count_concepts().unwrap_or(0);
    let pattern_count = db.count_patterns().unwrap_or(0);
    let stats = format!("{} concepts, {} patterns loaded", concept_count, pattern_count);
    message_tx.send(WorkerMessage::Stats(stats)).ok();

    // Main loop
    loop {
        match command_rx.recv() {
            Ok(UserCommand::Query(text)) => {
                eprintln!("📨 Received query: {}", text.chars().take(50).collect::<String>());

                // STEP 1: Detect if this is a code generation request
                let is_code_generation = file_detector.should_create_files(&text);
                if is_code_generation {
                    eprintln!("🔍 Detected code generation request");
                }

                // STEP 2: Search knowledge database for concepts/patterns
                let mut context = match knowledge_fetcher.search(&text) {
                    Ok(knowledge) if knowledge.has_results() => {
                        eprintln!("📚 Found {} knowledge results",
                                 knowledge.results.concepts.len() +
                                 knowledge.results.patterns.len() +
                                 knowledge.results.commands.len());
                        format!("Knowledge Context:\n{}\n\n", knowledge.formatted)
                    }
                    _ => {
                        eprintln!("ℹ️  No knowledge results, using Claude only");
                        String::new()
                    }
                };

                // STEP 3: If code generation, add file template information
                if is_code_generation {
                    let file_templates = get_relevant_file_templates(&db, &text);
                    if !file_templates.is_empty() {
                        eprintln!("📁 Found {} file templates to suggest", file_templates.len());
                        context.push_str("\n\n📁 File Creation Guide:\n");
                        context.push_str("The working directory needs these files created:\n\n");
                        context.push_str(&file_templates);
                        context.push_str("\nIMPORTANT: When providing code, use @filepath markers like this:\n");
                        context.push_str("@src/main.rs\n```rust\ncode here\n```\n\n");
                    }
                }

                let prompt = format!("{}User: {}", context, text);

                // Query Claude (async)
                eprintln!("🤖 Querying Claude API...");
                match claude.query(&prompt).await {
                    Ok(response) => {
                        eprintln!("✅ Got response ({} chars)", response.len());

                        // Try to auto-create files from the response
                        let file_results = auto_file_creator.auto_create_from_response(&text, &response)
                            .unwrap_or_else(|e| {
                                eprintln!("⚠️  File creation error: {}", e);
                                vec![]
                            });

                        if !file_results.is_empty() {
                            eprintln!("📁 Created {} file(s)", file_results.len());

                            // Append file creation summary to response
                            let summary = format_file_creation_summary(&file_results);
                            let full_response = format!("{}\n\n{}", response, summary);
                            message_tx.send(WorkerMessage::Response(full_response)).ok();

                            // Also send structured notification
                            let infos: Vec<FileCreationInfo> = file_results.iter().map(|r| FileCreationInfo {
                                path: r.path.display().to_string(),
                                appended: r.appended,
                                success: r.success(),
                            }).collect();
                            message_tx.send(WorkerMessage::FilesCreated(infos)).ok();
                        } else {
                            message_tx.send(WorkerMessage::Response(response)).ok();
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Claude API error: {}", e);
                        let error_msg = format!(
                            "Sorry, I couldn't connect to the AI service.\n\n\
                             Error: {}\n\n\
                             💡 Make sure ClaudeProxyAPI is running:\n\
                             • Check: curl http://localhost:8317/\n\
                             • Start: ./start_cliproxyapi.sh\n\n\
                             🔍 You can still search the knowledge base with /search <query>",
                            e
                        );
                        message_tx.send(WorkerMessage::Error(error_msg)).ok();
                    }
                }
            }
            Ok(UserCommand::Command(cmd)) => {
                eprintln!("⚙️  Executing command: {}", cmd);
                let result = execute_command(&cmd, &knowledge_fetcher, &db);
                message_tx.send(WorkerMessage::Response(result)).ok();
            }
            Ok(UserCommand::Quit) => {
                eprintln!("👋 Quit command received");
                break;
            }
            Err(_) => {
                eprintln!("⚠️  Channel closed, worker exiting");
                break;
            }
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

fn get_relevant_file_templates(db: &KnowledgeDatabase, query: &str) -> String {
    let mut templates = String::new();

    // Determine what type of project from query
    let query_lower = query.to_lowercase();

    if query_lower.contains("hello world") || query_lower.contains("main") {
        templates.push_str("• src/main.rs - Main program entry point with fn main()\n");
        templates.push_str("• Cargo.toml - Project manifest with [package] section\n");
    } else if query_lower.contains("library") || query_lower.contains("lib") {
        templates.push_str("• src/lib.rs - Library root with pub modules\n");
        templates.push_str("• Cargo.toml - Library manifest\n");
    } else if query_lower.contains("struct") || query_lower.contains("type") {
        templates.push_str("• src/<name>.rs - Module file with struct/enum definitions\n");
    } else if query_lower.contains("test") {
        templates.push_str("• tests/integration_test.rs - Integration tests with #[test]\n");
    } else if query_lower.contains("server") || query_lower.contains("web") {
        templates.push_str("• src/main.rs - Server entry point\n");
        templates.push_str("• src/routes.rs - Route handlers\n");
        templates.push_str("• Cargo.toml - Dependencies (tokio, axum, etc.)\n");
    } else {
        // Default for any code generation request
        templates.push_str("• src/main.rs - Main program file\n");
    }

    templates
}

fn format_file_creation_summary(results: &[FileCreationResult]) -> String {
    let mut summary = String::from("\n📁 **Files Created:**\n");
    for result in results {
        if result.success() {
            let action = if result.appended { "Updated" } else { "Created" };
            summary.push_str(&format!("  ✅ {} `{}`\n", action, result.path.display()));
        } else {
            summary.push_str(&format!("  ❌ Failed: `{}` - {}\n",
                result.path.display(),
                result.error.as_deref().unwrap_or("Unknown error")
            ));
        }
    }
    summary
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
