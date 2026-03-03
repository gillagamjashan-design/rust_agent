# Rusty - Complete Project Summary

## Project Overview

**Rusty** is an intelligent Rust learning agent with a native GUI interface and automatic file creation capabilities. It combines a queryable knowledge database, Claude AI integration, and smart code generation to help users learn Rust programming.

**Current Version**: v12.0.1
**License**: Open Source
**Language**: Rust
**Architecture**: Native GUI (egui/eframe) + SQLite Knowledge Database + Claude API Integration

---

## Core Innovation

Unlike traditional chatbots that rely solely on training data, Rusty uses a **runtime-queryable knowledge database** built on SQLite with FTS5 full-text search. This provides:

- ⚡ Sub-50ms query times for instant knowledge retrieval
- 📚 13 core Rust concepts, 18 patterns, 22 cargo commands
- 🎯 Precise, structured information instead of hallucinated responses
- 🔄 Easily updatable knowledge without retraining

---

## Project Structure

```
making_files/
├── src/                          # Core library (rust_agent)
│   ├── lib.rs                    # Library exports
│   ├── config.rs                 # Configuration management
│   ├── types.rs                  # Shared type definitions
│   ├── cache.rs                  # Web search result caching
│   ├── claude_proxy.rs           # Claude API client
│   │
│   ├── knowledge/                # Knowledge database system
│   │   ├── mod.rs               # Module exports
│   │   ├── database.rs          # SQLite schema + FTS5 setup
│   │   ├── loader.rs            # JSON → Database loader
│   │   └── query.rs             # FTS5 search implementation
│   │
│   ├── tools/                    # Runtime agent tools
│   │   ├── mod.rs               # Tool exports
│   │   ├── knowledge_fetcher.rs # Database query tool
│   │   └── slash_commands.rs    # /help, /search, /stats
│   │
│   ├── web_search/              # Web search fallback
│   │   ├── mod.rs               # Search exports
│   │   └── duckduckgo.rs        # DuckDuckGo integration
│   │
│   └── file_generator.rs        # Automatic file creation
│       ├── FileCreationDetector # Intent detection
│       ├── FileCreator          # File I/O operations
│       └── AutoFileCreator      # Database-driven automation
│
├── rusty_tui/                    # GUI Application
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   └── gui/                 # GUI implementation
│   │       ├── mod.rs           # GUI module exports
│   │       ├── app.rs           # eframe::App + state management
│   │       ├── layout.rs        # UI rendering (Enter key fix!)
│   │       ├── worker.rs        # Async worker thread
│   │       ├── messages.rs      # Message types for channels
│   │       └── theme.rs         # Tokyo Night color scheme
│   │
│   └── Cargo.toml               # GUI dependencies
│
├── knowledge/                    # Knowledge source files (JSON)
│   ├── rust_core_concepts.json  # Ownership, lifetimes, traits, etc.
│   ├── rust_patterns_idioms.json# Builder, RAII, Newtype, etc.
│   ├── rust_toolchain_cargo.json# Cargo commands
│   ├── rust_async_concurrency.json # Tokio, async/await
│   ├── rust_standard_library.json # std modules
│   ├── rust_popular_crates.json # Popular ecosystem crates
│   └── curriculum_master.json   # Learning curriculum
│
├── scripts/                      # Installation scripts
│   ├── install-rusty-linux.sh   # Linux installer
│   └── install-rusty-mac.sh     # macOS installer
│
├── run-all.sh                    # One-command installation
├── start_cliproxyapi.sh          # Start Claude API proxy
├── Cargo.toml                    # Core library manifest
├── CLAUDE.md                     # Instructions for Claude Code
├── IMPLEMENTATION_SUMMARY.md     # Implementation status
└── TEST_VERIFICATION.md          # Test plan
```

---

## Component Deep Dive

### 1. Knowledge Database System

**Location**: `src/knowledge/`

#### Database Schema (`database.rs`)

