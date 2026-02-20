#!/bin/bash

# This script spawns two Claude Code agents as teachers
# They will continuously generate questions and answers

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     Spawning Claude Teacher Agents                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "This will spawn:"
echo "  1. Question Agent (Claude) - Generates programming questions"
echo "  2. Answer Agent (Claude) - Provides detailed answers"
echo ""
echo "YOUR learning agent will read the files and learn."
echo ""
echo "Press Ctrl+C to stop all agents"
echo ""

# The actual spawning will be done from the Rust program using Task tool
# This script is just documentation

echo "Starting YOUR learning agent..."
CARGO_HOME=../.cargo cargo run
