# Rust Knowledge Base

This directory contains a comprehensive, focused knowledge base for learning Rust programming.

## Purpose

This knowledge base is designed to teach **ONLY Rust** - no general programming concepts, no other languages. It provides a structured, efficient learning path from beginner to advanced Rust programmer.

## Knowledge Files

### Core Concepts
- **rust_core_concepts.json** - Ownership, borrowing, lifetimes, traits, and generics
  - The fundamental Rust concepts that make it unique
  - Priority: 1 (Learn first)

### Standard Library
- **rust_standard_library.json** - Collections, error handling, smart pointers, iterators
  - Essential types and modules from std
  - Priority: 2

### Concurrency
- **rust_async_concurrency.json** - async/await, threads, synchronization primitives
  - Complete guide to concurrent programming in Rust
  - Priority: 3

### Popular Crates
- **rust_popular_crates.json** - tokio, serde, anyhow, reqwest, clap, etc.
  - Most commonly used crates in the ecosystem
  - Priority: 4

### Patterns & Idioms
- **rust_patterns_idioms.json** - Common patterns and best practices
  - Builder pattern, newtype, RAII, error handling patterns, etc.
  - Priority: 5

### Toolchain
- **rust_toolchain_cargo.json** - Cargo, rustup, testing, project structure
  - Complete guide to Rust development tools
  - Priority: 2

### Master Curriculum
- **curriculum_master.json** - Complete learning plan
  - 6-stage progressive curriculum
  - Daily learning routine
  - Question generation strategy
  - Learning metrics and advancement criteria

## Learning Stages

### Stage 1: Ownership Foundations (2-3 weeks)
- Ownership rules
- Move semantics
- Stack vs heap
- Drop trait

### Stage 2: Borrowing & References (2-3 weeks)
- Borrowing rules
- Reference lifetimes
- Slices
- Avoiding dangling references

### Stage 3: Lifetimes & Type System (3-4 weeks)
- Explicit lifetime annotations
- Lifetime elision
- Trait system
- Generic programming with trait bounds

### Stage 4: Standard Library Mastery (3-4 weeks)
- Collections (Vec, String, HashMap)
- Error handling (Result, Option)
- Smart pointers (Box, Rc, Arc, RefCell)
- Iterators and combinators

### Stage 5: Concurrency & Async (4-5 weeks)
- async/await with tokio
- Thread safety (Send/Sync)
- Synchronization primitives
- Concurrent patterns

### Stage 6: Ecosystem & Tooling (3-4 weeks)
- Cargo mastery
- Popular crates
- Testing
- Rust patterns and idioms

## Usage

The agent system reads from these JSON files to:

1. **Generate focused questions** - Questions drawn from knowledge base topics
2. **Provide comprehensive answers** - Answers include examples from knowledge base
3. **Track progress** - Monitor which topics have been covered
4. **Adapt difficulty** - Progress from basic to advanced within each topic

## Key Principles

### Focus on Rust ONLY
- ✅ Rust ownership, borrowing, lifetimes
- ✅ Rust standard library
- ✅ Popular Rust crates (tokio, serde, etc.)
- ✅ Rust toolchain (cargo, rustc, clippy)
- ❌ General programming theory
- ❌ Other programming languages
- ❌ Non-Rust frameworks

### Progressive Learning
- Start with fundamentals (ownership)
- Build on previous knowledge
- Advance through structured stages
- Review earlier topics periodically

### Practical Focus
- Real code examples
- Practical patterns
- Common use cases
- Ecosystem best practices

## Integration with Agent System

### Legacy Approach (Q&A Training)

The learning agent can:
1. **Load knowledge base** on startup
2. **Generate questions** from current stage topics
3. **Provide answers** with examples from knowledge base
4. **Track progress** through stages
5. **Advance stages** when criteria met

### New Approach (Queryable Knowledge Database)

The new knowledge system provides a more efficient alternative:

**Setup (One-time):**
```rust
use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeLoader};

// Create database and load knowledge
let db = KnowledgeDatabase::new("data/knowledge.db")?;
let loader = KnowledgeLoader::new(db);
let stats = loader.load_all_from_directory("knowledge")?;
```

**Runtime Queries:**
```rust
use rust_agent::tools::KnowledgeFetcher;

let fetcher = KnowledgeFetcher::new(query);

// User asks: "What is ownership?"
let response = fetcher.explain_concept("ownership")?;

// User asks: "How do I build a struct?"
let response = fetcher.find_pattern("builder")?;
```

**Direct Instruction:**
```rust
use rust_agent::training::{DirectInstruction, LearningStage};

let instructor = DirectInstruction::new(query);

// Generate textbook-style instruction
let instruction = instructor.generate_instruction("ownership")?;

// Generate full curriculum
let curriculum = instructor.generate_curriculum(LearningStage::Beginner)?;
```

**Benefits:**
- ✅ **Instant access** - No training time needed
- ✅ **Efficient queries** - <50ms with SQLite FTS5
- ✅ **Always current** - Update JSON files, no retraining
- ✅ **Scalable** - Add knowledge without training cycles

See [../architecture/knowledge-system.md](../architecture/knowledge-system.md) for full details.

## Updating Knowledge Base

To add new content:

1. Identify which file it belongs in
2. Follow existing JSON structure
3. Include code examples
4. Add to appropriate learning stage in curriculum_master.json
5. Update this README if adding new files

## Statistics

- **Total concepts covered**: 100+
- **Code examples**: 200+
- **Learning stages**: 6
- **Estimated total time**: 17-23 weeks (4-6 months)
- **Focus**: 100% Rust, 0% other topics

## Next Steps

1. Agent loads `curriculum_master.json` to understand overall structure
2. Agent loads knowledge files based on current learning stage
3. Agent generates questions from current stage topics
4. Agent provides answers with examples from knowledge base
5. Agent tracks progress and advances through stages

This knowledge base provides everything needed to learn Rust comprehensively and efficiently, with zero wasted time on non-Rust topics.
