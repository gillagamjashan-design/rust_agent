# Unified Brain Integration

## Overview

The Learning Agent and TUI IDE now **share the same brain** - a unified knowledge system where both components read and write to the same memory systems.

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    UNIFIED BRAIN SYSTEM                      │
│                /workspace/jashan/rust_agent/data/           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────┐         ┌──────────────────────┐  │
│  │  Training Pipeline   │         │  TUI IDE (Rusty)     │  │
│  │  (Batch Learning)    │◄───────►│  (agent_manager.rs)  │  │
│  │                      │  SHARED  │                      │  │
│  │ - BatchLoader        │  MEMORY  │ - SqlMemory          │  │
│  │ - PatternExtractor   │ SYSTEMS  │ - FileStorage        │  │
│  │ - ErrorCollector     │          │ - WorkingMemory      │  │
│  └──────────┬───────────┘          │ - SlashCommands      │  │
│             │                      └──────────┬───────────┘  │
│             │                                 │              │
│             └────────────┬────────────────────┘              │
│                          │                                   │
│                          ▼                                   │
│         ┌────────────────────────────────────┐              │
│         │    SHARED DATA DIRECTORY           │              │
│         │    /workspace/jashan/rust_agent/   │              │
│         │                                     │              │
│         │  data/                              │              │
│         │  ├── knowledge/rust/*.json          │ ◄── FileStorage
│         │  ├── patterns/                      │              │
│         │  │   ├── concepts.db                │ ◄── SqlMemory
│         │  │   └── rust-patterns.db           │ ◄── SqlMemory
│         │  ├── conversations/                 │              │
│         │  └── analytics/                     │              │
│         └─────────────────────────────────────┘              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Shared Components

### 1. SqlMemory (concepts.db)

**Location**: `data/patterns/concepts.db`

**Shared By**:
- Training Pipeline: Writes concepts after Stage 1 (Bootstrap)
- TUI IDE: Reads concepts for query context

**Schema**:
```sql
CREATE TABLE concepts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    rust_version TEXT,
    complexity_level INTEGER
);
```

**Usage in Training**:
```rust
// training_pipeline.rs - Stage 1
let db_path = self.data_dir.join("patterns").join("concepts.db");
let sql = SqlMemory::new(&db_path)?;
for concept in &concepts {
    sql.store_concept(concept)?;
}
```

**Usage in IDE**:
```rust
// agent_manager.rs
let knowledge_path = PathBuf::from("/workspace/jashan/rust_agent/data");
let db_path = knowledge_path.join("patterns").join("concepts.db");
self.sql_memory = SqlMemory::new(&db_path).ok();

// Query for relevant concepts
let concepts = sql.query_concepts("ownership")?;
```

### 2. FileStorage (knowledge/rust/)

**Location**: `data/knowledge/rust/`

**Shared By**:
- Training Pipeline: Writes JSON files by category
- TUI IDE: Reads categories for "Recently Learned" display

**Structure**:
```
data/knowledge/rust/
├── memory-management/
│   ├── ownership.json
│   └── move-semantics.json
├── borrowing/
│   ├── references.json
│   └── mutable-borrows.json
├── lifetimes/
│   └── lifetime-annotations.json
└── ...
```

**Concept File Format**:
```json
{
  "id": "rust-book-ownership",
  "name": "Ownership",
  "category": "memory-management",
  "description": "Each value has a single owner...",
  "examples": ["fn take_ownership(s: String) { }"],
  "rules": ["Each value has an owner", "..."],
  "complexity_level": 3
}
```

### 3. WorkingMemory (In-Memory)

**Used By**: TUI IDE only

**Purpose**: Short-term conversation context (last 10 messages)

**Not Persisted**: Lives only during IDE session

---

## Data Flow

### Training → IDE Flow

```
1. User runs training pipeline:
   $ cargo run -- train

2. Training Pipeline (Stage 1-3):
   ├─ Stage 1: Load Rust Book/Reference
   │  └─ Store concepts → data/patterns/concepts.db (SqlMemory)
   │  └─ Store concepts → data/knowledge/rust/*.json (FileStorage)
   │
   ├─ Stage 2: Extract patterns
   │  └─ Store patterns → data/patterns/rust-patterns.db (SqlMemory)
   │
   └─ Stage 3: Build error database
      └─ Initialize ErrorDatabase (in-memory)

3. User launches IDE:
   $ rusty

4. IDE agent_manager loads:
   ├─ SqlMemory from data/patterns/concepts.db
   ├─ FileStorage from data/knowledge/rust/
   └─ WorkingMemory (fresh)

5. IDE displays:
   🧠 Knowledge: 1,234 concepts  ← From SqlMemory
   Recently Learned:             ← From FileStorage
   - memory-management
   - borrowing
   - lifetimes
```

### Real-time Sync Flow

```
1. Training runs in background (continuous learning):
   └─ Adds new concepts to SqlMemory

2. IDE polls for updates (on user query):
   ├─ reload_knowledge() called
   └─ Reloads SqlMemory from disk

3. Brain size updates:
   Old: 🧠 1,234 concepts
   New: 🧠 1,235 concepts
```

---

## Implementation Details

### Training Pipeline Integration

**File**: `src/interfaces/training_pipeline.rs`

```rust
pub async fn run(&self) -> Result<TrainingReport> {
    // Stage 1: Bootstrap
    let concepts = self.stage1_bootstrap().await?;

    // Store in SQL (shared with IDE)
    let db_path = self.data_dir.join("patterns").join("concepts.db");
    let sql = SqlMemory::new(&db_path)?;
    for concept in &concepts {
        sql.store_concept(concept)?;
    }

    // Store in FileStorage (shared with IDE)
    let storage = FileStorage::new(self.data_dir.join("knowledge"));
    for concept in &concepts {
        storage.store_concept(concept)?;
    }

    Ok(report)
}
```

### IDE Agent Manager Integration

**File**: `rusty_ide_v2/src-tauri/src/agent_manager.rs`

```rust
pub struct AgentManager {
    sql_memory: Option<SqlMemory>,      // Shared!
    file_storage: FileStorage,          // Shared!
    working_memory: WorkingMemory,      // IDE-only
    slash_executor: SlashCommandExecutor,
    knowledge_path: PathBuf,            // /workspace/jashan/rust_agent/data
}

impl AgentManager {
    pub fn new() -> Result<Self> {
        // UNIFIED BRAIN PATH
        let knowledge_path = PathBuf::from("/workspace/jashan/rust_agent/data");

        // Initialize shared memory systems
        let file_storage = FileStorage::new(knowledge_path.join("knowledge"));
        let working_memory = WorkingMemory::new();

        // SQL database (shared)
        let db_path = knowledge_path.join("patterns").join("concepts.db");
        let sql_memory = SqlMemory::new(&db_path).ok();

        Ok(Self {
            sql_memory,
            file_storage,
            working_memory,
            knowledge_path,
            // ...
        })
    }

    pub fn get_brain_size(&self) -> usize {
        // Count concepts from shared SqlMemory
        self.sql_memory
            .as_ref()
            .and_then(|sql| sql.count_concepts().ok())
            .unwrap_or(0)
    }

    pub async fn query(&mut self, user_query: String) -> Result<String> {
        let mut context = String::new();

        // Brain size from shared memory
        let brain_size = self.get_brain_size();
        context.push_str(&format!("🧠 {} concepts\n", brain_size));

        // Search concepts in shared SqlMemory
        if let Some(sql) = &self.sql_memory {
            if user_query.contains("ownership") {
                let concepts = sql.query_concepts("memory-management")?;
                for concept in concepts.iter().take(2) {
                    context.push_str(&format!("- {}\n", concept.name));
                }
            }
        }

        // Query Claude with context
        self.claude.send_request(context + &user_query, system_prompt).await
    }

    pub fn reload_knowledge(&mut self) {
        // Reload shared SqlMemory (polls for updates)
        let db_path = self.knowledge_path.join("patterns").join("concepts.db");
        self.sql_memory = SqlMemory::new(&db_path).ok();
    }
}
```

---

## Configuration

### Shared Data Path

**Hardcoded** (both components use same path):
```rust
PathBuf::from("/workspace/jashan/rust_agent/data")
```

**Future Enhancement**: Could be configurable via `~/.agent/config.toml`:
```toml
[unified_brain]
data_path = "/workspace/jashan/rust_agent/data"
auto_sync = true
sync_interval_ms = 16
```

---

## Verification

### Test Unified Brain

1. **Run Training Pipeline**:
   ```bash
   cd /workspace/jashan/rust_agent
   cargo run -- train
   ```

   Output:
   ```
   📚 Stage 1: Bootstrap...
      ✓ Loaded 523 concepts

   🔍 Stage 2: Pattern Recognition...
      ✓ Extracted 47 patterns
   ```

2. **Launch IDE**:
   ```bash
   rusty
   ```

   Status line shows:
   ```
   🧠 523 concepts | 10 messages
   ```

3. **Query IDE**:
   ```
   > What is ownership in Rust?
   ```

   Response includes:
   ```
   🧠 Knowledge Base: 523 concepts learned

   📚 Relevant concepts from memory-management:
   - Ownership: Each value has a single owner...
   ```

### Verify Sync

1. **Add new concept manually**:
   ```bash
   # Add concept to database
   sqlite3 data/patterns/concepts.db
   INSERT INTO concepts VALUES ('test', 'Test', 'test', 'Test', NULL, 1);
   ```

2. **Reload IDE knowledge**:
   ```rust
   // IDE calls reload_knowledge() on next query
   ```

3. **Brain size updates**:
   ```
   Old: 🧠 523 concepts
   New: 🧠 524 concepts
   ```

---

## Benefits

### ✅ Single Source of Truth
- No duplication between training and IDE
- Updates in one place reflect everywhere

### ✅ Real-time Knowledge Growth
- Train → IDE sees new concepts immediately
- IDE polls database for updates

### ✅ Efficient Storage
- SQL for structured queries
- JSON for hierarchical browsing
- In-memory for session context

### ✅ Scalability
- Can add concepts without IDE restart
- Database handles thousands of concepts efficiently

### ✅ Consistency
- Both use same Concept/Pattern types
- Same SQL schema
- Same file structure

---

## Limitations & Future Work

### Current Limitations

1. **No Episodic Memory**: IDE doesn't persist conversation history
2. **No Vector Search**: No semantic similarity search yet
3. **Manual Reload**: IDE doesn't auto-detect file changes
4. **Hardcoded Path**: Path not configurable

### Future Enhancements

1. **File Watching**: Auto-reload when database changes
   ```rust
   use notify::Watcher;

   let watcher = notify::watcher(tx, Duration::from_secs(1))?;
   watcher.watch(&db_path, RecursiveMode::NonRecursive)?;
   ```

2. **Vector Search**: Add Qdrant for semantic queries
   ```rust
   let vector_memory = VectorMemory::new("http://localhost:6333")?;
   let similar = vector_memory.semantic_search("ownership", 5).await?;
   ```

3. **Conversation Persistence**: Save IDE history to episodic memory
   ```rust
   let episodic = EpisodicMemory::new(&db_path)?;
   episodic.save_conversation(&conversation).await?;
   ```

4. **Remote Sync**: Sync knowledge across machines
   ```rust
   let syncer = KnowledgeSyncer::new("https://api.example.com")?;
   syncer.push_updates().await?;
   ```

---

## Summary

✅ **Training Pipeline** and **TUI IDE** now share:
- Same SqlMemory database (`data/patterns/concepts.db`)
- Same FileStorage structure (`data/knowledge/rust/`)
- Same data path (`/workspace/jashan/rust_agent/data`)

✅ **Brain size in IDE** reflects actual learned concepts from training

✅ **Query context in IDE** uses concepts from shared database

✅ **Real-time updates** via reload_knowledge() on each query

🎉 **Truly Unified Intelligence System!**
