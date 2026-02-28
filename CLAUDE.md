# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**Rusty** is a Rust learning agent with instant access to a comprehensive knowledge database. The project consists of two main components:

1. **Core Library (`rust_agent`)**: Knowledge database system with SQLite FTS5 search, Claude API integration, web search fallback, and agent tools
2. **GUI Application (`rusty_tui`)**: Native graphical interface built with egui/eframe that provides a chat-style interface to the learning agent

**Current Version**: v12.0.0 (GUI Migration Release)

---

## Important Constraints

**CRITICAL - Must Follow These Rules:**

1. **Git Tags**: Never create git tags without explicit user permission
2. **Testing Required**: Always test the product thoroughly before delivering it to the user
3. **Quality Assurance**: Always use a rating tool to ensure the product quality is very good

---

## Build Commands

### Building the GUI Application (Primary)

```bash
# Build the rusty GUI application (recommended)
cd rusty_tui
cargo build --release

# The binary will be at: rusty_tui/target/release/rusty
```

### Building the Core Library

```bash
# Build the library
cargo build --release

# Build the CLI agent binary
cargo build --release --bin agent
```

### One-Command Installation

```bash
# Full installation (builds, installs to PATH, starts services)
./run-all.sh
```

---

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
# Run a specific test by name
cargo test test_name

# Run tests in a specific file
cargo test --test exercises_test
```

### Test Knowledge Database

```bash
# The knowledge system is tested on first run
# Database location: ~/.agent/data/knowledge.db
```

---

## Running the Application

### GUI Mode (Recommended)

```bash
# After building
cd rusty_tui
./target/release/rusty

# Or if installed to PATH
rusty
```

### CLI Mode (Alternative)

```bash
# Interactive terminal mode
./target/release/agent --interactive
```

---

## Architecture Overview

### Knowledge Database System

The core innovation is a **queryable knowledge database** instead of traditional training:

- **Location**: `~/.agent/data/knowledge.db`
- **Technology**: SQLite with FTS5 full-text search
- **Performance**: <50ms query times
- **Source Files**: JSON files in `knowledge/` directory

**Key Components**:

1. **Database Schema** (`src/knowledge/database.rs`):
   - `concepts` table: Rust language concepts (ownership, lifetimes, traits)
   - `patterns` table: Code patterns and idioms (Builder, RAII, Newtype)
   - `commands` table: Cargo and rustup commands
   - FTS5 virtual tables for full-text search on all tables

2. **Loader** (`src/knowledge/loader.rs`):
   - Loads JSON files from `knowledge/` on first run
   - Populates SQLite database with structured knowledge
   - Takes ~1-2 seconds on first run, instant on subsequent runs

3. **Query Engine** (`src/knowledge/query.rs`):
   - FTS5-powered full-text search
   - Returns ranked results with relevance scores
   - Supports filtering by concept type, tags, etc.

### Runtime Tools System

The agent uses **runtime tools** to fetch information dynamically:

1. **KnowledgeFetcher** (`src/tools/knowledge_fetcher.rs`):
   - Queries knowledge database during runtime
   - Returns relevant concepts, patterns, and examples
   - Provides context to Claude API for accurate responses

2. **Web Search Fallback** (`src/web_search/`):
   - DuckDuckGo integration for unknown topics
   - Intelligent caching system (SHA-256 hash-based)
   - Cache location: `~/.agent/cache/`
   - 7-day cache expiry for web results

### GUI Architecture (v12.0.0)

**Framework**: egui + eframe (native windowing)

**Key Files**:
- `rusty_tui/src/gui/app.rs`: Main application state and eframe::App implementation
- `rusty_tui/src/gui/layout.rs`: UI rendering with **Enter key fix**
- `rusty_tui/src/gui/worker.rs`: Async worker thread for non-blocking API calls
- `rusty_tui/src/gui/theme.rs`: Tokyo Night color scheme
- `rusty_tui/src/gui/messages.rs`: Channel communication between UI and worker

**Critical Implementation Detail - Enter Key Fix**:

The Enter key works properly because we use egui's native event handling:

```rust
// In layout.rs
if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
    if !app.input.is_empty() {
        let input = app.input.clone();
        app.input.clear();
        app.send_message(input);
        app.scroll_to_bottom = true;
    }
    response.request_focus();
}
```

This pattern avoids the async polling issues that plagued the previous TUI implementation.

### Integration Flow

```
User types question in GUI
        ↓
KnowledgeFetcher.search(query) - searches SQLite DB
        ↓
If found (confidence > 0.7):
    Inject knowledge into prompt → Claude API
Else:
    DuckDuckGoClient.search(query) → web results
        ↓
Claude generates response with context
        ↓
