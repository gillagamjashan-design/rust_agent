use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents a knowledge source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSource {
    pub file: String,
    pub topics: Vec<String>,
    pub priority: u8,
    pub description: String,
}

/// Learning stage with objectives and topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStage {
    pub stage: u8,
    pub name: String,
    pub duration_estimate: String,
    pub objectives: Vec<String>,
    pub topics: Vec<StageTopic>,
    pub exercises: Vec<String>,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageTopic {
    pub topic: String,
    pub subtopics: Vec<String>,
    pub source: String,
}

/// Master curriculum structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterCurriculum {
    pub title: String,
    pub version: String,
    pub description: String,
    pub knowledge_sources: Vec<KnowledgeSource>,
    pub learning_stages: Vec<LearningStage>,
}

/// Knowledge-based learning system
pub struct KnowledgeBasedLearning {
    curriculum: MasterCurriculum,
    knowledge_dir: PathBuf,
    current_stage: u8,
    topics_mastered: HashMap<String, usize>,
}

impl KnowledgeBasedLearning {
    /// Load the knowledge base from disk
    pub fn load(knowledge_dir: PathBuf) -> Result<Self> {
        let curriculum_path = knowledge_dir.join("curriculum_master.json");

        let curriculum_content = fs::read_to_string(&curriculum_path)
            .with_context(|| format!("Failed to read curriculum from {:?}", curriculum_path))?;

        let curriculum: MasterCurriculum = serde_json::from_str(&curriculum_content)
            .context("Failed to parse curriculum JSON")?;

        Ok(Self {
            curriculum,
            knowledge_dir,
            current_stage: 1,
            topics_mastered: HashMap::new(),
        })
    }

    /// Get the current learning stage
    pub fn get_current_stage(&self) -> Option<&LearningStage> {
        self.curriculum
            .learning_stages
            .iter()
            .find(|s| s.stage == self.current_stage)
    }

    /// Get all topics for the current stage
    pub fn get_current_stage_topics(&self) -> Vec<String> {
        if let Some(stage) = self.get_current_stage() {
            stage.topics.iter().map(|t| t.topic.clone()).collect()
        } else {
            Vec::new()
        }
    }

    /// Get a random topic from the current stage
    pub fn get_random_topic(&self) -> Option<String> {
        let topics = self.get_current_stage_topics();
        if topics.is_empty() {
            return None;
        }

        // Simple rotation based on mastery
        let least_mastered = topics
            .iter()
            .min_by_key(|t| self.topics_mastered.get(*t).unwrap_or(&0))
            .cloned();

        least_mastered
    }

    /// Mark a topic as practiced
    pub fn mark_topic_practiced(&mut self, topic: &str) {
        *self.topics_mastered.entry(topic.to_string()).or_insert(0) += 1;
    }

    /// Load content from a specific knowledge file
    pub fn load_knowledge_file(&self, filename: &str) -> Result<serde_json::Value> {
        let file_path = self.knowledge_dir.join(filename);
        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read knowledge file {:?}", file_path))?;

        let json: serde_json::Value = serde_json::from_str(&content)
            .context("Failed to parse knowledge JSON")?;

        Ok(json)
    }

    /// Generate a question prompt for the current topic
    pub fn generate_question_prompt(&self, topic: &str) -> String {
        let stage = self.current_stage;

        format!(
            "You are teaching Rust programming. Generate a focused question about: {}\n\
             \n\
             Context:\n\
             - Learning Stage: {} ({})\n\
             - Topic: {}\n\
             - Focus: Rust ONLY (no other languages, no general programming)\n\
             \n\
             Generate ONE specific question about {}. The question should:\n\
             1. Test understanding of this specific Rust concept\n\
             2. Be practical and applicable to real Rust code\n\
             3. Include a code example if relevant\n\
             4. Be appropriate for Stage {} level\n\
             \n\
             Return ONLY the question text, no preamble.",
            topic,
            stage,
            self.get_current_stage().map(|s| s.name.as_str()).unwrap_or("Unknown"),
            topic,
            topic,
            stage
        )
    }

    /// Generate an answer prompt with knowledge base context
    pub fn generate_answer_prompt(&self, question: &str, topic: &str) -> String {
        format!(
            "You are teaching Rust programming. Answer this question comprehensively:\n\
             \n\
             Question: {}\n\
             \n\
             Topic: {}\n\
             Learning Stage: {}\n\
             \n\
             Provide a detailed answer that:\n\
             1. Explains the concept clearly\n\
             2. Includes code examples\n\
             3. Focuses exclusively on Rust (no other languages)\n\
             4. Is practical and applicable\n\
             5. Uses proper Rust terminology\n\
             \n\
             Format your answer with:\n\
             - Clear explanation\n\
             - Code examples in Rust\n\
             - Key points to remember\n\
             \n\
             Answer:",
            question,
            topic,
            self.current_stage
        )
    }

    /// Get learning progress summary
    pub fn get_progress_summary(&self) -> String {
        let total_stages = self.curriculum.learning_stages.len();
        let current_stage = self.get_current_stage();

        let stage_name = current_stage
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");

        let topics_count = current_stage
            .map(|s| s.topics.len())
            .unwrap_or(0);

        let practiced_count = self.topics_mastered.len();

        format!(
            "Stage {}/{}: {} | Topics practiced: {}/{}",
            self.current_stage,
            total_stages,
            stage_name,
            practiced_count,
            topics_count
        )
    }

    /// Check if ready to advance to next stage
    pub fn should_advance_stage(&self) -> bool {
        if let Some(stage) = self.get_current_stage() {
            let total_topics = stage.topics.len();
            let practiced_topics: usize = stage
                .topics
                .iter()
                .filter(|t| self.topics_mastered.get(&t.topic).unwrap_or(&0) >= &3)
                .count();

            // Advance if practiced 80% of topics at least 3 times each
            practiced_topics >= (total_topics * 80 / 100)
        } else {
            false
        }
    }

    /// Advance to next stage
    pub fn advance_stage(&mut self) -> bool {
        if self.current_stage < self.curriculum.learning_stages.len() as u8 {
            self.current_stage += 1;
            println!(
                "\n🎓 Advanced to Stage {}: {}\n",
                self.current_stage,
                self.get_current_stage().map(|s| s.name.as_str()).unwrap_or("Unknown")
            );
            true
        } else {
            false
        }
    }

    /// Get current stage number
    pub fn get_stage_number(&self) -> u8 {
        self.current_stage
    }

    /// Get statistics
    pub fn get_statistics(&self) -> KnowledgeStatistics {
        KnowledgeStatistics {
            current_stage: self.current_stage,
            total_stages: self.curriculum.learning_stages.len() as u8,
            topics_practiced: self.topics_mastered.len(),
            total_practice_count: self.topics_mastered.values().sum(),
            stage_name: self
                .get_current_stage()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeStatistics {
    pub current_stage: u8,
    pub total_stages: u8,
    pub topics_practiced: usize,
    pub total_practice_count: usize,
    pub stage_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curriculum_structure() {
        // This test would load actual curriculum if run in project
        let curriculum = MasterCurriculum {
            title: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test curriculum".to_string(),
            knowledge_sources: vec![],
            learning_stages: vec![LearningStage {
                stage: 1,
                name: "Test Stage".to_string(),
                duration_estimate: "1 week".to_string(),
                objectives: vec!["Learn test".to_string()],
                topics: vec![],
                exercises: vec![],
                success_criteria: vec![],
            }],
        };

        assert_eq!(curriculum.learning_stages.len(), 1);
        assert_eq!(curriculum.learning_stages[0].stage, 1);
    }
}