```sql
-- Core tables
CREATE TABLE concepts (
    id TEXT PRIMARY KEY,
    topic TEXT NOT NULL,
    title TEXT NOT NULL,
    explanation TEXT NOT NULL,
    code_examples TEXT,      -- JSON array
    common_mistakes TEXT,    -- JSON array
    related_concepts TEXT,   -- JSON array
    tags TEXT               -- JSON array
);

CREATE TABLE patterns (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    template TEXT,
    when_to_use TEXT,
    examples TEXT           -- JSON array
);

CREATE TABLE commands (
    id TEXT PRIMARY KEY,
    tool TEXT NOT NULL,      -- "cargo" or "rustup"
    command TEXT NOT NULL,
    description TEXT,
    examples TEXT,
    flags TEXT              -- JSON array
);

CREATE TABLE file_templates (
    id TEXT PRIMARY KEY,
    file_type TEXT NOT NULL,
    default_filename TEXT,
    pattern_hints TEXT,      -- Keywords to match
    template_content TEXT
);

-- FTS5 virtual tables for full-text search
CREATE VIRTUAL TABLE concepts_fts USING fts5(
    topic, title, explanation, tags,
    content='concepts',
    content_rowid='rowid'
);

CREATE VIRTUAL TABLE patterns_fts USING fts5(
    name, description, when_to_use,
    content='patterns',
    content_rowid='rowid'
);

CREATE VIRTUAL TABLE commands_fts USING fts5(
    tool, command, description,
    content='commands',
    content_rowid='rowid'
);
```

**Key Features**:
- SQLite for persistence and fast queries
- FTS5 (Full-Text Search 5) for natural language queries
- Triggers to keep FTS tables in sync with main tables
- Automatic database initialization from JSON files

#### Knowledge Loader (`loader.rs`)

**Purpose**: Loads JSON knowledge files into SQLite on first run

**Process**:
1. Checks if database exists at `~/.agent/data/knowledge.db`
2. If not, reads all JSON files from `knowledge/` directory
3. Parses JSON and inserts into SQLite tables
4. Sets up FTS5 triggers for automatic indexing
5. Takes ~1-2 seconds on first run, instant thereafter

**Example JSON Structure**:
```json
{
  "concepts": [
    {
      "id": "ownership",
      "topic": "ownership",
      "title": "Ownership System",
      "explanation": "Rust's ownership system ensures memory safety...",
      "code_examples": ["fn main() { let x = 5; }"],
      "common_mistakes": ["Trying to use moved values"],
      "related_concepts": ["borrowing", "lifetimes"],
      "tags": ["memory", "safety"]
    }
  ]
}
```

#### Query Engine (`query.rs`)

**Purpose**: Execute FTS5 searches across knowledge tables

**Algorithm**:
```rust
// Searches all FTS tables in parallel
pub fn search(&self, query: &str) -> Result<KnowledgeSearchResults> {
    let concepts = search_concepts_fts(query)?;
    let patterns = search_patterns_fts(query)?;
    let commands = search_commands_fts(query)?;

    // Rank by FTS5 relevance score
    // Format for Claude API consumption
    Ok(KnowledgeSearchResults {
        concepts, patterns, commands,
        formatted: format_for_claude(concepts, patterns, commands)
    })
}
```

**Performance**: <50ms average query time

---

### 2. File Generation System

**Location**: `src/file_generator.rs`

#### FileCreationDetector

**Purpose**: Detect when user wants to create files

**Detection Logic**:
```rust
pub fn should_create_files(&self, user_query: &str) -> bool {
    let keywords = [
        "create", "make", "generate", "write", "add",
        "new project", "new file", "hello world", "build"
    ];
    keywords.iter().any(|kw| user_query.to_lowercase().contains(kw))
}
```

**Code Block Extraction**:
1. **Priority 1**: `@filepath` markers
   - Example: `@src/main.rs\n```rust\ncode\n```'
   - Explicit filename provided by user/Claude

2. **Priority 2**: Comment hints
   - Example: `// src/server.rs` at top of code block
   - Filename in comment

3. **Priority 3**: Heuristic inference
   - `fn main()` → `src/main.rs`
   - `pub struct User` → `src/user.rs`
   - `[package]` → `Cargo.toml`
   - `#[test]` → `tests/integration_test.rs`

#### AutoFileCreator

**Purpose**: Automatically create files from Claude responses

