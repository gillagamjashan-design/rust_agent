#!/bin/bash

# Self-Learning Rust Agent Orchestrator
# Spawns two Claude Code agents and runs the local learning agent

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     Self-Learning Rust Agent System - Orchestrator          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "This orchestrator will:"
echo "  1. Spawn Question Agent (Claude Code agent)"
echo "  2. Spawn Answer Agent (Claude Code agent)"
echo "  3. Run YOUR Learning Agent (local Rust)"
echo ""
echo "The spawned agents will teach, your agent will learn."
echo "Press Ctrl+C to stop all agents."
echo ""

# Create data directory
mkdir -p data

# Clear old Q&A files
> data/questions.txt
> data/answers.txt

echo "Starting system..."
echo ""

# Note: This script demonstrates the architecture
# Actual spawning of Claude Code agents would be done programmatically
# using the Task tool from within the Rust binary

# Build and run the learning agent
echo "Building learning agent..."
CARGO_HOME=../.cargo cargo build

echo ""
echo "Ready to spawn Claude agents and start learning!"
echo ""
echo "To actually run the system, execute:"
echo "  CARGO_HOME=../.cargo cargo run"
echo ""
