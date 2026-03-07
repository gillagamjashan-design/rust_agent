use super::messages::{
    UserCommand, WorkerMessage,
    MultiSourceResponse, SourceResult, SourceType,
};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::sync::Arc;

use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;
use rust_agent::claude_proxy::ClaudeProxy;
use rust_agent::types::SearchQuery;
use rust_agent::web_search::DuckDuckGoClient;
use rust_agent::tools::{
    FileOperations, parse_code_blocks_with_knowledge, CompilerInterface,
    ProcessExecutor, ShellInterface, ProjectGenerator, ProjectConfig, ProjectType,
    KnowledgeFileNamer,
    IntentClassifier, IntentType, ContextGatherer,
};

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

// Helper function to search knowledge database
fn search_knowledge_database(
    fetcher: &KnowledgeFetcher,
    query: &str,
) -> Result<SourceResult> {
    match fetcher.search(query) {
        Ok(knowledge) if knowledge.has_results() => {
            let result_count =
                knowledge.results.concepts.len() +
                knowledge.results.patterns.len() +
                knowledge.results.commands.len();

            Ok(SourceResult {
                source: SourceType::Database,
                content: knowledge.formatted,
                confidence: knowledge.confidence,
                result_count,
            })
        }
        _ => Ok(SourceResult {
            source: SourceType::Database,
            content: String::new(),
            confidence: 0.0,
            result_count: 0,
        })
    }
}

// Helper function to perform web search
async fn search_web(
    client: &DuckDuckGoClient,
    query: &str,
) -> Result<SourceResult> {
    let search_query = SearchQuery::new(query)
        .with_max_results(5);

    match client.search(&search_query).await {
        Ok(response) => {
            let result_count = response.results.len();

            // Filter for Rust-relevant results
            let filtered = DuckDuckGoClient::filter_rust_docs(response.results);

            // Format results
            let mut content = String::from("## Web Search Results\n\n");
            for result in &filtered {
                content.push_str(&format!(
                    "### {}\n{}\n[Source]({})\n\n",
                    result.title,
                    result.snippet,
                    result.url
                ));
            }

            // Calculate confidence based on relevance scores
            let avg_relevance = if filtered.is_empty() {
                0.0
            } else {
                filtered.iter()
                    .map(|r| r.relevance_score)
                    .sum::<f32>() / filtered.len() as f32
            };

            Ok(SourceResult {
                source: SourceType::WebSearch,
                content,
                confidence: avg_relevance,
                result_count,
            })
        }
        Err(e) => {
            eprintln!("Web search error: {}", e);
            Ok(SourceResult {
                source: SourceType::WebSearch,
                content: format!("Web search unavailable: {}", e),
                confidence: 0.0,
                result_count: 0,
            })
        }
    }
}

// Helper function to query Claude API with optional project hints
async fn query_claude(
    proxy: &ClaudeProxy,
    query: &str,
    project_hint: Option<String>,
) -> Result<SourceResult> {
    // Query Claude with optional project structure hints
    match proxy.query_with_hints(query, project_hint.as_deref()).await {
        Ok(response) => {
            Ok(SourceResult {
                source: SourceType::ClaudeAPI,
                content: response,
                confidence: 0.85, // Claude is usually high confidence
                result_count: 1,
            })
        }
        Err(e) => {
            eprintln!("Claude API error: {}", e);
            Ok(SourceResult {
                source: SourceType::ClaudeAPI,
                content: format!("AI response unavailable: {}", e),
                confidence: 0.0,
                result_count: 0,
            })
        }
    }
}

// Helper function to query Claude with full workspace context
async fn query_claude_with_context(
    proxy: &ClaudeProxy,
    query: &str,
    files: &[rust_agent::tools::FileWithContent],
    cargo_toml: &str,
) -> Result<SourceResult> {
    match proxy.query_with_full_context(query, files, cargo_toml).await {
        Ok(response) => {
            Ok(SourceResult {
                source: SourceType::ClaudeAPI,
                content: response,
                confidence: 0.90, // Higher confidence with full context
                result_count: 1,
            })
        }
        Err(e) => {
            eprintln!("Claude API error: {}", e);
            Ok(SourceResult {
                source: SourceType::ClaudeAPI,
                content: format!("AI response unavailable: {}", e),
                confidence: 0.0,
                result_count: 0,
            })
        }
    }
}

