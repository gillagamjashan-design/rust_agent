use rust_agent::exercises::{Curriculum, Difficulty};

fn main() {
    println!("=== Rust Learning Curriculum Demo ===\n");

    // Create the curriculum
    let curriculum = Curriculum::new();

    // Overview
    println!("Total Stages: {}", curriculum.stages.len());
    println!("Total Exercises: {}\n", curriculum.exercises.len());

    // Display all stages
    println!("=== CURRICULUM OVERVIEW ===\n");
    for stage in &curriculum.stages {
        println!("Stage {}: {}", stage.id, stage.name);
        println!("  Description: {}", stage.description);
        println!("  Concepts: {}", stage.concepts.join(", "));
        println!("  Exercises: {}", stage.exercise_ids.len());
        println!("  Advancement: {}", stage.advancement_criteria.description);
        println!();
    }

    // Show detailed view of Stage 1
    println!("\n=== STAGE 1 DETAILED VIEW ===\n");
    let stage1_exercises = curriculum.get_exercises_for_stage(1);

    for (i, exercise) in stage1_exercises.iter().enumerate() {
        println!("Exercise {}: {}", i + 1, exercise.title);
        println!("  ID: {}", exercise.id);
        println!("  Difficulty: {:?}", exercise.difficulty);
        println!("  Error Code: {}", exercise.error_code);
        println!("  Description: {}", exercise.description);
        println!("  Concepts: {}", exercise.concepts.join(", "));
        println!();
    }

    // Show a specific exercise in detail
    println!("\n=== EXAMPLE EXERCISE: Multiple Mutable References ===\n");
    if let Some(exercise) = curriculum.get_exercise("stage2_ex1") {
        println!("Title: {}", exercise.title);
        println!("Stage: {}", exercise.stage);
        println!("Difficulty: {:?}", exercise.difficulty);
        println!("Error Code: {}", exercise.error_code);
        println!("\nDescription: {}", exercise.description);

        println!("\nBroken Code:");
        println!("```rust");
        println!("{}", exercise.broken_code);
        println!("```");

        println!("\nExpected Error: {}", exercise.expected_error);

        println!("\nConstraints:");
        for constraint in &exercise.constraints {
            println!("  - {}", constraint);
        }

        println!("\nHints:");
        for (i, hint) in exercise.hints.iter().enumerate() {
            println!("  {}. {}", i + 1, hint);
        }

        println!("\nWorking Solution:");
        println!("```rust");
        println!("{}", exercise.working_solution);
        println!("```");
    }

    // Statistics by difficulty
    println!("\n=== EXERCISE STATISTICS ===\n");
    let mut beginner = 0;
    let mut intermediate = 0;
    let mut advanced = 0;
    let mut expert = 0;

    for exercise in curriculum.exercises.values() {
        match exercise.difficulty {
            Difficulty::Beginner => beginner += 1,
            Difficulty::Intermediate => intermediate += 1,
            Difficulty::Advanced => advanced += 1,
            Difficulty::Expert => expert += 1,
        }
    }

    println!("Beginner: {}", beginner);
    println!("Intermediate: {}", intermediate);
    println!("Advanced: {}", advanced);
    println!("Expert: {}", expert);

    // Save curriculum to JSON
    println!("\n=== SAVING CURRICULUM ===\n");
    match curriculum.save_to_json("curriculum_data.json") {
        Ok(_) => println!("Curriculum saved to curriculum_data.json"),
        Err(e) => println!("Error saving curriculum: {}", e),
    }

    // Show all error codes
    println!("\n=== ERROR CODES COVERED ===\n");
    let mut error_codes: Vec<String> = curriculum
        .exercises
        .values()
        .map(|e| e.error_code.clone())
        .collect();
    error_codes.sort();
    error_codes.dedup();

    for code in error_codes {
        let count = curriculum
            .exercises
            .values()
            .filter(|e| e.error_code == code)
            .count();
        println!("{}: {} exercises", code, count);
    }
}
