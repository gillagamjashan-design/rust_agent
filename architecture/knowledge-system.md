# Knowledge System - Queryable Database Architecture

## Overview

The knowledge system provides an efficient alternative to traditional agent training. Instead of training the agent to "remember" everything, we:

1. **Build a comprehensive knowledge database** with all Rust information
2. **Give the agent a fetch/query tool** to look up information at runtime
3. **Use direct instruction prompts** that inject relevant knowledge into context

## Benefits

- **Immediate availability**: No "learning" time needed - knowledge is instantly accessible
- **Scalable**: Agent can handle any query by fetching relevant information
- **Maintainable**: Database can be expanded without retraining
- **Efficient**: No expensive training cycles required

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Agent Runtime                           │
│                                                              │
│  ┌──────────────┐      ┌────────────────┐                  │
│  │ User Query   │─────▶│ Confidence     │                  │
│  │              │      │ Check          │                  │
│  └──────────────┘      └────────┬───────┘                  │
│                                  │                           │
│                         High     │     Low                   │
│                         (>0.7)   │     (<0.7)               │
│                                  │                           │
│                    ┌─────────────┼──────────────┐           │
│                    ▼             ▼              ▼           │
│            ┌──────────┐   ┌──────────┐   ┌──────────┐      │
│            │ Answer   │   │ Fetch    │   │ Search   │      │
│            │ Directly │   │ Knowledge│   │ Database │      │
│            └──────────┘   └────┬─────┘   └────┬─────┘      │
│                                 │              │            │
│                                 └──────┬───────┘            │
│                                        ▼                     │
│                                ┌───────────────┐            │
│                                │ Generate      │            │
│                                │ Response      │            │
│                                │ with Context  │            │
│                                └───────────────┘            │
└─────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
                    ┌──────────────────────────────┐
                    │   Knowledge Database         │
                    │   (SQLite + FTS5)            │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ Concepts               │  │
                    │  │ - Ownership            │  │
                    │  │ - Lifetimes            │  │
                    │  │ - Traits               │  │
                    │  └────────────────────────┘  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ Patterns               │  │
                    │  │ - Builder              │  │
                    │  │ - RAII                 │  │
                    │  │ - Newtype              │  │
                    │  └────────────────────────┘  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ Errors                 │  │
                    │  │ - E0382                │  │
                    │  │ - E0499                │  │
                    │  └────────────────────────┘  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ Commands               │  │
                    │  │ - cargo                │  │
                    │  │ - rustup               │  │
                    │  └────────────────────────┘  │
                    └──────────────────────────────┘
```

## Components

### 1. Knowledge Database (`src/knowledge/database.rs`)

SQLite database with FTS5 (Full-Text Search) for efficient querying.

**Schema:**
- `concepts` - Core Rust concepts with explanations and examples
- `patterns` - Reusable code patterns and idioms
- `errors` - Compiler errors with fixes
- `commands` - Cargo/rustup command reference

**Features:**
- Full-text search using SQLite FTS5
- Automatic sync triggers to keep FTS index updated
- JSON storage for complex fields (arrays, examples)
- Efficient indexing for fast lookups

### 2. Knowledge Loader (`src/knowledge/loader.rs`)

Parses JSON knowledge files and populates the database.

**Supported Files:**
- `knowledge/rust_core_concepts.json` → Concepts
- `knowledge/rust_patterns_idioms.json` → Patterns
- `knowledge/rust_toolchain_cargo.json` → Commands

**Usage:**
```rust
let db = KnowledgeDatabase::new("data/knowledge.db")?;
let loader = KnowledgeLoader::new(db);
let stats = loader.load_all_from_directory("knowledge")?;
println!("Loaded: {}", stats);
```

### 3. Knowledge Query (`src/knowledge/query.rs`)

Interface for searching the knowledge database.

**API:**
```rust
let query = KnowledgeQuery::new(db);

// Search concepts
let concepts = query.search_concepts("ownership")?;

// Get specific concept
let concept = query.get_concept("ownership-move")?;

// Search by topic
let concepts = query.search_by_topic("ownership")?;

// Find patterns
let patterns = query.find_patterns("builder")?;

// Explain error
let error = query.explain_error("E0382")?;

// Search commands
let commands = query.search_commands("cargo", "test")?;

// Search all knowledge types
let results = query.search_all("ownership")?;
```

### 4. Knowledge Fetcher (`src/tools/knowledge_fetcher.rs`)

Agent tool for querying knowledge at runtime.

**Request Types:**
```rust
pub enum KnowledgeFetchRequest {
    ExplainConcept { topic: String },
    FindPattern { use_case: String },
    ExplainError { error_code: String },
    FindCommand { tool: String, action: String },
    Search { query: String },
}
```

**Usage:**
```rust
let fetcher = KnowledgeFetcher::new(query);

// Explain concept
let response = fetcher.explain_concept("ownership")?;

// Find pattern
let response = fetcher.find_pattern("builder")?;

// Explain error
let response = fetcher.explain_error("E0382")?;

// Find command
let response = fetcher.find_command("cargo", "test")?;

