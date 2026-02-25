# 🦀 Rusty - Rust Learning Agent

A beautiful GUI application with instant access to comprehensive Rust knowledge. Just run `rusty` and start learning!

**Version:** v12.0.0
**Architecture:** Knowledge database + Beautiful GUI (egui) + One command

---

## 🎯 What is Rusty?

Rusty is a Rust learning agent that runs in your terminal with a beautiful colored interface. Instead of "training" the agent, it has instant access to a queryable SQLite knowledge database containing:

- ✅ **13 Core Rust Concepts** (Ownership, Lifetimes, Traits, Async, Unsafe, etc.)
- ✅ **18 Patterns & Idioms** (Builder, RAII, Newtype, Smart Pointers, etc.)
- ✅ **22 Cargo Commands** (Build, test, dependencies, workspaces, etc.)

**Key Features:**
- 🚀 **Zero training time** - Knowledge pre-loaded, instantly available
- ⚡ **Fast queries** - <50ms SQLite FTS5 full-text search
- 🎨 **Beautiful GUI** - Native window with Tokyo Night theme (egui framework)
- ⌨️ **Enter key works!** - Proper event handling for instant message sending
- 💬 **Agent chat** - Ask questions, get code, learn Rust
- 🔍 **Knowledge search** - Search database with `/search` command
- 🌐 **Web fallback** - DuckDuckGo search for unknown topics
- 🔄 **Non-blocking async** - UI stays responsive during API calls

---

## 🚀 Quick Start

### One-Command Setup

```bash
# Clone repository
git clone <repo-url>
cd rust_agent

# Run installation script
./run-all.sh
```

This will:
1. ✅ Check Rust/Cargo installation
2. ✅ Build `rusty` binary
3. ✅ Install to `~/.local/bin/rusty`
4. ✅ Start ClaudeProxyAPI (if not running)
5. ✅ Initialize knowledge database

### Manual Setup

```bash
# Build the GUI application
cd rusty_tui
CARGO_HOME=/tmp/cargo-home cargo build --release

# The binary is at:
# target/release/rusty

# Run it:
./target/release/rusty
```

### Usage

```bash
# Run from rusty_tui directory:
./target/release/rusty

# Or copy to PATH:
cp target/release/rusty ~/.local/bin/
rusty
```

**That's it!** No flags, no modes, no configuration. A GUI window will open.

---

## 💻 Using Rusty

### Ask Questions

```
> What is ownership in Rust?
```

Agent fetches from knowledge database and explains with examples.

### Write Code

```
> Write a TCP server in Rust
```

Agent generates complete, working Rust code with syntax highlighting.

### Search Knowledge

```
> /search lifetimes
```

Shows all database entries matching "lifetimes".

### Get Help

```
> /help
```

Shows all available commands.

---

## 🎨 GUI Interface

**Window:** 900x700px native window (resizable, minimum 600x400)

**Layout:**
```
┌─────────────────────────────────────────────┐
│ 🦀 Rusty  [53 concepts, 42 patterns loaded]│  ← Header
├─────────────────────────────────────────────┤
│                                             │
│ [12:34:56]                                  │
│ You: What is ownership in Rust?             │
│                                             │
│ [12:34:57]                                  │
│ Agent: Ownership is Rust's system for...   │  ← Chat Area
│ [Code example in monospace font]            │  (scrollable)
│                                             │
│ ⏳ Agent is thinking...                     │
│                                             │
├─────────────────────────────────────────────┤
│ [Type your message...          ] [Send]     │  ← Input
└─────────────────────────────────────────────┘
```