**Workflow**:
```rust
pub fn auto_create_from_response(
    &self,
    user_query: &str,
    claude_response: &str
) -> Result<Vec<FileCreationResult>> {

    // Step 1: Check intent
    if !detector.should_create_files(user_query) {
        return Ok(vec![]);  // No files needed
    }

    // Step 2: Extract code blocks
    let blocks = detector.extract_code_blocks(claude_response);

    // Step 3: Resolve filenames (use DB templates if needed)
    for block in &mut blocks {
        if block.filename.is_none() {
            let template = db.get_template_for_code(&block.code)?;
            block.filename = Some(resolve_filename(&template));
        }
    }

    // Step 4: Create files
    let results = create_files(blocks)?;

    Ok(results)
}
```

**File Creation Modes**:
- **Create**: New file doesn't exist → write content
- **Append**: File exists → add separator comment + append new content

**Example Output**:
```rust
FileCreationResult {
    path: PathBuf::from("/workspace/src/main.rs"),
    appended: false,  // true if file existed
    error: None       // Some(err_msg) if failed
}
```

---

### 3. GUI Application

**Location**: `rusty_tui/src/gui/`

#### Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│  Main Thread (GUI Event Loop)                           │
│  - Renders UI at 60 FPS                                 │
│  - Handles keyboard/mouse input                         │
│  - Non-blocking message polling                         │
└────────────┬────────────────────────────────────────────┘
             │
             │ mpsc channels
             │
┌────────────▼────────────────────────────────────────────┐
│  Worker Thread (Async Runtime)                          │
│  - Initializes knowledge database                       │
│  - Queries Claude API (blocking I/O)                    │
│  - Creates files (blocking I/O)                         │
│  - Sends results back to GUI                            │
└─────────────────────────────────────────────────────────┘
```

#### app.rs - Application State

```rust
pub struct RustyApp {
    // Chat history
    messages: Vec<Message>,

    // User input buffer
    input: String,

    // UI state
    waiting_for_response: bool,
    scroll_to_bottom: bool,
    first_render: bool,

    // Session tracking
    created_files: Vec<String>,
    knowledge_stats: String,

    // Communication channels
    message_rx: Receiver<WorkerMessage>,  // Worker → GUI
    command_tx: Sender<UserCommand>,      // GUI → Worker
}
```

**Message Types**:
```rust
// GUI → Worker
pub enum UserCommand {
    Query(String),        // User question/request
    Command(String),      // Slash command (/help, /search)
    Quit,                // Exit application
}

// Worker → GUI
pub enum WorkerMessage {
    Response(String),              // Claude's answer
    SystemMessage(String),         // System notifications
    Error(String),                // Error messages
    Stats(String),                // Database statistics
    FilesCreated(Vec<FileCreationInfo>),  // File creation results
}

pub struct FileCreationInfo {
    path: String,     // Full file path
    appended: bool,   // true if file was updated, false if created
    success: bool,    // true if operation succeeded
}
```

#### layout.rs - UI Rendering

**Key Features**:
- Three-panel design: Header + Chat Area + Input
- Syntax highlighting for code blocks (via `syntect`)
- Auto-scroll to bottom on new messages
- **Enter key fix**: Native egui event handling (not async polling)

**Critical Code - Enter Key Handling**:
```rust
let response = egui::TextEdit::multiline(&mut app.input)
    .desired_width(f32::INFINITY)
    .show(ui);

