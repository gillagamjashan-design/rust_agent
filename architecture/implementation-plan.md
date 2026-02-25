# Implementation Plan

## Overview

Simple plan to build `rusty` - a beautiful TUI that shows only the agent chat with instant knowledge database access.

**Timeline**: 1-2 days
**Approach**: Build TUI, integrate existing knowledge system, delete unnecessary files

---

## Phase 1: Clean Up (2 hours)

### Delete Unnecessary Shell Scripts

**Keep only:**
- `start_cliproxyapi.sh`

**Delete:**
- `start_agent.sh`
- `test_agent.sh`
- `install.sh`
- `update.sh`
- `spawn_teachers.sh`
- `answer_teacher.sh`
- `orchestrator.sh`
- `test_websearch.sh`
- `rusty_launcher.sh`
- `start_agent_daemon.sh`
- Any other .sh files

### Remove Unnecessary Directories

**Keep:**
- `src/knowledge/` - Knowledge database (already built)
- `src/tools/` - Knowledge fetcher (already built)
- `src/claude_proxy.rs` - API client
- `src/web_search/` - DuckDuckGo fallback
- `knowledge/` - JSON files

**Consider removing (not needed for TUI-only):**
- `src/training/` - Old batch learning (not used)
- `src/memory/` - Complex memory systems (not needed)
- `src/orchestration/` - Workflows (not needed)
- `src/interfaces/` - Old interfaces
- `rusty_ide/` - Old IDE
- `rusty_ide_v2/` - Tauri app (not working)

---

## Phase 2: Build Beautiful TUI (6-8 hours)

### Step 1: Create rusty_tui Binary Crate

**File**: `rusty_tui/Cargo.toml`

```toml
[package]
name = "rusty"
version = "1.0.0"
edition = "2021"

[[bin]]
name = "rusty"
path = "src/main.rs"

[dependencies]
rust_agent = { path = ".." }
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
syntect = "5.0"
```

### Step 2: Main Entry Point

**File**: `rusty_tui/src/main.rs`

**Responsibilities:**
- Initialize knowledge database
- Launch ratatui TUI
- Handle startup errors
- Graceful shutdown

**Pseudocode:**
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize knowledge database
    let db = load_or_create_knowledge_database()?;

    // Create knowledge fetcher
    let fetcher = KnowledgeFetcher::new(KnowledgeQuery::new(db));

    // Create app state
    let mut app = App::new(fetcher);

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Main event loop
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if !app.handle_key(key) {
                break;  // User quit
            }
        }
    }

    // Cleanup
    restore_terminal()?;
    Ok(())
}
```

### Step 3: App State

**File**: `rusty_tui/src/app.rs`

**State:**
```rust
pub struct App {
    messages: Vec<Message>,
    input: String,
    fetcher: KnowledgeFetcher,
    claude: ClaudeProxy,
    scroll: usize,
    mode: InputMode,  // Normal, Command
}

pub struct Message {
    role: Role,  // User, Assistant
    content: String,
    timestamp: DateTime<Utc>,
}

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) -> bool;
    pub async fn send_message(&mut self);
    pub fn handle_command(&mut self, cmd: &str);
}
```

### Step 4: UI Rendering

**File**: `rusty_tui/src/ui.rs`

**Layout:**
```
┌────────────────────────────────────────────────┐
│ 🦀 Rusty          [53 items loaded] [12:34 PM] │
├────────────────────────────────────────────────┤
│                                                │
│ [Cyan] You: What is ownership in Rust?        │
│                                                │
│ [Green] Agent: Ownership is Rust's system...  │
│ ```rust                                        │
│ fn main() {                                    │
│     let s = String::from("hello");            │
│ }                                              │
│ ```                                            │
│                                                │
│ [Cyan] You: Can you show me the builder...    │
│ ...                                            │
│                                                │
│ [Scrollbar]                                    │
├────────────────────────────────────────────────┤
│ > Type your question... (/help for commands)  │
└────────────────────────────────────────────────┘
```

**Colors (Tokyo Night theme):**
- Background: #1a1b26
- User messages: #7aa2f7 (Cyan)
- Agent messages: #9ece6a (Green)
- Code blocks: Syntax highlighted
- Status bar: #414868 (Dark gray)

### Step 5: Chat Component

**File**: `rusty_tui/src/chat.rs`

**Responsibilities:**
- Render message history
- Scroll handling (↑/↓)
- Syntax highlighting for code blocks
- Markdown formatting

### Step 6: Input Component

**File**: `rusty_tui/src/input.rs`

**Features:**
- Text input
- Command detection (`/` prefix)
- Multi-line support (Shift+Enter)
- History (↑/↓ arrows)
- Auto-complete for commands

### Step 7: Commands

**File**: `rusty_tui/src/commands.rs`

**Commands:**
```rust
pub enum Command {
    Help,                   // /help
    Search(String),         // /search <query>
    Stats,                  // /stats
    Web(String),            // /web <query>
    Clear,                  // /clear
    Quit,                   // /quit
}

impl Command {
    pub fn parse(input: &str) -> Option<Command>;
    pub async fn execute(&self, app: &mut App) -> Result<()>;
}
```

**Command Implementations:**
- `/help` → Show command list in chat
- `/search ownership` → Query database, show results table
- `/stats` → Show: "53 items loaded (13 concepts, 18 patterns, 22 commands)"
- `/web rust 2026` → Force web search, show results
- `/clear` → Clear chat history
- `/quit` → Exit TUI

---

## Phase 3: Integration (2-3 hours)

### Connect Knowledge Database

The knowledge system is already built (`src/knowledge/`), just use it:

```rust
// In main.rs
use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeLoader, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;

