# Rusty - Architecture Diagram

## High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         USER INTERFACE                              │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │  GUI (egui/eframe) - 60 FPS Native Window                     │ │
│  │                                                                │ │
│  │  ┌──────────────────────────────────────────────────────────┐ │ │
│  │  │  Header: "Rusty - Your Rust Learning Agent"             │ │ │
│  │  │  Stats: "13 concepts, 18 patterns loaded"               │ │ │
│  │  └──────────────────────────────────────────────────────────┘ │ │
│  │                                                                │ │
│  │  ┌──────────────────────────────────────────────────────────┐ │ │
│  │  │  Chat Area (Scrollable)                                  │ │ │
│  │  │  ┌────────────────────────────────────────────────────┐  │ │ │
│  │  │  │ User: Create a hello world program               │  │ │ │
│  │  │  └────────────────────────────────────────────────────┘  │ │ │
│  │  │  ┌────────────────────────────────────────────────────┐  │ │ │
│  │  │  │ Assistant: Here's a hello world program...        │  │ │ │
│  │  │  │ ```rust                                           │  │ │ │
│  │  │  │ fn main() { println!("Hello"); }                  │  │ │ │
│  │  │  │ ```                                               │  │ │ │
│  │  │  │ 📁 Files Created: ✅ Created `src/main.rs`        │  │ │ │
│  │  │  └────────────────────────────────────────────────────┘  │ │ │
│  │  └──────────────────────────────────────────────────────────┘ │ │
│  │                                                                │ │
│  │  ┌──────────────────────────────────────────────────────────┐ │ │
│  │  │  Input Box: [Type your question here...]  [Enter ⏎]    │ │ │
│  │  └──────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────────┘ │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           │ mpsc::channel
                           │ UserCommand (Query/Command/Quit)
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      WORKER THREAD (Async)                          │
│                                                                     │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │  Tokio Runtime                                                 ││
│  │  ┌──────────────────────────────────────────────────────────┐ ││
│  │  │ 1. Receive UserCommand from GUI                          │ ││
│  │  │ 2. Search Knowledge Database (FTS5)                      │ ││
│  │  │ 3. Query Claude API with context                         │ ││
│  │  │ 4. Detect file creation intent                           │ ││
│  │  │ 5. Extract code blocks from response                     │ ││
│  │  │ 6. Create files in working directory                     │ ││
│  │  │ 7. Send WorkerMessage back to GUI                        │ ││
│  │  └──────────────────────────────────────────────────────────┘ ││
│  └────────────────────────────────────────────────────────────────┘│
└──────────┬──────────────────┬───────────────────┬──────────────────┘
           │                  │                   │
           ▼                  ▼                   ▼
     ┌─────────────┐   ┌─────────────┐   ┌──────────────┐
     │  Knowledge  │   │   Claude    │   │     File     │
     │  Database   │   │     API     │   │  Generator   │
     │  (SQLite)   │   │  (HTTP)     │   │  (Rust I/O)  │
     └─────────────┘   └─────────────┘   └──────────────┘
```

---

## Component Interaction Flow

### Query Processing Pipeline

```
┌─────────────┐
│    User     │
│   Input     │
└──────┬──────┘
       │
       │ "Create a hello world program"
       ▼
