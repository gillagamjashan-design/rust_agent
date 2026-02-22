use crate::exercises::{Curriculum, Exercise};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseProgress {
    pub exercise_id: String,
    pub stage: u8,
    pub attempts: usize,
    pub completed: bool,
    pub last_attempt: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgress {
    pub current_stage: u8,
    pub exercises_completed: HashMap<String, ExerciseProgress>,
    pub stage_completion: HashMap<u8, StageProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageProgress {
    pub stage_id: u8,
    pub exercises_attempted: usize,
    pub exercises_completed: usize,
    pub concepts_learned: Vec<String>,
    pub ready_to_advance: bool,
}

pub struct ExerciseManager {
    curriculum: Curriculum,
    progress: LearningProgress,
    progress_file: PathBuf,
}

impl ExerciseManager {
    pub fn new(progress_file: PathBuf) -> Self {
        let curriculum = Curriculum::new();
        let progress = Self::load_progress(&progress_file).unwrap_or_else(|_| {
            LearningProgress {
                current_stage: 1,
                exercises_completed: HashMap::new(),
                stage_completion: HashMap::new(),
            }
        });

        Self {
            curriculum,
            progress,
            progress_file,
        }
    }

    pub fn get_current_stage_exercises(&self) -> Vec<&Exercise> {
        self.curriculum
            .get_exercises_for_stage(self.progress.current_stage)
    }

    pub fn get_next_exercise(&self) -> Option<&Exercise> {
        let exercises = self.get_current_stage_exercises();

        // Find first uncompleted exercise
        exercises.into_iter().find(|ex| {
            !self
                .progress
                .exercises_completed
                .get(&ex.id)
                .map(|p| p.completed)
                .unwrap_or(false)
        })
    }

    pub fn record_attempt(&mut self, exercise_id: &str, code_attempt: String, success: bool) {
        let exercise = self.curriculum.get_exercise(exercise_id);
        if exercise.is_none() {
            return;
        }

        let stage = exercise.unwrap().stage;

        let progress = self
            .progress
            .exercises_completed
            .entry(exercise_id.to_string())
            .or_insert(ExerciseProgress {
                exercise_id: exercise_id.to_string(),
                stage,
                attempts: 0,
                completed: false,
                last_attempt: None,
                timestamp: chrono::Utc::now(),
            });

        progress.attempts += 1;
        progress.last_attempt = Some(code_attempt);
        progress.timestamp = chrono::Utc::now();

        if success {
            progress.completed = true;
            self.update_stage_progress(stage);
        }

        self.save_progress().ok();
    }

    fn update_stage_progress(&mut self, stage_id: u8) {
        let stage_exercises = self.curriculum.get_exercises_for_stage(stage_id);
        let completed = stage_exercises
            .iter()
            .filter(|ex| {
                self.progress
                    .exercises_completed
                    .get(&ex.id)
                    .map(|p| p.completed)
                    .unwrap_or(false)
            })
            .count();

        let attempted = stage_exercises
            .iter()
            .filter(|ex| self.progress.exercises_completed.contains_key(&ex.id))
            .count();

        // Collect all concepts from completed exercises
        let mut concepts_learned = Vec::new();
        for ex in stage_exercises.iter() {
            if self
                .progress
                .exercises_completed
                .get(&ex.id)
                .map(|p| p.completed)
                .unwrap_or(false)
            {
                for concept in &ex.concepts {
                    if !concepts_learned.contains(concept) {
                        concepts_learned.push(concept.clone());
                    }
                }
            }
        }

        // Check advancement criteria
        let stage = self.curriculum.get_stage(stage_id);
        let ready_to_advance = if let Some(stage) = stage {
            let criteria = &stage.advancement_criteria;
            let min_completed = completed >= criteria.min_exercises_completed;
            let has_required_concepts = criteria
                .required_concepts
                .iter()
                .all(|c| concepts_learned.contains(c));

            min_completed && has_required_concepts
        } else {
            false
        };

        let stage_progress = StageProgress {
            stage_id,
            exercises_attempted: attempted,
            exercises_completed: completed,
            concepts_learned,
            ready_to_advance,
        };

        self.progress
            .stage_completion
            .insert(stage_id, stage_progress);
    }

    pub fn can_advance(&self) -> bool {
        self.progress
            .stage_completion
            .get(&self.progress.current_stage)
            .map(|p| p.ready_to_advance)
            .unwrap_or(false)
    }

    pub fn advance_stage(&mut self) -> Result<u8, String> {
        if !self.can_advance() {
            return Err("Not ready to advance yet. Complete more exercises.".to_string());
        }

        if self.progress.current_stage >= 5 {
            return Err("Already at the final stage!".to_string());
        }

        self.progress.current_stage += 1;
        self.save_progress()
            .map_err(|e| format!("Failed to save progress: {}", e))?;

        Ok(self.progress.current_stage)
    }

    pub fn get_progress_summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str(&format!(
            "Current Stage: {} - {}\n",
            self.progress.current_stage,
            self.curriculum
                .get_stage(self.progress.current_stage)
                .map(|s| s.name.as_str())
                .unwrap_or("Unknown")
        ));

        if let Some(stage_progress) = self
            .progress
            .stage_completion
            .get(&self.progress.current_stage)
        {
            summary.push_str(&format!(
                "Progress: {}/{} exercises completed\n",
                stage_progress.exercises_completed, stage_progress.exercises_attempted
            ));
            summary.push_str(&format!(
                "Concepts Learned: {}\n",
                stage_progress.concepts_learned.join(", ")
            ));
            summary.push_str(&format!(
                "Ready to Advance: {}\n",
                if stage_progress.ready_to_advance {
                    "Yes"
                } else {
                    "No"
                }
            ));
        }

        summary
    }

    pub fn generate_learning_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== RUST LEARNING PROGRESS REPORT ===\n\n");

        for stage_id in 1..=5 {
            if let Some(stage) = self.curriculum.get_stage(stage_id) {
                report.push_str(&format!("Stage {}: {}\n", stage_id, stage.name));

                if let Some(progress) = self.progress.stage_completion.get(&stage_id) {
                    report.push_str(&format!(
                        "  Exercises: {}/{}\n",
                        progress.exercises_completed,
                        self.curriculum.get_exercises_for_stage(stage_id).len()
                    ));
                    report.push_str(&format!(
                        "  Concepts: {}\n",
                        progress.concepts_learned.join(", ")
                    ));
                } else {
                    report.push_str("  Not started\n");
                }
                report.push('\n');
            }
        }

        report
    }

    pub fn save_exercise_to_file(&self, exercise_id: &str, output_dir: &PathBuf) -> std::io::Result<PathBuf> {
        if let Some(exercise) = self.curriculum.get_exercise(exercise_id) {
            let filename = format!("{}.rs", exercise_id);
            let filepath = output_dir.join(filename);

            let mut file = File::create(&filepath)?;

            writeln!(file, "// {}", exercise.title)?;
            writeln!(file, "// Stage: {}", exercise.stage)?;
            writeln!(file, "// Difficulty: {:?}", exercise.difficulty)?;
            writeln!(file, "// Expected Error: {}", exercise.error_code)?;
            writeln!(file, "//")?;
            writeln!(file, "// Description: {}", exercise.description)?;
            writeln!(file, "//")?;
            writeln!(file, "// Constraints:")?;
            for constraint in &exercise.constraints {
                writeln!(file, "//   - {}", constraint)?;
            }
            writeln!(file, "//")?;
            writeln!(file, "// Hints:")?;
            for (i, hint) in exercise.hints.iter().enumerate() {
                writeln!(file, "//   {}. {}", i + 1, hint)?;
            }
            writeln!(file)?;
            writeln!(file, "{}", exercise.broken_code)?;

            Ok(filepath)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Exercise not found",
            ))
        }
    }

    fn load_progress(path: &PathBuf) -> Result<LearningProgress, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let progress = serde_json::from_str(&json)?;
        Ok(progress)
    }

    fn save_progress(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.progress)?;
        std::fs::write(&self.progress_file, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_exercise_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let progress_file = temp_dir.path().join("progress.json");

        let manager = ExerciseManager::new(progress_file);
        assert_eq!(manager.progress.current_stage, 1);
    }

    #[test]
    fn test_get_next_exercise() {
        let temp_dir = TempDir::new().unwrap();
        let progress_file = temp_dir.path().join("progress.json");

        let manager = ExerciseManager::new(progress_file);
        let next = manager.get_next_exercise();
        assert!(next.is_some());
        assert_eq!(next.unwrap().stage, 1);
    }

    #[test]
    fn test_record_attempt() {
        let temp_dir = TempDir::new().unwrap();
        let progress_file = temp_dir.path().join("progress.json");

        let mut manager = ExerciseManager::new(progress_file);
        manager.record_attempt("stage1_ex1", "fn main() {}".to_string(), true);

        assert!(manager
            .progress
            .exercises_completed
            .get("stage1_ex1")
            .unwrap()
            .completed);
    }
}
