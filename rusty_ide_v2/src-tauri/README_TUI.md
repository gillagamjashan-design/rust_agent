# Rusty TUI - Terminal-based Rust IDE with AI Agent

A VS Code-style terminal user interface for Rust development with integrated AI agent assistance.

## Features

- **VS Code-style Layout**: File tree, editor, AI agent panel, and integrated terminal
- **AI Agent Integration**: Chat with Claude directly from the IDE using the same ClaudeProxy as the main rust_agent
- **Terminal Integration**: Real PTY-based terminal with shell interaction
- **File Management**: Browse, open, and edit files with syntax highlighting
- **Modal Editing**: Vim-inspired Normal/Insert/Command modes
- **Smart Input Handling**: Dedicated input fields for agent queries and terminal commands with visible cursors

## Installation

The TUI is installed via two binaries:

```bash
# Build from source
cd /workspace/jashan/rust_agent/rusty_ide_v2/src-tauri
CARGO_HOME=/workspace/jashan/.cargo cargo build --release

# Install
cp target/release/rusty-tui ~/.local/bin/rusty-tui
cp ../rusty_launcher.sh ~/.local/bin/rusty
chmod +x ~/.local/bin/rusty*
```

Ensure `~/.local/bin` is in your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

### Launch

```bash
# Launch in new window (recommended)
rusty

# Or run directly in current terminal
rusty-tui
```

### Layout Overview

```
┌────────────────────────────────────────────────────────────┐
│ Title Bar - Shows mode and focused panel                  │
├───────────┬─────────────────────────┬──────────────────────┤
│   File    │                         │   AI Agent           │
│   Tree    │       Editor            │   Chat History       │
│  (20%)    │       (50%)             │   (30%)              │
│           │                         ├──────────────────────┤
│           │                         │ > input_█            │
├───────────┴─────────────────────────┴──────────────────────┤
│                Terminal Output                             │
├────────────────────────────────────────────────────────────┤
│ $ command_█                                                │
├────────────────────────────────────────────────────────────┤
│ Status message and context-aware help                     │
└────────────────────────────────────────────────────────────┘
```

### Modes

**Normal Mode** (default)
- Navigate between panels
- Browse file tree
- View content
- Enter other modes

**Insert Mode**
- Edit files in editor
- Type agent queries
- Enter terminal commands

**Command Mode**
- Execute Vim-style commands
- Save files, quit, change directory

### Key Bindings

#### Normal Mode

**Panel Navigation:**
- `h` - Focus File Tree
- `l` - Focus Editor
- `a` - Focus Agent Panel
- `t` - Focus Terminal Panel

**File Tree:**
- `j` - Move down
- `k` - Move up
- `Enter` - Open file/directory

**General:**
- `i` - Enter Insert mode for current panel
- `:` - Enter Command mode
- `q` - Quit application
- `Ctrl+s` - Save current file

#### Insert Mode

**Common:**
- `Esc` - Return to Normal mode
- Type to input text
- `Backspace` - Delete character

**Panel-Specific:**
- `Enter` in Agent panel - Send query to AI agent
- `Enter` in Terminal panel - Execute command
- `Enter` in Editor - Insert newline

#### Command Mode

- `:w` or `:write` - Save file
- `:q` or `:quit` - Quit
- `:wq` - Save and quit
- `:e <file>` - Open file
- `:cd <dir>` - Change directory
- `Esc` - Cancel command

## Panels

### File Tree Panel

- Shows files and directories in current working directory
- Navigate with `j`/`k` keys
- Press `Enter` to open files or enter directories
- Selected item is highlighted
- Border changes color when focused

### Editor Panel

- Displays file content with Rust syntax highlighting
- Enter Insert mode (`i`) to edit
- Supports scrolling for large files
- Shows current file path in title

### AI Agent Panel

**Chat History** (top section):
- Displays conversation with AI agent
- User messages prefixed with "You:" (green)
- Agent responses prefixed with "Agent:" (white)
- Shows agent status when no history

