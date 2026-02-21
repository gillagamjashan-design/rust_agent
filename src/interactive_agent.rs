use crate::cache::SearchCache;
use crate::claude_proxy::ClaudeProxy;
use crate::config::Config;
use crate::types::{KnowledgeBase, SearchResult};
use crate::web_search::DuckDuckGoClient;
use anyhow::Result;
use std::io::{self, Write};

pub struct InteractiveAgent {
    claude: ClaudeProxy,
    knowledge_base: Option<KnowledgeBase>,
    web_search: Option<DuckDuckGoClient>,
    config: Config,
    cache: SearchCache,
}

impl InteractiveAgent {
    pub fn new() -> Self {
        // Load config
        let config = Config::load().unwrap_or_default();

        // Get cache directory
        let home_dir = dirs::home_dir().expect("Cannot find home directory");
        let cache_dir = home_dir.join(".agent").join("cache");

        // Initialize cache
        let cache = SearchCache::new(
            cache_dir,
            config.web_search.cache_ttl_hours as i64,
        ).expect("Failed to create search cache");

        // Create web search client if enabled
        let web_search = if config.web_search.enabled {
            DuckDuckGoClient::new().ok()
        } else {
            None
        };

        Self {
            claude: ClaudeProxy::new(),
            knowledge_base: None,
            web_search,
            config,
            cache,
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
        println!("  /help         - Show this help");
        println!("  /stats        - Show knowledge statistics");
        println!("  /search       - Search knowledge base");
        println!("  /web <query>  - Force web search");
        println!("  /cache clear  - Clear search cache");
        println!("  /quit         - Exit");
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
                "/cache clear" => {
                    if let Err(e) = self.cache.clear() {
                        println!("❌ Failed to clear cache: {}", e);
                    } else {
                        println!("✅ Search cache cleared");
                    }
                }
                cmd if cmd.starts_with("/search ") => {
                    let query = &cmd[8..];
                    self.search_knowledge(query);
                }
                cmd if cmd.starts_with("/web ") => {
                    let query = &cmd[5..];
                    self.handle_web_search(query).await?;
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
        println!("  /web <query>       - Force web search for latest information");
        println!("  /cache clear       - Clear web search cache");
        println!("  /quit              - Exit interactive mode");
        println!("\n💡 Usage:");
        println!("  Just type your question or request, and I'll help you!");
        println!("  Examples:");
        println!("    - How do I create a git branch?");
        println!("    - Write a bash script to backup files");
        println!("    - Explain docker containers");
        println!("\n🌐 Web Search:");
        if self.config.web_search.enabled {
            println!("  Enabled - I'll search the web if needed");
        } else {
            println!("  Disabled - Only using knowledge base");
        }
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

        // Tier 1: Search knowledge base
        let kb_answer = self.search_in_knowledge(question);

        // Tier 2: Search web if needed and enabled
        let web_results = if kb_answer.is_none() && self.web_search.is_some() {
            self.search_web(question).await.ok()
        } else {
            None
        };

        // Tier 3: Build context and send to Claude
        let full_prompt = self.build_context(question, kb_answer.as_ref(), web_results.as_ref());

        // Get response from Claude via proxy
        match self.claude.send_request(full_prompt).await {
            Ok(response) => {
                println!("💡 Answer:\n");
                println!("{}", response);

                // Show sources if from web search
                if let Some(results) = web_results {
                    self.show_sources(&results);
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                println!("\nMake sure CLIProxyAPI is running on localhost:8317");
                println!("Start it with: cd cliproxyapi && npm start");
            }
        }

        Ok(())
    }

    async fn handle_web_search(&mut self, query: &str) -> Result<()> {
        println!("\n🌐 Searching the web...\n");

        match self.search_web(query).await {
            Ok(results) => {
                if results.is_empty() {
                    println!("No web results found.");
                } else {
                    println!("Found {} results:\n", results.len());
                    for (i, result) in results.iter().enumerate() {
                        println!("{}. {}", i + 1, result.title);
                        println!("   {}", result.url);
                        println!("   {}\n", result.snippet);
                    }
                }
            }
            Err(e) => {
                println!("❌ Web search failed: {}", e);
            }
        }

        Ok(())
    }

    async fn search_web(&mut self, query: &str) -> Result<Vec<SearchResult>> {
        // Check cache first
        if let Some(cached_response) = self.cache.get(query) {
            println!("📦 Using cached results");
            return Ok(cached_response.results);
        }

        // Enhance query if Rust-focused (always enhance for Rust questions)
        let search_query_str = if self.is_rust_question(query) {
            format!("Rust programming {}", query)
        } else {
            query.to_string()
        };

        // Perform web search
        if let Some(client) = &self.web_search {
            println!("🌐 Searching the web...");

            // Create SearchQuery with max_results from config
            let search_query = crate::web_search::SearchQuery::new(search_query_str)
                .with_max_results(self.config.web_search.max_results);

            let response = client.search(&search_query).await?;

            // Cache the full response
            let _ = self.cache.set(query, response.clone());

            Ok(response.results)
        } else {
            Err(anyhow::anyhow!("Web search is disabled"))
        }
    }

    fn build_context(&self, question: &str, kb_answer: Option<&String>, web_results: Option<&Vec<SearchResult>>) -> String {
        let mut context = String::new();

        // Base context
        if let Some(kb) = &self.knowledge_base {
            context.push_str(&format!(
                "You are a Rust programming assistant with knowledge from {} Q&A pairs.\n\n",
                kb.qa_pairs.len()
            ));
        } else {
            context.push_str("You are a Rust programming assistant.\n\n");
        }

        // Add knowledge base answer if found
        if let Some(kb_ans) = kb_answer {
            context.push_str("From your knowledge base:\n");
            context.push_str(kb_ans);
            context.push_str("\n\n");
        }

        // Add web results if available
        if let Some(results) = web_results {
            if !results.is_empty() {
                context.push_str("Additional information from the web:\n");
                for (i, result) in results.iter().enumerate() {
                    context.push_str(&format!(
                        "{}. {} ({})\n   {}\n\n",
                        i + 1,
                        result.title,
                        result.url,
                        result.snippet
                    ));
                }
            }
        }

        // Add the user question
        context.push_str(&format!("User question: {}\n\n", question));
        context.push_str("Provide a helpful, accurate answer with code examples if relevant. ");
        context.push_str("Focus on Rust programming best practices.");

        context
    }

    fn show_sources(&self, results: &[SearchResult]) {
        if !results.is_empty() {
            println!("\n📚 Sources:");
            for result in results {
                println!("  • {} - {}", result.title, result.url);
            }
        }
    }

    fn is_rust_question(&self, question: &str) -> bool {
        let rust_keywords = [
            "rust", "cargo", "crate", "trait", "impl", "borrow", "lifetime",
            "ownership", "mut", "unsafe", "async", "await", "tokio", "serde",
            "match", "enum", "struct", "mod", "pub", "fn", "let", "const",
        ];

        let question_lower = question.to_lowercase();
        rust_keywords.iter().any(|&keyword| question_lower.contains(keyword))
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

