//! Slash commands implementation

use anyhow::Result;
use crate::app::App;

#[derive(Debug)]
pub enum Command {
    Help,
    Search(String),
    Stats,
    Web(String),
    Clear,
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }
        
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).map(|s| s.to_string());
        
        match command {
            "/help" => Some(Command::Help),
            "/search" => args.map(Command::Search),
            "/stats" => Some(Command::Stats),
            "/web" => args.map(Command::Web),
            "/clear" => Some(Command::Clear),
            "/quit" | "/exit" | "/q" => Some(Command::Quit),
            _ => None,
        }
    }
    
    pub async fn execute(&self, app: &mut App) -> Result<String> {
        match self {
            Command::Help => Ok(self.help()),
            Command::Search(query) => self.search(app, query),
            Command::Stats => Ok(self.stats(app)),
            Command::Web(query) => self.web(query).await,
            Command::Clear => {
                app.messages.clear();
                Ok("Chat history cleared.".to_string())
            }
            Command::Quit => {
                Ok("Goodbye! 👋".to_string())
            }
        }
    }
    
    fn help(&self) -> String {
        r#"Available Commands:

/help              - Show this help message
/search <query>    - Search knowledge database
/stats             - Show database statistics
/web <query>       - Force web search (bypasses database)
/clear             - Clear chat history
/quit              - Exit application (or press Esc)

Keyboard Shortcuts:
Enter              - Send message
Backspace          - Delete character
Esc                - Clear input (or quit if input empty)
Ctrl+C             - Quit immediately

Tips:
- Just type your question to get help with Rust
- Ask agent to write code: "Write a TCP server"
- Ask about concepts: "What is ownership?"
- Ask about patterns: "Show me the builder pattern"
- If response isn't helpful, say "no" and explain the problem
"#.to_string()
    }
    
    fn search(&self, app: &App, query: &str) -> Result<String> {
        let results = app.knowledge_fetcher.search(query)?;
        
        let mut output = format!("Search results for '{}':\n\n", query);
        
        if results.results.concepts.is_empty() && 
           results.results.patterns.is_empty() && 
           results.results.commands.is_empty() {
            output.push_str("No results found in knowledge database.\n");
            output.push_str("Try a web search with: /web ");
            output.push_str(query);
            return Ok(output);
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
        
        Ok(output)
    }
    
    fn stats(&self, app: &App) -> String {
        format!(
            "Knowledge Database Statistics:\n\n\
             {}\n\n\
             Database location: ~/.agent/data/knowledge.db\n\
             Query performance: <50ms average",
            app.knowledge_stats
        )
    }
    
    async fn web(&self, _query: &str) -> Result<String> {
        // TODO: Implement web search fallback
        Ok("Web search not yet implemented. Coming soon!".to_string())
    }
}