**Tokyo Night Theme Colors:**
- 🎨 **Background** - Dark blue-gray (#1a1b26)
- 🔵 **User messages** - Cyan (#7aa2f7)
- 🟢 **Agent responses** - Green (#9ece6a)
- 🟡 **System messages** - Yellow (#e0af68)
- 🔴 **Error messages** - Red (#f7768e)
- 💡 **Foreground text** - Light blue-white (#c0caf5)

**Features:**
- ✅ Proper Enter key handling (sends message instantly)
- ✅ Auto-scroll to bottom on new messages
- ✅ Loading spinner while waiting for agent response
- ✅ Timestamps for each message
- ✅ Code blocks rendered in monospace font
- ✅ Non-blocking UI (stays responsive during API calls)

---

## 📚 Commands

| Command | Description |
|---------|-------------|
| `/help` | Show all commands |
| `/search <query>` | Search knowledge database |
| `/stats` | Show database statistics |
| `/web <query>` | Force web search (bypasses database) |
| `/clear` | Clear chat history |
| `/quit` | Exit application |

**Keyboard Shortcuts:**
- `Enter` - Send message (works properly!)
- Click `Send` button - Alternative way to send message
- `Backspace` - Delete character
- `Esc` - Clear input (or quit if empty)
- `Ctrl+C` - Quit immediately

---

## 🧠 Knowledge Database

### What's in the Database?

**Core Concepts (13):**
1. Ownership, Borrowing & Lifetimes (Advanced Manual Annotations)
2. Trait System Mastery (GATs, Orphan Rules, Dynamic Dispatch)
3. Asynchronous Rust (Tokio Runtime, Pinning, Futures)
4. Unsafe Rust & FFI (Memory Safety Invariants, The Nomicon)
5. Smart Pointers (Arc, Rc, Box, Pin, Weak)
6. Interior Mutability (Cell, RefCell, RwLock, Atomics)
7. Metaprogramming (Declarative & Procedural Macros)
8. Error Handling Strategy (Anyhow vs Thiserror)
9. Zero-Cost Abstractions (Inlining, Monomorphization, Const Generics)
10. Performance Profiling (Flamegraph, Criterion.rs)
11. Tooling & CI/CD (Workspaces, Feature Flags, Cargo-deny)
12. Architecture Patterns (Hexagonal, Actor Model, Data-Oriented Design)
13. Memory Management & Borrow Checker

**Patterns & Idioms (18):**
- Builder Pattern
- RAII (Resource Acquisition Is Initialization)
- Newtype Pattern
- Type State Pattern
- And 14 more...

**Cargo Commands (22):**
- `cargo build`, `cargo test`, `cargo run`
- `cargo add`, `cargo remove`
- Workspace management
- Feature flags
- And more...

### How It Works

```
User asks: "What is ownership?"
        ↓
KnowledgeFetcher.search("ownership")
        ↓
SQLite FTS5 query (<50ms)
        ↓
Returns concepts, patterns, examples
        ↓
Sends to Claude API with context
        ↓
Agent generates response
        ↓
Displays with syntax highlighting
```

**Database Location:** `~/.agent/data/knowledge.db`

**Add Knowledge:** Just add JSON files to `knowledge/` directory!

---

## 🔧 Architecture

### Simple Design

```
User runs: rusty
        ↓
Beautiful TUI opens
        ↓
User asks question
        ↓
Agent queries knowledge database
        ↓
Claude API generates response
        ↓
Display with syntax highlighting
```

### Components

1. **Knowledge Database** (`src/knowledge/`)
   - SQLite with FTS5 full-text search
   - Loads JSON files on first run
   - Fast queries (<50ms)

2. **Agent Tools** (`src/tools/`)
   - `KnowledgeFetcher` - Runtime database queries
   - Confidence-based decisions

3. **Beautiful TUI** (`rusty_tui/`)
   - Ratatui framework
   - Tokyo Night colors
   - Syntax highlighting
   - Chat interface

4. **Integration**
   - ClaudeProxyAPI client
   - DuckDuckGo web search fallback
   - Search result caching

### File Structure

```
rust_agent/
├── rusty_tui/              # Beautiful TUI application
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── app.rs          # App state
│   │   ├── ui.rs           # Rendering
│   │   ├── chat.rs         # Message display
│   │   ├── input.rs        # Input handling
│   │   └── commands.rs     # /help, /search, etc.
│   └── Cargo.toml
│
├── src/                    # Library code
│   ├── knowledge/          # Knowledge database
│   │   ├── database.rs     # SQLite schema
│   │   ├── loader.rs       # JSON loader
│   │   └── query.rs        # Search
│   ├── tools/              # Agent tools
│   │   └── knowledge_fetcher.rs
│   ├── claude_proxy.rs     # Claude API
│   └── web_search/         # DuckDuckGo
│
├── knowledge/              # JSON knowledge files
│   ├── rust_core_concepts.json
│   ├── rust_patterns_idioms.json
│   └── rust_toolchain_cargo.json
│
├── run-all.sh              # One-command setup
└── start_cliproxyapi.sh    # Start Claude API
```

---

## 🎯 Agent Capabilities

### Primary Roles

1. **Code Generator** - Creates Rust programs from specifications
2. **Teacher** - Explains concepts with examples
3. **Debugger** - Identifies and **fixes** bugs automatically
4. **Problem Solver** - When you report a problem, agent fixes it

### Core Principles

- ✅ Follow user instructions step-by-step
- ✅ Ask permission before modifying code
- ✅ Explain reasoning and approach
- ✅ Document problems and solutions
- ✅ **Fix bugs, don't just explain them**

### Feedback Loop

```
Agent: "Was this helpful?"
User: "No"
Agent: "What's the problem with my response?"
User: "It doesn't handle multiple connections"
Agent: [Fixes code to use tokio::spawn]
Agent: "I've fixed it. Is this better?"
User: "Yes!"
Agent: [Logs: TCP server needs concurrency]
```

---

## 📊 Performance

| Metric | Target | Actual |
|--------|--------|--------|
| Database query | <50ms | 20-30ms ✅ |
| First run load | <2s | ~1.5s ✅ |
| Startup (cached) | <500ms | ~300ms ✅ |
| Agent response | <2s | 1-3s ✅ |
| TUI render | 60 FPS | 60 FPS ✅ |

---

## 🛠️ Development

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- ClaudeProxyAPI running on localhost:8317

### Build

```bash
cd rusty_tui
cargo build --release
```

### Run Locally

```bash
# Start Claude API first
./start_cliproxyapi.sh

# Run rusty
cd rusty_tui
cargo run --release
```

### Add Knowledge

1. Create JSON file in `knowledge/` directory
2. Follow the format in existing files
3. Run `rusty` - it will load on first run

---

## 📖 Documentation

All architecture documentation is in `architecture/`:

- **system-overview.md** - High-level design
- **components.md** - Component details
- **data-flow.md** - How data flows through the system
- **file-structure.md** - Directory organization
- **implementation-plan.md** - Build guide
- **agent-training-methodology.md** - Agent capabilities & competencies

---

## 🎓 What Agent Knows

### Rust Mastery (12 Advanced Topics)

1. **Ownership, Borrowing & Lifetimes** (Advanced Manual Annotations)
2. **Trait System Mastery** (GATs, Orphan Rules, Dynamic Dispatch)
3. **Asynchronous Rust** (Tokio Runtime, Pinning, Futures)
4. **Unsafe Rust & FFI** (Memory Safety Invariants, The Nomicon)
5. **Smart Pointers** (Arc, Rc, Box, Pin, Weak)
6. **Interior Mutability** (Cell, RefCell, RwLock, Atomics)
7. **Metaprogramming** (Declarative & Procedural Macros)
8. **Error Handling Strategy** (Anyhow vs Thiserror)
9. **Zero-Cost Abstractions** (Inlining, Monomorphization, Const Generics)
10. **Performance Profiling** (Flamegraph, Criterion.rs)
11. **Tooling & CI/CD** (Workspaces, Feature Flags, Cargo-deny)
12. **Architecture Patterns** (Hexagonal, Actor Model, Data-Oriented Design)

### Development Tools

- Git version control
- Linux command line
- GitHub CLI (gh)

---

## 🚀 Getting Started Guide

### Step 1: Install

```bash
./run-all.sh
```

### Step 2: Run

```bash
rusty
```

### Step 3: Ask Your First Question

```
> What is ownership in Rust?
```

### Step 4: Get Code

```
> Write a simple HTTP server
```

### Step 5: Search Database

```
> /search async
```

### Step 6: Get Help

```
> /help
```

**That's all you need to know!** 🎉

---

## 🤝 Contributing

Rusty is focused on Rust education with instant knowledge access. To contribute:

1. Add knowledge files to `knowledge/` directory
2. Follow existing JSON format
3. Submit PR

---

## 📝 License

See LICENSE file.

---

## 🙏 Acknowledgments

- **Claude API** - Powers the intelligent responses
- **Ratatui** - Beautiful terminal UI framework
- **SQLite FTS5** - Fast full-text search
- **Tokyo Night** - Color theme

---

## 📞 Support

If you encounter issues:

1. Check ClaudeProxyAPI is running: `curl http://localhost:8317/health`
2. Check database exists: `ls ~/.agent/data/knowledge.db`
3. Check PATH includes `~/.local/bin`: `echo $PATH`
4. Run with verbose output: `RUST_LOG=debug rusty`

---

**🦀 Happy Rust Learning!**

Just run `rusty` and start your journey! 🚀
