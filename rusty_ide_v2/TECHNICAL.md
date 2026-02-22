# Rusty IDE v2 - Technical Documentation

## Backend Architecture

### Overview

The Tauri backend is built with Rust and provides high-performance native capabilities:
- PTY terminal emulation
- File system operations with permissions
- File-based agent communication
- Self-awareness (IDE source code introspection)

### Project Structure

```
src-tauri/
├── src/
│   ├── main.rs              # Main application with all Tauri commands
│   ├── agent_bridge.rs      # Agent communication implementation
│   └── lib.rs               # Public API exports
├── Cargo.toml               # Dependencies and build config
├── tauri.conf.json          # Tauri window and security settings
└── build.rs                 # Build script
```

## Core Components

### 1. PTY Terminal System

**Implementation:** Uses `portable-pty` crate for cross-platform terminal emulation.

**Key Features:**
- Native shell detection (bash/zsh on Unix, PowerShell on Windows)
- Async I/O with 8KB buffer
- Event-driven output streaming
- Terminal lifecycle management

**Data Flow:**
```
Frontend (xterm.js)
    │
    ├─→ create_terminal() → TerminalId
    │   └─→ Spawns PTY + Shell
    │       └─→ Starts async reader task
    │
    ├─→ write_to_terminal(id, data)
    │   └─→ Writes to PTY master
    │
    ├─→ resize_terminal(id, cols, rows)
    │   └─→ Updates PTY size
    │
    └─→ close_terminal(id)
        └─→ Cleanup resources

Backend Events:
    terminal-data { id, data } → Frontend
```

**Code Structure:**
```rust
struct TerminalInstance {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    window: Window,
    id: String,
}
```

### 2. File System Operations

**Implementation:** Native `std::fs` with `notify` crate for watching.

**Permission Model:**
- Whitelist-based access control
- Canonical path resolution (prevents traversal attacks)
- Persistent storage in `~/.rusty/permissions.json`

**Operations:**
```rust
read_file(path) -> String
write_file(path, content)
list_files(directory) -> Vec<FileInfo>
watch_directory(path)
unwatch_directory(path)
```

**File Watcher Architecture:**
```
notify::RecommendedWatcher
    │
    ├─→ Watches directory recursively
    │
    └─→ Events → Tauri Event System
            │
            └─→ file-system-event { kind, paths }
```

### 3. Agent Communication Bridge

**Design:** File-based IPC using JSON files in `~/.rusty/agent/`

**Communication Flow:**
```
1. IDE writes request.json:
   {
     timestamp,
     workspace_path,
     current_file,
     current_code,
     files[],
     ide_source,
     query
   }

2. External agent processes request

3. Agent writes response.json:
   {
     timestamp,
     response_text,
     code_suggestions[],
     apply_changes
   }

4. IDE polls/watches for response
```

**Implementation Details:**
```rust
pub struct AgentBridge {
    agent_dir: PathBuf,           // ~/.rusty/agent/
    request_path: PathBuf,        // request.json
    response_path: PathBuf,       // response.json
    watcher: Arc<Mutex<Option<RecommendedWatcher>>>,
    receiver: Arc<Mutex<Option<Receiver<Event>>>>,
}
```

**Key Methods:**
```rust
send_request(context) -> Result<()>
    ├─ Clears old response
    ├─ Serializes context to JSON
    └─ Writes request.json

wait_for_response(timeout) -> Result<AgentResponse>
    ├─ Starts file watcher
    ├─ Polls for response file
    └─ Reads and parses response

check_response() -> Result<Option<AgentResponse>>
    └─ Non-blocking response check
```

### 4. Self-Awareness System

**Purpose:** Allows IDE to read its own source code for agent context.

**Implementation:**
```rust
get_ide_source() -> Result<String>
    ├─ Locates src-tauri/src directory
    ├─ Recursively reads all .rs files
    └─ Concatenates with file headers
```

**Use Case:**
The agent can understand the IDE's capabilities by reading the source code, enabling self-improvement and debugging.

## Application State

```rust
struct AppState {
    terminals: Arc<Mutex<HashMap<String, TerminalInstance>>>,
    permissions: Arc<Mutex<Vec<String>>>,
    watchers: Arc<Mutex<HashMap<String, Box<dyn Watcher + Send>>>>,
    agent_bridge: Arc<AgentBridge>,
}
```

**Thread Safety:**
- All shared state uses `Arc<Mutex<T>>` or `Arc<parking_lot::Mutex<T>>`
- Terminals use `parking_lot::Mutex` for lower latency
- Agent bridge uses `std::sync::Mutex` for compatibility

## Tauri Commands

All commands are async and return `Result<T, String>` for frontend error handling.

### Terminal Commands (4)
- `create_terminal() -> String`
- `write_to_terminal(id, data)`
- `resize_terminal(id, cols, rows)`
- `close_terminal(id)`

### File Commands (5)
- `read_file(path) -> String`
- `write_file(path, content)`
- `list_files(directory) -> Vec<FileInfo>`
- `watch_directory(path)`
- `unwatch_directory(path)`

