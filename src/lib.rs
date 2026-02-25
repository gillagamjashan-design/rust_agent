// Library interface for rust_agent
// Provides access to knowledge database and agent tools

// Core modules
pub mod config;
pub mod web_search;
pub mod claude_proxy;
pub mod types;
pub mod cache;

// Knowledge database system
pub mod knowledge;      // SQLite with FTS5 full-text search
pub mod tools;          // Runtime tools for agent (KnowledgeFetcher)
