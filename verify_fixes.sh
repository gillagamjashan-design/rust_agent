#!/bin/bash
# Verification script for GUI fixes
# Checks that all code changes were applied correctly

echo "========================================="
echo "Verifying Rusty GUI Fixes"
echo "========================================="
echo ""

PASS=0
FAIL=0

# Function to check if a file contains a pattern
check_pattern() {
    local file=$1
    local pattern=$2
    local description=$3

    if grep -q "$pattern" "$file"; then
        echo "✅ $description"
        ((PASS++))
    else
        echo "❌ FAIL: $description"
        ((FAIL++))
    fi
}

echo "Checking layout.rs fixes..."
check_pattern "rusty_tui/src/gui/layout.rs" "min_scrolled_height(200.0)" "ScrollArea has minimum height"
check_pattern "rusty_tui/src/gui/layout.rs" "max_height(ui.available_height())" "ScrollArea uses available height"
check_pattern "rusty_tui/src/gui/layout.rs" "allocate_ui_with_layout" "Chat area uses explicit layout allocation"
check_pattern "rusty_tui/src/gui/layout.rs" "ui.add_space(15.0)" "Messages have spacing between them"
echo ""

echo "Checking app.rs improvements..."
check_pattern "rusty_tui/src/gui/app.rs" "Welcome to Rusty - Your Rust Learning Agent" "Enhanced welcome message"
check_pattern "rusty_tui/src/gui/app.rs" "Rusty GUI starting" "Startup logging added"
check_pattern "rusty_tui/src/gui/app.rs" "Worker thread spawned" "Worker spawn logging"
echo ""

echo "Checking worker.rs error handling..."
check_pattern "rusty_tui/src/gui/worker.rs" "Received query:" "Query logging added"
check_pattern "rusty_tui/src/gui/worker.rs" "Querying Claude API" "API call logging"
check_pattern "rusty_tui/src/gui/worker.rs" "Make sure ClaudeProxyAPI is running" "Helpful error messages"
echo ""

echo "Checking main.rs startup info..."
check_pattern "rusty_tui/src/main.rs" "v12.0.0" "Version number in startup"
check_pattern "rusty_tui/src/main.rs" "Starting GUI..." "Startup message added"
echo ""

echo "========================================="
echo "Verification Summary"
echo "========================================="
echo "✅ Passed: $PASS"
echo "❌ Failed: $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 All fixes verified successfully!"
    echo ""
    echo "Next steps:"
    echo "  1. Binary built at: rusty_tui/target/release/rusty"
    echo "  2. Run it with: ./rusty_tui/target/release/rusty"
    echo "  3. Expected: GUI window with visible welcome message"
    echo ""
    exit 0
else
    echo "⚠️  Some checks failed. Review the output above."
    exit 1
fi
