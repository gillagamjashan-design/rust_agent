mod claude_proxy;
mod learning_agent;
mod proxy_agents;
mod types;

use learning_agent::LearningAgent;
use proxy_agents::{ProxyAnswerAgent, ProxyQuestionAgent};

use anyhow::Result;
use std::path::PathBuf;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   Self-Learning Agent with CLIProxyAPI (Claude Max)         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("System:");
    println!("  🤖 Question Agent → CLIProxyAPI → questions.txt");
    println!("  🤖 Answer Agent   → CLIProxyAPI → answers.txt");
    println!("  🧠 Learning Agent → reads files → knowledge_base.json");
    println!();
    println!("Using: Claude Max subscription via CLIProxyAPI");
    println!("Proxy: http://localhost:8000");
    println!();
    println!("Press Ctrl+C to stop");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Setup paths
    let data_dir = PathBuf::from("data");
    std::fs::create_dir_all(&data_dir)?;

    let questions_file = data_dir.join("questions.txt");
    let answers_file = data_dir.join("answers.txt");
    let knowledge_file = data_dir.join("knowledge_base.json");

    // Create agents
    let mut question_agent = ProxyQuestionAgent::new(questions_file.clone());
    let mut answer_agent = ProxyAnswerAgent::new(questions_file.clone(), answers_file.clone());
    let mut learning_agent = LearningAgent::new(questions_file, answers_file, knowledge_file);

    // Spawn agents
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
            println!("═══════════════════════════════════════════════════════════════");
            println!("🛑 Stopping all agents...");
            println!("═══════════════════════════════════════════════════════════════");
        }
    }

    question_task.abort();
    answer_task.abort();
    learning_task.abort();

    println!();
    println!("✨ System stopped");
    println!("📊 Knowledge saved to: data/knowledge_base.json");
    println!();

    Ok(())
}
