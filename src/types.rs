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
pub struct LearningStage {
    pub stage: u8,
    pub name: String,
    pub topics_mastered: Vec<String>,
    pub questions_answered: usize,
    pub started_at: DateTime<Utc>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub version: String,
    pub qa_pairs: Vec<QAPair>,
    pub patterns: HashMap<String, Pattern>,
    pub topics_covered: Vec<String>,
    pub last_updated: DateTime<Utc>,
    pub current_stage: u8,
    pub stages: HashMap<u8, LearningStage>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        let mut stages = HashMap::new();

        // Initialize 5-stage curriculum
        stages.insert(1, LearningStage {
            stage: 1,
            name: "Foundation".to_string(),
            topics_mastered: Vec::new(),
            questions_answered: 0,
            started_at: Utc::now(),
            completed: false,
        });

        stages.insert(2, LearningStage {
            stage: 2,
            name: "Borrowing Mastery".to_string(),
            topics_mastered: Vec::new(),
            questions_answered: 0,
            started_at: Utc::now(),
            completed: false,
        });

        stages.insert(3, LearningStage {
            stage: 3,
            name: "Lifetime Patterns".to_string(),
            topics_mastered: Vec::new(),
            questions_answered: 0,
            started_at: Utc::now(),
            completed: false,
        });

        stages.insert(4, LearningStage {
            stage: 4,
            name: "Advanced Ownership".to_string(),
            topics_mastered: Vec::new(),
            questions_answered: 0,
            started_at: Utc::now(),
            completed: false,
        });

        stages.insert(5, LearningStage {
            stage: 5,
            name: "Systems Thinking".to_string(),
            topics_mastered: Vec::new(),
            questions_answered: 0,
            started_at: Utc::now(),
            completed: false,
        });

        Self {
            version: "2.0".to_string(),
            qa_pairs: Vec::new(),
            patterns: HashMap::new(),
            topics_covered: Vec::new(),
            last_updated: Utc::now(),
            current_stage: 1,
            stages,
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
            self.topics_covered.push(topic.clone());

            // Track topic in stage if it follows Stage_N_Topic format
            if topic.starts_with("Stage_") {
                let parts: Vec<&str> = topic.split('_').collect();
                if parts.len() >= 2 {
                    if let Ok(stage_num) = parts[1].parse::<u8>() {
                        if let Some(stage) = self.stages.get_mut(&stage_num) {
                            stage.topics_mastered.push(topic.clone());
                            stage.questions_answered += 1;

                            // Auto-advance stage after 5 questions
                            if stage.questions_answered >= 5 && !stage.completed {
                                stage.completed = true;
                                if stage_num < 5 {
                                    self.current_stage = stage_num + 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_total_knowledge_count(&self) -> usize {
        self.qa_pairs.len() + self.patterns.len()
    }

    pub fn get_current_stage_info(&self) -> Option<&LearningStage> {
        self.stages.get(&self.current_stage)
    }

    pub fn get_learning_progress(&self) -> String {
        format!(
            "Stage {}/5: {} ({} topics mastered, {} questions answered)",
            self.current_stage,
            self.stages.get(&self.current_stage).map(|s| s.name.as_str()).unwrap_or("Unknown"),
            self.stages.get(&self.current_stage).map(|s| s.topics_mastered.len()).unwrap_or(0),
            self.stages.get(&self.current_stage).map(|s| s.questions_answered).unwrap_or(0)
        )
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export web search types for convenience
pub use crate::web_search::{SearchProvider, SearchQuery, SearchResponse, SearchResult};