let db_path = dirs::home_dir()
    .unwrap()
    .join(".agent/data/knowledge.db");

let db = if db_path.exists() {
    KnowledgeDatabase::new(&db_path)?
} else {
    // First run - load from JSON
    let db = KnowledgeDatabase::new(&db_path)?;
    let loader = KnowledgeLoader::new(db);
    loader.load_all_from_directory("knowledge")?;
    db
};

let fetcher = KnowledgeFetcher::new(KnowledgeQuery::new(db));
```

### Connect Claude API

Use existing `claude_proxy.rs`:

```rust
use rust_agent::claude_proxy::ClaudeProxy;

let claude = ClaudeProxy::new("http://localhost:8317")?;
```

### Agent Response Flow

```rust
async fn handle_user_query(query: &str) -> Result<String> {
    // 1. Fetch knowledge if needed
    let knowledge = fetcher.search(query)?;

    // 2. Build context
    let context = if knowledge.has_results() {
        format!("Relevant knowledge:\n{}", knowledge.formatted)
    } else {
        String::new()
    };

    // 3. Send to Claude
    let prompt = format!("{}\n\nUser question: {}", context, query);
    let response = claude.query(&prompt).await?;

    Ok(response)
}
```

---

## Phase 4: Installation (1 hour)

### Build and Install

```bash
# Build release binary
cd rusty_tui
cargo build --release

# Copy to PATH
cp target/release/rusty ~/.local/bin/rusty
chmod +x ~/.local/bin/rusty

# Test
rusty
```

### First Run Experience

```
User runs: rusty
    ↓
Check ~/.agent/data/knowledge.db
    ↓
Not found - first run!
    ↓
Show loading message:
"📚 Loading knowledge database (first time)..."
    ↓
Load JSON files (1-2 seconds)
    ↓
"✅ Loaded 53 items"
    ↓
Launch TUI
    ↓
"Welcome! Type your question or /help for commands"
```

### Subsequent Runs

```
User runs: rusty
    ↓
Load database (~50ms)
    ↓
Launch TUI (~200ms)
    ↓
Ready!
```

---

## Phase 5: Polish (2-3 hours)

### Syntax Highlighting

Use `syntect` with Tokyo Night theme:
```rust
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

let ss = SyntaxSet::load_defaults_newlines();
let ts = ThemeSet::load_defaults();
let theme = &ts.themes["base16-ocean.dark"];

// Highlight code block
let syntax = ss.find_syntax_by_extension("rs").unwrap();
let highlighted = highlight_lines(&code, syntax, theme);
```

### Markdown Rendering

Parse markdown and render:
- **Bold** → Bold style
- *Italic* → Italic style
- `code` → Inline code style
- ```rust code blocks ``` → Syntax highlighted
- Lists → Bullet points
- Links → Clickable (optional)

### Responsive Layout

- Adapt to terminal size
- Minimum size: 80x24
- Scroll when content exceeds viewport
- Status bar shows: `[80x24] [53 items] [12:34 PM]`

### Error Messages

Beautiful error display:
```
┌──────────────────────────────────────────┐
│ ❌ Error                                 │
├──────────────────────────────────────────┤
│ ClaudeProxyAPI not responding           │
│                                          │
│ Please start it with:                    │
│ ./start_cliproxyapi.sh                   │
└──────────────────────────────────────────┘
```

---

## Phase 6: Testing (2-3 hours)

### Manual Testing

1. **First run**: Verify database loads
2. **Search**: `/search ownership` shows results
3. **Query**: "What is ownership?" gets answer with knowledge
4. **Code generation**: "Write a TCP server" generates code
5. **Commands**: All `/` commands work
6. **Scrolling**: Long conversations scroll properly
7. **Syntax highlighting**: Code blocks are colored
8. **Resize**: Terminal resize works
9. **Quit**: `/quit` and Ctrl+C exit cleanly

### Edge Cases

- Empty database (no JSON files)
- ClaudeProxyAPI down
- Very long messages
- Special characters
- Invalid commands
- Network errors

---

## Final Checklist

- [✓] Delete all .sh files except start_cliproxyapi.sh
- [✓] Build rusty_tui with ratatui
- [✓] Beautiful colored interface (Tokyo Night)
- [✓] Syntax highlighting for code blocks
- [✓] Integrate knowledge database (already done)
- [✓] Chat history with scrolling
- [✓] Commands: /help, /search, /stats, /quit
- [✓] Install to ~/.local/bin/rusty
- [✓] First run loads knowledge database
- [✓] Subsequent runs instant startup
- [✓] Documentation updated
- [✓] README shows: `rusty` - that's it!

---

## Success Criteria

✅ User runs `rusty` → beautiful TUI opens
✅ User asks "What is ownership?" → gets answer with knowledge
✅ User asks agent to write code → gets syntax highlighted code
✅ Commands work: /search, /stats, /help, /quit
✅ Startup time: <500ms (after first run)
✅ Query response: <2s (database + API)
✅ Beautiful interface with colors and syntax highlighting
✅ No flags, no modes, no complexity

**Just one command: `rusty`**

That's the entire implementation plan!