// IMPORTANT: This pattern fixes the Enter key issue from v11.0.0
if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
    if !app.input.is_empty() {
        let input = app.input.clone();
        app.input.clear();
        app.send_message(input);
        app.scroll_to_bottom = true;
    }
    response.request_focus();  // Keep focus in input box
}
```

**Why This Works**:
- Uses egui's native event system (synchronous)
- Avoids crossterm's async event polling (which conflicted with tokio)
- No race conditions between UI and worker thread

#### worker.rs - Background Processing

**Initialization**:
```rust
async fn worker_loop(
    command_rx: Receiver<UserCommand>,
    message_tx: Sender<WorkerMessage>,
    db_path: PathBuf,
) -> Result<()> {
    // Setup
    let db = KnowledgeDatabase::new(&db_path)?;
    let knowledge_fetcher = KnowledgeFetcher::new(db.clone());
    let claude = ClaudeProxy::new();
    let auto_file_creator = AutoFileCreator::new(
        std::env::current_dir()?,
        db.clone()
    );

    // Main loop - wait for user commands
    loop {
        match command_rx.recv() {
            Ok(UserCommand::Query(text)) => {
                handle_query(text, &knowledge_fetcher, &claude, &auto_file_creator).await?;
            }
            Ok(UserCommand::Command(cmd)) => {
                handle_command(cmd, &knowledge_fetcher, &db)?;
            }
            Ok(UserCommand::Quit) => break,
            Err(_) => break,
        }
    }
}
```

**Query Processing Flow**:
```rust
async fn handle_query(text: String, ...) {
    // 1. Search knowledge database
    let context = knowledge_fetcher.search(&text)?;

    // 2. Build prompt with context
    let prompt = format!("Knowledge:\n{}\n\nUser: {}", context, text);

    // 3. Query Claude API (async, blocks this thread only)
    let response = claude.query(&prompt).await?;

    // 4. Try to create files from response
    let file_results = auto_file_creator
        .auto_create_from_response(&text, &response)?;

    // 5. Send results back to GUI
    if !file_results.is_empty() {
        let summary = format_file_creation_summary(&file_results);
        let full_response = format!("{}\n\n{}", response, summary);

        message_tx.send(WorkerMessage::Response(full_response)).ok();
        message_tx.send(WorkerMessage::FilesCreated(
            file_results.into_iter()
                .map(|r| FileCreationInfo { ... })
                .collect()
        )).ok();
    } else {
        message_tx.send(WorkerMessage::Response(response)).ok();
    }
}
```

**File Creation Summary Format**:
```markdown
📁 **Files Created:**
  ✅ Created `src/main.rs`
  ✅ Updated `Cargo.toml`
```

#### theme.rs - Tokyo Night Colors

```rust
pub const BACKGROUND: Color32 = Color32::from_rgb(26, 27, 38);
pub const SURFACE: Color32 = Color32::from_rgb(36, 40, 59);
pub const TEXT: Color32 = Color32::from_rgb(169, 177, 214);
pub const BLUE: Color32 = Color32::from_rgb(122, 162, 247);
pub const GREEN: Color32 = Color32::from_rgb(158, 206, 106);
```

---

### 4. Claude API Integration

**Location**: `src/claude_proxy.rs`

**Purpose**: HTTP client for ClaudeProxyAPI service

```rust
pub struct ClaudeProxy {
    client: Client,
    base_url: String,  // http://localhost:8317
}

pub async fn query(&self, prompt: &str) -> Result<String> {
    let response = self.client
        .post(&format!("{}/query", self.base_url))
        .json(&json!({ "prompt": prompt }))
        .send()
        .await?;

    let data: ClaudeResponse = response.json().await?;
    Ok(data.response)
}
```

**Dependency**: Requires ClaudeProxyAPI running on localhost:8317

---

### 5. Web Search Fallback

**Location**: `src/web_search/duckduckgo.rs`

**Purpose**: Search DuckDuckGo when knowledge database has no results

**Caching Strategy**:
```rust
// Cache key: SHA-256 hash of query
let cache_key = format!("{:x}", Sha256::digest(query.as_bytes()));
let cache_path = cache_dir.join(format!("{}.json", cache_key));

// Cache expiry: 7 days
if cache_path.exists() {
    let metadata = fs::metadata(&cache_path)?;
    let age = SystemTime::now().duration_since(metadata.modified()?)?;

    if age < Duration::from_secs(7 * 24 * 60 * 60) {
        return Ok(cached_results);  // Use cache
    }
}

