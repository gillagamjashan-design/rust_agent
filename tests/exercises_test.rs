use rust_agent::exercises::{Curriculum, Difficulty};
use rust_agent::exercise_integration::ExerciseManager;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_curriculum_has_all_stages() {
    let curriculum = Curriculum::new();
    assert_eq!(curriculum.stages.len(), 5, "Should have 5 stages");
}

#[test]
fn test_curriculum_has_25_exercises() {
    let curriculum = Curriculum::new();
    assert!(
        curriculum.exercises.len() >= 25,
        "Should have at least 25 exercises"
    );
}

#[test]
fn test_each_stage_has_5_exercises() {
    let curriculum = Curriculum::new();

    for stage_id in 1..=5 {
        let exercises = curriculum.get_exercises_for_stage(stage_id);
        assert_eq!(
            exercises.len(),
            5,
            "Stage {} should have 5 exercises",
            stage_id
        );
    }
}

#[test]
fn test_exercise_ids_are_unique() {
    let curriculum = Curriculum::new();
    let mut ids: Vec<String> = curriculum.exercises.keys().cloned().collect();
    let original_len = ids.len();

    ids.sort();
    ids.dedup();

    assert_eq!(
        ids.len(),
        original_len,
        "All exercise IDs should be unique"
    );
}

#[test]
fn test_all_exercises_have_working_solutions() {
    let curriculum = Curriculum::new();

    for (id, exercise) in &curriculum.exercises {
        assert!(
            !exercise.working_solution.is_empty(),
            "Exercise {} should have a working solution",
            id
        );
    }
}

#[test]
fn test_all_exercises_have_broken_code() {
    let curriculum = Curriculum::new();

    for (id, exercise) in &curriculum.exercises {
        assert!(
            !exercise.broken_code.is_empty(),
            "Exercise {} should have broken code",
            id
        );
    }
}

#[test]
fn test_all_exercises_have_hints() {
    let curriculum = Curriculum::new();

    for (id, exercise) in &curriculum.exercises {
        assert!(
            !exercise.hints.is_empty(),
            "Exercise {} should have hints",
            id
        );
    }
}

#[test]
fn test_stage_advancement_criteria() {
    let curriculum = Curriculum::new();

    for stage in &curriculum.stages {
        assert!(
            stage.advancement_criteria.min_exercises_completed > 0,
            "Stage {} should require completing exercises",
            stage.id
        );
        assert!(
            !stage.advancement_criteria.required_concepts.is_empty(),
            "Stage {} should have required concepts",
            stage.id
        );
    }
}

#[test]
fn test_difficulty_progression() {
    let curriculum = Curriculum::new();

    // Stage 1 should have beginner exercises
    let stage1 = curriculum.get_exercises_for_stage(1);
    let beginner_count = stage1
        .iter()
        .filter(|e| matches!(e.difficulty, Difficulty::Beginner))
        .count();
    assert!(beginner_count > 0, "Stage 1 should have beginner exercises");

    // Stage 5 should have advanced/expert exercises
    let stage5 = curriculum.get_exercises_for_stage(5);
    let advanced_count = stage5
        .iter()
        .filter(|e| {
            matches!(e.difficulty, Difficulty::Advanced | Difficulty::Expert)
        })
        .count();
    assert!(
        advanced_count > 0,
        "Stage 5 should have advanced exercises"
    );
}

#[test]
fn test_exercise_manager_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let manager = ExerciseManager::new(progress_file);
    assert_eq!(
        manager.progress.current_stage, 1,
        "Should start at stage 1"
    );
}

#[test]
fn test_get_next_exercise() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let manager = ExerciseManager::new(progress_file);
    let next = manager.get_next_exercise();

    assert!(next.is_some(), "Should have a next exercise");
    assert_eq!(next.unwrap().stage, 1, "First exercise should be stage 1");
}

#[test]
fn test_record_successful_attempt() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let mut manager = ExerciseManager::new(progress_file);
    let exercise_id = "stage1_ex1";

    manager.record_attempt(exercise_id, "fn main() {}".to_string(), true);

    let progress = manager.progress.exercises_completed.get(exercise_id);
    assert!(progress.is_some(), "Should record progress");
    assert!(progress.unwrap().completed, "Should mark as completed");
    assert_eq!(progress.unwrap().attempts, 1, "Should have 1 attempt");
}

