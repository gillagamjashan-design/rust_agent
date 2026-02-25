# System Overview

## Rust Learning Agent with Instant Knowledge Access

A Rust programming agent with a queryable knowledge database. Instead of training the agent to "remember" everything, the system provides instant access to a comprehensive SQLite knowledge database with FTS5 full-text search.

## Core Concept

One command: `rusty` → Beautiful colored TUI → Agent chat interface

The agent has:
1. **Knowledge Database** - SQLite with FTS5 search containing all Rust concepts, patterns, and commands
2. **Runtime Query** - Agent fetches information on-demand (<50ms queries)
3. **Beautiful TUI** - Colored terminal interface in a new window

## Key Features

- **Single Command**: Just run `rusty` - no flags, no configuration
- **Instant Knowledge**: No training time - knowledge pre-loaded in SQLite database
- **Fast Queries**: FTS5 full-text search provides <50ms response times
- **Beautiful TUI**: Colored, formatted chat interface in new terminal window
- **Comprehensive Database**: 53+ Rust concepts, patterns, and Cargo commands
- **Agent-Only Interface**: No IDE, no file tree - just the agent chat
- **Code Generation**: Agent can write Rust code in addition to teaching
- **Web Search Fallback**: DuckDuckGo integration for topics not in database

## Knowledge System

### No Training Required

Instead of training the agent to "learn" Rust, we provide a queryable knowledge database:

1. **Pre-loaded Knowledge** (0 seconds) - JSON files loaded into SQLite on first run
2. **Runtime Queries** (<50ms) - Agent fetches information when needed
3. **FTS5 Search** - Full-text search across concepts, patterns, commands
4. **Expandable** - Add new knowledge files without retraining

**Result**: Instant access to comprehensive Rust knowledge with zero training time.

## Architecture Layers

### Layer 1: Knowledge Database (src/knowledge/)
- **Database** - SQLite with FTS5 full-text search
- **Loader** - Parse JSON knowledge files and populate database
- **Query** - Search interface for concepts, patterns, commands
- **Tables**: `concepts`, `patterns`, `errors`, `commands`

### Layer 2: Tools (src/tools/)
- **Knowledge Fetcher** - Runtime tool for agent to query database
- **Confidence Decision** - Determine when to fetch vs answer directly
- **Request Types**: ExplainConcept, FindPattern, ExplainError, FindCommand, Search

### Layer 3: Agent Interface
- **Beautiful TUI** - Colored terminal UI with ratatui
- **Chat Window** - Main conversation area with formatted responses
- **Status Bar** - Knowledge database stats, query count
- **Command Palette** - `/help`, `/search`, `/stats`, `/quit`
- **Syntax Highlighting** - Code blocks with Tokyo Night theme

### Layer 4: Integration
- **Claude Proxy** - ClaudeProxyAPI client for AI responses
- **Web Search** - DuckDuckGo fallback for unknown topics
- **Cache** - Search result caching to reduce API calls

## Why This Design?

### Instant Access
- **Zero training time**: Knowledge pre-loaded, immediately available
- **Fast queries**: <50ms SQLite FTS5 search
- **One command**: Just `rusty` - no flags, no configuration
- **Simple UX**: Beautiful TUI, no complex IDE features

### Knowledge Database Approach
- **Fetch, don't train**: Agent queries database at runtime
- **Expandable**: Add JSON files without retraining
- **Maintainable**: Update knowledge by editing JSON
- **Efficient**: One-time load on first run

### User Experience
- **Single purpose**: Agent chat only - no distractions
- **Beautiful interface**: Colored TUI with syntax highlighting
- **Fast responses**: Database queries + Claude API
- **Code capable**: Agent can write Rust code, not just teach

## Agent Capabilities

### Primary Role
- **Code Generator** - Creates Rust programs from specifications
- **Teacher** - Explains how code works to the reader
- **Debugger** - Identifies issues and explains fixes
- **Documentation Writer** - Documents solutions clearly

### Core Principles
1. ✅ Follow user instructions step-by-step
2. ✅ Ask permission before modifying code
3. ✅ Explain reasoning and approach
4. ✅ Document problems encountered and solutions applied
5. ❌ Never modify code without explicit permission

### Knowledge Domains
- **Rust** (Perfect understanding)
  - Language fundamentals
  - Advanced patterns
  - Unsafe code
  - Async/concurrency
  - FFI and systems programming
- **Development Tools**
  - Git version control
  - Linux command line
  - GitHub CLI (gh)

## Use Cases

### Learning Rust
- Ask questions: "What is ownership in Rust?"
- Get instant answers from knowledge database
- See code examples with syntax highlighting
- Understand concepts: lifetimes, traits, async, macros

### Writing Code
- Generate Rust programs: "Write a TCP server"
- Get pattern suggestions: "Show me the builder pattern"
- Learn Cargo commands: "How do I add dependencies?"
- Understand errors: "What does E0382 mean?"

### Quick Reference
- Search database: `/search ownership`
- View stats: `/stats` shows loaded knowledge count
- Get help: `/help` lists all commands
- Web search fallback for unknown topics

## Data Flow

```
User → rusty command → TUI Window → Agent Chat
                           ↓
                    Knowledge Database (SQLite)
                           ↓
        Concepts | Patterns | Commands | Errors
                           ↓
                    Agent queries (<50ms)
                           ↓
                Claude API + Knowledge → Response
```

## Performance Targets

- **Database query**: <50ms (FTS5 search)
- **TUI response**: <100ms (total with API)
- **First run load**: <2 seconds (populate database)
- **Startup time**: <500ms (database already loaded)
- **Knowledge accuracy**: >95%

## Technology Stack

- **Language**: Rust (edition 2021)
- **TUI Framework**: Ratatui (beautiful colored terminal UI)
- **Database**: SQLite with FTS5 full-text search
- **AI Provider**: ClaudeProxyAPI (localhost:8317)
- **Web Search**: DuckDuckGo client
- **Syntax Highlighting**: Syntect with Tokyo Night theme
- **Launcher**: Single `rusty` command in PATH
