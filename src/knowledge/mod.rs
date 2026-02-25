// Knowledge module - Queryable knowledge database system
//
// Instead of training the agent to "remember" everything, this module provides:
// 1. A comprehensive knowledge database with all Rust information
// 2. A fetch/query tool for the agent to look up information at runtime
// 3. Direct instruction prompts that inject relevant knowledge into context
//
// This is more efficient because:
// - No "learning" time needed - knowledge is immediately available
// - Agent can handle any query by fetching relevant info
// - Database can be expanded without retraining

pub mod database;
pub mod loader;
pub mod query;

pub use database::*;
pub use loader::{KnowledgeLoader, LoadStats};
pub use query::{KnowledgeQuery, SearchResults};
