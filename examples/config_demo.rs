/// Example demonstrating the configuration system
///
/// Run with: CARGO_HOME=../.cargo cargo run --example config_demo
///
/// This example shows how to:
/// - Load configuration from ~/.agent/config.toml
/// - Use default configuration if file doesn't exist
/// - Save configuration to file
/// - Access configuration values

use rust_agent::config::Config;

fn main() -> anyhow::Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        Configuration System Demo                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Load configuration (will use defaults if file doesn't exist)
    println!("Loading configuration...");
    let config = Config::load()?;

    println!();
    println!("Current Configuration:");
    println!("════════════════════════════════════════════════════════════════");

    // Display web search settings
    println!("\n[Web Search]");
    println!("  Enabled:          {}", config.web_search.enabled);
    println!("  Cache TTL:        {} hours", config.web_search.cache_ttl_hours);
    println!("  Timeout:          {} seconds", config.web_search.timeout_seconds);
    println!("  Max Results:      {}", config.web_search.max_results);

    // Display API keys
    println!("\n[API Keys]");
    match &config.api_keys.serper {
        Some(key) => {
            // Show only first 8 characters for security
            let preview = if key.len() > 8 {
                format!("{}...", &key[..8])
            } else {
                key.clone()
            };
            println!("  Serper API Key:   {} (configured)", preview);
        }
        None => println!("  Serper API Key:   (not configured)"),
    }

    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // Demonstrate saving configuration
    println!("💡 TIP: To customize settings, edit ~/.agent/config.toml");
    println!("    Or copy config.toml.example to ~/.agent/config.toml");
    println!();
    println!("Example configuration file:");
    println!("────────────────────────────────────────────────────────────────");
    println!("[web_search]");
    println!("enabled = true");
    println!("cache_ttl_hours = 24");
    println!("timeout_seconds = 10");
    println!("max_results = 5");
    println!();
    println!("[api_keys]");
    println!("serper = \"your-api-key-here\"  # Optional");
    println!("────────────────────────────────────────────────────────────────");
    println!();

    Ok(())
}
