#!/bin/bash
set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║              Agent Updater                                   ║"
echo "║   Updating agent with new features                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check if agent is currently running
if pgrep -x "agent" > /dev/null; then
    echo "⚠️  Agent is currently running. Stopping it..."
    pkill -x "agent"
    sleep 2
fi

# Build the release binary
echo "🔨 Building updated agent binary..."
CARGO_HOME=../.cargo cargo build --release

# Install the binary
echo "📦 Installing updated agent to ~/.local/bin/agent..."
cp target/release/agent "$HOME/.local/bin/agent"
chmod +x "$HOME/.local/bin/agent"

# Knowledge base is preserved in ~/.agent/data/ - no need to copy

echo ""
echo "✨ Update complete!"
echo ""
echo "Your agent's brain (knowledge base) is preserved at:"
echo "  ~/.agent/data/knowledge_base.json"
echo ""
echo "Usage:"
echo "  agent                  - Run in learning mode"
echo "  agent --interactive    - Run in interactive mode"
echo ""
