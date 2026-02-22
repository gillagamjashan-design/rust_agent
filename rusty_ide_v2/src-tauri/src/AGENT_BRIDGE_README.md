# Agent Bridge - File-Based Communication System

## Overview

The Agent Bridge provides a robust, file-based communication system between Rusty IDE and external AI agents. It uses a simple JSON protocol over the filesystem, making it easy to integrate with any agent implementation.

## Architecture

```
┌─────────────────┐                           ┌──────────────────┐
│   Rusty IDE     │                           │  External Agent  │
│   (Tauri App)   │                           │   (rust_agent)   │
└────────┬────────┘                           └────────┬─────────┘
         │                                              │
         │  1. Write request.json                       │
         ├─────────────────────────────────────────────>│
         │                                              │
         │                                              │  2. Process
         │                                              │     request
         │                                              │
         │  3. Read response.json                       │
         <─────────────────────────────────────────────┤
         │                                              │
         
         ~/.rusty/agent/
         ├── request.json    (IDE → Agent)
         ├── response.json   (Agent → IDE)
         └── wrapper.log     (Agent wrapper logs)
```

## File Protocol

### Request Format (`request.json`)

```json
{
  "timestamp": "2026-02-21T10:30:00.000000000Z",
  "workspace_path": "/home/user/my_project",
  "current_file": "src/main.rs",
  "current_code": "fn main() {\n    println!(\"Hello\");\n}",
  "files": ["src/main.rs", "src/lib.rs", "Cargo.toml"],
  "ide_source": "// Complete IDE source code for self-awareness...",
  "query": "Add error handling to this function"
}
```

**Fields:**
- `timestamp`: ISO 8601 timestamp when request was created
- `workspace_path`: Absolute path to the current workspace/project
- `current_file`: Name of the file currently being edited (optional)
- `current_code`: Contents of the current file (optional)
- `files`: List of all files in the workspace
- `ide_source`: Complete source code of the IDE itself (for self-modification)
- `query`: The user's request/question for the agent

### Response Format (`response.json`)

```json
{
  "timestamp": "2026-02-21T10:30:15.000000000Z",
  "response_text": "I've added proper error handling to your function...",
  "code_suggestions": [
    {
      "file": "src/main.rs",
      "code": "fn main() -> Result<(), Box<dyn Error>> {\n    println!(\"Hello\");\n    Ok(())\n}",
      "language": "rust",
      "description": "Added Result return type and error propagation"
    }
  ],
  "apply_changes": true
}
```

**Fields:**
- `timestamp`: ISO 8601 timestamp when response was created
- `response_text`: Human-readable explanation from the agent
- `code_suggestions`: Array of code changes to apply
  - `file`: Which file to modify
  - `code`: The new/modified code
  - `language`: Programming language (for syntax highlighting)
  - `description`: Brief description of the change
- `apply_changes`: Whether to automatically apply the suggestions

## Usage

### Rust API (Backend)

```rust
use rusty_ide::{AgentBridge, AgentContext};
use std::time::Duration;

// Initialize the bridge
let bridge = AgentBridge::new()?;

// Create a context with your query
let context = AgentContext::new("Explain this code".to_string())
    .with_workspace("/path/to/workspace".to_string())
    .with_current_file("main.rs".to_string(), "fn main() {}".to_string())
    .with_files(vec!["main.rs".to_string(), "lib.rs".to_string()]);

// Send request to agent
bridge.send_request(context)?;

// Wait for response (blocking with timeout)
match bridge.wait_for_response(Duration::from_secs(30)) {
    Ok(response) => {
        println!("Agent says: {}", response.response_text);
        
        for suggestion in response.code_suggestions {
            println!("Suggestion for {}: {}", suggestion.file, suggestion.description);
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}

// Or check without blocking
if let Some(response) = bridge.check_response()? {
    println!("Got response: {}", response.response_text);
}
```

### Tauri Commands (Frontend Integration)

```typescript
// TypeScript/JavaScript frontend code

import { invoke } from '@tauri-apps/api/tauri';

// Query the agent
const response = await invoke('query_agent', {
  workspace: '/path/to/workspace',
  currentFile: 'main.rs',
  currentCode: 'fn main() {}',
  files: ['main.rs', 'lib.rs'],
  query: 'Add error handling'
});

console.log(response.response_text);
response.code_suggestions.forEach(suggestion => {
  console.log(`${suggestion.file}: ${suggestion.description}`);
});

// Check for response without blocking
const maybeResponse = await invoke('check_agent_response');
if (maybeResponse) {
  console.log('Got response:', maybeResponse);
}

// Clear agent files
await invoke('clear_agent_files');

// Get agent info
const info = await invoke('get_agent_info');
console.log('Agent directory:', info.agent_dir);
```