**Input Box** (bottom section):
- Enter Insert mode to type query
- Press `Enter` to send
- Cursor (█) visible when in Insert mode
- Async communication with Claude

### Terminal Panel

**Output** (top section):
- Shows all terminal output
- Scrollable with command history
- Maintains last 1000 lines

**Input Line** (bottom section):
- Shell command input with "$ " prefix
- Press `Enter` to execute
- Cursor (█) visible when in Insert mode
- Real PTY integration

## Architecture

### Components

**App State** (`src/app.rs`):
- Manages all panel states
- Handles keyboard input
- Coordinates between components

**UI Rendering** (`src/ui.rs`):
- Ratatui-based rendering
- Tokyo Night color scheme
- Responsive layout

**Agent Manager** (`src/agent_manager.rs`):
- Direct ClaudeProxy integration
- Knowledge base access
- TUI source code awareness

**Terminal Manager** (`src/terminal_manager.rs`):
- PTY-based terminal emulation
- Shell process management
- Non-blocking I/O

**File Manager** (`src/file_manager.rs`):
- File system operations
- Content reading/writing

### Data Flow

1. User input → Event loop (main.rs)
2. Key event → App.handle_key() (app.rs)
3. State update → UI render() (ui.rs)
4. Frame rendered → Terminal display

Agent queries:
1. User types in agent input field
2. Press Enter → send_to_agent()
3. Query sent to AgentManager
4. Response added to chat_history
5. UI updates with new message

Terminal commands:
1. User types in terminal input field
2. Press Enter → execute_terminal_command()
3. Command sent to TerminalManager
4. Output appears in terminal panel

## Theme

Uses Tokyo Night color scheme:
- Background: #1a1b26
- Foreground: #c0caf5
- Selection: #33467c
- Border: #7dcfff
- Keyword: #bb9af7
- Comment: #565f89
- String: #9ece6a
- Number: #ff9e64

## Development

### Building

```bash
cd /workspace/jashan/rust_agent/rusty_ide_v2/src-tauri
CARGO_HOME=/workspace/jashan/.cargo cargo build --release
```

### Testing

```bash
# Run directly
CARGO_HOME=/workspace/jashan/.cargo cargo run --release

# Or after installation
rusty-tui
```

### File Structure

```
src-tauri/
├── src/
│   ├── main.rs           # Entry point, event loop
│   ├── app.rs            # App state and key handling
│   ├── ui.rs             # Rendering logic
│   ├── agent_manager.rs  # AI agent integration
│   ├── terminal_manager.rs # Terminal management
│   ├── file_manager.rs   # File operations
│   └── agent_bridge.rs   # (unused, legacy)
├── Cargo.toml
└── UPDATES_SUMMARY.md    # Recent changes

../
├── rusty_launcher.sh     # Window spawner script
└── verify_install.sh     # Installation checker
```

## Troubleshooting

**Terminal emulator not found:**
- Install one of: gnome-terminal, xterm, konsole, alacritty, kitty, terminator
- Or run `rusty-tui` directly in current terminal

**Agent not responding:**
- Check Claude API configuration
- Ensure rust_agent is properly configured
- Check `~/.agent/config.yaml`

**Terminal not working:**
- Check PTY support on your system
- Verify shell is accessible ($SHELL env var)

**File tree empty:**
- Ensure you're in a directory with files
- Use `:cd <dir>` to change directory
- Check file permissions

**Input not visible:**
- Press `i` to enter Insert mode
- Cursor (█) should appear
- Check focused panel (border color)

## Future Enhancements

Potential improvements:
- Multi-file editing with tabs/buffers
- Split panes for editor
- File search functionality
- Git integration
- More syntax highlighting languages
- Scrolling in agent chat history
- Terminal tab support
- Code completion
- Find/replace in editor

## License

Part of the rust_agent project.

## Credits

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [portable-pty](https://github.com/wez/wezterm/tree/main/pty) - PTY implementation
- [tokio](https://tokio.rs/) - Async runtime

Inspired by:
- VS Code layout
- Vim keybindings
- Tokyo Night theme
