mod learning_agent;
mod types;

use learning_agent::LearningAgent;
use anyhow::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     Self-Learning Agent with Claude Teachers                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("System Design:");
    println!("  🤖 Question Agent (Claude) → questions.txt");
    println!("  🤖 Answer Agent (Claude) → answers.txt");
    println!("  🧠 YOUR Learning Agent → reads files & stores knowledge");
    println!();
    println!("Note: The two Claude teacher agents should be spawned separately");
    println!("      using Claude Code. This is YOUR learning agent that will");
    println!("      monitor the Q&A files and learn from them.");
    println!();

    // Setup paths
    let data_dir = PathBuf::from("data");
    std::fs::create_dir_all(&data_dir)?;

    let questions_file = data_dir.join("questions.txt");
    let answers_file = data_dir.join("answers.txt");
    let knowledge_file = data_dir.join("knowledge_base.json");

    println!("📁 Monitoring files:");
    println!("   - {:?}", questions_file);
    println!("   - {:?}", answers_file);
    println!();
    println!("💾 Saving knowledge to:");
    println!("   - {:?}", knowledge_file);
    println!();
    println!("🧠 Starting learning process...");
    println!("   Waiting for Claude teachers to generate Q&A...");
    println!();

    // Create YOUR learning agent
    let mut learning_agent = LearningAgent::new(
        questions_file,
        answers_file,
        knowledge_file,
    );

    // Run YOUR learning agent
    learning_agent.run().await?;

    Ok(())
}
