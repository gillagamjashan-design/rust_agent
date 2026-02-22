# Rusty IDE Agent Bridge - Architecture Diagram

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         RUSTY IDE (Tauri Application)                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │                     FRONTEND (TypeScript/React)                │    │
│  │                                                                │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐          │    │
│  │  │   Editor    │  │  File Tree  │  │ Agent Panel  │          │    │
│  │  │   Monaco    │  │             │  │              │          │    │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬───────┘          │    │
│  │         │                │                 │                  │    │
│  │         └────────────────┴─────────────────┘                  │    │
│  │                          │                                    │    │
│  │                          v                                    │    │
│  │                ┌──────────────────┐                           │    │
│  │                │  Tauri Commands  │                           │    │
│  │                │  invoke('...')   │                           │    │
│  │                └────────┬─────────┘                           │    │
│  └─────────────────────────┼────────────────────────────────────┘    │
│                            │                                          │
│  ══════════════════════════╪══════════════════════════════════════    │
│                            │ IPC Bridge                               │
│  ══════════════════════════╪══════════════════════════════════════    │
│                            │                                          │
│  ┌─────────────────────────┼────────────────────────────────────┐    │
│  │                   BACKEND (Rust)                             │    │
│  │                         │                                    │    │
│  │     ┌───────────────────┴────────────────────┐               │    │
│  │     │      Tauri Command Handlers            │               │    │
│  │     │                                        │               │    │
│  │     │  - query_agent()                       │               │    │
│  │     │  - check_agent_response()              │               │    │
│  │     │  - clear_agent_files()                 │               │    │
│  │     │  - get_agent_info()                    │               │    │
│  │     └───────────────────┬────────────────────┘               │    │
│  │                         │                                    │    │
│  │                         v                                    │    │
│  │            ┌────────────────────────────┐                    │    │
│  │            │     AgentBridge            │                    │    │
│  │            │                            │                    │    │
│  │            │  - send_request()          │                    │    │
│  │            │  - wait_for_response()     │                    │    │
│  │            │  - check_response()        │                    │    │
│  │            │  - clear()                 │                    │    │
│  │            │  - get_ide_source()        │                    │    │
│  │            └────────────┬───────────────┘                    │    │
│  │                         │                                    │    │
│  │            ┌────────────┴───────────────┐                    │    │
│  │            │   File Watcher (notify)    │                    │    │
│  │            │   - Event-driven           │                    │    │
│  │            │   - Low CPU usage          │                    │    │
│  │            └────────────┬───────────────┘                    │    │
│  └─────────────────────────┼────────────────────────────────────┘    │
└─────────────────────────────┼─────────────────────────────────────────┘
                              │
                              │ File I/O
                              v
┌─────────────────────────────────────────────────────────────────────────┐
│                    FILE SYSTEM (~/.rusty/agent/)                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  request.json          ◄────── IDE writes request                      │
│  {                                                                      │
│    "timestamp": "...",                                                  │
│    "workspace_path": "/path",                                           │
│    "current_file": "main.rs",                                           │
│    "current_code": "fn main() {}",                                      │
│    "files": ["..."],                                                    │
│    "ide_source": "...",                                                 │
│    "query": "Add error handling"                                        │
│  }                                                                      │
│                                                                         │
│  response.json         ──────► IDE reads response                       │
│  {                                                                      │
│    "timestamp": "...",                                                  │
│    "response_text": "...",                                              │
│    "code_suggestions": [...],                                           │
│    "apply_changes": true                                                │
│  }                                                                      │
│                                                                         │
│  wrapper.log           ──────► Diagnostic logs                          │
│  .processing           ──────► Lock flag                                │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              │ inotify events
                              v
┌─────────────────────────────────────────────────────────────────────────┐
│                    AGENT WRAPPER (agent_wrapper.sh)                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────────────────────────────────────────────────┐          │
│  │  File Watcher Loop (inotifywait)                         │          │
│  │                                                           │          │
│  │  1. Wait for request.json creation/modification          │          │
│  │  2. Read request.json                                    │          │
│  │  3. Extract query and context                            │          │
│  │  4. Call external agent                                  │          │
│  │  5. Parse agent output                                   │          │
│  │  6. Extract code blocks                                  │          │
│  │  7. Generate response.json                               │          │
│  │  8. Clean up request.json                                │          │
│  │  9. Loop                                                 │          │
│  │                                                           │          │
│  └──────────────────────────────────────────────────────────┘          │
│                                                                         │
│  Features:                                                              │
│  - Daemon mode (--daemon)                                               │
│  - Single-shot mode                                                     │
│  - Timeout handling                                                     │
│  - Logging to wrapper.log                                               │
│  - Process locking (.processing flag)                                   │
│  - Code block extraction                                                │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              │ Process execution
                              v
