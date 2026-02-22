//! Example integration of AgentBridge in a Tauri application
//!
//! This example shows various use cases and patterns for using the agent bridge.

use rusty_ide::{AgentBridge, AgentContext, AgentResponse, CodeSuggestion};
use std::path::PathBuf;
use std::time::Duration;

/// Example 1: Basic query to the agent
pub fn basic_query_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Query Example ===\n");

    let bridge = AgentBridge::new()?;

    let context = AgentContext::new("What is the best way to handle errors in Rust?".to_string());

    println!("Sending request to agent...");
    bridge.send_request(context)?;

    println!("Waiting for response (30s timeout)...");
    match bridge.wait_for_response(Duration::from_secs(30)) {
        Ok(response) => {
            println!("\nAgent response:");
            println!("{}", response.response_text);
            println!("\nSuggestions: {}", response.code_suggestions.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

/// Example 2: Query with file context
pub fn query_with_context_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Query with Context Example ===\n");

    let bridge = AgentBridge::new()?;

    let current_code = r#"
fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
"#;

    let context = AgentContext::new("Make this function safer".to_string())
        .with_workspace("/home/user/my_project".to_string())
        .with_current_file("src/main.rs".to_string(), current_code.to_string())
        .with_files(vec!["src/main.rs".to_string(), "Cargo.toml".to_string()]);

    bridge.send_request(context)?;

    match bridge.wait_for_response(Duration::from_secs(30)) {
        Ok(response) => {
            println!("Agent response:");
            println!("{}\n", response.response_text);

            if response.has_suggestions() {
                println!("Code suggestions:");
                for suggestion in &response.code_suggestions {
                    println!("\nFile: {}", suggestion.file);
                    println!("Description: {}", suggestion.description);
                    println!("Code:\n{}", suggestion.code);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

/// Example 3: IDE self-modification request
pub fn self_modification_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== IDE Self-Modification Example ===\n");

    let bridge = AgentBridge::new()?;

    let ide_source = bridge.get_ide_source()?;
    println!("Retrieved IDE source ({} bytes)", ide_source.len());

    let context = AgentContext::new(
        "Add a new Tauri command to save the current file".to_string(),
    )
    .with_ide_source(ide_source);

    bridge.send_request(context)?;

    match bridge.wait_for_response(Duration::from_secs(60)) {
        Ok(response) => {
            println!("\nAgent response:");
            println!("{}\n", response.response_text);

            if response.has_suggestions() {
                println!("IDE modifications:");
                for suggestion in &response.code_suggestions {
                    println!("\nFile: {}", suggestion.file);
                    println!("Description: {}", suggestion.description);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Run examples with: cargo run --example agent_integration");
    Ok(())
}