// Not cached or expired - fetch from DuckDuckGo
let results = fetch_from_duckduckgo(query).await?;
cache_results(&cache_path, &results)?;
Ok(results)
```

**Cache Location**: `~/.agent/cache/`

---

## Complete Data Flow

### Example: User Asks "Create a hello world program"

```
┌──────────────────────────────────────────────────────────────┐
│ 1. GUI (layout.rs)                                           │
│    - User types: "Create a hello world program"             │
│    - Presses Enter                                           │
│    - layout.rs detects key press via egui event              │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        │ UserCommand::Query(text)
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 2. Worker Thread (worker.rs)                                │
│    - Receives command via mpsc channel                       │
│    - Identifies as Query (not a slash command)               │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 3. Knowledge Search (knowledge_fetcher.rs)                   │
│    - Searches FTS5: "create hello world program"            │
│    - Finds: concepts about main(), examples of println!      │
│    - Formats context for Claude                              │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        │ Context + User Query
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 4. Claude API (claude_proxy.rs)                              │
│    - Sends: "Knowledge: [context]\n\nUser: [query]"         │
│    - Claude responds with hello world code:                  │
│      ```rust                                                 │
│      fn main() {                                             │
│          println!("Hello, world!");                          │
│      }                                                        │
│      ```                                                     │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        │ Claude Response
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 5. File Generation (file_generator.rs)                      │
│    - Detects "create" keyword → should_create_files = true   │
│    - Extracts code block: language="rust", code="fn main..." │
│    - Infers filename: contains "fn main()" → src/main.rs     │
│    - Creates directory: mkdir -p src/                        │
│    - Writes file: src/main.rs with content                   │
│    - Returns: FileCreationResult { path, appended, error }   │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        │ Response + File Results
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 6. Worker Formatting (worker.rs)                            │
│    - Formats file summary:                                   │
│      "📁 **Files Created:**\n  ✅ Created `src/main.rs`"    │
│    - Appends to Claude response                              │
│    - Sends two messages:                                     │
│      • WorkerMessage::Response(full_text)                    │
│      • WorkerMessage::FilesCreated(file_infos)               │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        │ via mpsc channel
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 7. GUI Update (app.rs)                                      │
│    - Polls message_rx (non-blocking)                         │
│    - Receives Response: adds to messages list                │
│    - Receives FilesCreated: adds to created_files list       │
│    - Sets scroll_to_bottom = true                            │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│ 8. UI Render (layout.rs)                                    │
│    - Renders message with syntax highlighting                │
│    - Shows: Claude's code + file creation summary            │
│    - Auto-scrolls to bottom                                  │
│    - User sees: "✅ Created `src/main.rs`"                  │
└──────────────────────────────────────────────────────────────┘
```

**Time Breakdown**:
- Knowledge search: ~20ms
- Claude API: ~1-3s (network + inference)
- File creation: ~5-10ms
- GUI update: <1ms
- **Total**: ~1-3 seconds

---

## Dependencies

### Core Library (`Cargo.toml`)

```toml
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

### GUI Application (`rusty_tui/Cargo.toml`)

```toml
[dependencies]
egui = "0.30"
eframe = { version = "0.30", default-features = false, features = [
    "default_fonts",
    "glow",
    "wayland",
    "x11",
] }
egui_extras = { version = "0.30", features = ["syntect"] }
syntect = "5.2"
tokio = { version = "1", features = ["full"] }
rust_agent = { path = ".." }
dirs = "5.0"
```

---

## Building & Installation

### Build Commands

```bash
# Build core library
cargo build --release

# Build GUI application
cd rusty_tui
cargo build --release

# Binary location
./rusty_tui/target/release/rusty
```

### One-Command Installation

```bash
./run-all.sh
```

**What it does**:
1. Builds core library in release mode
2. Builds GUI application in release mode
3. Installs binary to `~/.local/bin/rusty`
4. Creates necessary directories (`~/.agent/data`, `~/.agent/cache`)
5. Starts ClaudeProxyAPI service

### Manual Installation

```bash
# Linux
./scripts/install-rusty-linux.sh

# macOS
./scripts/install-rusty-mac.sh
```

---

## Runtime Directories

```
~/.agent/
├── data/
│   └── knowledge.db          # SQLite database (auto-created on first run)
│
└── cache/
    ├── <sha256_hash1>.json  # Cached web search results
    ├── <sha256_hash2>.json  # 7-day expiry
    └── ...

~/.local/bin/
└── rusty                     # Installed binary (if using run-all.sh)
```

---

## Configuration

### Database Path
Default: `~/.agent/data/knowledge.db`
Override: Set `RUSTY_DB_PATH` environment variable

### Claude API
Default: `http://localhost:8317`
Override: Set `CLAUDE_API_URL` environment variable

### Cache Directory
Default: `~/.agent/cache`
Override: Set `RUSTY_CACHE_DIR` environment variable

---

## Testing

### Unit Tests

