mod answer_agent;
mod learning_agent;
mod question_agent;
mod types;

use answer_agent::AnswerAgent;
use learning_agent::LearningAgent;
use question_agent::QuestionAgent;

use anyhow::Result;
use std::path::PathBuf;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        Self-Learning Rust Agent System                      ║");
    println!("║        Three agents teaching and learning                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Starting three concurrent agents:");
    println!("  🤔 Question Agent - Generates Rust questions");
    println!("  💡 Answer Agent   - Provides detailed answers");
    println!("  🧠 Learning Agent - Learns from Q&A pairs");
    println!();
    println!("Press Ctrl+C to stop the learning process");
    println!();

    // Define file paths
    let data_dir = PathBuf::from("data");
    std::fs::create_dir_all(&data_dir)?;

    let questions_file = data_dir.join("questions.txt");
    let answers_file = data_dir.join("answers.txt");
    let knowledge_file = data_dir.join("knowledge_base.json");

    // Create agents
    let mut question_agent = QuestionAgent::new(questions_file.clone());
    let mut answer_agent = AnswerAgent::new(questions_file.clone(), answers_file.clone());
    let mut learning_agent = LearningAgent::new(
        questions_file,
        answers_file,
        knowledge_file,
    );

    // Spawn all three agents as concurrent tasks
    let question_task = tokio::spawn(async move {
        if let Err(e) = question_agent.run().await {
            eprintln!("Question Agent error: {}", e);
        }
    });

    let answer_task = tokio::spawn(async move {
        if let Err(e) = answer_agent.run().await {
            eprintln!("Answer Agent error: {}", e);
        }
    });

    let learning_task = tokio::spawn(async move {
        if let Err(e) = learning_agent.run().await {
            eprintln!("Learning Agent error: {}", e);
        }
    });

    // Wait for Ctrl+C
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!();
            println!("╔══════════════════════════════════════════════════════════════╗");
            println!("║  Shutdown signal received - stopping all agents...          ║");
            println!("╚══════════════════════════════════════════════════════════════╝");
        }
    }

    // Abort all tasks
    question_task.abort();
    answer_task.abort();
    learning_task.abort();

    println!();
    println!("✨ All agents stopped successfully");
    println!("📊 Knowledge has been saved to: data/knowledge_base.json");
    println!("📝 Questions saved to: data/questions.txt");
    println!("💬 Answers saved to: data/answers.txt");
    println!();
    println!("Thank you for using Self-Learning Rust Agent System!");

    Ok(())
}
