# Rusty - Complete File Reference

## All Files in the Project with Descriptions

---

## Core Library Files (`src/`)

### Main Library Files

| File | Lines | Purpose |
|------|-------|---------|
| **lib.rs** | ~17 | Library entry point, exports all public modules |
| **config.rs** | ~100 | Configuration management (DB path, API URL, cache dir) |
| **types.rs** | ~150 | Shared type definitions used across modules |
| **cache.rs** | ~200 | Web search result caching (SHA-256 based, 7-day expiry) |
| **claude_proxy.rs** | ~150 | HTTP client for ClaudeProxyAPI (localhost:8317) |

### Knowledge System (`src/knowledge/`)

| File | Lines | Purpose |
|------|-------|---------|
| **mod.rs** | ~30 | Module exports for knowledge system |
| **database.rs** | ~400 | SQLite schema, FTS5 setup, database initialization |
| **loader.rs** | ~300 | JSON → Database loader, parses knowledge files |
| **query.rs** | ~250 | FTS5 search implementation, result ranking |

**Key Functions in database.rs**:
- `create_tables()` - Creates concepts, patterns, commands, file_templates tables
- `setup_fts5()` - Creates FTS5 virtual tables and triggers
- `count_concepts()`, `count_patterns()`, `count_commands()` - Statistics

**Key Functions in loader.rs**:
- `load_all_knowledge()` - Loads all JSON files from knowledge/ directory
- `parse_concepts_json()` - Parses rust_core_concepts.json
- `parse_patterns_json()` - Parses rust_patterns_idioms.json
- `insert_concept()`, `insert_pattern()`, etc. - Database insertion

**Key Functions in query.rs**:
- `search()` - Main search entry point, queries all FTS tables
- `search_concepts_fts()` - Search concepts table
- `search_patterns_fts()` - Search patterns table
- `format_results()` - Format for Claude API consumption

### Tools (`src/tools/`)

| File | Lines | Purpose |
|------|-------|---------|
| **mod.rs** | ~20 | Tool exports |
| **knowledge_fetcher.rs** | ~150 | Runtime tool to query knowledge database |
| **slash_commands.rs** | ~200 | /help, /search, /stats command implementations |

**KnowledgeFetcher API**:
```rust
pub struct KnowledgeFetcher {
    query: KnowledgeQuery,
}

impl KnowledgeFetcher {
    pub fn search(&self, query: &str) -> Result<KnowledgeResults>
    pub fn get_concept(&self, id: &str) -> Result<Concept>
    pub fn get_pattern(&self, id: &str) -> Result<Pattern>
}
```

### Web Search (`src/web_search/`)

| File | Lines | Purpose |
|------|-------|---------|
| **mod.rs** | ~15 | Web search exports |
| **duckduckgo.rs** | ~350 | DuckDuckGo integration, caching, HTML parsing |

**Key Functions**:
- `search()` - Main search entry point with caching
- `fetch_from_web()` - HTTP request to DuckDuckGo
- `parse_results()` - HTML parsing
- `cache_results()` - Save to ~/.agent/cache/
- `load_cached()` - Load from cache if fresh

### File Generation (`src/`)

| File | Lines | Purpose |
|------|-------|---------|
| **file_generator.rs** | ~473 | Complete file creation system |

**Structures**:
```rust
// Intent detection
struct FileCreationDetector {
    at_filepath_regex: Regex,
    code_block_regex: Regex,
    filename_hint_regex: Regex,
    type_name_regex: Regex,
}

// File operations
struct FileCreator {
    workspace_root: PathBuf,
}

// Database-driven automation
struct AutoFileCreator {
    workspace_root: PathBuf,
    db: KnowledgeDatabase,
    detector: FileCreationDetector,
}

// Results
struct CodeBlock {
    language: String,
    code: String,
    filename: Option<PathBuf>,
}

struct FileCreationResult {
    path: PathBuf,
    appended: bool,
    error: Option<String>,
}
```

