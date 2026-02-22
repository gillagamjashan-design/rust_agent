use rust_agent::exercise_integration::ExerciseManager;
use std::path::PathBuf;

fn main() {
    println!("=== Rust Learning Agent - Exercise Integration ===\n");

    // Initialize exercise manager with progress tracking
    let data_dir = PathBuf::from("./data");
    std::fs::create_dir_all(&data_dir).ok();

    let progress_file = data_dir.join("exercise_progress.json");
    let mut manager = ExerciseManager::new(progress_file);

    // Show current progress
    println!("Current Learning Status:");
    println!("{}\n", manager.get_progress_summary());

    // Get next exercise to work on
    match manager.get_next_exercise() {
        Some(exercise) => {
            println!("=== NEXT EXERCISE ===");
            println!("Title: {}", exercise.title);
            println!("Stage: {}", exercise.stage);
            println!("Difficulty: {:?}", exercise.difficulty);
            println!("\nDescription: {}", exercise.description);
            println!("\nExpected Error: {} ({})", exercise.expected_error, exercise.error_code);

            println!("\nBroken Code:");
            println!("─".repeat(60));
            println!("{}", exercise.broken_code);
            println!("─".repeat(60));

            println!("\nHints:");
            for (i, hint) in exercise.hints.iter().enumerate() {
                println!("  {}. {}", i + 1, hint);
            }

            println!("\nConstraints:");
            for constraint in &exercise.constraints {
                println!("  - {}", constraint);
            }

            // Simulate saving the exercise to a file
            let exercises_dir = data_dir.join("exercises");
            std::fs::create_dir_all(&exercises_dir).ok();

            match manager.save_exercise_to_file(&exercise.id, &exercises_dir) {
                Ok(path) => println!("\nExercise saved to: {}", path.display()),
                Err(e) => println!("\nError saving exercise: {}", e),
            }

            // Simulate attempting the exercise
            println!("\n=== SIMULATION: Attempting Exercise ===");

            // First attempt - wrong solution
            println!("\nAttempt 1: Incorrect solution");
            let wrong_attempt = r#"fn main() {
    let s = String::from("hello");
    println!("{}", s);
}"#;
            manager.record_attempt(&exercise.id, wrong_attempt.to_string(), false);
            println!("Recorded failed attempt");

            // Second attempt - correct solution
            println!("\nAttempt 2: Correct solution");
            manager.record_attempt(&exercise.id, exercise.working_solution.clone(), true);
            println!("Recorded successful attempt!");

            println!("\n{}", manager.get_progress_summary());
        }
        None => {
            println!("No more exercises in current stage!");

            if manager.can_advance() {
                println!("\nYou're ready to advance to the next stage!");
                match manager.advance_stage() {
                    Ok(new_stage) => println!("Advanced to Stage {}", new_stage),
                    Err(e) => println!("Error advancing: {}", e),
                }
            } else {
                println!("\nComplete more exercises to advance.");
            }
        }
    }

    // Show full learning report
    println!("\n{}", "=".repeat(60));
    println!("{}", manager.generate_learning_report());
    println!("{}", "=".repeat(60));

    // Show all current stage exercises
    println!("\nAll Exercises in Current Stage:");
    for (i, ex) in manager.get_current_stage_exercises().iter().enumerate() {
        println!("  {}. {} ({:?})", i + 1, ex.title, ex.difficulty);
    }

    println!("\n=== Example Workflow ===");
    println!("1. Agent reads next exercise");
    println!("2. Agent presents broken code to student");
    println!("3. Student attempts to fix the code");
    println!("4. Agent records attempt and provides feedback");
    println!("5. On success, move to next exercise");
    println!("6. After completing enough exercises, advance to next stage");
}