┌─────────────────────────────────────────────────────────────────────────┐
│                    EXTERNAL AGENT (rust_agent)                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Your existing rust_agent or any compatible AI agent                   │
│                                                                         │
│  Input:  Query string from request.json                                │
│  Output: Text response with optional code blocks                       │
│                                                                         │
│  Example:                                                               │
│    rust_agent -p "Add error handling to: fn main() {}"                 │
│                                                                         │
│  Output:                                                                │
│    Here's the improved code:                                            │
│                                                                         │
│    ```rust                                                              │
│    fn main() -> Result<(), Box<dyn Error>> {                            │
│        // Your code here                                                │
│        Ok(())                                                            │
│    }                                                                    │
│    ```                                                                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Data Flow Sequence

```
User Action                IDE                 FileSystem           Wrapper              Agent
    │                      │                       │                  │                    │
    │   Type query         │                       │                  │                    │
    ├─────────────────────>│                       │                  │                    │
    │                      │                       │                  │                    │
    │                      │ Write request.json    │                  │                    │
    │                      ├──────────────────────>│                  │                    │
    │                      │                       │                  │                    │
    │                      │ Start watching        │                  │                    │
    │                      │ (notify crate)        │                  │                    │
    │                      │                       │                  │                    │
    │                      │                       │ Detect change    │                    │
    │                      │                       │ (inotifywait)    │                    │
    │                      │                       │<─────────────────┤                    │
    │                      │                       │                  │                    │
    │                      │                       │ Read request     │                    │
    │                      │                       ├─────────────────>│                    │
    │                      │                       │                  │                    │
    │                      │                       │                  │ Execute agent      │
    │                      │                       │                  ├───────────────────>│
    │                      │                       │                  │                    │
    │                      │                       │                  │ Process & respond  │
    │                      │                       │                  │<───────────────────┤
    │                      │                       │                  │                    │
    │                      │                       │ Write response   │                    │
    │                      │                       │<─────────────────┤                    │
    │                      │                       │                  │                    │
    │                      │ File event received   │                  │                    │
    │                      │ (notify crate)        │                  │                    │
    │                      │<──────────────────────┤                  │                    │
    │                      │                       │                  │                    │
    │                      │ Read response.json    │                  │                    │
    │                      ├──────────────────────>│                  │                    │
    │                      │                       │                  │                    │
    │  Display result      │                       │                  │                    │
    │<─────────────────────┤                       │                  │                    │
    │                      │                       │                  │                    │
    │                      │ Clean up files        │                  │                    │
    │                      ├──────────────────────>│                  │                    │
    │                      │                       │                  │                    │
```

## Component Responsibilities

### Frontend (TypeScript)
- User interface
- Editor integration
- Display agent responses
- Apply code suggestions

### Backend (Rust)
- Tauri command handlers
- AgentBridge management
- File I/O operations
- Response parsing

### AgentBridge (Rust module)
- File-based communication protocol
- Request/response serialization
- File watching (notify crate)
- Timeout handling
- Error management

### Agent Wrapper (Bash)
- Daemon process
- File system monitoring
- Request processing
- Agent execution
- Response generation

### External Agent
- AI processing
- Code generation
- Query answering
- Context understanding

## Key Technologies

- **Tauri**: Cross-platform desktop framework
- **notify**: Rust file system watcher
- **serde/serde_json**: JSON serialization
- **inotify-tools**: Linux file monitoring
- **jq**: JSON processing in bash
- **systemd**: Service management

## Deployment Modes

### Development
```bash
# Terminal 1: Wrapper daemon
./agent_wrapper.sh --daemon

# Terminal 2: IDE
cargo run
```

### Production
```bash
# Install as system service
./scripts/install-agent-service.sh

# Run IDE
./rusty-ide
```

## Performance Characteristics

| Operation              | Latency    | Notes                          |
|------------------------|------------|--------------------------------|
| Write request.json     | 1-5ms      | File system dependent          |
| Detect file change     | 10-50ms    | inotify/notify crate           |
| Read response.json     | 1-5ms      | File system dependent          |
| Agent processing       | 1-30s      | Depends on query complexity    |
| Total roundtrip        | ~100ms     | Excluding agent time           |

## Security Model

1. All files in user home directory (~/.rusty/agent/)
2. User-level file permissions
3. No network communication
4. Local process execution only
5. Optional encryption can be added

## Error Handling

```
Error Types:
├─ AgentBridgeError::Io          → File system errors
├─ AgentBridgeError::Serialization → JSON errors
├─ AgentBridgeError::Timeout     → Agent timeout
├─ AgentBridgeError::Watcher     → File watcher errors
├─ AgentBridgeError::InvalidResponse → Bad response format
└─ AgentBridgeError::NoResponse  → Response not available
```

## Extension Points

1. Custom agent implementations
2. Alternative communication protocols
3. Request/response middleware
4. Encryption layers
5. Caching mechanisms
6. Request queuing
7. Multi-agent support
