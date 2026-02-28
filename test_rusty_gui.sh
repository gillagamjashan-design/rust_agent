#!/bin/bash
# Test script for Rusty GUI
# This will start the GUI and capture startup messages

echo "========================================="
echo "Testing Rusty GUI v12.0.0"
echo "========================================="
echo ""

# Check if binary exists
if [ ! -f "rusty_tui/target/release/rusty" ]; then
    echo "❌ Binary not found!"
    exit 1
fi

echo "✅ Binary found: rusty_tui/target/release/rusty"
echo ""

# Check if knowledge database exists
DB_PATH="$HOME/.agent/data/knowledge.db"
if [ -f "$DB_PATH" ]; then
    echo "✅ Knowledge database exists at: $DB_PATH"
    echo "   Size: $(du -h "$DB_PATH" | cut -f1)"
else
    echo "⚠️  Knowledge database not found - will be created on first run"
fi
echo ""

echo "========================================="
echo "Starting Rusty GUI..."
echo "========================================="
echo ""
echo "Expected startup messages:"
echo "  🦀 Rusty - Rust Learning Agent v12.0.0"
echo "  ====================================="
echo "  ✅ Knowledge database found"
echo "  🚀 Starting GUI..."
echo "  🚀 Rusty GUI starting..."
echo "  📂 Database path: ..."
echo "  ✅ Worker thread spawned"
echo "  🎨 Initializing GUI..."
echo ""
echo "Expected GUI window:"
echo "  • Header with '🦀 Rusty' and stats"
echo "  • Chat area with welcome message visible"
echo "  • Input field at bottom"
echo ""
echo "Starting application now..."
echo "Press Ctrl+C in terminal to stop or close the window"
echo ""
echo "-----------------------------------------"

# Run the GUI (it will open a window)
cd rusty_tui
timeout 60 ./target/release/rusty 2>&1 &
GUI_PID=$!

# Give it a moment to start
sleep 2

echo ""
echo "-----------------------------------------"
echo "GUI process started with PID: $GUI_PID"
echo ""
echo "To test the GUI:"
echo "  1. Look for the window titled 'Rusty 🦀 - Rust Learning Agent'"
echo "  2. Check that the chat area shows the welcome message"
echo "  3. Try typing: 'What is ownership?' and press Enter"
echo "  4. Check /help command works"
echo "  5. Try /search ownership"
echo ""
echo "If GUI doesn't appear, check display settings or run:"
echo "  DISPLAY=:0 ./rusty_tui/target/release/rusty"
echo ""
echo "Process will timeout in 60 seconds or close the window to exit"
echo ""

# Wait for the process
wait $GUI_PID
EXIT_CODE=$?

echo ""
echo "========================================="
echo "GUI exited with code: $EXIT_CODE"
echo "========================================="
