// Knowledge fetcher - Agent tool for querying knowledge database at runtime

use crate::knowledge::{KnowledgeQuery, SearchResults};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Request types for knowledge fetching
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KnowledgeFetchRequest {
    /// "What is ownership in Rust?"
    ExplainConcept { topic: String },

    /// "How do I implement the builder pattern?"
    FindPattern { use_case: String },

    /// "What does E0382 mean?"
    ExplainError { error_code: String },

    /// "How do I use cargo test?"
    FindCommand { tool: String, action: String },

    /// General search across all knowledge
    Search { query: String },
}

/// Response from knowledge fetcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeResponse {
    pub request: KnowledgeFetchRequest,
    pub results: SearchResults,
    pub formatted: String,
    pub confidence: f32, // 0.0-1.0, based on number and quality of results
}

impl KnowledgeResponse {
    /// Check if response has useful results
    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    /// Get confidence score based on results
    fn calculate_confidence(results: &SearchResults) -> f32 {
        let total = results.total();
        if total == 0 {
            0.0
        } else if total >= 5 {
            0.9
        } else {
            0.5 + (total as f32 * 0.08)
        }
    }
}

/// Knowledge fetcher tool
pub struct KnowledgeFetcher {
    query: KnowledgeQuery,
}

impl KnowledgeFetcher {
    /// Create new knowledge fetcher
    pub fn new(query: KnowledgeQuery) -> Self {
        Self { query }
    }

    /// Fetch knowledge based on request
    pub fn fetch(&self, request: KnowledgeFetchRequest) -> Result<KnowledgeResponse> {
        let results = match &request {
            KnowledgeFetchRequest::ExplainConcept { topic } => {
                // Search concepts by topic
                let concepts = self.query.search_concepts(topic)?;
                SearchResults {
                    concepts,
                    patterns: vec![],
                    commands: vec![],
                }
            }

            KnowledgeFetchRequest::FindPattern { use_case } => {
                // Find patterns matching the use case
                let patterns = self.query.find_patterns(use_case)?;
                SearchResults {
                    concepts: vec![],
                    patterns,
                    commands: vec![],
                }
            }

            KnowledgeFetchRequest::ExplainError { error_code } => {
                // Look up specific error
                let error = self.query.explain_error(error_code)?;
                if let Some(_err) = error {
                    // Format error as a concept for now
                    SearchResults {
                        concepts: vec![],
                        patterns: vec![],
                        commands: vec![],
                    }
                } else {
                    SearchResults {
                        concepts: vec![],
                        patterns: vec![],
                        commands: vec![],
                    }
                }
            }

            KnowledgeFetchRequest::FindCommand { tool, action } => {
                // Search commands
                let commands = self.query.search_commands(tool, action)?;
                SearchResults {
                    concepts: vec![],
                    patterns: vec![],
                    commands,
                }
            }

            KnowledgeFetchRequest::Search { query } => {
                // Full-text search across all knowledge
                self.query.search_all(query)?
            }
        };

        let formatted = results.format();
        let confidence = KnowledgeResponse::calculate_confidence(&results);

        Ok(KnowledgeResponse {
            request,
            results,
            formatted,
            confidence,
        })
    }

    /// Convenience method: Explain a Rust concept
    pub fn explain_concept(&self, topic: &str) -> Result<KnowledgeResponse> {
        self.fetch(KnowledgeFetchRequest::ExplainConcept {
            topic: topic.to_string(),
        })
    }

    /// Convenience method: Find a pattern
    pub fn find_pattern(&self, use_case: &str) -> Result<KnowledgeResponse> {
        self.fetch(KnowledgeFetchRequest::FindPattern {
            use_case: use_case.to_string(),
        })
    }

    /// Convenience method: Explain an error
    pub fn explain_error(&self, error_code: &str) -> Result<KnowledgeResponse> {
        self.fetch(KnowledgeFetchRequest::ExplainError {
            error_code: error_code.to_string(),
        })
    }

    /// Convenience method: Find a command
    pub fn find_command(&self, tool: &str, action: &str) -> Result<KnowledgeResponse> {
        self.fetch(KnowledgeFetchRequest::FindCommand {
            tool: tool.to_string(),
            action: action.to_string(),
        })
    }

    /// Convenience method: General search
    pub fn search(&self, query: &str) -> Result<KnowledgeResponse> {
        self.fetch(KnowledgeFetchRequest::Search {
            query: query.to_string(),
        })
    }
}

/// Confidence-based decision helper
pub struct ConfidenceDecision {
    threshold_high: f32,
    threshold_low: f32,
}

impl Default for ConfidenceDecision {
    fn default() -> Self {
        Self {
            threshold_high: 0.7,
            threshold_low: 0.4,
        }
    }
}

impl ConfidenceDecision {
    /// Determine if agent should fetch from knowledge database
    pub fn should_fetch(&self, confidence: f32) -> bool {
        confidence < self.threshold_high
    }

    /// Determine if agent can answer directly
    pub fn can_answer_directly(&self, confidence: f32) -> bool {
        confidence >= self.threshold_high
    }

    /// Determine if agent needs verification
    pub fn needs_verification(&self, confidence: f32) -> bool {
        confidence >= self.threshold_low && confidence < self.threshold_high
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::{KnowledgeDatabase, KnowledgeQuery};
    use crate::knowledge::database::{KnowledgeConcept, KnowledgePattern};

    #[test]
    fn test_fetch_concept() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        let query = KnowledgeQuery::new(db);
        let fetcher = KnowledgeFetcher::new(query);

        // Add test concept
        fetcher.query.db.store_concept(&KnowledgeConcept {
            id: "ownership-test".to_string(),
            topic: "ownership".to_string(),
            title: "Ownership Rules".to_string(),
            explanation: "Test explanation".to_string(),
            code_examples: vec![],
            common_mistakes: vec![],
            related_concepts: vec![],
            tags: vec!["ownership".to_string()],
        }).unwrap();

        let response = fetcher.explain_concept("ownership").unwrap();
        assert!(response.has_results());
        assert!(response.confidence > 0.0);
    }

    #[test]
    fn test_fetch_pattern() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        let query = KnowledgeQuery::new(db);
        let fetcher = KnowledgeFetcher::new(query);

        // Add test pattern
        fetcher.query.db.store_pattern(&KnowledgePattern {
            id: "builder".to_string(),
            name: "Builder Pattern".to_string(),
            description: "Builder pattern for construction".to_string(),
            template: "...".to_string(),
            when_to_use: "For complex objects".to_string(),
            when_not_to_use: "".to_string(),
            examples: vec![],
        }).unwrap();

        let response = fetcher.find_pattern("builder").unwrap();
        assert!(response.has_results());
    }

    #[test]
    fn test_confidence_decision() {
        let decision = ConfidenceDecision::default();

        assert!(decision.can_answer_directly(0.8));
        assert!(!decision.can_answer_directly(0.6));

        assert!(decision.should_fetch(0.6));
        assert!(!decision.should_fetch(0.8));

        assert!(decision.needs_verification(0.5));
        assert!(!decision.needs_verification(0.3));
        assert!(!decision.needs_verification(0.8));
    }
}
