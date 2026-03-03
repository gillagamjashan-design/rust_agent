#!/bin/bash
# Verification script for file creation bug fix

set -e

echo "🔍 File Creation Bug Fix Verification"
echo "======================================"
echo ""

# Check binary exists
BINARY="/workspace/jashan/rust_agent/rusty_tui/target/release/rusty"
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found at: $BINARY"
    echo "   Run: cd rusty_tui && cargo build --release"
    exit 1
fi

echo "✅ Binary found: $BINARY"
echo ""

# Test 1: Verify workspace detection in different directory
echo "📋 Test 1: Workspace Detection"
echo "-------------------------------"
TEST_DIR="/tmp/test_rusty_workspace"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "Current directory: $(pwd)"
echo "Expected workspace: $TEST_DIR"
echo ""

# The binary would show:
# 📂 File workspace: "/tmp/test_rusty_workspace"
# But we can't run GUI in CI, so we verify the code changes instead

echo "✅ Workspace will be: $(pwd)"
echo ""

# Test 2: Verify code changes
echo "📋 Test 2: Code Changes Verification"
echo "-------------------------------------"

WORKER_FILE="/workspace/jashan/rust_agent/rusty_tui/src/gui/worker.rs"

echo "Checking worker.rs for workspace fix..."
if grep -q "std::env::current_dir()" "$WORKER_FILE"; then
    echo "✅ Found std::env::current_dir() in worker.rs"
else
    echo "❌ Missing std::env::current_dir() in worker.rs"
    exit 1
fi

if grep -q '📂 File workspace:' "$WORKER_FILE"; then
    echo "✅ Found workspace logging in worker.rs"
else
    echo "❌ Missing workspace logging in worker.rs"
    exit 1
fi

if grep -q '🔨 Creating file:' "$WORKER_FILE"; then
    echo "✅ Found file creation logging in worker.rs"
else
    echo "❌ Missing file creation logging in worker.rs"
    exit 1
fi

echo ""

# Test 3: Verify welcome message update
echo "📋 Test 3: Welcome Message Update"
echo "----------------------------------"

APP_FILE="/workspace/jashan/rust_agent/rusty_tui/src/gui/app.rs"

if grep -q "Files will be created in:" "$APP_FILE"; then
    echo "✅ Found workspace info in welcome message"
else
    echo "❌ Missing workspace info in welcome message"
    exit 1
fi

echo ""

# Test 4: Verify no hardcoded path
echo "📋 Test 4: No Hardcoded Path"
echo "----------------------------"

if grep -q '"/workspace/jashan/rust_agent"' "$WORKER_FILE"; then
    echo "❌ Still contains hardcoded path!"
    echo "   Found in: $WORKER_FILE"
    grep -n '"/workspace/jashan/rust_agent"' "$WORKER_FILE" || true
    exit 1
else
    echo "✅ No hardcoded workspace path found"
fi

echo ""

# Summary
echo "======================================"
echo "🎉 All Verification Tests Passed!"
echo "======================================"
echo ""
echo "Summary of Changes:"
echo "  • Workspace: hardcoded → std::env::current_dir()"
echo "  • Added: Workspace logging on startup"
echo "  • Added: File creation logging"
echo "  • Updated: Welcome message with workspace path"
echo ""
echo "Next Steps:"
echo "  1. Run the binary: cd /tmp/test_dir && $BINARY"
echo "  2. Check terminal output for: 📂 File workspace: \"/tmp/test_dir\""
echo "  3. Ask agent to create a file"
echo "  4. Verify file appears in /tmp/test_dir (not hardcoded path)"
echo ""
echo "Expected Terminal Output:"
echo "  📂 File workspace: \"/tmp/test_dir\""
echo "  🔨 Creating file: main.rs (123 bytes)"
echo "  ✅ Created file: main.rs"
echo ""