## Agent Wrapper Script

The `agent_wrapper.sh` script provides a daemon that watches for requests and processes them automatically.

### Running the Wrapper

```bash
# Process a single request and exit
./agent_wrapper.sh

# Run as a daemon (continuous monitoring)
./agent_wrapper.sh --daemon

# Use custom agent path
./agent_wrapper.sh --daemon --agent-path /usr/local/bin/my_agent

# Set timeout
./agent_wrapper.sh --daemon --timeout 300
```

### Environment Variables

- `RUSTY_AGENT_DIR`: Directory for request/response files (default: `~/.rusty/agent`)
- `RUST_AGENT_PATH`: Path to the rust_agent executable

### Running as a System Service

Create a systemd service file (`~/.config/systemd/user/rusty-agent.service`):

```ini
[Unit]
Description=Rusty IDE Agent Wrapper
After=network.target

[Service]
Type=simple
ExecStart=/path/to/rusty_ide_v2/src-tauri/src/agent_wrapper.sh --daemon
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
```

Enable and start:
```bash
systemctl --user enable rusty-agent
systemctl --user start rusty-agent
systemctl --user status rusty-agent
```

## Features

### File Watching
- Uses `notify` crate for efficient file system monitoring
- Instant detection of response files
- Low CPU usage (event-driven, not polling)

### Timeout Handling
- Configurable timeouts (default: 30 seconds)
- Graceful timeout errors
- No hanging operations

### Thread Safety
- Safe to use from multiple threads
- Uses `Arc<Mutex<>>` for shared state
- Proper cleanup on errors

### Error Handling
- Comprehensive error types with `thiserror`
- Detailed error messages
- Graceful degradation

### Automatic Cleanup
- Response files removed after reading
- No stale data accumulation
- Processing flags for concurrency control

## Testing

Run the test suite:

```bash
cd src-tauri
cargo test
```

Example tests:
```rust
#[test]
fn test_send_and_receive() {
    let bridge = AgentBridge::new().unwrap();
    
    let context = AgentContext::new("Test query".to_string());
    bridge.send_request(context).unwrap();
    
    // Simulate agent writing response
    let response = AgentResponse {
        timestamp: get_timestamp(),
        response_text: "Test response".to_string(),
        code_suggestions: vec![],
        apply_changes: false,
    };
    
    let json = serde_json::to_string(&response).unwrap();
    fs::write(bridge.response_path(), json).unwrap();
    
    // Check response
    let received = bridge.check_response().unwrap().unwrap();
    assert_eq!(received.response_text, "Test response");
}
```

## Debugging

Enable debug logging:

```bash
# Set environment variable
export RUST_LOG=debug

# Check wrapper logs
tail -f ~/.rusty/agent/wrapper.log

# Monitor file system events
inotifywait -m ~/.rusty/agent/
```

## Troubleshooting

### Request not being processed
1. Check if agent wrapper is running: `ps aux | grep agent_wrapper`
2. Check wrapper logs: `cat ~/.rusty/agent/wrapper.log`
3. Verify agent is installed: `which rust_agent`
4. Check file permissions: `ls -la ~/.rusty/agent/`

### Timeout errors
1. Increase timeout: `bridge.wait_for_response(Duration::from_secs(60))`
2. Check agent performance
3. Verify agent is receiving requests
4. Check system resources

### Missing dependencies
- Install `notify` crate: Already in Cargo.toml
- Install `jq`: `sudo apt-get install jq`
- Install `inotify-tools`: `sudo apt-get install inotify-tools`

## Best Practices

1. **Always use timeouts**: Don't wait indefinitely for responses
2. **Handle errors gracefully**: Agent might be unavailable
3. **Clear old files**: Call `clear()` before sending new requests
4. **Provide context**: Include relevant files and code for better responses
5. **Test extensively**: Use the test suite and add your own tests

## Performance Considerations

- **File I/O**: Minimal overhead, writes are async from agent's perspective
- **Watching**: Event-driven, not polling-based
- **Memory**: Small JSON files, minimal memory usage
- **Latency**: ~100ms overhead (file writes + event detection)

## Security Notes

- Files are written to `~/.rusty/agent/` (user-owned directory)
- No network communication (all local file-based)
- No sensitive data in request/response by default
- Consider encrypting IDE source code in requests if needed

## Future Enhancements

Potential improvements:
- [ ] Add encryption for sensitive data
- [ ] Support for binary data (images, compiled artifacts)
- [ ] Multiple concurrent requests with request IDs
- [ ] Streaming responses for long-running operations
- [ ] WebSocket-based communication as alternative
- [ ] Agent health monitoring and auto-restart
- [ ] Request queue for multiple pending requests
- [ ] Response caching for repeated queries

## License

MIT License - See LICENSE file for details