┌─────────────────────────────────────────────────────────┐
│  GUI Thread (app.rs)                                    │
│  • Validates input                                      │
│  • Adds to message history                              │
│  • Sends UserCommand::Query via channel                 │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  Worker Thread (worker.rs)                              │
│  ┌─────────────────────────────────────────────────────┐│
│  │ Step 1: Knowledge Search                            ││
│  │  ┌───────────────────────────────────────────────┐  ││
│  │  │ knowledge_fetcher.search("create hello...")   │  ││
│  │  │ • Queries FTS5 virtual tables                 │  ││
│  │  │ • Returns: concepts, patterns, examples       │  ││
│  │  │ • Formats context for Claude                  │  ││
│  │  └───────────────────────────────────────────────┘  ││
│  │                                                      ││
│  │ Step 2: Claude API Call                             ││
│  │  ┌───────────────────────────────────────────────┐  ││
│  │  │ claude.query("Context: ...\nUser: ...")       │  ││
│  │  │ • HTTP POST to localhost:8317                 │  ││
│  │  │ • Waits for response (1-3 sec)                │  ││
│  │  │ • Returns: code + explanation                 │  ││
│  │  └───────────────────────────────────────────────┘  ││
│  │                                                      ││
│  │ Step 3: File Creation                               ││
│  │  ┌───────────────────────────────────────────────┐  ││
│  │  │ auto_file_creator.auto_create_from_response   │  ││
│  │  │ • Detects "create" keyword                    │  ││
│  │  │ • Extracts code blocks                        │  ││
│  │  │ • Infers filename: fn main() → src/main.rs    │  ││
│  │  │ • Creates file with content                   │  ││
│  │  │ • Returns: Vec<FileCreationResult>            │  ││
│  │  └───────────────────────────────────────────────┘  ││
│  │                                                      ││
│  │ Step 4: Send Response                               ││
│  │  ┌───────────────────────────────────────────────┐  ││
│  │  │ message_tx.send(WorkerMessage::Response)      │  ││
│  │  │ message_tx.send(WorkerMessage::FilesCreated)  │  ││
│  │  └───────────────────────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────┘│
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  GUI Thread (app.rs)                                    │
│  • Polls message_rx (non-blocking)                      │
│  • Updates messages list                                │
│  • Updates created_files list                           │
│  • Triggers re-render                                   │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  GUI Render (layout.rs)                                 │
│  • Displays new message with syntax highlighting        │
│  • Shows file creation summary                          │
│  • Auto-scrolls to bottom                               │
└─────────────────────────────────────────────────────────┘
```

---

## Knowledge Database Architecture

```
┌────────────────────────────────────────────────────────────┐
│  ~/.agent/data/knowledge.db (SQLite)                       │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Regular Tables (Persistent Storage)                 │ │
│  │                                                       │ │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐     │ │
│  │  │ concepts   │  │  patterns  │  │  commands  │     │ │
│  │  ├────────────┤  ├────────────┤  ├────────────┤     │ │
│  │  │ id         │  │ id         │  │ id         │     │ │
│  │  │ topic      │  │ name       │  │ tool       │     │ │
│  │  │ title      │  │ description│  │ command    │     │ │
│  │  │ explanation│  │ template   │  │ description│     │ │
│  │  │ examples   │  │ when_to_use│  │ examples   │     │ │
│  │  │ mistakes   │  │ examples   │  │ flags      │     │ │
│  │  │ related    │  └────────────┘  └────────────┘     │ │
│  │  │ tags       │                                      │ │
│  │  └────────────┘                                      │ │
│  │                                                       │ │
│  │  ┌────────────────┐                                  │ │
│  │  │ file_templates │                                  │ │
│  │  ├────────────────┤                                  │ │
│  │  │ id             │                                  │ │
│  │  │ file_type      │                                  │ │
│  │  │ default_filename                                  │ │
│  │  │ pattern_hints  │                                  │ │
│  │  │ template_content                                  │ │
│  │  └────────────────┘                                  │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  FTS5 Virtual Tables (Full-Text Search Index)        │ │
│  │                                                       │ │
│  │  ┌───────────────┐  ┌──────────────┐  ┌───────────┐ │ │
│  │  │ concepts_fts  │  │ patterns_fts │  │commands_fts││ │
│  │  ├───────────────┤  ├──────────────┤  ├───────────┤ │ │
│  │  │ topic         │  │ name         │  │ tool      │ │ │
│  │  │ title         │  │ description  │  │ command   │ │ │
│  │  │ explanation   │  │ when_to_use  │  │description│ │ │
│  │  │ tags          │  └──────────────┘  └───────────┘ │ │
│  │  └───────────────┘                                   │ │
│  │                                                       │ │
│  │  Triggers: Auto-update FTS on INSERT/UPDATE/DELETE   │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Query Example                                        │ │
│  │                                                       │ │
│  │  SELECT * FROM concepts_fts                           │ │
│  │  WHERE concepts_fts MATCH 'ownership'                 │ │
│  │  ORDER BY rank                                        │ │
│  │  LIMIT 5;                                             │ │
│  │                                                       │ │
│  │  → Returns ranked results in ~20ms                    │ │
│  └──────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
                            │
                            │ Data loaded from
                            ▼
