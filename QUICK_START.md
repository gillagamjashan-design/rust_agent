# Quick Start - Rust Learning Agent 🦀

## Installation Complete! ✅

Everything is set up. Here's how to run the agent:

## Method 1: Interactive Terminal Mode (Recommended)

```bash
./start_agent.sh
```

Or manually:
```bash
./target/release/agent --interactive
```

This starts the knowledge-powered chat agent in your terminal.

## Method 2: Desktop App (Simplified UI)

```bash
cd rusty_ide_v2
npm install
npm run tauri dev
```

This launches the full-screen agent chat in a desktop window.

## What Happens on First Run

1. **Knowledge Database Loading:**
   ```
   📚 Loading knowledge database (first time)...
   ✅ Loaded 25 concepts, 18 patterns, 0 errors, 35 commands (total: 78)
   ```

2. **Agent Ready:**
   ```
   ╔══════════════════════════════════════════════════════════════╗
   ║      Rust Learning Agent with Knowledge Database            ║
   ║          Instant Access to Rust Knowledge                   ║
   ╚══════════════════════════════════════════════════════════════╝

   🧠 Knowledge Database:
      ✅ Loaded and ready
      - Queryable database with FTS5 search
      - Concepts, patterns, commands available

   I can help you with:
     • Learning Rust programming
     • Explaining ownership, lifetimes, traits
     • Common Rust patterns and idioms
     • Cargo and Rust toolchain commands

   Type your question:
   ════════════════════════════════════════════════════════════════
   ```

## Example Session

```bash
$ ./start_agent.sh

> What is ownership in Rust?

🤔 Processing your request...

📚 Found in knowledge database (confidence: 0.90)

💡 Answer:

# Rust Programming: OWNERSHIP

## Core Concepts

### Ownership Rules

The three fundamental rules of Rust ownership:
1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

### Move Semantics

**Basic ownership transfer:**
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}", s1); // ERROR: s1 no longer valid
```

String is heap-allocated, so assignment moves ownership...

[Detailed explanation with examples]

> /search builder pattern

🔍 Searching knowledge database for: "builder pattern"

Confidence: 0.95

## Patterns

### Builder Pattern

Construct complex objects step by step

**When to use:** When constructing objects with many optional parameters

```rust
impl ServerBuilder {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port: 8080,
            timeout: None,
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn build(self) -> Server { ... }
}
```

> Write me a function to read a file with error handling

💡 Answer:

Here's a robust implementation using Rust's error handling patterns:

```rust
use std::fs;
use anyhow::{Context, Result};

pub fn read_config_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;

    Ok(content)
}

// Usage:
fn main() -> Result<()> {
    let config = read_config_file("config.toml")?;
    println!("Config loaded: {} bytes", config.len());
    Ok(())
}
```

This uses the `?` operator for error propagation and `anyhow::Context`
for adding context to errors...
```

## Available Commands

Once running, you can use:

| Command | Description |
|---------|-------------|
| (Ask anything) | "What is ownership?" / "Write a TCP server" |
| `/search <query>` | Search the knowledge database |
| `/stats` | Show knowledge database statistics |
| `/web <query>` | Force web search for latest info |
| `/cache clear` | Clear web search cache |
| `/help` | Show all available commands |
| `/quit` | Exit the agent |

## Features

✅ **Instant Knowledge** - Database loads in <1 second
✅ **Fast Search** - <50ms query times with SQLite FTS5
✅ **Comprehensive** - 25+ concepts, 18+ patterns, 35+ commands
✅ **Code Writing** - Full Rust coding assistant
✅ **Debug Help** - Explain errors, suggest fixes
✅ **Web Search** - Can search web for latest information

## Troubleshooting

### "No knowledge base found"
The agent will auto-create it on first run from `knowledge/*.json` files.

### "Failed to communicate with agent"
Make sure CLIProxyAPI is running on localhost:8317 if you're using Claude proxy.

### Build errors
Try:
```bash
export CARGO_HOME="$(pwd)/.cargo"
cargo build --release
```

## What Next?

1. **Learn Rust:** Ask about ownership, lifetimes, traits, etc.
2. **Code Projects:** "Create a web server" / "Build a CLI tool"
3. **Debug Code:** Paste your code, ask what's wrong
4. **Explore Patterns:** "/search RAII" / "/search error handling"

## Files & Directories

- `~/.agent/data/knowledge.db` - Knowledge database (auto-created)
- `knowledge/*.json` - Source knowledge files
- `target/release/agent` - Compiled binary
- `start_agent.sh` - Easy startup script

---

**Ready to learn Rust? Start the agent and ask your first question!** 🚀