**Key Functions**:
- `should_create_files()` - Keyword detection (create, make, generate)
- `extract_code_blocks()` - Parse ```rust ... ``` blocks
- `infer_filename()` - Heuristic filename inference
- `auto_create_from_response()` - Main entry point
- `create_or_append_file()` - File I/O with append mode

---

## GUI Application Files (`rusty_tui/src/`)

### Main Application

| File | Lines | Purpose |
|------|-------|---------|
| **main.rs** | ~50 | Entry point, initializes eframe with native options |

**Key Code**:
```rust
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Rusty - Your Rust Learning Agent"),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty",
        options,
        Box::new(|cc| Ok(Box::new(RustyApp::new(cc)))),
    )
}
```

### GUI Module (`rusty_tui/src/gui/`)

| File | Lines | Purpose |
|------|-------|---------|
| **mod.rs** | ~10 | GUI module exports |
| **app.rs** | ~148 | Application state, eframe::App implementation |
| **layout.rs** | ~250 | UI rendering, Enter key handling (**critical!**) |
| **worker.rs** | ~230 | Async worker thread, Claude API calls, file creation |
| **messages.rs** | ~45 | Message types for channel communication |
| **theme.rs** | ~50 | Tokyo Night color scheme constants |

#### app.rs Details

**RustyApp Structure**:
```rust
pub struct RustyApp {
    // Chat state
    pub messages: Vec<Message>,           // All chat messages
    pub input: String,                    // Current input buffer
    pub waiting_for_response: bool,       // Show loading spinner
    pub scroll_to_bottom: bool,           // Auto-scroll flag
    pub first_render: bool,               // First render optimization

    // Session data
    pub created_files: Vec<String>,       // Files created this session
    pub knowledge_stats: String,          // "13 concepts, 18 patterns"

    // IPC channels
    message_rx: Receiver<WorkerMessage>,  // Worker → GUI
    command_tx: Sender<UserCommand>,      // GUI → Worker
}
```

**Key Methods**:
- `new()` - Initialize app, spawn worker thread, create welcome message
- `send_message()` - Send user input to worker
- `handle_worker_message()` - Process responses from worker
- `update()` - eframe::App trait method, called every frame

#### layout.rs Details

**UI Structure**:
```rust
pub fn render_ui(ctx: &egui::Context, app: &mut RustyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            render_header(ui, app);      // Top bar with stats
            render_chat_area(ui, app);   // Scrollable message list
            render_input_area(ui, app);  // Input box at bottom
        });
    });
}
```

**Enter Key Fix** (Lines 180-190):
```rust
// CRITICAL: This pattern fixes the Enter key issue
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

**Syntax Highlighting**:
- Uses `egui_extras::syntax_highlighting`
- Powered by `syntect` crate
- Supports rust, toml, json, bash, etc.

#### worker.rs Details

**Worker Thread Setup**:
```rust
pub fn spawn_worker(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            worker_loop(command_rx, message_tx, db_path).await
        })
    })
}
```

**Main Loop** (Simplified):
```rust
async fn worker_loop(...) {
    // Initialize resources
    let db = KnowledgeDatabase::new(&db_path)?;
    let knowledge_fetcher = KnowledgeFetcher::new(db.clone());
    let claude = ClaudeProxy::new();
    let auto_file_creator = AutoFileCreator::new(cwd, db.clone());

    // Process commands
    loop {
        match command_rx.recv() {
            Ok(UserCommand::Query(text)) => {
                // 1. Search knowledge
                let context = knowledge_fetcher.search(&text)?;

                // 2. Query Claude
                let response = claude.query(&prompt).await?;

                // 3. Create files
                let files = auto_file_creator.auto_create_from_response(&text, &response)?;

                // 4. Send results
                message_tx.send(WorkerMessage::Response(response))?;
                if !files.is_empty() {
                    message_tx.send(WorkerMessage::FilesCreated(files))?;
                }
            }
            Ok(UserCommand::Command(cmd)) => {
                execute_command(&cmd, &knowledge_fetcher, &db)?;
            }
            _ => break,
        }
    }
}
```

**Helper Functions**:
- `format_file_creation_summary()` - Formats "📁 Files Created: ✅ ..."
- `execute_command()` - Handles /help, /search, /stats
- `help_text()` - Returns help message text
- `search_command()` - Executes /search command
- `stats_command()` - Returns database statistics

#### messages.rs Details

**Message Types**:
```rust
// GUI → Worker
pub enum UserCommand {
    Query(String),     // User question
    Command(String),   // /search, /help, etc.
    Quit,             // Exit signal
}

