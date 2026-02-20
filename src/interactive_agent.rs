use crate::claude_proxy::ClaudeProxy;
use crate::types::KnowledgeBase;
use anyhow::Result;
use std::io::{self, Write};

pub struct InteractiveAgent {
    claude: ClaudeProxy,
    knowledge_base: Option<KnowledgeBase>,
}

impl InteractiveAgent {
    pub fn new() -> Self {
        Self {
            claude: ClaudeProxy::new(),
            knowledge_base: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.load_knowledge()?;
        self.show_welcome();
        self.interactive_loop().await?;
        Ok(())
    }

    fn load_knowledge(&mut self) -> Result<()> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        let knowledge_path = home_dir.join(".agent").join("data").join("knowledge_base.json");

        if knowledge_path.exists() {
            let content = std::fs::read_to_string(&knowledge_path)?;
            self.knowledge_base = Some(serde_json::from_str(&content)?);
            println!("✅ Loaded knowledge base with {} Q&A pairs",
                self.knowledge_base.as_ref().unwrap().qa_pairs.len());
        } else {
            println!("⚠️  No knowledge base found. Run learning mode first to train the agent.");
        }

        Ok(())
    }

    fn show_welcome(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║          Interactive Programming Assistant                  ║");
        println!("║          Powered by YOUR Learned Knowledge                  ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();

        if let Some(kb) = &self.knowledge_base {
            println!("🧠 Knowledge Loaded:");
            println!("   - {} Q&A pairs", kb.qa_pairs.len());
            println!("   - {} patterns", kb.patterns.len());
            println!("   - Topics: {:?}", kb.topics_covered.iter()
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .take(5)
                .collect::<Vec<_>>());
        }

        println!();
        println!("I can help you with:");
        println!("  • Writing code and scripts");
        println!("  • Explaining commands (Linux, Git, Docker, etc.)");
        println!("  • Building projects");
        println!("  • Solving programming problems");
        println!();
        println!("Commands:");
        println!("  /help    - Show this help");
        println!("  /stats   - Show knowledge statistics");
        println!("  /search  - Search knowledge base");
        println!("  /quit    - Exit");
        println!();
        println!("Type your question or command:");
        println!("════════════════════════════════════════════════════════════════");
    }

    async fn interactive_loop(&mut self) -> Result<()> {
        loop {
            print!("\n> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            match input {
                "/quit" | "/exit" => {
                    println!("👋 Goodbye!");
                    break;
                }
                "/help" => self.show_help(),
                "/stats" => self.show_stats(),
                cmd if cmd.starts_with("/search ") => {
                    let query = &cmd[8..];
                    self.search_knowledge(query);
                }
                _ => {
                    self.handle_question(input).await?;
                }
            }
        }

        Ok(())
    }

    fn show_help(&self) {
        println!("\n📖 Available Commands:");
        println!("  /help              - Show this help message");
        println!("  /stats             - Display knowledge statistics");
        println!("  /search <query>    - Search your knowledge base");
        println!("  /quit              - Exit interactive mode");
        println!("\n💡 Usage:");
        println!("  Just type your question or request, and I'll help you!");
        println!("  Examples:");
        println!("    - How do I create a git branch?");
        println!("    - Write a bash script to backup files");
        println!("    - Explain docker containers");
    }

    fn show_stats(&self) {
        println!("\n📊 Knowledge Base Statistics:");

        if let Some(kb) = &self.knowledge_base {
            println!("  Total Q&A Pairs: {}", kb.qa_pairs.len());
            println!("  Total Patterns: {}", kb.patterns.len());
            println!("  Topics Covered: {}", kb.topics_covered.len());

            let unique_topics: std::collections::HashSet<_> =
                kb.topics_covered.iter().collect();

            println!("\n  📚 Topics:");
            for topic in unique_topics.iter().take(10) {
                let count = kb.topics_covered.iter()
                    .filter(|t| t == topic)
                    .count();
                println!("    - {}: {} items", topic, count);
            }

            println!("\n  Last Updated: {}", kb.last_updated);
        } else {
            println!("  ⚠️  No knowledge base loaded.");
            println!("  Run learning mode first: cargo run");
        }
    }

    fn search_knowledge(&self, query: &str) {
        println!("\n🔍 Searching for: \"{}\"", query);

        if let Some(kb) = &self.knowledge_base {
            let query_lower = query.to_lowercase();
            let mut found = 0;

            for qa_pair in &kb.qa_pairs {
                if qa_pair.question.text.to_lowercase().contains(&query_lower) ||
                   qa_pair.answer.text.to_lowercase().contains(&query_lower) {
                    found += 1;
                    println!("\n  Q{}: {}", qa_pair.question.id, qa_pair.question.text);
                    println!("  A: {}",
                        if qa_pair.answer.text.len() > 100 {
                            format!("{}...", &qa_pair.answer.text[..100])
                        } else {
                            qa_pair.answer.text.clone()
                        }
                    );

                    if found >= 3 {
                        println!("\n  ... and more results");
                        break;
                    }
                }
            }

            if found == 0 {
                println!("  No results found.");
            } else {
                println!("\n  Found {} result(s)", found);
            }
        } else {
            println!("  ⚠️  No knowledge base loaded.");
        }
    }

    async fn handle_question(&mut self, question: &str) -> Result<()> {
        println!("\n🤔 Processing your request...\n");

        // Check if question is in knowledge base first
        let kb_answer = self.search_in_knowledge(question);

        // Build context from knowledge base
        let context = if let Some(kb) = &self.knowledge_base {
            format!(
                "You are a programming assistant with knowledge from {} Q&A pairs. \
                Use your learned knowledge to help answer questions.\n\n",
                kb.qa_pairs.len()
            )
        } else {
            String::new()
        };

        // Add knowledge base answer if found
        let full_prompt = if let Some(kb_ans) = kb_answer {
            format!("{}Based on my knowledge base:\n{}\n\nUser question: {}\n\nProvide a helpful answer:",
                context, kb_ans, question)
        } else {
            format!("{}User question: {}\n\nProvide a helpful answer with code examples if relevant:",
                context, question)
        };

        // Get response from Claude via proxy
        match self.claude.send_request(full_prompt).await {
            Ok(response) => {
                println!("💡 Answer:\n");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                println!("\nMake sure CLIProxyAPI is running on localhost:8317");
                println!("Start it with: cd cliproxyapi && npm start");
            }
        }

        Ok(())
    }

    fn search_in_knowledge(&self, query: &str) -> Option<String> {
        if let Some(kb) = &self.knowledge_base {
            let query_lower = query.to_lowercase();

            for qa_pair in &kb.qa_pairs {
                if qa_pair.question.text.to_lowercase().contains(&query_lower) {
                    return Some(format!("Q: {}\nA: {}",
                        qa_pair.question.text,
                        qa_pair.answer.text));
                }
            }
        }
        None
    }
}