Display in GUI with syntax highlighting
```

---

## Key File Locations

### Knowledge System
- `src/knowledge/database.rs` - SQLite schema and connection
- `src/knowledge/loader.rs` - JSON to database loading
- `src/knowledge/query.rs` - FTS5 search implementation
- `knowledge/*.json` - Source knowledge files (7 files)

### Tools
- `src/tools/knowledge_fetcher.rs` - Runtime knowledge queries
- `src/tools/slash_commands.rs` - /help, /search, /stats commands

### Integration
- `src/claude_proxy.rs` - Claude API client
- `src/web_search/duckduckgo.rs` - Web search implementation
- `src/cache.rs` - Search result caching

### GUI Application
- `rusty_tui/src/main.rs` - Entry point
- `rusty_tui/src/gui/` - All GUI components
- `rusty_tui/Cargo.toml` - GUI dependencies (egui, eframe, tokio)

### Scripts
- `run-all.sh` - One-command installation script
- `start_cliproxyapi.sh` - Start Claude API proxy
- `scripts/install-rusty-linux.sh` - Linux installation
- `scripts/install-rusty-mac.sh` - macOS installation

---

## Knowledge Database

### Adding New Knowledge

1. Create or edit JSON files in `knowledge/` directory
2. Follow the existing format (see `rust_core_concepts.json` for examples)
3. Delete `~/.agent/data/knowledge.db` to force reload
4. Run `rusty` - it will rebuild the database automatically

### Knowledge File Format

**Concepts** (`rust_core_concepts.json`):
```json
{
  "concepts": [
    {
      "id": "ownership",
      "topic": "ownership",
      "title": "Ownership System",
      "explanation": "Detailed explanation...",
      "code_examples": [...],
      "common_mistakes": [...],
      "related_concepts": ["borrowing", "lifetimes"],
      "tags": ["memory", "safety"]
    }
  ]
}
```

**Patterns** (`rust_patterns_idioms.json`):
```json
{
  "patterns": [
    {
      "id": "builder",
      "name": "Builder Pattern",
      "description": "Construct complex objects step by step",
      "template": "impl Builder { ... }",
      "when_to_use": "Objects with many optional parameters",
      "examples": [...]
    }
  ]
}
```

### Available Knowledge Files

1. `rust_core_concepts.json` - 13 core concepts
2. `rust_patterns_idioms.json` - 18 patterns
3. `rust_toolchain_cargo.json` - 22 cargo commands
4. `rust_async_concurrency.json` - Async/await, Tokio
5. `rust_standard_library.json` - Common std modules
6. `rust_popular_crates.json` - Popular crates
7. `curriculum_master.json` - Learning curriculum

---

## ClaudeProxyAPI Dependency

The agent requires ClaudeProxyAPI running on `localhost:8317`:

```bash
# Check if running
curl http://localhost:8317/health

# Start it
./start_cliproxyapi.sh
```

**Note**: The knowledge database works independently of the Claude API. If the API is unavailable, the agent can still search and display knowledge entries directly.

---

## Development Patterns

### When Building New Features

1. **Knowledge-First Approach**: Add knowledge to JSON files rather than hardcoding information
2. **Tool-Based**: Create runtime tools that query the knowledge database
3. **Non-Blocking UI**: Use worker threads for any long-running operations in GUI

### Testing Knowledge System

```bash
# Test database initialization
rm ~/.agent/data/knowledge.db
cargo run --bin agent -- --interactive

# Should show: "Loading knowledge database..."
# Then: "Loaded X concepts, Y patterns..."
```

### Debugging Tips

```bash
# Enable debug logging
RUST_LOG=debug rusty

# Check database directly
sqlite3 ~/.agent/data/knowledge.db
sqlite> SELECT COUNT(*) FROM concepts;
sqlite> SELECT * FROM concepts WHERE topic LIKE '%ownership%';
```

---

## Common Workflows

### Rebuilding After Changes

```bash
# Clean build
cargo clean
cd rusty_tui
cargo build --release

# Test it
./target/release/rusty
```

### Adding a New Rust Concept

1. Edit `knowledge/rust_core_concepts.json`
2. Add your concept following the existing format
3. Delete `~/.agent/data/knowledge.db`
4. Run `rusty` to rebuild database
5. Test with: `/search your_concept_name`

### Modifying the GUI

1. Edit files in `rusty_tui/src/gui/`
2. Rebuild: `cd rusty_tui && cargo build --release`
3. Test: `./target/release/rusty`

**Important**: When modifying event handling, always test the Enter key functionality to ensure it still works!

---

## Dependencies

### Core Library Dependencies

- `tokio` - Async runtime
- `rusqlite` - SQLite database (bundled)
- `reqwest` - HTTP client for Claude API and web search
- `serde`, `serde_json` - Serialization
- `anyhow` - Error handling
- `sha2` - Cache key hashing
- `urlencoding` - URL encoding for search queries

### GUI Application Dependencies

- `egui` - Immediate mode GUI framework
- `eframe` - Native window integration for egui
- `egui_extras` - Syntax highlighting support
- `syntect` - Code syntax highlighting
- `tokio` - Async runtime for worker thread
- `rust_agent` - Core library (path dependency)

---

## Migration Notes (TUI → GUI)

**v11.0.0 (TUI)** → **v12.0.0 (GUI)**:

- **Old**: ratatui + crossterm (terminal UI with Enter key issues)
- **New**: egui + eframe (native window with working Enter key)
- **Reason**: Terminal event polling conflicted with async API calls, causing UI freezes and broken Enter key

**Files Removed**: `rusty_tui/src/{app.rs, ui.rs, chat.rs, input.rs, commands.rs}` (old TUI code)

**Files Added**: `rusty_tui/src/gui/{mod.rs, app.rs, layout.rs, worker.rs, theme.rs, messages.rs}` (new GUI code)

If you need to understand the old TUI architecture, check git history before commit `aa6a171`.

---

## Agent Capabilities

The agent is designed to:

1. **Teach Rust** - Explain concepts using knowledge database
2. **Generate Code** - Create working Rust programs
3. **Debug** - Identify and fix bugs
4. **Search** - Find relevant information in knowledge base or web

**Important**: The agent gets its Rust knowledge from the database, not from training. Always verify the database has up-to-date information for the agent to reference.

---

## Performance Targets

- Knowledge database query: <50ms
- First run database load: <2s
- GUI startup (after DB loaded): <300ms
- Agent response time: 1-3s (depends on Claude API)
- GUI render: 60 FPS

---

## Installation Directories

- Binary: `~/.local/bin/rusty` (installed by `run-all.sh`)
- Knowledge DB: `~/.agent/data/knowledge.db`
- Web cache: `~/.agent/cache/`
- Build artifacts: `target/` and `rusty_tui/target/`