// Worker → GUI
pub enum WorkerMessage {
    Response(String),              // Claude's answer
    SystemMessage(String),         // System notifications
    Error(String),                // Error messages
    Stats(String),                // "13 concepts, 18 patterns"
    FilesCreated(Vec<FileCreationInfo>),  // File creation results
}

// File creation info
pub struct FileCreationInfo {
    pub path: String,     // Full path to file
    pub appended: bool,   // true if updated, false if created
    pub success: bool,    // true if operation succeeded
}

// Chat message
pub struct Message {
    pub role: Role,              // User, Assistant, System
    pub content: String,         // Message text
    pub timestamp: DateTime<Local>,  // When sent
}

pub enum Role {
    User,       // User messages (blue)
    Assistant,  // Claude responses (green)
    System,     // System messages (gray)
}
```

#### theme.rs Details

**Tokyo Night Color Palette**:
```rust
// Background colors
pub const BACKGROUND: Color32 = Color32::from_rgb(26, 27, 38);   // #1a1b26
pub const SURFACE: Color32 = Color32::from_rgb(36, 40, 59);      // #24283b

// Text colors
pub const TEXT: Color32 = Color32::from_rgb(169, 177, 214);      // #a9b1d6
pub const COMMENT: Color32 = Color32::from_rgb(86, 95, 137);     // #565f89

