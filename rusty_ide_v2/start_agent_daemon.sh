#!/bin/bash
# Start the agent daemon for Rusty TUI

AGENT_PATH="$HOME/.local/bin/agent"
WRAPPER_PATH="/workspace/jashan/rust_agent/rusty_ide_v2/src-tauri/src/agent_wrapper.sh"

if [ ! -f "$AGENT_PATH" ]; then
    echo "Error: Agent not found at $AGENT_PATH"
    echo "Please run: cd /workspace/jashan/rust_agent && cargo build --release && ./install.sh"
    exit 1
fi

export RUST_AGENT_PATH="$AGENT_PATH"
echo "Starting agent daemon..."
echo "Agent path: $AGENT_PATH"
echo "Watching: ~/.rusty/agent/"
echo ""
echo "Press Ctrl+C to stop"

exec "$WRAPPER_PATH"
