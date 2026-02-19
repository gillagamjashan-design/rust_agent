# System Overview

## Self-Learning Rust Agent

A self-learning agent system where a locally-built Rust agent learns from two **spawned Claude Code agents** acting as teachers. The local agent learns by reading Q&A files without using any APIs itself.

## Core Concept

The system uses three main components:
1. **Question Agent (Spawned Claude Agent)** - A Claude Code agent spawned via Task tool that generates questions about Rust
2. **Answer Agent (Spawned Claude Agent)** - A Claude Code agent spawned via Task tool that provides detailed answers
3. **Learning Agent (YOUR Local Rust Agent)** - The agent YOU are building that reads Q&A files and learns WITHOUT any API calls

## Key Features

- **Hybrid Architecture**: Spawned Claude agents teach, your local Rust agent learns
- **API-Free Local Learning**: YOUR learning agent reads files only - no API calls
- **Continuous Learning**: Spawned agents keep teaching until stopped
- **Knowledge Accumulation**: Each Q&A cycle adds to YOUR agent's knowledge base
- **Practical Application**: YOUR agent will eventually generate Rust projects from learned knowledge

## Learning Mechanism

1. Spawned Question Agent (Claude) generates questions → writes to `questions.txt`
2. Spawned Answer Agent (Claude) reads questions → writes answers to `answers.txt`
3. YOUR Learning Agent (Rust) reads both files → extracts knowledge → stores in `knowledge_base.json`
4. Process continues until user stops the teaching cycle

## Why This Design?

- **Claude teaches**: Spawned agents provide high-quality Rust Q&A
- **You learn**: YOUR agent builds knowledge by observation only
- **No API dependency**: Once trained, YOUR agent works without APIs

## Use Cases

- Automated Rust project scaffolding
- Code generation based on accumulated patterns
- Self-improving development assistance
- Learning from historical Q&A interactions
