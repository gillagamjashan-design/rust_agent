#!/bin/bash

# Test file creation functionality

echo "Testing file creation detection..."
echo ""

# Create a test directory
TEST_DIR="/tmp/rusty_test_$$"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "Test directory: $TEST_DIR"
echo ""

# Test 1: Check if keywords are detected
echo "Test 1: Keyword detection"
echo "Query: 'create a hello world program'"
echo "Expected: should_create_files = true"
echo ""

# Test 2: Check if code block extraction works
echo "Test 2: Code block extraction"
cat > test_response.txt << 'EOF'
Here's a hello world program:

```rust
fn main() {
    println!("Hello, world!");
}
```
EOF

echo "Response contains rust code block: YES"
echo ""

# Test 3: Check actual binary
echo "Test 3: Binary check"
BINARY="/workspace/jashan/making_files/rusty_tui/target/release/rusty"
if [ -f "$BINARY" ]; then
    echo "✅ Binary exists: $BINARY"
    echo "   Size: $(ls -lh "$BINARY" | awk '{print $5}')"
    echo "   Built: $(ls -l "$BINARY" | awk '{print $6, $7, $8}')"
else
    echo "❌ Binary NOT found at $BINARY"
fi
echo ""

# Test 4: Check if running from correct directory
echo "Test 4: Working directory check"
echo "Current directory: $(pwd)"
echo "Files will be created in: $(pwd)/src/"
echo ""

# Test 5: Check ClaudeProxyAPI
echo "Test 5: Claude API check"
if curl -s http://localhost:8317/ > /dev/null 2>&1; then
    echo "✅ ClaudeProxyAPI is running"
else
    echo "❌ ClaudeProxyAPI is NOT running"
    echo "   Start it with: ./start_cliproxyapi.sh"
fi
echo ""

# Cleanup
cd /
rm -rf "$TEST_DIR"

echo "======================================"
echo "DEBUGGING TIPS:"
echo "======================================"
echo ""
echo "1. Run from a test directory:"
echo "   mkdir ~/test-rusty && cd ~/test-rusty"
echo "   /workspace/jashan/making_files/rusty_tui/target/release/rusty"
echo ""
echo "2. Check logs for debug output:"
echo "   Look for these in terminal:"
echo "   📁 Created N file(s)"
echo "   ⚠️  File creation error: ..."
echo ""
echo "3. Test with explicit @filepath:"
echo "   Type: 'Write hello world to @src/main.rs'"
echo ""
echo "4. Verify Claude returns code blocks:"
echo "   Make sure response contains \`\`\`rust ... \`\`\`"
echo ""