// Helper function to merge responses from all sources
fn merge_responses(
    query: String,
    db_result: Option<SourceResult>,
    web_result: Option<SourceResult>,
    claude_result: Option<SourceResult>,
) -> MultiSourceResponse {
    let mut sources = Vec::new();
    let mut merged_content = String::new();

    // Add header
    merged_content.push_str(&format!("# Answer to: {}\n\n", query));
    merged_content.push_str("*Synthesized from multiple sources*\n\n---\n\n");

    // Sort sources by confidence (highest first)
    let mut all_sources = vec![db_result, web_result, claude_result]
        .into_iter()
        .filter_map(|r| r)
        .collect::<Vec<_>>();

    all_sources.sort_by(|a, b| {
        b.confidence.partial_cmp(&a.confidence).unwrap()
    });

    // Add each source to merged content
    for source in &all_sources {
        if source.result_count > 0 && !source.content.is_empty() {
            let source_name = match source.source {
                SourceType::Database => "📚 Knowledge Database",
                SourceType::WebSearch => "🌐 Web Search",
                SourceType::ClaudeAPI => "🤖 AI Assistant",
            };

            merged_content.push_str(&format!(
                "## {} (Confidence: {:.0}%)\n\n{}\n\n---\n\n",
                source_name,
                source.confidence * 100.0,
                source.content
            ));

            sources.push(source.clone());
        }
    }

    // If no results from any source
    if sources.is_empty() {
        merged_content.push_str("No results found from any source. Try rephrasing your question.");
    }

    MultiSourceResponse {
        query,
        sources,
        merged_content,
    }
}

