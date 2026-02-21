use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub category: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question_id: usize,
    pub text: String,
    pub code_examples: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAPair {
    pub question: Question,
    pub answer: Answer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub code_pattern: String,
    pub usage_examples: Vec<String>,
    pub confidence: f32,
    pub occurrences: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub version: String,
    pub qa_pairs: Vec<QAPair>,
    pub patterns: HashMap<String, Pattern>,
    pub topics_covered: Vec<String>,
    pub last_updated: DateTime<Utc>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            qa_pairs: Vec::new(),
            patterns: HashMap::new(),
            topics_covered: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    pub fn add_qa_pair(&mut self, qa_pair: QAPair) {
        self.qa_pairs.push(qa_pair);
        self.last_updated = Utc::now();
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        let name = pattern.name.clone();
        self.patterns.insert(name, pattern);
        self.last_updated = Utc::now();
    }

    pub fn add_topic(&mut self, topic: String) {
        if !self.topics_covered.contains(&topic) {
            self.topics_covered.push(topic);
        }
    }

    pub fn get_total_knowledge_count(&self) -> usize {
        self.qa_pairs.len() + self.patterns.len()
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export web search types for convenience
pub use crate::web_search::{SearchProvider, SearchQuery, SearchResponse, SearchResult};
