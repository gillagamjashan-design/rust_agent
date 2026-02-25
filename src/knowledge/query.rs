// Knowledge query - Interface for searching the knowledge database

use super::database::{KnowledgeConcept, KnowledgeCommand, KnowledgeDatabase, KnowledgeError, KnowledgePattern};
use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// Query interface for knowledge database
pub struct KnowledgeQuery {
    db: KnowledgeDatabase,
}

impl KnowledgeQuery {
    /// Create new query interface
    pub fn new(db: KnowledgeDatabase) -> Self {
        Self { db }
    }

    /// Search concepts using full-text search
    pub fn search_concepts(&self, query: &str) -> Result<Vec<KnowledgeConcept>> {
        self.search_concepts_limit(query, 10)
    }

    /// Search concepts with custom limit
    pub fn search_concepts_limit(&self, query: &str, limit: usize) -> Result<Vec<KnowledgeConcept>> {
        let conn = self.db.conn.lock().unwrap();

        // Use FTS5 MATCH for full-text search
        let mut stmt = conn.prepare(
            "SELECT c.id, c.topic, c.title, c.explanation, c.code_examples,
                    c.common_mistakes, c.related_concepts, c.tags
             FROM concepts c
             JOIN concepts_fts fts ON c.rowid = fts.rowid
             WHERE concepts_fts MATCH ?1
             LIMIT ?2",
        )?;

        let concepts = stmt
            .query_map(params![query, limit], |row: &rusqlite::Row| {
                Ok(KnowledgeConcept {
                    id: row.get(0)?,
                    topic: row.get(1)?,
                    title: row.get(2)?,
                    explanation: row.get(3)?,
                    code_examples: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    common_mistakes: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                    related_concepts: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                    tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(concepts)
    }

    /// Get specific concept by ID
    pub fn get_concept(&self, id: &str) -> Result<Option<KnowledgeConcept>> {
        self.db.get_concept(id)
    }

    /// Search concepts by topic
    pub fn search_by_topic(&self, topic: &str) -> Result<Vec<KnowledgeConcept>> {
        let conn = self.db.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, topic, title, explanation, code_examples, common_mistakes, related_concepts, tags
             FROM concepts
             WHERE topic = ?1",
        )?;

        let concepts = stmt
            .query_map([topic], |row: &rusqlite::Row| {
                Ok(KnowledgeConcept {
                    id: row.get(0)?,
                    topic: row.get(1)?,
                    title: row.get(2)?,
                    explanation: row.get(3)?,
                    code_examples: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    common_mistakes: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                    related_concepts: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                    tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(concepts)
    }

    /// Find patterns matching a use case
    pub fn find_patterns(&self, use_case: &str) -> Result<Vec<KnowledgePattern>> {
        self.find_patterns_limit(use_case, 10)
    }

    /// Find patterns with custom limit
    pub fn find_patterns_limit(&self, use_case: &str, limit: usize) -> Result<Vec<KnowledgePattern>> {
        let conn = self.db.conn.lock().unwrap();

        // Use FTS5 MATCH for full-text search
        let mut stmt = conn.prepare(
            "SELECT p.id, p.name, p.description, p.template, p.when_to_use,
                    p.when_not_to_use, p.examples
             FROM patterns p
             JOIN patterns_fts fts ON p.rowid = fts.rowid
             WHERE patterns_fts MATCH ?1
             LIMIT ?2",
        )?;

        let patterns = stmt
            .query_map(params![use_case, limit], |row: &rusqlite::Row| {
                Ok(KnowledgePattern {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    template: row.get(3)?,
                    when_to_use: row.get(4)?,
                    when_not_to_use: row.get(5)?,
                    examples: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(patterns)
    }

    /// Get specific pattern by ID
    pub fn get_pattern(&self, id: &str) -> Result<Option<KnowledgePattern>> {
        self.db.get_pattern(id)
    }

    /// Explain a compiler error by error code
    pub fn explain_error(&self, error_code: &str) -> Result<Option<KnowledgeError>> {
        self.db.get_error(error_code)
    }

    /// Search commands by tool and keyword
    pub fn search_commands(&self, tool: &str, keyword: &str) -> Result<Vec<KnowledgeCommand>> {
        let conn = self.db.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, tool, command, description, flags, examples
             FROM commands
             WHERE tool = ?1 AND (command LIKE ?2 OR description LIKE ?2)
             LIMIT 20",
        )?;

        let search_pattern = format!("%{}%", keyword);
        let commands = stmt
            .query_map(params![tool, search_pattern], |row: &rusqlite::Row| {
                Ok(KnowledgeCommand {
                    tool: row.get(1)?,
                    command: row.get(2)?,
                    description: row.get(3)?,
                    flags: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    examples: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(commands)
    }

    /// Get all commands for a specific tool
    pub fn get_tool_commands(&self, tool: &str) -> Result<Vec<KnowledgeCommand>> {
        let conn = self.db.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, tool, command, description, flags, examples
             FROM commands
             WHERE tool = ?1",
        )?;

        let commands = stmt
            .query_map([tool], |row: &rusqlite::Row| {
                Ok(KnowledgeCommand {
                    tool: row.get(1)?,
                    command: row.get(2)?,
                    description: row.get(3)?,
                    flags: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    examples: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(commands)
    }

    /// Full-text search across all knowledge types
    pub fn search_all(&self, query: &str) -> Result<SearchResults> {
        Ok(SearchResults {
            concepts: self.search_concepts_limit(query, 5)?,
            patterns: self.find_patterns_limit(query, 5)?,
            commands: vec![], // Commands don't use FTS5, skip for now
        })
    }
}

/// Combined search results across all knowledge types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub concepts: Vec<KnowledgeConcept>,
    pub patterns: Vec<KnowledgePattern>,
    pub commands: Vec<KnowledgeCommand>,
}

impl SearchResults {
    /// Check if results are empty
    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty() && self.patterns.is_empty() && self.commands.is_empty()
    }

    /// Total number of results
    pub fn total(&self) -> usize {
        self.concepts.len() + self.patterns.len() + self.commands.len()
    }

    /// Format results as a readable string
    pub fn format(&self) -> String {
        let mut output = String::new();

        if !self.concepts.is_empty() {
            output.push_str("## Concepts\n\n");
            for concept in &self.concepts {
                output.push_str(&format!("### {}\n", concept.title));
                output.push_str(&format!("**Topic:** {}\n\n", concept.topic));
                output.push_str(&format!("{}\n\n", concept.explanation));

                if !concept.code_examples.is_empty() {
                    output.push_str("**Examples:**\n");
                    for example in &concept.code_examples {
                        output.push_str(&format!("\n**{}:**\n```rust\n{}\n```\n", example.title, example.code));
                        output.push_str(&format!("{}\n", example.explanation));
                    }
                    output.push('\n');
                }
            }
        }

        if !self.patterns.is_empty() {
            output.push_str("## Patterns\n\n");
            for pattern in &self.patterns {
                output.push_str(&format!("### {}\n", pattern.name));
                output.push_str(&format!("{}\n\n", pattern.description));
                output.push_str(&format!("**When to use:** {}\n\n", pattern.when_to_use));
                output.push_str(&format!("```rust\n{}\n```\n\n", pattern.template));
            }
        }

        if !self.commands.is_empty() {
            output.push_str("## Commands\n\n");
            for cmd in &self.commands {
                output.push_str(&format!("### {} {}\n", cmd.tool, cmd.command));
                output.push_str(&format!("{}\n\n", cmd.description));

                if !cmd.examples.is_empty() {
                    output.push_str("**Examples:**\n");
                    for example in &cmd.examples {
                        output.push_str(&format!("- `{}`\n", example));
                    }
                    output.push('\n');
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::database::CodeExample;

    #[test]
    fn test_search_concepts() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        let query = KnowledgeQuery::new(db);

        // Add a test concept
        query.db.store_concept(&KnowledgeConcept {
            id: "ownership-move".to_string(),
            topic: "ownership".to_string(),
            title: "Move Semantics".to_string(),
            explanation: "How Rust transfers ownership between variables".to_string(),
            code_examples: vec![],
            common_mistakes: vec![],
            related_concepts: vec![],
            tags: vec!["ownership".to_string()],
        }).unwrap();

        // Search should find it
        let results = query.search_concepts("ownership").unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_all() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        let query = KnowledgeQuery::new(db);

        // Add test data
        query.db.store_concept(&KnowledgeConcept {
            id: "test".to_string(),
            topic: "testing".to_string(),
            title: "Test Concept".to_string(),
            explanation: "Testing explanation".to_string(),
            code_examples: vec![],
            common_mistakes: vec![],
            related_concepts: vec![],
            tags: vec![],
        }).unwrap();

        query.db.store_pattern(&KnowledgePattern {
            id: "test-pattern".to_string(),
            name: "Test Pattern".to_string(),
            description: "Testing pattern".to_string(),
            template: "...".to_string(),
            when_to_use: "For testing".to_string(),
            when_not_to_use: "".to_string(),
            examples: vec![],
        }).unwrap();

        let results = query.search_all("testing").unwrap();
        assert!(!results.is_empty());
    }
}