#[test]
fn test_record_multiple_attempts() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let mut manager = ExerciseManager::new(progress_file);
    let exercise_id = "stage1_ex1";

    // Failed attempt
    manager.record_attempt(exercise_id, "bad code".to_string(), false);

    // Successful attempt
    manager.record_attempt(exercise_id, "good code".to_string(), true);

    let progress = manager.progress.exercises_completed.get(exercise_id);
    assert_eq!(progress.unwrap().attempts, 2, "Should have 2 attempts");
    assert!(progress.unwrap().completed, "Should be completed");
}

#[test]
fn test_cannot_advance_initially() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let manager = ExerciseManager::new(progress_file);
    assert!(
        !manager.can_advance(),
        "Should not be able to advance without completing exercises"
    );
}

#[test]
fn test_can_advance_after_completing_exercises() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let mut manager = ExerciseManager::new(progress_file);

    // Complete 4 exercises in stage 1 (minimum required)
    for i in 1..=4 {
        let exercise_id = format!("stage1_ex{}", i);
        manager.record_attempt(&exercise_id, "code".to_string(), true);
    }

    assert!(
        manager.can_advance(),
        "Should be able to advance after completing 4 exercises"
    );
}

#[test]
fn test_stage_advancement() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let mut manager = ExerciseManager::new(progress_file);

    // Complete 4 exercises
    for i in 1..=4 {
        let exercise_id = format!("stage1_ex{}", i);
        manager.record_attempt(&exercise_id, "code".to_string(), true);
    }

    let result = manager.advance_stage();
    assert!(result.is_ok(), "Should advance successfully");
    assert_eq!(result.unwrap(), 2, "Should advance to stage 2");
}

#[test]
fn test_save_exercise_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");
    let output_dir = temp_dir.path().join("exercises");
    std::fs::create_dir_all(&output_dir).unwrap();

    let manager = ExerciseManager::new(progress_file);

    let result = manager.save_exercise_to_file("stage1_ex1", &output_dir);
    assert!(result.is_ok(), "Should save exercise to file");

    let filepath = result.unwrap();
    assert!(filepath.exists(), "File should exist");

    let content = std::fs::read_to_string(&filepath).unwrap();
    assert!(content.contains("Basic Ownership Transfer"));
    assert!(content.contains("fn main()"));
}

#[test]
fn test_progress_summary() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let manager = ExerciseManager::new(progress_file);
    let summary = manager.get_progress_summary();

    assert!(summary.contains("Current Stage: 1"));
    assert!(!summary.is_empty(), "Summary should not be empty");
}

#[test]
fn test_learning_report() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("progress.json");

    let mut manager = ExerciseManager::new(progress_file);

    // Complete some exercises
    manager.record_attempt("stage1_ex1", "code".to_string(), true);
    manager.record_attempt("stage1_ex2", "code".to_string(), true);

    let report = manager.generate_learning_report();
    assert!(report.contains("RUST LEARNING PROGRESS REPORT"));
    assert!(report.contains("Stage 1"));
}

#[test]
fn test_curriculum_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("curriculum.json");

    let curriculum = Curriculum::new();

    // Save
    let save_result = curriculum.save_to_json(json_path.to_str().unwrap());
    assert!(save_result.is_ok(), "Should save curriculum");

    // Load
    let loaded = Curriculum::load_from_json(json_path.to_str().unwrap());
    assert!(loaded.is_ok(), "Should load curriculum");

    let loaded = loaded.unwrap();
    assert_eq!(loaded.stages.len(), curriculum.stages.len());
    assert_eq!(loaded.exercises.len(), curriculum.exercises.len());
}

#[test]
fn test_error_code_coverage() {
    let curriculum = Curriculum::new();

    let error_codes: Vec<String> = curriculum
        .exercises
        .values()
        .map(|e| e.error_code.clone())
        .collect();

    // Should cover E0382 (most common)
    assert!(
        error_codes.iter().any(|c| c == "E0382"),
        "Should cover E0382"
    );

    // Should cover E0106 (lifetimes)
    assert!(
        error_codes.iter().any(|c| c == "E0106"),
        "Should cover E0106"
    );

    // Should cover E0277 (trait bounds)
    assert!(
        error_codes.iter().any(|c| c == "E0277"),
        "Should cover E0277"
    );
}

#[test]
fn test_concepts_are_tagged() {
    let curriculum = Curriculum::new();

    for (id, exercise) in &curriculum.exercises {
        assert!(
            !exercise.concepts.is_empty(),
            "Exercise {} should have concepts tagged",
            id
        );
    }
}