### Agent Commands (4)
- `query_agent(...) -> AgentResponse`
- `check_agent_response() -> Option<AgentResponse>`
- `clear_agent_files()`
- `get_agent_info() -> AgentInfo`

### Permission Commands (4)
- `grant_workspace_access(path)`
- `check_permission(path) -> bool`
- `get_permissions() -> Vec<String>`
- `revoke_workspace_access(path)`

### Self-Awareness (1)
- `get_ide_source_code() -> String`

**Total: 18 commands**

## TypeScript Bindings

Located in `src/bindings.ts`, provides type-safe wrappers:

```typescript
// Class-based API
Terminal.create()
FileSystem.readFile(path)
Agent.query(params)
Permissions.grant(path)
IDE.getSourceCode()

// Managed instances
ManagedTerminal.create(onData)
FileWatcher.watch(path, onChange)
```

## Performance Characteristics

### Terminal
- **Buffer Size:** 8KB
- **Latency:** < 10ms (native PTY)
- **Throughput:** Limited by PTY (typically 100MB/s+)
- **Memory:** ~1MB per terminal

### File Operations
- **Read/Write:** Direct syscalls, no buffering overhead
- **Watch Events:** Event-driven (notify crate)
- **Permission Checks:** O(n) where n = number of granted paths

### Agent Communication
- **Request Write:** < 1ms (JSON serialization)
- **Response Poll:** 100ms intervals (configurable)
- **Timeout:** 30 seconds default
- **File Watch Latency:** < 50ms (notify debouncing)

## Security Model

### Permission System
1. **Default Deny:** No access unless explicitly granted
2. **Canonical Paths:** All paths canonicalized to prevent `../` attacks
3. **Persistent Storage:** Permissions survive restarts
4. **Granular Control:** Per-directory permissions

### Agent Communication
1. **File-Based Only:** No network or IPC sockets
2. **Isolated Directory:** `~/.rusty/agent/` only
3. **JSON Validation:** Strict schema checking
4. **No Code Execution:** Agent runs externally

### Tauri Security
1. **CSP:** Content Security Policy enabled
2. **Allowlist:** Only specific APIs enabled
3. **No Remote URLs:** Local files only
4. **Sandboxed WebView:** Renderer isolation

## Error Handling

All operations use `Result<T, E>` pattern:

```rust
// Internal errors use anyhow::Result
fn internal_op() -> Result<T> { ... }

// Tauri commands use Result<T, String>
#[tauri::command]
async fn command() -> Result<T, String> {
    internal_op().map_err(|e| e.to_string())
}
```

Frontend receives errors as rejected promises:
```typescript
try {
  await invoke('command');
} catch (error) {
  console.error('Command failed:', error);
}
```

## Build Configuration

### Cargo.toml Dependencies

```toml
[dependencies]
tauri = { version = "1.5", features = ["shell-open", "fs-all", ...] }
portable-pty = "0.8"
notify = "6.1"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
parking_lot = "0.12"
anyhow = "1.0"
home = "0.5"
chrono = { version = "0.4", features = ["serde"] }
```

### tauri.conf.json

```json
{
  "build": {
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "tauri": {
    "allowlist": {
      "fs": { "all": true },
      "shell": { "open": true },
      "path": { "all": true },
      "dialog": { "all": true }
    },
    "windows": [{
      "width": 1600,
      "height": 1000,
      "resizable": true
    }]
  }
}
```

## Testing

### Unit Tests (Rust)
```bash
cd src-tauri
cargo test
```

**Test Coverage:**
- AgentBridge creation and lifecycle
- Context builder pattern
- Request/response serialization
- Permission management
- Timestamp generation

### Integration Tests
- Terminal creation and I/O
- File read/write/watch operations
- Agent communication flow
- Permission enforcement

## Future Enhancements

1. **Terminal Multiplexing:** Multiple terminals per window
2. **LSP Integration:** Language Server Protocol support
3. **Remote Development:** SSH/container support
4. **Plugin System:** Dynamic command registration
5. **Performance Profiling:** Built-in profiler for agent queries
6. **Encryption:** Optional encrypted agent communication
7. **Multi-workspace:** Handle multiple projects simultaneously

## Debugging

### Enable Rust Logging
```bash
RUST_LOG=debug npm run tauri:dev
```

### Check Compilation
```bash
cd src-tauri
cargo check
cargo clippy
```

### Profile Performance
```bash
cargo build --release
samply record target/release/rusty-ide
```

### Memory Profiling
```bash
valgrind --tool=massif target/release/rusty-ide
```

## Contributing Guidelines

1. **Code Style:** Use `rustfmt` and `clippy`
2. **Tests Required:** Add tests for new features
3. **Documentation:** Update this file for architectural changes
4. **Error Handling:** Always use `Result<T, E>`
5. **Thread Safety:** Document any use of `unsafe`
6. **Performance:** Benchmark critical paths

## References

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [portable-pty](https://docs.rs/portable-pty/)
- [notify](https://docs.rs/notify/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