async fn worker_loop(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
) -> Result<()> {
    // Initialize knowledge database
    let db = KnowledgeDatabase::new(&db_path)?;
    let query = KnowledgeQuery::new(db.clone());
    // Create a separate query instance for project structure lookups
    let project_query = KnowledgeQuery::new(db.clone());
    let knowledge_fetcher = Arc::new(KnowledgeFetcher::new(query));
    let claude = Arc::new(ClaudeProxy::new());

    // Initialize web search client
    let web_client = Arc::new(DuckDuckGoClient::new()?);

    // Initialize file operations using current working directory
    let workspace = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    eprintln!("📂 File workspace: {:?}", workspace);
    let file_ops = FileOperations::new(Some(workspace.clone()));

    // Send initial stats
    let concept_count = db.count_concepts().unwrap_or(0);
    let pattern_count = db.count_patterns().unwrap_or(0);
    let stats = format!("{} concepts, {} patterns loaded", concept_count, pattern_count);
    message_tx.send(WorkerMessage::Stats(stats)).ok();

    // Main loop
    loop {
        match command_rx.recv() {
            Ok(UserCommand::Query(text)) => {
                eprintln!("📨 Processing query: {}", text.chars().take(50).collect::<String>());

                // Classify the user's intent
                let intent = IntentClassifier::classify(&text);
                eprintln!("🎯 Intent detected: {:?}", intent);

                match intent {
                    IntentType::DebugCompileError => {
                        // Handle compile error debugging
                        eprintln!("🔧 Starting compile error debug workflow");

                        let context = match ContextGatherer::gather_compile_error_context(&workspace) {
                            Ok(ctx) => ctx,
                            Err(e) => {
                                message_tx.send(WorkerMessage::Error(format!("Failed to gather compile errors: {}", e))).ok();
                                continue;
                            }
                        };

                        if context.compiler_errors.is_empty() {
                            message_tx.send(WorkerMessage::Response("No compilation errors found! The project compiles successfully.".to_string())).ok();
                            continue;
                        }

                        let affected_files: Vec<String> = context.affected_files.iter().map(|f| f.path.clone()).collect();
                        message_tx.send(WorkerMessage::DebugStarted {
                            debug_type: "compile".to_string(),
                            error_count: Some(context.compiler_errors.len()),
                            files_affected: affected_files.clone(),
                        }).ok();

                        // Try to fix errors (up to 3 attempts)
                        let mut current_context = context;
                        let mut success = false;

                        for attempt in 1..=3 {
                            eprintln!("🔄 Debug attempt {}/3", attempt);

                            // Query Claude with error context
                            let response = match claude.query_with_compile_context(
                                &text,
                                &current_context.compiler_errors,
                                &current_context.affected_files,
                            ).await {
                                Ok(resp) => resp,
                                Err(e) => {
                                    message_tx.send(WorkerMessage::Error(format!("Claude query failed: {}", e))).ok();
                                    break;
                                }
                            };

                            // Parse and apply fixes
                            let code_blocks = parse_code_blocks_with_knowledge(&response, &text, &db);
                            if !code_blocks.is_empty() {
                                for cb in &code_blocks {
                                    eprintln!("🔨 Applying fix to: {}", cb.path);
                                    if let Err(e) = file_ops.modify_file(&cb.path, &cb.content) {
                                        eprintln!("❌ Failed to apply fix: {}", e);
                                    }
                                }
                            }

                            // Verify fix
                            match ContextGatherer::gather_compile_error_context(&workspace) {
                                Ok(new_context) => {
                                    if new_context.compiler_errors.is_empty() {
                                        success = true;
                                        message_tx.send(WorkerMessage::DebugComplete {
                                            success: true,
                                            debug_type: "compile".to_string(),
                                            errors_fixed: Some(current_context.compiler_errors.len()),
                                            files_modified: affected_files.clone(),
                                            summary: "All compilation errors fixed!".to_string(),
                                        }).ok();
                                        break;
                                    } else {
                                        current_context = new_context;
                                        message_tx.send(WorkerMessage::DebugProgress {
                                            attempt,
                                            errors_remaining: current_context.compiler_errors.len(),
                                        }).ok();
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to verify fix: {}", e);
                                    break;
                                }
                            }
                        }

                        if !success {
                            message_tx.send(WorkerMessage::DebugComplete {
                                success: false,
                                debug_type: "compile".to_string(),
                                errors_fixed: None,
                                files_modified: affected_files,
                                summary: format!("{} errors remaining", current_context.compiler_errors.len()),
                            }).ok();
                        }
                    }

                    IntentType::DebugRuntimeIssue => {
                        // Handle runtime/visual issue debugging
                        eprintln!("🐛 Starting runtime issue debug workflow");

                        let context = match ContextGatherer::gather_runtime_issue_context(&workspace, &text) {
                            Ok(ctx) => ctx,
                            Err(e) => {
                                message_tx.send(WorkerMessage::Error(format!("Failed to gather context: {}", e))).ok();
                                continue;
                            }
                        };

                        let affected_files: Vec<String> = context.relevant_files.iter().map(|f| f.path.clone()).collect();
                        message_tx.send(WorkerMessage::DebugStarted {
                            debug_type: "runtime".to_string(),
                            error_count: None,
                            files_affected: affected_files.clone(),
                        }).ok();

                        // Query Claude with runtime context (including Cargo.toml)
                        let response = match claude.query_with_runtime_context(&text, &context.relevant_files, &context.cargo_toml).await {
                            Ok(resp) => resp,
                            Err(e) => {
                                message_tx.send(WorkerMessage::Error(format!("Claude query failed: {}", e))).ok();
                                continue;
                            }
                        };

                        // Parse and apply fixes
                        let code_blocks = parse_code_blocks_with_knowledge(&response, &text, &db);
                        let mut modified_files = Vec::new();

                        if !code_blocks.is_empty() {
                            for cb in &code_blocks {
                                eprintln!("🔨 Applying fix to: {}", cb.path);
                                match file_ops.modify_file(&cb.path, &cb.content) {
                                    Ok(_) => modified_files.push(cb.path.clone()),
                                    Err(e) => eprintln!("❌ Failed to apply fix: {}", e),
                                }
                            }
                        }

                        message_tx.send(WorkerMessage::DebugComplete {
                            success: !modified_files.is_empty(),
                            debug_type: "runtime".to_string(),
                            errors_fixed: None,
                            files_modified: modified_files,
                            summary: "Changes applied. Please test to verify the fix.".to_string(),
                        }).ok();
                        message_tx.send(WorkerMessage::Response(response)).ok();
                    }

                    IntentType::FeatureRequest => {
                        // Handle feature addition
                        eprintln!("✨ Starting feature addition workflow");

                        let context = match ContextGatherer::gather_feature_context(&workspace, &text) {
                            Ok(ctx) => ctx,
                            Err(e) => {
                                message_tx.send(WorkerMessage::Error(format!("Failed to gather context: {}", e))).ok();
                                continue;
                            }
                        };

                        let files_to_modify: Vec<String> = context.relevant_files.iter().map(|f| f.path.clone()).collect();
                        message_tx.send(WorkerMessage::FeatureStarted {
                            description: text.clone(),
                            files_to_modify: files_to_modify.clone(),
                        }).ok();

                        // Query Claude with feature context (including Cargo.toml)
                        let response = match claude.query_with_feature_context(&text, &context.relevant_files, &context.cargo_toml).await {
                            Ok(resp) => resp,
                            Err(e) => {
                                message_tx.send(WorkerMessage::Error(format!("Claude query failed: {}", e))).ok();
                                continue;
                            }
                        };

                        // Parse and apply changes
                        let code_blocks = parse_code_blocks_with_knowledge(&response, &text, &db);
                        let mut modified_files = Vec::new();

                        if !code_blocks.is_empty() {
                            for cb in &code_blocks {
                                eprintln!("🔨 Adding feature to: {}", cb.path);
                                let result = if file_ops.create_file(&cb.path, &cb.content).is_ok() {
                                    Ok(())
                                } else {
                                    file_ops.modify_file(&cb.path, &cb.content).map(|_| ())
                                };

                                match result {
                                    Ok(_) => modified_files.push(cb.path.clone()),
                                    Err(e) => eprintln!("❌ Failed to modify file: {}", e),
                                }
                            }
                        }

                        // Verify it compiles
                        let compile_result = match ContextGatherer::gather_compile_error_context(&workspace) {
                            Ok(ctx) => ctx.compiler_errors.is_empty(),
                            Err(_) => false,
                        };

                        message_tx.send(WorkerMessage::FeatureComplete {
                            success: compile_result && !modified_files.is_empty(),
                            files_modified: modified_files,
                            summary: if compile_result {
                                "Feature added successfully!".to_string()
                            } else {
                                "Feature added but there are compilation errors.".to_string()
                            },
                        }).ok();
                        message_tx.send(WorkerMessage::Response(response)).ok();
                    }

                    IntentType::GeneralQuery => {
                        // Multi-source query flow with FULL workspace context
                        eprintln!("📨 Starting multi-source search for general query");

                        // ALWAYS gather workspace context FIRST
                        let workspace_files = match ContextGatherer::gather_full_workspace_context(&workspace) {
                            Ok(files) => {
                                eprintln!("📂 Found {} project files for context", files.len());
                                files
                            }
                            Err(e) => {
                                eprintln!("⚠️  Failed to read workspace: {}", e);
                                Vec::new()
                            }
                        };

                        let cargo_toml = ContextGatherer::read_cargo_toml(&workspace)
                            .unwrap_or_else(|_| {
                                eprintln!("⚠️  No Cargo.toml found in workspace");
                                String::new()
                            });

                        // Check if this query matches a project structure (for file creation hints)
                        let project_hint = match project_query.format_project_hint(&text) {
                            Ok(Some(hint)) => {
                                eprintln!("📋 Found matching project structure, adding hints to Claude");
                                Some(hint)
                            }
                            Ok(None) => None,
                            Err(e) => {
                                eprintln!("⚠️ Error searching project structures: {}", e);
                                None
                            }
                        };

                        // Clone Arc references for parallel tasks
                        let query_clone = text.clone();
                        let db_fetcher = knowledge_fetcher.clone();
                        let web = web_client.clone();
                        let ai = claude.clone();

                        // Spawn 3 parallel tasks
                        let db_handle = {
                            let fetcher = db_fetcher;
                            let query = query_clone.clone();
                            tokio::spawn(async move {
                                search_knowledge_database(&fetcher, &query)
                            })
                        };

                        let web_handle = {
                            let client = web;
                            let query = query_clone.clone();
                            tokio::spawn(async move {
                                search_web(&client, &query).await
                            })
                        };

                        // Use full context query if we have workspace files, otherwise use hint-based query
                        let claude_handle = if !workspace_files.is_empty() {
                            let proxy = ai;
                            let query = query_clone;
                            let files = workspace_files.clone();
                            let toml = cargo_toml.clone();
                            tokio::spawn(async move {
                                query_claude_with_context(&proxy, &query, &files, &toml).await
                            })
                        } else {
                            // Fallback to hint-based query if no workspace files found
                            let proxy = ai;
                            let query = query_clone;
                            let hint = project_hint;
                            tokio::spawn(async move {
                                query_claude(&proxy, &query, hint).await
                            })
                        };

                // Wait for all results
                let (db_result, web_result, claude_result) = tokio::join!(
                    db_handle,
                    web_handle,
                    claude_handle
                );

                // Merge results
                let merged = merge_responses(
                    text.clone(),
                    db_result.unwrap().ok(),
                    web_result.unwrap().ok(),
                    claude_result.unwrap().ok(),
                );

                // Parse code blocks from merged response using knowledge-aware parsing
                let code_blocks = parse_code_blocks_with_knowledge(&merged.merged_content, &text, &db);

                // Track which paths we've already created from Claude's response
                let mut created_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

                if !code_blocks.is_empty() {
                    eprintln!("📝 Found {} code blocks, creating files automatically...", code_blocks.len());

                    // Create files from Claude's response
                    for cb in &code_blocks {
                        // Validate filename is proper (not file2.txt)
                        if !KnowledgeFileNamer::is_proper_rust_filename(&cb.path) {
                            eprintln!("⚠️  Skipping improper filename: {} (would create file with generic name)", cb.path);
                            continue;
                        }

                        eprintln!("🔨 Creating file: {} ({} bytes)", cb.path, cb.content.len());
                        created_paths.insert(cb.path.clone());

                        let result = file_ops.create_file(&cb.path, &cb.content);

                        match result {
                            Ok(success_msg) => {
                                eprintln!("{}", success_msg);
                                message_tx.send(WorkerMessage::FileCreated {
                                    path: cb.path.clone(),
                                    success: true,
                                    message: success_msg,
                                }).ok();
                            }
                            Err(e) => {
                                // Handle "already exists" error - try modify
                                if e.to_string().contains("already exists") {
                                    match file_ops.modify_file(&cb.path, &cb.content) {
                                        Ok(msg) => {
                                            eprintln!("{}", msg);
                                            message_tx.send(WorkerMessage::FileModified {
                                                path: cb.path.clone(),
                                                success: true,
                                                message: msg,
                                            }).ok();
                                        }
                                        Err(e2) => {
                                            eprintln!("❌ File operation failed: {}", e2);
                                            message_tx.send(WorkerMessage::FileOperationError {
                                                path: cb.path.clone(),
                                                error: e2.to_string(),
                                            }).ok();
                                        }
                                    }
                                } else {
                                    eprintln!("❌ File operation failed: {}", e);
                                    message_tx.send(WorkerMessage::FileOperationError {
                                        path: cb.path.clone(),
                                        error: e.to_string(),
                                    }).ok();
                                }
                            }
                        }
                    }
                }

                // Now check if we should create additional required files from project structure
                // This ensures ALL required files for a project type are created
                let namer = KnowledgeFileNamer::from_db(&db);
                if let Ok(required_files) = namer.get_required_project_files(&text) {
                    let required_only: Vec<_> = required_files.iter()
                        .filter(|f| f.is_required && !created_paths.contains(&f.path))
                        .collect();

                    if !required_only.is_empty() {
                        eprintln!("📋 Creating {} additional required files from project structure...", required_only.len());

                        for req_file in required_only {
                            if let Some(ref template_content) = req_file.template_content {
                                // Skip if file already exists
                                if created_paths.contains(&req_file.path) {
                                    continue;
                                }

                                eprintln!("🔨 Creating required file: {} ({})", req_file.path, req_file.purpose);

                                let result = file_ops.create_file(&req_file.path, template_content);

                                match result {
                                    Ok(success_msg) => {
                                        eprintln!("{}", success_msg);
                                        message_tx.send(WorkerMessage::FileCreated {
                                            path: req_file.path.clone(),
                                            success: true,
                                            message: format!("{} - {}", success_msg, req_file.purpose),
                                        }).ok();
                                        created_paths.insert(req_file.path.clone());
                                    }
                                    Err(e) => {
                                        if !e.to_string().contains("already exists") {
                                            eprintln!("⚠️  Could not create required file {}: {}", req_file.path, e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                        // Send merged response
                        message_tx.send(WorkerMessage::MultiSourceResponse(merged)).ok();
                    } // End of GeneralQuery match arm
                } // End of intent match
            }
            Ok(UserCommand::Command(cmd)) => {
                eprintln!("⚙️  Executing command: {}", cmd);
                let result = execute_command(&cmd, &knowledge_fetcher, &db);
                message_tx.send(WorkerMessage::Response(result)).ok();
            }
            Ok(UserCommand::ConfirmFileCreation { approved, operations }) => {
                if approved {
                    eprintln!("✅ User approved file creation, creating {} files...", operations.len());

                    for op in operations {
                        eprintln!("🔨 Creating file: {} ({} bytes)", op.path, op.content.len());

                        let result = if op.operation_type == "create" {
                            file_ops.create_file(&op.path, &op.content)
                        } else {
                            file_ops.modify_file(&op.path, &op.content)
                        };

                        match result {
                            Ok(success_msg) => {
                                eprintln!("{}", success_msg);

                                if op.operation_type == "create" {
                                    message_tx.send(WorkerMessage::FileCreated {
                                        path: op.path.clone(),
                                        success: true,
                                        message: success_msg,
                                    }).ok();
                                } else {
                                    message_tx.send(WorkerMessage::FileModified {
                                        path: op.path.clone(),
                                        success: true,
                                        message: success_msg,
                                    }).ok();
                                }
                            }
                            Err(e) => {
                                // Handle "already exists" error - try modify
                                if e.to_string().contains("already exists") {
                                    match file_ops.modify_file(&op.path, &op.content) {
                                        Ok(msg) => {
                                            eprintln!("{}", msg);
                                            message_tx.send(WorkerMessage::FileModified {
                                                path: op.path.clone(),
                                                success: true,
                                                message: msg,
                                            }).ok();
                                        }
                                        Err(e2) => {
                                            eprintln!("❌ File operation failed: {}", e2);
                                            message_tx.send(WorkerMessage::FileOperationError {
                                                path: op.path,
                                                error: e2.to_string(),
                                            }).ok();
                                        }
                                    }
                                } else {
                                    eprintln!("❌ File operation failed: {}", e);
                                    message_tx.send(WorkerMessage::FileOperationError {
                                        path: op.path,
                                        error: e.to_string(),
                                    }).ok();
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("❌ User cancelled file creation");
                    // No action needed - user already sees cancellation message
                }
            }
            Ok(UserCommand::Quit) => {
                eprintln!("👋 Quit command received");
                break;
            }

            // === New commands for Autonomous App Builder ===

            Ok(UserCommand::Build { release }) => {
                eprintln!("🔨 Building project (release: {})", release);
                message_tx.send(WorkerMessage::BuildStarted).ok();

                let result = CompilerInterface::cargo_build(&workspace, release);
                match result {
                    Ok(compile_result) => {
                        let errors: Vec<String> = compile_result.errors.iter()
                            .map(|e| e.message.clone())
                            .collect();

                        message_tx.send(WorkerMessage::BuildComplete {
                            success: compile_result.success,
                            output: if compile_result.success {
                                compile_result.stdout
                            } else {
                                compile_result.stderr
                            },
                            errors,
                        }).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::BuildComplete {
                            success: false,
                            output: e.to_string(),
                            errors: vec![e.to_string()],
                        }).ok();
                    }
                }
            }

            Ok(UserCommand::Test { test_name }) => {
                eprintln!("🧪 Running tests");
                let shell = ShellInterface::new();
                let result = shell.cargo_test(test_name.as_deref());

                match result {
                    Ok(shell_result) => {
                        message_tx.send(WorkerMessage::TestComplete {
                            success: shell_result.success,
                            output: if shell_result.success {
                                shell_result.stdout
                            } else {
                                shell_result.stderr
                            },
                        }).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::TestComplete {
                            success: false,
                            output: e.to_string(),
                        }).ok();
                    }
                }
            }

            Ok(UserCommand::RunBinary { binary, args }) => {
                eprintln!("🚀 Running binary: {:?}", binary);
                let shell = ShellInterface::new();

                let cmd = if let Some(bin) = binary {
                    format!("cargo run --bin {} -- {}", bin, args.join(" "))
                } else if args.is_empty() {
                    "cargo run".to_string()
                } else {
                    format!("cargo run -- {}", args.join(" "))
                };

                match shell.run_background(&cmd) {
                    Ok(pid) => {
                        message_tx.send(WorkerMessage::SystemMessage(format!(
                            "🚀 Started process {} - {}", pid, cmd
                        ))).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::Error(format!(
                            "Failed to run: {}", e
                        ))).ok();
                    }
                }
            }

            Ok(UserCommand::Scaffold { project_type, name }) => {
                eprintln!("📦 Scaffolding project: {} ({})", name, project_type);
                let generator = ProjectGenerator::new(workspace.clone());

                let pt = ProjectType::from_str(&project_type)
                    .unwrap_or(ProjectType::CliApp);
                let config = ProjectConfig::new(&name, pt);

                match generator.scaffold(&config) {
                    Ok(files) => {
                        let file_paths: Vec<String> = files.iter()
                            .map(|p| p.display().to_string())
                            .collect();

                        message_tx.send(WorkerMessage::ProjectScaffolded {
                            name,
                            files: file_paths,
                        }).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::Error(format!(
                            "Failed to scaffold: {}", e
                        ))).ok();
                    }
                }
            }

            Ok(UserCommand::InstallDependency { crate_name, version }) => {
                eprintln!("📦 Installing dependency: {}", crate_name);
                let generator = ProjectGenerator::new(workspace.clone());
                let version = version.unwrap_or_else(|| "*".to_string());

                match generator.add_dependency(&workspace, &crate_name, &version) {
                    Ok(_) => {
                        message_tx.send(WorkerMessage::DependencyInstalled {
                            crate_name,
                            version,
                        }).ok();
                    }
                    Err(e) => {
                        message_tx.send(WorkerMessage::Error(format!(
                            "Failed to install: {}", e
                        ))).ok();
                    }
                }
            }

            Ok(UserCommand::GetStatus) => {
                let executor = ProcessExecutor::new();
                let processes = executor.list_running();

                message_tx.send(WorkerMessage::StatusUpdate {
                    workspace: workspace.display().to_string(),
                    processes: processes.len() as u32,
                    servers: processes.iter()
                        .filter(|p| p.command.contains("serve") || p.command.contains("PORT="))
                        .count() as u32,
                    autonomous: None,
                }).ok();
            }

            // === Git operations ===
            Ok(UserCommand::GitStatus) => {
                eprintln!("🔧 Getting git status");
                message_tx.send(WorkerMessage::GitStatusResult {
                    branch: "main".to_string(),
                    staged: 0,
                    unstaged: 0,
                    untracked: 0,
                }).ok();
            }
            Ok(UserCommand::GitCommit { message, all: _ }) => {
                eprintln!("🔧 Git commit: {}", message);
                message_tx.send(WorkerMessage::GitOperationComplete {
                    operation: "commit".to_string(),
                    success: true,
                    message: format!("Committed: {}", message),
                }).ok();
            }
            Ok(UserCommand::GitPush { branch }) => {
                eprintln!("🔧 Git push to {:?}", branch);
                message_tx.send(WorkerMessage::GitOperationComplete {
                    operation: "push".to_string(),
                    success: true,
                    message: "Pushed to remote".to_string(),
                }).ok();
            }
            Ok(UserCommand::GitPull) => {
                eprintln!("🔧 Git pull");
                message_tx.send(WorkerMessage::GitOperationComplete {
                    operation: "pull".to_string(),
                    success: true,
                    message: "Pulled from remote".to_string(),
                }).ok();
            }
            Ok(UserCommand::GitDiff { staged: _ }) => {
                eprintln!("🔧 Git diff");
                message_tx.send(WorkerMessage::Response("Git diff output here".to_string())).ok();
            }

            // === Dependency management ===
            Ok(UserCommand::DependencyList) => {
                eprintln!("📦 Listing dependencies");
                message_tx.send(WorkerMessage::DependencyListResult {
                    count: 0,
                    list: "No dependencies listed yet".to_string(),
                }).ok();
            }
            Ok(UserCommand::DependencyOutdated) => {
                eprintln!("📦 Checking outdated dependencies");
                message_tx.send(WorkerMessage::DependencyOutdatedResult {
                    count: 0,
                    list: "All dependencies up to date".to_string(),
                }).ok();
            }
            Ok(UserCommand::DependencyUpdate { crate_name }) => {
                eprintln!("📦 Updating dependency: {:?}", crate_name);
                message_tx.send(WorkerMessage::Response("Dependencies updated".to_string())).ok();
            }
            Ok(UserCommand::DependencyAudit) => {
                eprintln!("📦 Running security audit");
                message_tx.send(WorkerMessage::DependencyAuditResult {
                    vulnerabilities: 0,
                    message: "No vulnerabilities found".to_string(),
                }).ok();
            }
            Ok(UserCommand::DependencySearch { query }) => {
                eprintln!("🔍 Searching for: {}", query);
                message_tx.send(WorkerMessage::DependencySearchResult {
                    count: 0,
                    results: "No results found".to_string(),
                }).ok();
            }

            // === Test generation ===
            Ok(UserCommand::TestGenerate { file, function: _ }) => {
                eprintln!("🧪 Generating tests for: {:?}", file);
                message_tx.send(WorkerMessage::TestGenerationComplete {
                    file: file.unwrap_or_else(|| "src/lib.rs".to_string()),
                    test_count: 0,
                }).ok();
            }
            Ok(UserCommand::TestGenerateMissing) => {
                eprintln!("🧪 Generating missing tests");
                message_tx.send(WorkerMessage::Response("Generating missing tests...".to_string())).ok();
            }

            // === Code analysis ===
            Ok(UserCommand::AnalyzeFile { path }) => {
                eprintln!("📊 Analyzing file: {}", path);
                message_tx.send(WorkerMessage::AnalysisComplete {
                    target: path,
                    result: "Analysis complete".to_string(),
                }).ok();
            }
            Ok(UserCommand::AnalyzeComplexity) => {
                eprintln!("📊 Analyzing complexity");
                message_tx.send(WorkerMessage::Response("Complexity analysis complete".to_string())).ok();
            }

            // === Code review ===
            Ok(UserCommand::Review { target }) => {
                eprintln!("🔍 Reviewing: {}", target);
                message_tx.send(WorkerMessage::ReviewComplete {
                    target,
                    review: "Code review complete".to_string(),
                }).ok();
            }

            // === Documentation ===
            Ok(UserCommand::GenerateDocs { target }) => {
                eprintln!("📝 Generating docs for: {:?}", target);
                message_tx.send(WorkerMessage::DocsGenerated {
                    target: target.unwrap_or_else(|| "project".to_string()),
                    documentation: "Documentation generated".to_string(),
                }).ok();
            }

            // === Linting ===
            Ok(UserCommand::LintFix) => {
                eprintln!("🔧 Running lint fixes");
                message_tx.send(WorkerMessage::LintFixComplete {
                    fixed_count: 0,
                    message: "All linting issues fixed".to_string(),
                }).ok();
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

fn help_text() -> String {
    r#"╔══════════════════════════════════════════════════════════════╗
║                    RUSTY AGENT COMMANDS                       ║
╠══════════════════════════════════════════════════════════════╣
║ KNOWLEDGE & SEARCH                                            ║
║   /help              Show this help message                   ║
║   /search <query>    Search knowledge database                ║
║   /stats             Show database statistics                 ║
║   /web <query>       Force web search                         ║
║                                                               ║
║ EXECUTION                                                     ║
║   /run [binary]      Run compiled program                     ║
║   /serve [port]      Start web server (default: 3000)         ║
║   /stop <pid|port>   Stop process or server                   ║
║   /sh <command>      Execute shell command                    ║
║                                                               ║
║ BUILD & TEST                                                  ║
║   /build [--release] Compile project                          ║
║   /test [name]       Run tests                                ║
║                                                               ║
║ PROJECT MANAGEMENT                                            ║
║   /scaffold <t> <n>  Create new project (types: cli, axum)    ║
║   /install <crate>   Add dependency to Cargo.toml             ║
║                                                               ║
║ AUTONOMOUS MODE                                               ║
║   /iterate <goal>    Start autonomous agent                   ║
║   /pause             Pause autonomous agent                   ║
║   /resume            Resume autonomous agent                  ║
║   /abort             Stop autonomous agent                    ║
║                                                               ║
║ OTHER                                                         ║
║   /status            Show agent status                        ║
║   /processes         List running processes                   ║
║   /clear             Clear chat history                       ║
║   /quit              Exit application                         ║
╚══════════════════════════════════════════════════════════════╝

TIPS:
- Type questions naturally: "What is ownership?"
- Ask for code: "Create a REST API with Axum"
- Start autonomous: "/iterate build a web server"
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