// Accent colors
pub const BLUE: Color32 = Color32::from_rgb(122, 162, 247);      // #7aa2f7
pub const GREEN: Color32 = Color32::from_rgb(158, 206, 106);     // #9ece6a
pub const CYAN: Color32 = Color32::from_rgb(125, 207, 255);      // #7dcfff
pub const YELLOW: Color32 = Color32::from_rgb(224, 175, 104);    // #e0af68
pub const MAGENTA: Color32 = Color32::from_rgb(187, 154, 247);   // #bb9af7
pub const RED: Color32 = Color32::from_rgb(247, 118, 142);       // #f7768e

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.window_fill = BACKGROUND;
    style.visuals.panel_fill = SURFACE;
    ctx.set_style(style);
}
```

---

## Knowledge Files (`knowledge/`)

### JSON Source Files

| File | Entries | Purpose |
|------|---------|---------|
| **rust_core_concepts.json** | 13 | Core Rust concepts (ownership, lifetimes, traits) |
| **rust_patterns_idioms.json** | 18 | Design patterns (Builder, RAII, Newtype) |
| **rust_toolchain_cargo.json** | 22 | Cargo commands (build, test, publish) |
| **rust_async_concurrency.json** | 15 | Async/await, Tokio, futures |
| **rust_standard_library.json** | 20 | std modules (collections, io, fs) |
| **rust_popular_crates.json** | 25 | Ecosystem crates (serde, tokio, clap) |
| **curriculum_master.json** | 12 | Learning path structure |

**Total Knowledge Base**: ~125 entries

### Example Entry Structure

**Concept** (rust_core_concepts.json):
```json
{
  "id": "ownership",
  "topic": "ownership",
  "title": "Ownership System",
  "explanation": "Rust's ownership system ensures memory safety without garbage collection...",
  "code_examples": [
    {
      "code": "let s = String::from(\"hello\");",
      "explanation": "Creates owned String on heap"
    }
  ],
  "common_mistakes": [
    "Trying to use a value after it's been moved",
    "Not understanding when copies vs moves happen"
  ],
  "related_concepts": ["borrowing", "lifetimes", "drop"],
  "tags": ["memory", "safety", "core"]
}
```

**Pattern** (rust_patterns_idioms.json):
```json
{
  "id": "builder",
  "name": "Builder Pattern",
  "description": "Construct complex objects step by step",
  "template": "pub struct FooBuilder { ... }",
  "when_to_use": "Objects with many optional parameters",
  "examples": [
    {
      "code": "let foo = FooBuilder::new().with_name(\"test\").build();",
      "description": "Fluent API for construction"
    }
  ],
  "related_patterns": ["typestate", "RAII"]
}
```

**Command** (rust_toolchain_cargo.json):
```json
{
  "id": "cargo_build",
  "tool": "cargo",
  "command": "build",
  "description": "Compile the current package",
  "examples": [
    "cargo build",
    "cargo build --release",
    "cargo build --target x86_64-unknown-linux-gnu"
  ],
  "flags": [
    {
      "flag": "--release",
      "description": "Build with optimizations"
    }
  ]
}
```

---

## Scripts

| File | Lines | Purpose |
|------|-------|---------|
| **run-all.sh** | ~100 | One-command installation (build + install + start services) |
| **start_cliproxyapi.sh** | ~50 | Start ClaudeProxyAPI on localhost:8317 |
| **scripts/install-rusty-linux.sh** | ~150 | Linux installation script |
| **scripts/install-rusty-mac.sh** | ~150 | macOS installation script |

---

## Configuration Files

| File | Purpose |
|------|---------|
| **Cargo.toml** | Core library manifest (workspace root) |
| **rusty_tui/Cargo.toml** | GUI application manifest |
| **CLAUDE.md** | Instructions for Claude Code AI assistant |
| **.gitignore** | Git ignore patterns (target/, *.db, cache/) |

### Cargo.toml (Core Library)

```toml
[package]
name = "rust_agent"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.31", features = ["bundled"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
sha2 = "0.10"
urlencoding = "2.1"
regex = "1.10"
dirs = "5.0"
chrono = "0.4"
```

### Cargo.toml (GUI Application)

```toml
[package]
name = "rusty"
version = "1.0.0"
edition = "2021"

[[bin]]
name = "rusty"
path = "src/main.rs"

[dependencies]
egui = "0.30"
eframe = { version = "0.30", features = ["glow", "wayland", "x11"] }
egui_extras = { version = "0.30", features = ["syntect"] }
syntect = "5.2"
tokio = { version = "1", features = ["full"] }
rust_agent = { path = ".." }
dirs = "5.0"
```

---

## Documentation Files

| File | Lines | Purpose |
|------|-------|---------|
| **README.md** | ~200 | Project overview and quick start guide |
| **CLAUDE.md** | ~400 | Detailed instructions for Claude Code |
| **PROJECT_COMPLETE_SUMMARY.md** | ~1100 | Complete project documentation (this doc) |
| **ARCHITECTURE_DIAGRAM.md** | ~800 | Visual architecture diagrams |
| **FILE_REFERENCE.md** | ~600 | This file - complete file listing |
| **IMPLEMENTATION_SUMMARY.md** | ~150 | Implementation status and testing results |
| **TEST_VERIFICATION.md** | ~250 | Test plan and verification checklist |

---

## Build Artifacts (Generated)

### Core Library

```
target/
├── debug/
│   ├── librust_agent.rlib          # Debug library
│   └── deps/                       # Dependencies
└── release/
    ├── librust_agent.rlib          # Release library
    └── deps/
