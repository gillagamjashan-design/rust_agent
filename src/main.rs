mod learning_agent;
mod orchestrator;
mod question_agent;
mod answer_agent;
mod types;

use learning_agent::LearningAgent;
use orchestrator::Orchestrator;
use question_agent::QuestionAgent;
use answer_agent::AnswerAgent;

use anyhow::Result;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        Self-Learning Rust Agent System                      ║");
    println!("║        Spawned Claude Teachers + Local Learner              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize orchestrator
    let orchestrator = Orchestrator::new();
    orchestrator.run().await?;

    println!("🚀 Starting teaching and learning system...");
    println!();
    println!("System Components:");
    println!("  🤔 Question Agent - Generates Rust questions (embedded for demo)");
    println!("  💡 Answer Agent   - Provides detailed answers (embedded for demo)");
    println!("  🧠 Learning Agent - YOUR agent that learns by reading files");
    println!();
    println!("Press Ctrl+C to stop the system");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Get file paths from orchestrator
    let questions_file = orchestrator.get_questions_file();
    let answers_file = orchestrator.get_answers_file();
    let knowledge_file = orchestrator.get_knowledge_file();

    // In the full implementation, you would spawn Claude Code agents here
    // using the Task tool. For this demo, we use embedded agents.

    println!("📝 NOTE: In production, this would spawn two Claude Code agents");
    println!("   using the Task tool. Currently using embedded agents for demo.");
    println!();

    // Create teacher agents (these simulate spawned Claude agents)
    let mut question_agent = QuestionAgent::new(questions_file.clone());
    let mut answer_agent = AnswerAgent::new(questions_file.clone(), answers_file.clone());

    // Create YOUR learning agent (this is the agent YOU are building)
    let mut learning_agent = LearningAgent::new(
        questions_file,
        answers_file,
        knowledge_file,
    );

    println!("✅ All agents initialized");
    println!("🎓 Starting teaching session...");
    println!();

    // Spawn teacher agents (simulating spawned Claude agents)
    let question_task = tokio::spawn(async move {
        if let Err(e) = question_agent.run().await {
            eprintln!("❌ Question Agent error: {}", e);
        }
    });

    let answer_task = tokio::spawn(async move {
        if let Err(e) = answer_agent.run().await {
            eprintln!("❌ Answer Agent error: {}", e);
        }
    });

    // Run YOUR learning agent
    let learning_task = tokio::spawn(async move {
        if let Err(e) = learning_agent.run().await {
            eprintln!("❌ Learning Agent error: {}", e);
        }
    });

    // Wait for Ctrl+C
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!();
            println!("═══════════════════════════════════════════════════════════════");
            println!("🛑 Shutdown signal received - stopping all agents...");
            println!("═══════════════════════════════════════════════════════════════");
        }
    }

    // Stop all agents
    question_task.abort();
    answer_task.abort();
    learning_task.abort();

    println!();
    println!("✨ System stopped successfully");
    println!();
    println!("📊 Knowledge saved to: data/knowledge_base.json");
    println!("📝 Questions saved to: data/questions.txt");
    println!("💬 Answers saved to: data/answers.txt");
    println!();
    println!("🎉 Your learning agent has finished its training session!");
    println!();

    Ok(())
}
