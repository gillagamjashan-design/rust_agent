mod cache;
mod claude_proxy;
mod config;
mod interactive_agent;
mod learning_agent;
mod proxy_agents;
mod types;
mod web_search;

use clap::Parser;
use interactive_agent::InteractiveAgent;
use learning_agent::LearningAgent;
use proxy_agents::{ProxyAnswerAgent, ProxyQuestionAgent};

use anyhow::Result;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in interactive mode (assistant that uses learned knowledge)
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.interactive {
        run_interactive_mode().await?;
    } else {
        run_learning_mode().await?;
    }

    Ok(())
}

async fn run_interactive_mode() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              INTERACTIVE MODE                                ║");
    println!("║      Programming Assistant with Learned Knowledge            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let mut agent = InteractiveAgent::new();
    agent.run().await?;

    Ok(())
}

async fn run_learning_mode() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              LEARNING MODE                                   ║");
    println!("║   Self-Learning Agent with CLIProxyAPI (Claude Max)         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("System:");
    println!("  🤖 Question Agent → CLIProxyAPI → questions.txt");
    println!("  🤖 Answer Agent   → CLIProxyAPI → answers.txt");
    println!("  🧠 Learning Agent → reads files → knowledge_base.json");
    println!();
    println!("Using: Claude Max subscription via CLIProxyAPI");
    println!("Proxy: http://localhost:8317");
    println!();
    println!("Press Ctrl+C to stop learning and save knowledge");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Setup paths in ~/.agent/data/
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    let data_dir = home_dir.join(".agent").join("data");
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
            eprintln!("❌ Question Agent error: {}", e);
        }
    });

    let answer_task = tokio::spawn(async move {
        if let Err(e) = answer_agent.run().await {
            eprintln!("❌ Answer Agent error: {}", e);
        }
    });

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
            println!("🛑 Stopping all agents and saving knowledge...");
            println!("═══════════════════════════════════════════════════════════════");
        }
    }

    question_task.abort();
    answer_task.abort();
    learning_task.abort();

    println!();
    println!("✨ Learning session complete!");
    println!("📊 Knowledge saved to: data/knowledge_base.json");
    println!();
    println!("💡 TIP: Run in interactive mode to use your learned knowledge:");
    println!("   CARGO_HOME=../.cargo cargo run -- --interactive");
    println!();

    Ok(())
}
