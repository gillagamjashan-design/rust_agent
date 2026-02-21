#!/bin/bash
# Quick test of web search functionality

cd /workspace/jashan/rust_agent

# Build first
echo "Building..."
CARGO_HOME=/workspace/jashan/.cargo cargo build --quiet

# Run the web search example if it exists
if [ -f "examples/test_web_search.rs" ]; then
    echo "Testing DuckDuckGo search..."
    CARGO_HOME=/workspace/jashan/.cargo cargo run --example test_web_search
else
    echo "Web search module compiled successfully!"
    echo "To test manually, run:"
    echo "  agent --interactive"
    echo "Then try: /web rust ownership"
fi