// General search
let response = fetcher.search("how to use async")?;
```

**Confidence Scoring:**
- High (>0.7): Answer directly without fetching
- Medium (0.4-0.7): May need verification
- Low (<0.4): Definitely fetch from database

### 5. Direct Instruction (`src/training/direct_instruction.rs`)

Generates textbook-style instruction prompts from knowledge.

**Features:**
- Topic-based instruction generation
- Curriculum generation by learning stage
- Focused instruction for specific queries
- Context injection for agent responses
- Quick reference cards

**Learning Stages:**
1. **Beginner**: ownership, borrowing, basic syntax, types
2. **Intermediate**: traits, generics, error handling, collections
3. **Advanced**: lifetimes, async, concurrency, smart pointers
4. **Expert**: unsafe, macros, optimization, FFI

**Usage:**
```rust
let instructor = DirectInstruction::new(query);

// Generate instruction for a topic
let instruction = instructor.generate_instruction("ownership")?;

// Generate curriculum for a stage
let curriculum = instructor.generate_curriculum(LearningStage::Beginner)?;

// Generate focused instruction
let instruction = instructor.generate_focused("how to use async")?;

// Inject context into agent response
let context = instructor.inject_context(user_query)?;
```

## Workflow

### Initial Setup

1. **Create database:**
   ```bash
   mkdir -p data
   ```

2. **Load knowledge:**
   ```rust
   let db = KnowledgeDatabase::new("data/knowledge.db")?;
   let loader = KnowledgeLoader::new(db);
   loader.load_all_from_directory("knowledge")?;
   ```

### Runtime Query Flow

1. **User asks question**: "What is ownership in Rust?"

2. **Agent checks confidence**:
   - If high confidence (>0.7): Answer directly
   - If low confidence (<0.7): Fetch from database

3. **Fetch knowledge**:
   ```rust
   let fetcher = KnowledgeFetcher::new(query);
   let response = fetcher.explain_concept("ownership")?;
   ```

4. **Generate response with context**:
   ```rust
   let instructor = DirectInstruction::new(query);
   let context = instructor.inject_context(user_query)?;
   // Prepend context to agent's response
   ```

## Database Statistics

After loading all knowledge files:
- **Concepts**: 20+ core Rust concepts
- **Patterns**: 15+ common patterns
- **Errors**: TBD (to be added)
- **Commands**: 30+ cargo/rustup commands

## Performance

- **Database size**: ~500 KB with all knowledge
- **Query speed**: <50ms for FTS5 searches
- **Load time**: <1 second to populate database
- **Memory usage**: Minimal (SQLite handles caching)

## Extension

To add new knowledge:

1. **Add to JSON files** in `knowledge/` directory
2. **Run loader** to populate database
3. **No retraining needed** - instantly available

## Integration Examples

### Example 1: Agent Response with Knowledge Injection

```rust
async fn handle_user_query(query: &str) -> String {
    let confidence = calculate_confidence(query);

    if confidence < 0.7 {
        // Fetch knowledge
        let fetcher = KnowledgeFetcher::new(db_query);
        let knowledge = fetcher.search(query)?;

        // Inject into context
        let instructor = DirectInstruction::new(db_query);
        let context = instructor.inject_context(query)?;

        // Generate response with knowledge
        generate_response_with_context(query, context, knowledge)
    } else {
        // Answer directly
        generate_direct_response(query)
    }
}
```

### Example 2: Learning Mode

```rust
fn start_learning_session(stage: LearningStage) -> String {
    let instructor = DirectInstruction::new(query);
    let curriculum = instructor.generate_curriculum(stage)?;

    // Present curriculum to user
    println!("{}", curriculum);

    // User can now ask questions with full context
}
```

### Example 3: Error Explanation

```rust
fn explain_compiler_error(error_code: &str) -> String {
    let fetcher = KnowledgeFetcher::new(query);
    let response = fetcher.explain_error(error_code)?;

    if response.has_results() {
        response.formatted
    } else {
        "Error not found in knowledge base".to_string()
    }
}
```

## Comparison: Old vs New

| Aspect | Q&A Training | Knowledge Database |
|--------|-------------|-------------------|
| Setup Time | Hours/days | <1 second |
| Learning | Sequential Q&A | Instant access |
| Unknown Topics | Agent guesses | Fetch from DB |
| Expansion | Retrain | Add to database |
| Efficiency | Low | High |
| Accuracy | Depends on training | Always current |
| Cost | High (training) | Low (queries) |

## Future Enhancements

1. **Add more error patterns** from Rust error index
2. **Clippy lint database** with explanations
3. **Crate documentation** from popular crates
4. **Code example database** from real projects
5. **Vector search** for semantic similarity
6. **Relevance scoring** to improve search results
7. **Cache frequently accessed knowledge** in memory
8. **Analytics** to track which knowledge is most useful

## Testing

Run the demo:
```bash
cargo run --example knowledge_system_demo
```

Run tests:
```bash
cargo test --package rust_agent --lib knowledge
cargo test --package rust_agent --lib tools::knowledge_fetcher
cargo test --package rust_agent --lib training::direct_instruction
```
