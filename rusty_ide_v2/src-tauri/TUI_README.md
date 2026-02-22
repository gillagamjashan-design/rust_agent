# Rusty TUI - A Terminal-Based Rust IDE

A fast, lightweight, terminal-based IDE with AI agent integration, built with Ratatui.

## Features

- 📁 **File Browser** - Navigate and open files
- ✍️ **Code Editor** - Edit files with syntax highlighting
- 🤖 **AI Agent** - File-based agent communication
- 💻 **Integrated Terminal** - Built-in PTY terminal
- 🎨 **Tokyo Night Theme** - Beautiful color scheme
- ⌨️ **Vim-style Navigation** - Familiar keybindings

## Quick Start

```bash
# Build
cargo build --release

# Run
cargo run

# Or run binary directly
./target/release/rusty-tui
```

## Keybindings

### Panel Navigation
| Key | Action |
|-----|--------|
| `h` | Focus File Tree |
| `l` | Focus Editor |
| `a` | Focus Agent Panel |
| `t` | Focus Terminal |

### File Tree (when focused)
| Key | Action |
|-----|--------|
| `j` | Move cursor down |
| `k` | Move cursor up |
| `Enter` | Open file / Enter directory |

### Editor (when focused)
| Key | Action |
|-----|--------|
| `i` | Enter Insert mode |
| `Esc` | Return to Normal mode |
| `Ctrl+S` | Save current file |

### Command Mode
| Command | Action |
|---------|--------|
| `:q` | Quit |
| `:w` | Save file |
| `:wq` | Save and quit |
| `:e <path>` | Open file |
| `:cd <path>` | Change directory |

### Global
| Key | Action |
|-----|--------|
| `q` | Quit (in Normal mode) |
| `:` | Enter command mode |
| `?` | Help (coming soon) |

## Modes

### Normal Mode
Default mode for navigation and commands.

### Insert Mode
Text editing mode (available in Editor panel).

### Command Mode
Execute commands (like `:w`, `:q`, etc.).

## Layout

```
┌─────────────────────────────────────────────────────┐
│  Rusty TUI - Mode | Panel                          │
├──────────┬───────────────────────┬──────────────────┤
│          │                       │                  │
│   File   │       Editor          │   AI Agent       │
│   Tree   │                       │                  │
│          │                       │                  │
├──────────┴───────────────────────┴──────────────────┤
│              Terminal Output                        │
├─────────────────────────────────────────────────────┤
│  Status / Command Line                              │
└─────────────────────────────────────────────────────┘
```

## AI Agent Integration

The TUI integrates with an external AI agent via file-based communication:

**Agent Directory:** `~/.rusty/agent/`

**Communication:**
- `request.json` - IDE writes queries here
- `response.json` - Agent writes responses here

**Usage:**
1. Focus Agent panel (`a`)
2. Type your query
3. Press Enter to send
4. Wait for response

## Permissions

File access is controlled by a permission system:

**Permission File:** `~/.rusty/permissions.json`

By default, no restrictions apply. Grant access to specific directories:

```bash
# Programmatically via API
# (or edit ~/.rusty/permissions.json)
```

## Configuration

Coming soon! Currently uses sensible defaults.

## Development

### Project Structure

```
src/
├── main.rs              # Entry point, event loop
├── lib.rs               # Module exports
├── app.rs               # Application state
├── ui.rs                # Rendering logic
├── agent_bridge.rs      # Agent communication (unchanged)
├── agent_manager.rs     # Agent wrapper
├── file_manager.rs      # File operations
└── terminal_manager.rs  # PTY terminal
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check without building
cargo check
```

### Dependencies

**Core:**
- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `tokio` - Async runtime

**Features:**
- `portable-pty` - PTY terminal
- `notify` - File watching
- `syntect` - Syntax highlighting
- `serde` - Serialization

## Troubleshooting

### Terminal not rendering correctly
Try resizing your terminal or pressing Ctrl+L to refresh.

### Permissions error
Check `~/.rusty/permissions.json` and ensure paths are correct.

### Agent not responding
Ensure the agent process is running and watching `~/.rusty/agent/request.json`.

## Performance

- **Startup time:** < 100ms
- **Memory usage:** ~20MB
- **Binary size:** ~35MB (debug), ~5MB (release)

## License

MIT

## Credits

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui)
- [Crossterm](https://github.com/crossterm-rs/crossterm)
- [Tokio](https://tokio.rs/)
