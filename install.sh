#!/bin/bash
set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║              Agent Installer                                 ║"
echo "║   Self-Learning Rust Agent with CLIProxyAPI                 ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check if CARGO_HOME is set, otherwise use default
if [ -z "$CARGO_HOME" ]; then
    export CARGO_HOME="$HOME/.cargo"
fi

# Build the release binary
echo "🔨 Building agent binary..."
CARGO_HOME=../.cargo cargo build --release

# Create ~/.local/bin if it doesn't exist
mkdir -p "$HOME/.local/bin"

# Install the binary
echo "📦 Installing agent to ~/.local/bin/agent..."
cp target/release/agent "$HOME/.local/bin/agent"
chmod +x "$HOME/.local/bin/agent"

# Create ~/.agent/data directory
echo "📁 Setting up data directory at ~/.agent/data/..."
mkdir -p "$HOME/.agent/data"

# Copy existing knowledge base if it exists locally
if [ -f "data/knowledge_base.json" ]; then
    echo "📚 Copying existing knowledge base..."
    cp data/knowledge_base.json "$HOME/.agent/data/knowledge_base.json"
fi

# Copy existing Q&A files if they exist
if [ -f "data/questions.txt" ]; then
    echo "❓ Copying existing questions..."
    cp data/questions.txt "$HOME/.agent/data/questions.txt"
fi

if [ -f "data/answers.txt" ]; then
    echo "✅ Copying existing answers..."
    cp data/answers.txt "$HOME/.agent/data/answers.txt"
fi

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  ~/.local/bin is not in your PATH!"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "Then run: source ~/.bashrc (or source ~/.zshrc)"
else
    echo ""
    echo "✅ ~/.local/bin is already in your PATH"
fi

echo ""
echo "✨ Installation complete!"
echo ""
echo "Usage:"
echo "  agent                  - Run in learning mode"
echo "  agent --interactive    - Run in interactive mode"
echo ""
echo "Data location: ~/.agent/data/"
echo "  - knowledge_base.json  - The agent's learned knowledge"
echo "  - questions.txt        - Generated questions"
echo "  - answers.txt          - Generated answers"
echo ""