```

### GUI Application

```
rusty_tui/target/
├── debug/
│   ├── rusty                       # Debug binary
│   └── deps/
└── release/
    ├── rusty                       # Release binary (23MB)
    └── deps/
```

---

## Runtime Data Files

### Generated on First Run

```
~/.agent/
├── data/
│   └── knowledge.db                # SQLite database (~2MB)
│       ├── concepts (13 rows)
│       ├── patterns (18 rows)
│       ├── commands (22 rows)
│       ├── file_templates (6 rows)
│       └── FTS5 indexes
│
└── cache/
    └── <hash>.json                # Web search cache (7-day expiry)
```

---

## File Statistics

### Source Code

| Category | Files | Total Lines |
|----------|-------|-------------|
| Core Library | 15 | ~2,500 |
| GUI Application | 6 | ~800 |
| Knowledge JSON | 7 | ~3,000 |
| Scripts | 4 | ~450 |
| Documentation | 7 | ~3,500 |
| Configuration | 3 | ~150 |
| **TOTAL** | **42** | **~10,400** |

### By Language

| Language | Files | Lines | Percentage |
|----------|-------|-------|-----------|
| Rust | 21 | ~3,300 | 32% |
| JSON | 7 | ~3,000 | 29% |
| Markdown | 7 | ~3,500 | 34% |
| Bash | 4 | ~450 | 4% |
| TOML | 3 | ~150 | 1% |

---

## File Dependencies

### Core Library Dependencies

```
lib.rs
├── config.rs
├── types.rs
├── cache.rs
├── claude_proxy.rs
├── knowledge/
│   ├── database.rs → types.rs
│   ├── loader.rs → database.rs, types.rs
│   └── query.rs → database.rs, types.rs
├── tools/
│   ├── knowledge_fetcher.rs → knowledge/query.rs
│   └── slash_commands.rs → knowledge/query.rs
├── web_search/
│   └── duckduckgo.rs → cache.rs
└── file_generator.rs → knowledge/database.rs
```

### GUI Dependencies

```
main.rs
└── gui/
    ├── app.rs → messages.rs, worker.rs, theme.rs
    ├── layout.rs → app.rs, theme.rs
    ├── worker.rs → messages.rs, rust_agent::*
    ├── messages.rs (standalone)
    └── theme.rs (standalone)
```

---

## Critical Files (Top 10)

**If you only read 10 files, read these:**

1. **CLAUDE.md** - Complete project instructions
2. **PROJECT_COMPLETE_SUMMARY.md** - Full documentation
3. **rusty_tui/src/gui/layout.rs** - Enter key fix ⚠️ CRITICAL
4. **rusty_tui/src/gui/worker.rs** - Main processing logic
5. **src/file_generator.rs** - File creation system
6. **src/knowledge/database.rs** - Database schema
7. **src/knowledge/query.rs** - Search implementation
8. **rusty_tui/src/gui/app.rs** - Application state
9. **src/claude_proxy.rs** - API integration
10. **knowledge/rust_core_concepts.json** - Knowledge example

---

## File Modification History

### Most Recently Modified (v12.0.1)

1. **rusty_tui/src/gui/messages.rs** - Added FilesCreated variant
2. **rusty_tui/src/gui/worker.rs** - Integrated AutoFileCreator
3. **rusty_tui/src/gui/app.rs** - Added created_files tracking
4. **IMPLEMENTATION_SUMMARY.md** - Updated with GUI integration
5. **TEST_VERIFICATION.md** - Created test plan

### Last Major Refactor (v12.0.0)

- Complete TUI → GUI migration
- Removed: app.rs, ui.rs, chat.rs, input.rs (old TUI code)
- Added: gui/ directory with new architecture

---

**Last Updated**: 2026-03-01
**Version**: v12.0.1
**Total Project Size**: ~10,400 lines of code across 42 files
**Status**: ✅ Production Ready