┌────────────────────────────────────────────────────────────┐
│  knowledge/ directory (JSON source files)                  │
│                                                            │
│  • rust_core_concepts.json        (13 concepts)           │
│  • rust_patterns_idioms.json      (18 patterns)           │
│  • rust_toolchain_cargo.json      (22 commands)           │
│  • rust_async_concurrency.json    (async/await)           │
│  • rust_standard_library.json     (std modules)           │
│  • rust_popular_crates.json       (ecosystem)             │
│  • curriculum_master.json         (learning path)         │
└────────────────────────────────────────────────────────────┘
```

---

## File Creation System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  AutoFileCreator (file_generator.rs)                        │
└──────────────────────────┬──────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
┌──────────────────┐ ┌──────────────┐ ┌─────────────────┐
│FileCreationDetector │KnowledgeDB   │ │  FileCreator    │
├──────────────────┤ ├──────────────┤ ├─────────────────┤
│• Keyword match   │ │• Template    │ │• Directory      │
│  (create, make)  │ │  lookup      │ │  creation       │
│• Extract code    │ │• Filename    │ │• File write     │
│  blocks          │ │  resolution  │ │• Append mode    │
│• Infer filenames │ │              │ │• Error handling │
└──────────────────┘ └──────────────┘ └─────────────────┘
         │                 │                 │
         └─────────────────┼─────────────────┘
                           │
                           ▼
                  ┌────────────────┐
                  │ Code Block     │
                  ├────────────────┤
                  │• language: rust│
                  │• code: "fn..." │
                  │• filename: ?   │
                  └────────────────┘
                           │
                           ▼
              ┌────────────────────────┐
              │ Filename Resolution    │
              │ (Priority Order)       │
              │                        │
              │ 1. @filepath marker    │
              │    @src/main.rs        │
              │         ↓              │
              │ 2. Comment hint        │
              │    // src/main.rs      │
              │         ↓              │
              │ 3. Heuristic inference │
              │    fn main() → main.rs │
              │    pub struct → *.rs   │
              │         ↓              │
              │ 4. Database template   │
              │    lookup by pattern   │
              └────────────────────────┘
                           │
                           ▼
                  ┌────────────────┐
                  │ File Operation │
                  └────────────────┘
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
      ┌──────────────┐          ┌──────────────┐
      │ File exists? │          │ File exists? │
      │     NO       │          │     YES      │
      └──────┬───────┘          └──────┬───────┘
             │                         │
             ▼                         ▼
      ┌──────────────┐          ┌──────────────┐
      │ CREATE NEW   │          │   APPEND     │
      │              │          │              │
      │ mkdir -p src/│          │ Add separator│
      │ write content│          │ Add content  │
      └──────────────┘          └──────────────┘
             │                         │
             └────────────┬────────────┘
                          ▼
                 ┌─────────────────┐
                 │ FileCreationResult
                 ├─────────────────┤
                 │• path: PathBuf  │
                 │• appended: bool │
                 │• error: Option  │
                 └─────────────────┘
```

---

## Message Flow Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  GUI Thread                        Worker Thread            │
│                                                              │
│  ┌──────────────┐                 ┌──────────────┐          │
│  │   RustyApp   │                 │ worker_loop  │          │
│  │              │                 │              │          │
│  │  ┌────────┐  │   UserCommand   │  ┌────────┐  │          │
│  │  │command_│──┼────────────────→│  │command_│  │          │
│  │  │tx      │  │   (mpsc)        │  │rx      │  │          │
│  │  └────────┘  │                 │  └────────┘  │          │
│  │              │                 │       │      │          │
│  │  ┌────────┐  │  WorkerMessage  │       ▼      │          │
│  │  │message_│←─┼─────────────────│  ┌─────────┐ │          │
│  │  │rx      │  │   (mpsc)        │  │ Process │ │          │
│  │  └────────┘  │                 │  │ Command │ │          │
│  │       │      │                 │  └─────────┘ │          │
│  │       ▼      │                 │       │      │          │
│  │  ┌─────────┐ │                 │       ▼      │          │
│  │  │ Handle  │ │                 │  ┌────────┐  │          │
│  │  │ Message │ │                 │  │message_│  │          │
│  │  └─────────┘ │                 │  │tx      │  │          │
│  │       │      │                 │  └────────┘  │          │
│  │       ▼      │                 │              │          │
│  │  ┌─────────┐ │                 │              │          │
│  │  │ Update  │ │                 │              │          │
│  │  │  State  │ │                 │              │          │
│  │  └─────────┘ │                 │              │          │
│  │       │      │                 │              │          │
│  │       ▼      │                 │              │          │
│  │  ┌─────────┐ │                 │              │          │
│  │  │ Render  │ │                 │              │          │
│  │  │   UI    │ │                 │              │          │
│  │  └─────────┘ │                 │              │          │
│  └──────────────┘                 └──────────────┘          │
└─────────────────────────────────────────────────────────────┘

Message Types:

UserCommand (GUI → Worker)
├── Query(String)         "Create a hello world program"
├── Command(String)       "/search ownership"
└── Quit                  Exit signal

WorkerMessage (Worker → GUI)
├── Response(String)      Claude's answer + file summary
├── SystemMessage(String) System notifications
├── Error(String)         Error messages
├── Stats(String)         "13 concepts, 18 patterns loaded"
└── FilesCreated(Vec<FileCreationInfo>)
    └── FileCreationInfo
        ├── path: String
        ├── appended: bool
        └── success: bool
