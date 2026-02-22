use anyhow::Result;
use rust_agent::claude_proxy::ClaudeProxy;
use rust_agent::config::Config;
use rust_agent::types::KnowledgeBase;
use std::path::PathBuf;
use tokio::runtime::Runtime;

/// Manages the AI agent with self-awareness of the Rusty TUI IDE
/// Uses the SAME ClaudeProxy as `cargo run` in rust_agent
pub struct AgentManager {
    claude: ClaudeProxy,
    knowledge_base: Option<KnowledgeBase>,
    config: Config,
    conversation_history: Vec<(String, String)>,
    runtime: Runtime,
    tui_source_code: String,
}

impl AgentManager {
    pub fn new() -> Result<Self> {
        let claude = ClaudeProxy::new();
        let config = Config::load().unwrap_or_default();
        let runtime = Runtime::new()?;
        let tui_source_code = Self::load_tui_source_code()?;
        let knowledge_base = Self::load_knowledge_base();

        Ok(Self {
            claude,
            knowledge_base,
            config,
            conversation_history: Vec::new(),
            runtime,
            tui_source_code,
        })
    }

    /// Load the knowledge base (same as interactive agent)
    fn load_knowledge_base() -> Option<KnowledgeBase> {
        let home_dir = dirs::home_dir()?;
        let knowledge_path = home_dir.join(".agent").join("data").join("knowledge_base.json");

        if knowledge_path.exists() {
            let content = std::fs::read_to_string(&knowledge_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    /// Load the Rusty TUI source code for self-awareness
    fn load_tui_source_code() -> Result<String> {
        use std::fs;

        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut source = String::from("# Rusty TUI Source Code (for self-awareness)\n\n");

        // Read all .rs files in src/
        if let Ok(entries) = fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        source.push_str(&format!("\n## File: {:?}\n```rust\n{}\n```\n",
                            path.file_name().unwrap(), content));
                    }
                }
            }
        }

        Ok(source)
    }

    /// Send a query to Claude (async version)
    pub async fn query(&mut self, user_query: String) -> Result<String> {
        // Build context from knowledge base
        let mut context = String::new();

        if let Some(kb) = &self.knowledge_base {
            context.push_str(&format!("You have access to {} learned Q&A pairs about Rust.\n\n", kb.qa_pairs.len()));

            // Search for relevant Q&A pairs
            let relevant: Vec<_> = kb.qa_pairs.iter()
                .filter(|qa| {
                    let query_lower = user_query.to_lowercase();
                    qa.question.text.to_lowercase().contains(&query_lower) ||
                    qa.answer.text.to_lowercase().contains(&query_lower)
                })
                .take(3)
                .collect();

            if !relevant.is_empty() {
                context.push_str("Relevant knowledge:\n");
                for qa in relevant {
                    context.push_str(&format!("Q: {}\nA: {}\n\n", qa.question.text, qa.answer.text));
                }
            }
        }

        // Add TUI self-awareness for IDE-related questions
        let is_ide_question = user_query.to_lowercase().contains("rusty") ||
                               user_query.to_lowercase().contains("ide") ||
                               user_query.to_lowercase().contains("tui");

        if is_ide_question {
            context.push_str(&format!("\n# Rusty TUI IDE Source Code\n{}\n", self.tui_source_code));
            context.push_str("\nNote: The user is asking about the IDE you're currently running in.\n");
        }

        // Build the full prompt
        let prompt = if context.is_empty() {
            user_query.clone()
        } else {
            format!("{}\n\nUser question: {}", context, user_query)
        };

        // Send to Claude (same API as `cargo run`)
        let system_prompt = Some("You are a Rust programming assistant running inside Rusty TUI IDE. \
                                  You help users learn Rust and can modify the IDE itself when asked.".to_string());

        let response = self.claude.send_request(prompt, system_prompt).await?;

        // Store in history
        self.conversation_history.push((user_query, response.clone()));

        Ok(response)
    }

    /// Send query (blocking version for non-async contexts)
    pub fn query_blocking(&mut self, user_query: String) -> Result<String> {
        // Build context from knowledge base
        let mut context = String::new();

        if let Some(kb) = &self.knowledge_base {
            context.push_str(&format!("You have access to {} learned Q&A pairs about Rust.\n\n", kb.qa_pairs.len()));

            // Search for relevant Q&A pairs
            let relevant: Vec<_> = kb.qa_pairs.iter()
                .filter(|qa| {
                    let query_lower = user_query.to_lowercase();
                    qa.question.text.to_lowercase().contains(&query_lower) ||
                    qa.answer.text.to_lowercase().contains(&query_lower)
                })
                .take(3)
                .collect();

            if !relevant.is_empty() {
                context.push_str("Relevant knowledge:\n");
                for qa in relevant {
                    context.push_str(&format!("Q: {}\nA: {}\n\n", qa.question.text, qa.answer.text));
                }
            }
        }

        // Add TUI self-awareness for IDE-related questions
        let is_ide_question = user_query.to_lowercase().contains("rusty") ||
                               user_query.to_lowercase().contains("ide") ||
                               user_query.to_lowercase().contains("tui");

        if is_ide_question {
            context.push_str(&format!("\n# Rusty TUI IDE Source Code\n{}\n", self.tui_source_code));
            context.push_str("\nNote: The user is asking about the IDE you're currently running in.\n");
        }

        // Build the full prompt
        let prompt = if context.is_empty() {
            user_query.clone()
        } else {
            format!("{}\n\nUser question: {}", context, user_query)
        };

        // Send to Claude (same API as `cargo run`)
        let system_prompt = Some("You are a Rust programming assistant running inside Rusty TUI IDE. \
                                  You help users learn Rust and can modify the IDE itself when asked.".to_string());

        let response = self.runtime.block_on(async {
            self.claude.send_request(prompt, system_prompt).await
        })?;

        // Store in history
        self.conversation_history.push((user_query, response.clone()));

        Ok(response)
    }

    /// Get conversation history
    pub fn get_history(&self) -> &[(String, String)] {
        &self.conversation_history
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }

    /// Add to history manually
    pub fn add_to_history(&mut self, query: String, response: String) {
        self.conversation_history.push((query, response));
    }

    /// Reload TUI source code (for when IDE code changes)
    pub fn reload_tui_source(&mut self) -> Result<()> {
        self.tui_source_code = Self::load_tui_source_code()?;
        Ok(())
    }

    /// Reload knowledge base
    pub fn reload_knowledge(&mut self) {
        self.knowledge_base = Self::load_knowledge_base();
    }

    /// Get the TUI source code
    pub fn get_tui_source(&self) -> &str {
        &self.tui_source_code
    }

    /// Get agent status (for UI display)
    pub fn get_status(&self) -> String {
        let kb_status = if let Some(kb) = &self.knowledge_base {
            format!("✓ Knowledge Base: {} Q&A pairs", kb.qa_pairs.len())
        } else {
            "⚠ No knowledge base loaded".to_string()
        };

        format!("✓ Direct Integration (same ClaudeProxy as `cargo run`)\n\
                 ✓ Self-awareness enabled ({} chars of TUI source)\n\
                 {}\n\
                 ✓ {} messages in history",
                self.tui_source_code.len(),
                kb_status,
                self.conversation_history.len())
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new().expect("Failed to create AgentManager")
    }
}
