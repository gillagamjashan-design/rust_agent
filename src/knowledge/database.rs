// Knowledge database - Schema and types for queryable knowledge
//
// Uses SQLite with FTS5 (Full-Text Search) for efficient querying

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Core Rust concept with detailed explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConcept {
    pub id: String,
    pub topic: String,           // e.g., "ownership", "lifetimes"
    pub title: String,           // e.g., "Move Semantics"
    pub explanation: String,     // Detailed textbook-style explanation
    pub code_examples: Vec<CodeExample>,
    pub common_mistakes: Vec<String>,
    pub related_concepts: Vec<String>,
    pub tags: Vec<String>,       // For search
}

/// Reusable code pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgePattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub template: String,        // Code template
    pub when_to_use: String,
    pub when_not_to_use: String,
    pub examples: Vec<CodeExample>,
}

/// Compiler error with explanation and fixes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeError {
    pub error_code: String,      // e.g., "E0382"
    pub title: String,
    pub explanation: String,
    pub example_bad: String,
    pub example_good: String,
    pub fix_strategies: Vec<String>,
}

/// Cargo/rustup command reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCommand {
    pub tool: String,            // "cargo", "rustup", etc.
    pub command: String,
    pub description: String,
    pub flags: Vec<CommandFlag>,
    pub examples: Vec<String>,
}

/// Code example with explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub title: String,
    pub code: String,
    pub explanation: String,
}

/// Command flag/option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFlag {
    pub flag: String,
    pub description: String,
}

/// Knowledge database backed by SQLite with FTS5
#[derive(Clone)]
pub struct KnowledgeDatabase {
    pub(crate) conn: Arc<Mutex<Connection>>,  // Allow access from query module
}