```

---

## Directory Structure at Runtime

```
/workspace/jashan/making_files/          # Working directory
│
├── rusty_tui/target/release/rusty       # Binary (23MB)
│
├── src/                                 # Created by agent
│   ├── main.rs                         # ← Auto-generated
│   ├── server.rs                       # ← Auto-generated
│   └── lib.rs                          # ← Auto-generated
│
├── Cargo.toml                           # ← Auto-generated
│
└── tests/                               # Created by agent
    └── integration_test.rs              # ← Auto-generated


~/.agent/                                # Config directory
│
├── data/
│   └── knowledge.db                    # SQLite (initialized on first run)
│       ├── concepts table (13 rows)
│       ├── patterns table (18 rows)
│       ├── commands table (22 rows)
│       ├── file_templates table (6 rows)
│       ├── concepts_fts (FTS5 index)
│       ├── patterns_fts (FTS5 index)
│       └── commands_fts (FTS5 index)
│
└── cache/
    ├── a3f5e8b9c2d1.json              # Cached web search result
    ├── f2d9c4a7e1b8.json              # SHA-256 hash as filename
    └── ...                             # 7-day expiry
```

---

## Performance & Scaling

```
┌─────────────────────────────────────────────────────────┐
│  Performance Characteristics                            │
│                                                          │
│  FTS5 Query:     O(log n) for index lookup              │
│                  ~20ms for 50+ entries                  │
│                                                          │
│  File Creation:  O(1) for single file                   │
│                  ~5-10ms per file                       │
│                  Bottleneck: disk I/O                   │
│                                                          │
│  GUI Render:     O(n) for message list                  │
│                  60 FPS maintained for <1000 messages   │
│                                                          │
│  Claude API:     O(1) per query                         │
│                  1-3s network + inference time          │
│                  Bottleneck: network latency            │
│                                                          │
│  Memory Usage:   ~50MB base (egui + tokio)              │
│                  +~10MB per 1000 messages               │
│                  Database: mmap (minimal RAM)           │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  Scalability Limits                                     │
│                                                          │
│  Knowledge DB:   Tested up to 1000 entries              │
│                  FTS5 scales to 100k+ rows              │
│                                                          │
│  Chat History:   ~1000 messages before UI slowdown      │
│                  (No pagination implemented)            │
│                                                          │
│  File Creation:  No limit on number of files            │
│                  Limited by disk space                  │
│                                                          │
│  Concurrent:     Single worker thread                   │
│                  Queries processed sequentially         │
│                  (Parallelization not implemented)      │
└─────────────────────────────────────────────────────────┘
```

---

## Error Handling Flow

```
┌─────────────────────────────────────────────────────────┐
│  Error Types & Recovery                                 │
│                                                          │
│  Database Error                                         │
│  ├─ Missing DB file → Auto-create from JSON            │
│  ├─ Corrupted DB → Delete and rebuild                  │
│  └─ Query error → Fall back to web search              │
│                                                          │
│  Network Error (Claude API)                             │
│  ├─ Connection refused → Show helpful error message    │
│  ├─ Timeout → Retry once, then show error              │
│  └─ Invalid response → Log and show error              │
│                                                          │
│  File Creation Error                                    │
│  ├─ Permission denied → Show error, continue           │
│  ├─ Disk full → Show error, continue                   │
│  ├─ Invalid path → Show error, skip file               │
│  └─ Directory creation failed → Show error             │
│                                                          │
│  GUI Error                                              │
│  ├─ Channel disconnected → Restart worker              │
│  ├─ Render panic → Log and continue                    │
│  └─ Input parsing error → Show validation message      │
└─────────────────────────────────────────────────────────┘
```

---

## Security Considerations

```
┌─────────────────────────────────────────────────────────┐
│  Security Model                                         │
│                                                          │
│  File Creation:                                         │
│  ✓ Sandboxed to working directory (uses relative paths)│
│  ✓ No shell command execution                          │
│  ✗ No path traversal protection (trusts Claude output) │
│  ✗ No file size limits                                 │
│                                                          │
│  Network:                                               │
│  ✓ Only connects to localhost:8317 (ClaudeProxyAPI)    │
│  ✓ HTTPS for web search (DuckDuckGo)                   │
│  ✗ No certificate pinning                              │
│  ✗ No rate limiting                                    │
│                                                          │
│  Data Storage:                                          │
│  ✓ SQLite database in user's home directory            │
│  ✓ No sensitive data stored                            │
│  ✗ Chat history not encrypted                          │
│  ✗ No automatic backups                                │
│                                                          │
│  Input Validation:                                      │
│  ✓ Query length limits (implicitly by Claude API)      │
│  ✗ No SQL injection protection needed (no user SQL)    │
│  ✗ No XSS protection (not a web app)                   │
└─────────────────────────────────────────────────────────┘
```

---

**Last Updated**: 2026-03-01
**Version**: v12.0.1
**Status**: ✅ Production Ready
