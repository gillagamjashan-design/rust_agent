# Rusty TUI - Quick Start Guide

## 🚀 Getting Started in 60 Seconds

### 1. Build
```bash
cd /workspace/jashan/rust_agent/rusty_ide_v2/src-tauri
cargo build --release
```

### 2. Run
```bash
./target/release/rusty-tui
```

### 3. Navigate
- Press `h` to focus **File Tree**
- Press `j`/`k` to navigate files
- Press `Enter` to open a file
- Press `l` to focus **Editor**
- Press `i` to enter **Insert mode**
- Type some code
- Press `Esc` to return to **Normal mode**
- Press `Ctrl+S` to save
- Press `q` to quit

## 🎯 Common Tasks

### Open a File
1. Press `h` (focus File Tree)
2. Navigate with `j`/`k`
3. Press `Enter`

Or use command mode:
1. Press `:`
2. Type `e src/main.rs`
3. Press `Enter`

### Save a File
- **Quick:** Press `Ctrl+S` in Editor
- **Command:** Press `:`, type `w`, press `Enter`

### Switch Panels
- `h` - File Tree
- `l` - Editor
- `a` - Agent
- `t` - Terminal

### Quit
- **Normal mode:** Press `q`
- **Command mode:** Press `:`, type `q`, press `Enter`

## 🎨 UI Layout

```
┌────────────┬─────────────────┬────────────┐
│ File Tree  │     Editor      │   Agent    │
│            │                 │            │
│ Files      │ Code goes here  │ AI Panel   │
│ j/k move   │ i = insert      │            │
│            │ Esc = normal    │            │
├────────────┴─────────────────┴────────────┤
│            Terminal Output                │
└───────────────────────────────────────────┘
│              Status Bar                   │
└───────────────────────────────────────────┘
```

## 📋 Essential Keybindings

| Key | Action |
|-----|--------|
| `h` | Focus File Tree |
| `l` | Focus Editor |
| `a` | Focus Agent |
| `t` | Focus Terminal |
| `j` | Move down (File Tree) |
| `k` | Move up (File Tree) |
| `Enter` | Open file/directory |
| `i` | Insert mode (Editor) |
| `Esc` | Normal mode |
| `:` | Command mode |
| `q` | Quit |
| `Ctrl+S` | Save |

## 💡 Pro Tips

### Vim Users
The navigation feels familiar:
- `hjkl` for movement (h=left, j=down, k=up, l=right)
- `i` for insert mode
- `Esc` for normal mode
- `:` for commands

### Command Mode
```
:w          Save file
:q          Quit
:wq         Save and quit
:e file     Open file
:cd dir     Change directory
```

### Permissions
Grant access to a workspace:
```bash
# Edit ~/.rusty/permissions.json
["/path/to/your/project"]
```

### AI Agent
The agent watches `~/.rusty/agent/request.json` for queries.
Responses appear in `~/.rusty/agent/response.json`.

## 🐛 Troubleshooting

### Build Fails
```bash
# Use local cargo home
CARGO_HOME=~/.cargo cargo build
```

### Terminal Broken
```bash
# Reset terminal
reset
# Or
export TERM=xterm-256color
```

### Can't See Colors
```bash
# Check your TERM
echo $TERM  # should be xterm-256color or similar

# Set it if needed
export TERM=xterm-256color
```

## 📚 More Information

- **Full Guide:** `TUI_README.md`
- **Conversion Details:** `CONVERSION_GUIDE.md`
- **Architecture:** `ARCHITECTURE_DIAGRAM.txt`
- **Metrics:** `COMPARISON.md`

## 🎉 You're Ready!

Start coding in your new terminal-based IDE!

```bash
cargo run
```

Press `h` to start browsing files. Happy coding! 🚀
