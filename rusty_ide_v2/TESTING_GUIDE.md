# Agent Bridge Testing Guide

This guide provides step-by-step instructions for testing the Agent Bridge implementation.

## Prerequisites

1. Rust toolchain installed (1.70+)
2. External agent (`rust_agent`) installed and in PATH
3. System dependencies:
   ```bash
   sudo apt-get install jq inotify-tools
   ```

## Testing Levels

### 1. Unit Tests (Automatic)

These tests are built into the `agent_bridge.rs` file.

```bash
cd /workspace/jashan/rusty_ide_v2/src-tauri

# Run all unit tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_agent_bridge_creation
```

Expected output:
```
running 7 tests
test tests::test_agent_bridge_creation ... ok
test tests::test_agent_context_builder ... ok
test tests::test_send_request ... ok
test tests::test_clear ... ok
test tests::test_timestamp_format ... ok
test tests::test_agent_response_methods ... ok
test tests::test_get_ide_source ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured
```

### 2. Integration Tests

```bash
cd /workspace/jashan/rusty_ide_v2/src-tauri

# Run integration tests
cargo test --test agent_bridge_tests

# Run specific integration test
cargo test --test agent_bridge_tests test_bridge_initialization
```

### 3. Manual File System Test

Test the file-based communication manually:

#### Terminal 1: Start the wrapper daemon

```bash
cd /workspace/jashan/rusty_ide_v2/src-tauri/src
./agent_wrapper.sh --daemon
```

You should see:
```
[2026-02-21 13:57:00] [INFO] === Rusty IDE Agent Wrapper ===
[2026-02-21 13:57:00] [INFO] Starting agent wrapper in daemon mode
[2026-02-21 13:57:00] [INFO] Monitoring directory: /home/user/.rusty/agent
```

#### Terminal 2: Create a test request

```bash
# Create request file
cat > ~/.rusty/agent/request.json << 'JSONEOF'
{
  "timestamp": "2026-02-21T13:57:00.000Z",
  "workspace_path": "/tmp/test",
  "current_file": "test.rs",
  "current_code": "fn main() { println!(\"Hello\"); }",
  "files": ["test.rs"],
  "ide_source": "",
  "query": "Add error handling to this function"
}
JSONEOF

echo "Request created. Waiting for response..."
sleep 5

# Check for response
if [ -f ~/.rusty/agent/response.json ]; then
    echo "✓ Response received!"
    cat ~/.rusty/agent/response.json | jq .
else
    echo "✗ No response yet"
fi
```

Expected response:
```json
{
  "timestamp": "2026-02-21T13:57:05.000Z",
  "response_text": "Here's the improved function with error handling...",
  "code_suggestions": [
    {
      "file": "test.rs",
      "code": "fn main() -> Result<(), Box<dyn Error>> { ... }",
      "language": "rust",
      "description": "Added Result return type"
    }
  ],
  "apply_changes": true
}
```

### 4. Rust API Test

Create a test file: `test_bridge.rs`

```rust
use rusty_ide::{AgentBridge, AgentContext};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Agent Bridge...\n");

    // Create bridge
    let bridge = AgentBridge::new()?;
    println!("✓ Bridge created");
    println!("  Agent dir: {}", bridge.agent_dir().display());

    // Create context
    let context = AgentContext::new("What is Rust?".to_string())
        .with_workspace("/tmp/test".to_string())
        .with_files(vec!["test.rs".to_string()]);
    
    println!("✓ Context created");

    // Send request
    bridge.send_request(context)?;
    println!("✓ Request sent");
    println!("  Waiting for response (30s timeout)...");

    // Wait for response
    match bridge.wait_for_response(Duration::from_secs(30)) {
        Ok(response) => {
            println!("✓ Response received!");
            println!("\nResponse text:");
            println!("{}", response.response_text);
            
            if response.has_suggestions() {
                println!("\nCode suggestions: {}", response.code_suggestions.len());
                for suggestion in &response.code_suggestions {
                    println!("  - {}: {}", suggestion.file, suggestion.description);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            return Err(e.into());
        }
    }

    // Clean up
    bridge.clear()?;
    println!("\n✓ Cleanup complete");

    Ok(())
}
```

Run it:
```bash
cd /workspace/jashan/rusty_ide_v2/src-tauri
cargo run --example test_bridge
```

### 5. Performance Test

Test response times and throughput:

```bash
cd /workspace/jashan/rusty_ide_v2/src-tauri

# Create performance test
cat > perf_test.rs << 'EOF'
use rusty_ide::{AgentBridge, AgentContext};
use std::time::{Duration, Instant};

fn main() {
    println!("Agent Bridge Performance Test\n");

    let bridge = AgentBridge::new().unwrap();
    
    // Test 1: Request send time
    let start = Instant::now();
    let context = AgentContext::new("Test".to_string());
    bridge.send_request(context).unwrap();
    let send_time = start.elapsed();
    println!("Request send time: {:?}", send_time);

    // Test 2: Non-blocking check time
    let start = Instant::now();
    let _ = bridge.check_response().unwrap();
    let check_time = start.elapsed();
    println!("Non-blocking check time: {:?}", check_time);

    // Clean up
    bridge.clear().unwrap();

    println!("\n✓ Performance test complete");
}
