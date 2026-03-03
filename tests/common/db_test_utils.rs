//! Test utilities for knowledge database testing

use rust_agent::knowledge::database::{
    KnowledgeDatabase, KnowledgeConcept, KnowledgePattern, KnowledgeError, CodeExample
};
use std::time::{Duration, Instant};

/// Creates an in-memory test database
pub fn create_test_db() -> KnowledgeDatabase {
    KnowledgeDatabase::new_in_memory().expect("Failed to create in-memory database")
}

/// Populates the database with sample concepts for testing
pub fn populate_sample_concepts(db: &KnowledgeDatabase, count: usize) {
    for i in 0..count {
        let concept = KnowledgeConcept {
            id: format!("test_concept_{}", i),
            topic: format!("topic_{}", i % 10), // Group into 10 topics
            title: format!("Test Concept {}", i),
            explanation: format!("This is a detailed explanation for concept {}. It covers ownership, borrowing, and lifetimes.", i),
            code_examples: vec![
                CodeExample {
                    title: format!("Example {}", i),
                    code: format!("fn example_{}() {{\n    println!(\"Test\");\n}}", i),
                    explanation: format!("Example for concept {}", i),
                }
            ],
            common_mistakes: vec![
                format!("Common mistake {} for concept {}", 1, i),
                format!("Common mistake {} for concept {}", 2, i),
            ],
            related_concepts: vec![
                format!("related_concept_{}", (i + 1) % count),
                format!("related_concept_{}", (i + 2) % count),
            ],
            tags: vec![
                "memory".to_string(),
                "safety".to_string(),
                format!("tag_{}", i % 5),
            ],
        };

        db.store_concept(&concept)
            .expect("Failed to store concept");
    }
}

/// Populates the database with sample patterns for testing
pub fn populate_sample_patterns(db: &KnowledgeDatabase, count: usize) {
    for i in 0..count {
        let pattern = KnowledgePattern {
            id: format!("test_pattern_{}", i),
            name: format!("Test Pattern {}", i),
            description: format!("Description for pattern {}", i),
            template: format!("impl Pattern{} {{\n    // implementation\n}}", i),
            when_to_use: format!("Use this pattern when condition {}", i),
            when_not_to_use: format!("Avoid when condition {}", i),
            examples: vec![
                CodeExample {
                    title: "Example".to_string(),
                    code: format!("// Example code for pattern {}", i),
                    explanation: "Example description".to_string(),
                }
            ],
        };

        db.store_pattern(&pattern)
            .expect("Failed to store pattern");
    }
}

/// Populates the database with sample errors for testing
pub fn populate_sample_errors(db: &KnowledgeDatabase, count: usize) {
    for i in 0..count {
        let error = KnowledgeError {
            error_code: format!("E{:04}", i),
            title: format!("Error {}", i),
            explanation: format!("Detailed explanation for error {}", i),
            example_bad: format!("// Code that causes error {}", i),
            example_good: format!("// Fixed code for error {}", i),
            fix_strategies: vec![
                format!("Fix strategy 1 for error {}", i),
                format!("Fix strategy 2 for error {}", i),
            ],
        };

        db.store_error(&error)
            .expect("Failed to store error");
    }
}

/// Measures the execution time of a function
pub fn measure_query_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Creates a test concept with specific content for search testing
pub fn create_test_concept(
    id: &str,
    topic: &str,
    title: &str,
    explanation: &str,
) -> KnowledgeConcept {
    KnowledgeConcept {
        id: id.to_string(),
        topic: topic.to_string(),
        title: title.to_string(),
        explanation: explanation.to_string(),
        code_examples: vec![],
        common_mistakes: vec![],
        related_concepts: vec![],
        tags: vec![],
    }
}

/// Creates a test pattern with specific content
pub fn create_test_pattern(
    id: &str,
    name: &str,
    description: &str,
) -> KnowledgePattern {
    KnowledgePattern {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        template: "// template code".to_string(),
        when_to_use: "when needed".to_string(),
        when_not_to_use: "when not needed".to_string(),
        examples: vec![],
    }
}