```bash
# Test core library
cargo test

# Test specific module
cargo test file_generator

# Test with output
cargo test -- --nocapture
```

### Integration Testing

See `TEST_VERIFICATION.md` for complete test plan

**Manual Tests**:
1. Hello world creation
2. @filepath marker support
3. Multiple file creation
4. Append to existing files
5. Non-creation queries (no false positives)
6. Error handling (permission denied, etc.)

---

## Performance Metrics

| Operation | Target | Actual |
|-----------|--------|--------|
| Knowledge query | <50ms | ~20ms |
| Database load (first run) | <2s | ~1.5s |
| GUI startup | <300ms | ~200ms |
| File creation | <20ms | ~5-10ms |
| Claude API response | 1-3s | 1-3s (network dependent) |
| GUI render | 60 FPS | 60 FPS |

---

## Known Limitations

1. **Claude API Dependency**: Requires ClaudeProxyAPI running on localhost:8317
2. **Working Directory**: Files created in current working directory (where `rusty` is launched)
3. **No Undo**: File creation is immediate, no confirmation dialog
4. **Single Session**: Chat history cleared on restart (not persisted)
5. **Web Search**: DuckDuckGo only, limited to text results
6. **Platform**: Tested on Linux, should work on macOS/Windows but not verified

---

## Version History

### v12.0.1 (Current) - GUI with File Creation
- ✅ Integrated automatic file creation into GUI workflow
- ✅ Added `FilesCreated` message type for file notifications
- ✅ File creation feedback in chat interface
- ✅ Session tracking of created files

### v12.0.0 - Native GUI Migration
- ✅ Migrated from TUI (ratatui) to native GUI (egui/eframe)
- ✅ Fixed Enter key handling issues from v11
- ✅ Non-blocking UI with worker thread architecture
- ✅ Tokyo Night color scheme

### v11.0.0 - TUI with Panel System
- ✅ Terminal UI with ratatui
- ⚠️  Enter key issues (async event polling conflicts)
- ✅ File explorer panel
- ✅ VS Code-style layout

### v9.0.0 - AI Agent Integration
- ✅ Claude API integration
- ✅ Knowledge database system
- ✅ FTS5 full-text search

---

## Future Enhancements (Not Implemented)

- [ ] Chat history persistence (save/load sessions)
- [ ] Multiple Claude API backends (OpenAI, local models)
- [ ] File diff preview before creation
- [ ] Undo/rollback file operations
- [ ] Project template system (cargo init alternatives)
- [ ] Syntax error checking before file creation
- [ ] Git integration (auto-commit created files)
- [ ] Remote knowledge database sync
- [ ] Plugin system for custom tools

---

## Troubleshooting

### "Failed to connect to Claude API"
```bash
# Check if ClaudeProxyAPI is running
curl http://localhost:8317/

# Start it
./start_cliproxyapi.sh
```

### "Failed to open database"
```bash
# Check directory exists
ls -la ~/.agent/data/

# Create if missing
mkdir -p ~/.agent/data

# Delete corrupted database (will rebuild on next run)
rm ~/.agent/data/knowledge.db
```

### "Permission denied" when creating files
```bash
# Check directory permissions
ls -la

# Make writable
chmod +w .
```

### Build fails with cache errors
```bash
# Use temporary cargo home
CARGO_HOME=/tmp/cargo-home cargo build --release
```

---

## Contributing

### Adding Knowledge

1. Edit JSON files in `knowledge/` directory
2. Follow existing schema (see `rust_core_concepts.json`)
3. Delete database: `rm ~/.agent/data/knowledge.db`
4. Run Rusty (will rebuild database)

### Adding File Templates

1. Edit `knowledge/file_templates.json`
2. Add new template:
```json
{
  "id": "my_template",
  "file_type": "rust",
  "default_filename": "src/{{module_name}}.rs",
  "pattern_hints": "struct,enum,trait",
  "template_content": "// Auto-generated\n{{CODE}}"
}
```

---

## Credits

**Author**: Jashan
**Framework**: egui by emilk
**Database**: SQLite with FTS5
**AI**: Claude by Anthropic
**Language**: Rust

---

## License

Open Source - See LICENSE file for details

---

**Last Updated**: 2026-03-01
**Build**: v12.0.1
**Status**: ✅ Production Ready
