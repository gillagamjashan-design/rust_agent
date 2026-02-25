# Project File Structure

## Overview

Simple structure: Knowledge database + Beautiful TUI + One launcher command.

---

## Directory Layout

```
rust_agent/
├── Cargo.toml                      # Workspace with rusty_tui binary
├── CLAUDE.md                       # Project instructions
├── README.md                       # Main documentation
│
├── architecture/                   # Architecture docs (all updated)
│   ├── system-overview.md          # High-level design
│   ├── components.md               # Component details
│   ├── data-flow.md                # Data flow diagrams
│   ├── file-structure.md           # This file
│   └── implementation-plan.md      # Implementation steps
│
├── knowledge/                      # JSON knowledge files
│   ├── rust_core_concepts.json     # Ownership, lifetimes, traits, etc.
│   ├── rust_patterns_idioms.json   # Builder, RAII, newtype patterns
│   ├── rust_toolchain_cargo.json   # Cargo commands and flags
│   └── rust_errors.json            # Compiler errors (planned)
│
├── src/                            # Library code
│   ├── lib.rs                      # Module exports
│   │
│   ├── knowledge/                  # Knowledge database system
│   │   ├── mod.rs                  # Module exports
│   │   ├── database.rs             # SQLite schema with FTS5
│   │   ├── loader.rs               # JSON → SQLite loader
│   │   └── query.rs                # Search interface
│   │
│   ├── tools/                      # Agent runtime tools
│   │   ├── mod.rs
│   │   └── knowledge_fetcher.rs    # Tool for agent to query DB
│   │
│   ├── claude_proxy.rs             # ClaudeProxyAPI client
│   ├── config.rs                   # Configuration
│   ├── cache.rs                    # Search cache
│   ├── types.rs                    # Shared types
│   │
│   └── web_search/                 # Web search fallback
│       ├── mod.rs
│       └── duckduckgo.rs           # DuckDuckGo client
│
├── rusty_tui/                      # Beautiful TUI binary
│   ├── Cargo.toml                  # Binary crate
│   └── src/
│       ├── main.rs                 # Entry point, TUI setup
│       ├── app.rs                  # App state and logic
│       ├── ui.rs                   # Ratatui rendering
│       ├── chat.rs                 # Chat window component
│       ├── input.rs                # Input handling
│       └── commands.rs             # /help, /search, /stats, /quit
│
├── start_cliproxyapi.sh            # Start ClaudeProxyAPI
│
├── target/                         # Build artifacts
│   └── release/
│       └── rusty                   # Compiled TUI binary
│
└── ~/.local/bin/                   # Installation target
    └── rusty                       # Symlink to binary
```

---

## Key Files

### Knowledge System

**src/knowledge/database.rs**
- SQLite schema with FTS5 full-text search
- Tables: concepts, patterns, commands, errors
- Methods: store_concept(), store_pattern(), count_concepts()

**src/knowledge/loader.rs**
- Load JSON files → SQLite database
- First run: populate database (~1.5 seconds)
- Returns LoadStats: concepts, patterns, commands

**src/knowledge/query.rs**
- Search interface for knowledge database
- Methods: search_concepts(), find_patterns(), search_all()
- Returns SearchResults with formatted markdown

### Agent Tools

**src/tools/knowledge_fetcher.rs**
- Runtime tool for agent to query knowledge
- Request types: ExplainConcept, FindPattern, Search
- Confidence calculation: 0.0-1.0 based on results
- Returns KnowledgeResponse with formatted text

### TUI Application

**rusty_tui/src/main.rs**
- Entry point for TUI
- Initialize knowledge database
- Launch ratatui interface
- Handle startup errors

**rusty_tui/src/app.rs**
- App state: messages, knowledge_fetcher, input
- Process user queries
- Fetch knowledge when needed
- Send to Claude API

**rusty_tui/src/ui.rs**
- Ratatui rendering
- Layout: chat window + input box + status bar
- Colors: Tokyo Night theme
- Syntax highlighting for code blocks

**rusty_tui/src/chat.rs**
- Message history display
- User messages: Cyan
- Agent messages: Green
- Scrollable with ↑/↓

**rusty_tui/src/input.rs**
- User input handling
- Command detection (/)
- Multi-line support
- History with arrow keys

**rusty_tui/src/commands.rs**
- `/help` - Show available commands
- `/search <query>` - Search knowledge database
- `/stats` - Show database statistics
- `/quit` - Exit application

---

## Data Files

### Runtime Data

```
~/.agent/
└── data/
    └── knowledge.db            # SQLite database
        ├── concepts            # Table with FTS5
        ├── patterns            # Table with FTS5
        ├── commands            # Table
        └── errors              # Table (planned)
```

### Knowledge JSON Files

**knowledge/rust_core_concepts.json** (Example structure):
```json
{
  "modules": [
    {
      "id": "ownership",
      "name": "Ownership",
      "concepts": [
        {
          "name": "Ownership Rules",
          "description": "Each value in Rust has an owner...",
          "rules": ["...", "..."],
          "examples": [{"title": "...", "code": "...", "explanation": "..."}],
          "common_errors": ["..."]
        }
      ]
    }
  ]
}
```

**knowledge/rust_patterns_idioms.json** (Example structure):
```json
{
  "categories": [
    {
      "name": "Creational Patterns",
      "patterns": [
        {
          "name": "Builder Pattern",
          "description": "...",
          "example": "struct Builder { ... }"
        }
      ]
    }
  ]
}
```

---

## Build Artifacts

```
target/
├── debug/                      # Debug builds
└── release/                    # Release builds
    └── rusty                   # Compiled TUI binary (installed to PATH)
```

---

## Installation

```bash
# Build the binary
cargo build --release --bin rusty

# Install to PATH
cp target/release/rusty ~/.local/bin/rusty
chmod +x ~/.local/bin/rusty

# Now just run:
rusty
```

---

## What Gets Deleted

All shell scripts except:
- `start_cliproxyapi.sh` - Needed to start Claude API

**Delete these:**
- `start_agent.sh`
- `test_agent.sh`
- `install.sh`
- `update.sh`
- `spawn_teachers.sh`
- `answer_teacher.sh`
- `orchestrator.sh`
- `test_websearch.sh`
- Any other .sh files

**Reason**: Single `rusty` command replaces all of them!

---

## Simplified Structure Benefits

**Before (complex):**
- Multiple binaries (agent, learning mode, interactive mode)
- Shell scripts for everything
- IDE features (file tree, editor, terminal)
- Multiple interfaces (TUI, web, API)
- Complex orchestration

**After (simple):**
- One binary: `rusty`
- One purpose: Agent chat
- One interface: Beautiful TUI
- No shell scripts (except ClaudeProxyAPI starter)
- No flags, no modes, no complexity

**Result**: Just run `rusty` and start chatting!
