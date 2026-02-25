//! Application state and logic

use anyhow::Result;
use chrono::{DateTime, Utc};
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;
use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeLoader, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;
use rust_agent::claude_proxy::ClaudeProxy;

use crate::commands::Command;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Typing,
}

pub struct App {
    pub messages: Vec<Message>,
    pub input: String,
    pub input_mode: InputMode,
    pub scroll: usize,
    pub knowledge_fetcher: KnowledgeFetcher,
    pub claude: ClaudeProxy,
    pub knowledge_stats: String,
    pub awaiting_feedback: bool,
    pub last_response_index: Option<usize>,
}

impl App {
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        // Initialize or load knowledge database
        let db = if db_path.exists() {
            KnowledgeDatabase::new(&db_path)?
        } else {
            let db = KnowledgeDatabase::new(&db_path)?;
            let loader = KnowledgeLoader::new(db.clone());
            let stats = loader.load_all_from_directory("knowledge")?;
            eprintln!("✅ Loaded {} concepts, {} patterns, {} commands", 
                     stats.concepts, stats.patterns, stats.commands);
            db
        };
        
        // Get knowledge stats
        let concepts = db.count_concepts()?;
        let patterns = db.count_patterns()?;
        let commands = db.count_commands()?;
        let total = concepts + patterns + commands;
        let knowledge_stats = format!("{} items loaded ({} concepts, {} patterns, {} commands)", 
                                     total, concepts, patterns, commands);
        
        // Create knowledge fetcher
        let query = KnowledgeQuery::new(db);
        let knowledge_fetcher = KnowledgeFetcher::new(query);
        
        // Create Claude proxy client
        let claude = ClaudeProxy::new();
        
        // Create welcome message
        let welcome = Message {
            role: Role::System,
            content: format!(
                "🦀 Welcome to Rusty - Your Rust Learning Agent!\n\n\
                 {} ready to help you learn Rust.\n\n\
                 Type your question or use commands:\n\
                 • /help - Show available commands\n\
                 • /search <query> - Search knowledge database\n\
                 • /stats - Show database statistics\n\
                 • /quit - Exit application",
                knowledge_stats
            ),
            timestamp: Utc::now(),
        };
        
        Ok(Self {
            messages: vec![welcome],
            input: String::new(),
            input_mode: InputMode::Typing,
            scroll: 0,
            knowledge_fetcher,
            claude,
            knowledge_stats,
            awaiting_feedback: false,
            last_response_index: None,
        })
    }
    
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    let input = self.input.clone();
                    self.input.clear();
                    self.handle_input(&input).await?;
                }
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Esc => {
                if self.input.is_empty() {
                    return Ok(false); // Quit
                } else {
                    self.input.clear();
                }
            }
            _ => {}
        }
        Ok(true)
    }
    
    async fn handle_input(&mut self, input: &str) -> Result<()> {
        // Add user message
        self.messages.push(Message {
            role: Role::User,
            content: input.to_string(),
            timestamp: Utc::now(),
        });
        
        // Check if this is feedback to a previous response
        if self.awaiting_feedback {
            self.awaiting_feedback = false;
            if input.to_lowercase().contains("no") || input.to_lowercase().contains("not") {
                // User said response wasn't helpful
                self.messages.push(Message {
                    role: Role::Assistant,
                    content: "I'm sorry that wasn't helpful. What's the problem with my response? Please explain what you need.".to_string(),
                    timestamp: Utc::now(),
                });
                return Ok(());
            }
        }
        
        // Check for commands
        if let Some(command) = Command::parse(input) {
            let response = command.execute(self).await?;
            self.messages.push(Message {
                role: Role::Assistant,
                content: response,
                timestamp: Utc::now(),
            });
            return Ok(());
        }
        
        // Check for "what's the problem" follow-up
        if self.last_response_index.is_some() && 
           (input.to_lowercase().contains("problem") || input.to_lowercase().contains("issue")) {
            // User is explaining the problem - generate a fix
            self.generate_response_with_fix(input).await?;
            return Ok(());
        }
        
        // Normal query - generate response
        self.generate_response(input).await?;
        
        Ok(())
    }
    
    async fn generate_response(&mut self, query: &str) -> Result<()> {
        // Search knowledge database
        let knowledge_result = self.knowledge_fetcher.search(query)?;
        
        // Build context for Claude
        let context = if !knowledge_result.results.concepts.is_empty() || 
                         !knowledge_result.results.patterns.is_empty() {
            format!("Relevant knowledge from database:\n{}\n\n", knowledge_result.formatted)
        } else {
            String::new()
        };
        
        // Build prompt
        let prompt = format!(
            "{}User question: {}\n\n\
             Please provide a helpful response. If you're writing code, use Rust syntax. \
             Explain concepts clearly and provide examples when appropriate.",
            context, query
        );
        
        // Get response from Claude
        let response = self.claude.query(&prompt).await?;
        
        // Add response to messages
        let response_index = self.messages.len();
        self.messages.push(Message {
            role: Role::Assistant,
            content: response,
            timestamp: Utc::now(),
        });
        
        // Ask for feedback
        self.awaiting_feedback = true;
        self.last_response_index = Some(response_index);
        self.messages.push(Message {
            role: Role::System,
            content: "Was this helpful? (yes/no)".to_string(),
            timestamp: Utc::now(),
        });
        
        Ok(())
    }
    
    async fn generate_response_with_fix(&mut self, problem_description: &str) -> Result<()> {
        // Get the previous response
        let prev_response = if let Some(idx) = self.last_response_index {
            self.messages.get(idx).map(|m| m.content.clone())
        } else {
            None
        };
        
        let prompt = format!(
            "The user reported a problem with my previous response.\n\n\
             Previous response: {}\n\n\
             User's problem: {}\n\n\
             Please provide a FIXED response that addresses their concern. \
             If code was incorrect, provide corrected code. \
             Explain what was wrong and how you fixed it.",
            prev_response.unwrap_or_else(|| "N/A".to_string()),
            problem_description
        );
        
        let response = self.claude.query(&prompt).await?;
        
        self.messages.push(Message {
            role: Role::Assistant,
            content: format!("I've fixed the issue:\n\n{}", response),
            timestamp: Utc::now(),
        });
        
        self.messages.push(Message {
            role: Role::System,
            content: "Is this better? (yes/no)".to_string(),
            timestamp: Utc::now(),
        });
        
        Ok(())
    }
    
    pub async fn update(&mut self) -> Result<()> {
        // Placeholder for any async updates
        Ok(())
    }
}