impl KnowledgeDatabase {
    /// Create new knowledge database
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn))
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Create in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn))
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema with FTS5 for full-text search
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            -- Core concepts table
            CREATE TABLE IF NOT EXISTS concepts (
                id TEXT PRIMARY KEY,
                topic TEXT NOT NULL,
                title TEXT NOT NULL,
                explanation TEXT NOT NULL,
                code_examples TEXT,        -- JSON array
                common_mistakes TEXT,      -- JSON array
                related_concepts TEXT,     -- JSON array
                tags TEXT,                 -- JSON array
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- FTS5 virtual table for concepts
            CREATE VIRTUAL TABLE IF NOT EXISTS concepts_fts USING fts5(
                topic, title, explanation, tags,
                content='concepts',
                content_rowid='rowid'
            );

            -- Triggers to keep FTS5 in sync
            CREATE TRIGGER IF NOT EXISTS concepts_ai AFTER INSERT ON concepts BEGIN
                INSERT INTO concepts_fts(rowid, topic, title, explanation, tags)
                VALUES (NEW.rowid, NEW.topic, NEW.title, NEW.explanation, NEW.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS concepts_ad AFTER DELETE ON concepts BEGIN
                INSERT INTO concepts_fts(concepts_fts, rowid, topic, title, explanation, tags)
                VALUES('delete', OLD.rowid, OLD.topic, OLD.title, OLD.explanation, OLD.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS concepts_au AFTER UPDATE ON concepts BEGIN
                INSERT INTO concepts_fts(concepts_fts, rowid, topic, title, explanation, tags)
                VALUES('delete', OLD.rowid, OLD.topic, OLD.title, OLD.explanation, OLD.tags);
                INSERT INTO concepts_fts(rowid, topic, title, explanation, tags)
                VALUES (NEW.rowid, NEW.topic, NEW.title, NEW.explanation, NEW.tags);
            END;

            -- Patterns table
            CREATE TABLE IF NOT EXISTS patterns (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                template TEXT NOT NULL,
                when_to_use TEXT,
                when_not_to_use TEXT,
                examples TEXT,             -- JSON array
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- FTS5 for patterns
            CREATE VIRTUAL TABLE IF NOT EXISTS patterns_fts USING fts5(
                name, description, when_to_use,
                content='patterns',
                content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS patterns_ai AFTER INSERT ON patterns BEGIN
                INSERT INTO patterns_fts(rowid, name, description, when_to_use)
                VALUES (NEW.rowid, NEW.name, NEW.description, NEW.when_to_use);
            END;

            CREATE TRIGGER IF NOT EXISTS patterns_ad AFTER DELETE ON patterns BEGIN
                INSERT INTO patterns_fts(patterns_fts, rowid, name, description, when_to_use)
                VALUES('delete', OLD.rowid, OLD.name, OLD.description, OLD.when_to_use);
            END;

            CREATE TRIGGER IF NOT EXISTS patterns_au AFTER UPDATE ON patterns BEGIN
                INSERT INTO patterns_fts(patterns_fts, rowid, name, description, when_to_use)
                VALUES('delete', OLD.rowid, OLD.name, OLD.description, OLD.when_to_use);
                INSERT INTO patterns_fts(rowid, name, description, when_to_use)
                VALUES (NEW.rowid, NEW.name, NEW.description, NEW.when_to_use);
            END;

            -- Errors table
            CREATE TABLE IF NOT EXISTS errors (
                error_code TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                explanation TEXT NOT NULL,
                example_bad TEXT,
                example_good TEXT,
                fix_strategies TEXT,       -- JSON array
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- Index on error_code for fast lookup
            CREATE INDEX IF NOT EXISTS idx_errors_code ON errors(error_code);

            -- Commands table
            CREATE TABLE IF NOT EXISTS commands (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tool TEXT NOT NULL,
                command TEXT NOT NULL,
                description TEXT NOT NULL,
                flags TEXT,                -- JSON array
                examples TEXT,             -- JSON array
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- Index on tool for filtering
            CREATE INDEX IF NOT EXISTS idx_commands_tool ON commands(tool);
            "#,
        )?;

        Ok(())
    }

    /// Store a concept
    pub fn store_concept(&self, concept: &KnowledgeConcept) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO concepts (id, topic, title, explanation, code_examples, common_mistakes, related_concepts, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                concept.id,
                concept.topic,
                concept.title,
                concept.explanation,
                serde_json::to_string(&concept.code_examples)?,
                serde_json::to_string(&concept.common_mistakes)?,
                serde_json::to_string(&concept.related_concepts)?,
                serde_json::to_string(&concept.tags)?,
            ],
        )?;

        Ok(())
    }

    /// Get concept by ID
    pub fn get_concept(&self, id: &str) -> Result<Option<KnowledgeConcept>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, topic, title, explanation, code_examples, common_mistakes, related_concepts, tags
             FROM concepts WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(KnowledgeConcept {
                id: row.get(0)?,
                topic: row.get(1)?,
                title: row.get(2)?,
                explanation: row.get(3)?,
                code_examples: serde_json::from_str(&row.get::<_, String>(4)?)?,
                common_mistakes: serde_json::from_str(&row.get::<_, String>(5)?)?,
                related_concepts: serde_json::from_str(&row.get::<_, String>(6)?)?,
                tags: serde_json::from_str(&row.get::<_, String>(7)?)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Store a pattern
    pub fn store_pattern(&self, pattern: &KnowledgePattern) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO patterns (id, name, description, template, when_to_use, when_not_to_use, examples)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                pattern.id,
                pattern.name,
                pattern.description,
                pattern.template,
                pattern.when_to_use,
                pattern.when_not_to_use,
                serde_json::to_string(&pattern.examples)?,
            ],
        )?;

        Ok(())
    }

    /// Get pattern by ID
    pub fn get_pattern(&self, id: &str) -> Result<Option<KnowledgePattern>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, template, when_to_use, when_not_to_use, examples
             FROM patterns WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(KnowledgePattern {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                template: row.get(3)?,
                when_to_use: row.get(4)?,
                when_not_to_use: row.get(5)?,
                examples: serde_json::from_str(&row.get::<_, String>(6)?)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Store an error
    pub fn store_error(&self, error: &KnowledgeError) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO errors (error_code, title, explanation, example_bad, example_good, fix_strategies)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                error.error_code,
                error.title,
                error.explanation,
                error.example_bad,
                error.example_good,
                serde_json::to_string(&error.fix_strategies)?,
            ],
        )?;

        Ok(())
    }

    /// Get error by error code
    pub fn get_error(&self, error_code: &str) -> Result<Option<KnowledgeError>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT error_code, title, explanation, example_bad, example_good, fix_strategies
             FROM errors WHERE error_code = ?1",
        )?;

        let mut rows = stmt.query([error_code])?;

        if let Some(row) = rows.next()? {
            Ok(Some(KnowledgeError {
                error_code: row.get(0)?,
                title: row.get(1)?,
                explanation: row.get(2)?,
                example_bad: row.get(3)?,
                example_good: row.get(4)?,
                fix_strategies: serde_json::from_str(&row.get::<_, String>(5)?)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Store a command
    pub fn store_command(&self, command: &KnowledgeCommand) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO commands (tool, command, description, flags, examples)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                command.tool,
                command.command,
                command.description,
                serde_json::to_string(&command.flags)?,
                serde_json::to_string(&command.examples)?,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Count total concepts
    pub fn count_concepts(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM concepts", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Count total patterns
    pub fn count_patterns(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM patterns", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Count total errors
    pub fn count_errors(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM errors", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Count total commands
    pub fn count_commands(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM commands", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_database() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        assert_eq!(db.count_concepts().unwrap(), 0);
        assert_eq!(db.count_patterns().unwrap(), 0);
    }

    #[test]
    fn test_store_and_retrieve_concept() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();

        let concept = KnowledgeConcept {
            id: "ownership-move".to_string(),
            topic: "ownership".to_string(),
            title: "Move Semantics".to_string(),
            explanation: "How Rust transfers ownership between variables".to_string(),
            code_examples: vec![
                CodeExample {
                    title: "Basic move".to_string(),
                    code: "let s1 = String::from(\"hello\");\nlet s2 = s1;".to_string(),
                    explanation: "s1 is moved to s2".to_string(),
                }
            ],
            common_mistakes: vec!["Using value after move".to_string()],
            related_concepts: vec!["borrowing".to_string()],
            tags: vec!["ownership".to_string(), "memory".to_string()],
        };

        db.store_concept(&concept).unwrap();
        assert_eq!(db.count_concepts().unwrap(), 1);

        let retrieved = db.get_concept("ownership-move").unwrap().unwrap();
        assert_eq!(retrieved.title, "Move Semantics");
        assert_eq!(retrieved.code_examples.len(), 1);
    }

    #[test]
    fn test_store_and_retrieve_pattern() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();

        let pattern = KnowledgePattern {
            id: "builder".to_string(),
            name: "Builder Pattern".to_string(),
            description: "Construct complex objects step by step".to_string(),
            template: "impl Builder { pub fn new() -> Self { ... } }".to_string(),
            when_to_use: "When constructing objects with many optional parameters".to_string(),
            when_not_to_use: "For simple structs with few fields".to_string(),
            examples: vec![],
        };

        db.store_pattern(&pattern).unwrap();
        assert_eq!(db.count_patterns().unwrap(), 1);

        let retrieved = db.get_pattern("builder").unwrap().unwrap();
        assert_eq!(retrieved.name, "Builder Pattern");
    }

    #[test]
    fn test_store_and_retrieve_error() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();

        let error = KnowledgeError {
            error_code: "E0382".to_string(),
            title: "Use of moved value".to_string(),
            explanation: "Value used after being moved".to_string(),
            example_bad: "let s1 = String::from(\"hello\");\nlet s2 = s1;\nprintln!(\"{}\", s1);".to_string(),
            example_good: "let s1 = String::from(\"hello\");\nlet s2 = s1.clone();\nprintln!(\"{}\", s1);".to_string(),
            fix_strategies: vec!["Use clone()".to_string(), "Use references".to_string()],
        };

        db.store_error(&error).unwrap();
        assert_eq!(db.count_errors().unwrap(), 1);

        let retrieved = db.get_error("E0382").unwrap().unwrap();
        assert_eq!(retrieved.title, "Use of moved value");
    }
}
