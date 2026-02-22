#!/bin/bash
# Rusty IDE Launcher - Spawns new terminal window

RUSTY_TUI="$HOME/.local/bin/rusty-tui"

# Check if rusty-tui is installed
if [ ! -f "$RUSTY_TUI" ]; then
    echo "Error: rusty-tui not found at $RUSTY_TUI"
    echo "Please install it first: cd /workspace/jashan/rust_agent/rusty_ide_v2/src-tauri && cargo build --release && cp target/release/rusty-tui ~/.local/bin/"
    exit 1
fi

# Parse arguments - default to current directory if no arg
WORK_DIR="${1:-.}"

# Resolve to absolute path
WORK_DIR=$(cd "$WORK_DIR" 2>/dev/null && pwd)
if [ $? -ne 0 ]; then
    echo "Error: Cannot access directory: ${1:-.}"
    exit 1
fi

# Detect terminal and spawn new window with working directory in title
TITLE="Rusty IDE - $WORK_DIR"

if command -v gnome-terminal &> /dev/null; then
    gnome-terminal --title="$TITLE" --maximize -- "$RUSTY_TUI" "$WORK_DIR"
elif command -v xterm &> /dev/null; then
    xterm -title "$TITLE" -maximized -e "$RUSTY_TUI" "$WORK_DIR" &
elif command -v konsole &> /dev/null; then
    konsole --title "$TITLE" -e "$RUSTY_TUI" "$WORK_DIR" &
elif command -v alacritty &> /dev/null; then
    alacritty -t "$TITLE" -e "$RUSTY_TUI" "$WORK_DIR" &
elif command -v kitty &> /dev/null; then
    kitty --title "$TITLE" "$RUSTY_TUI" "$WORK_DIR" &
elif command -v terminator &> /dev/null; then
    terminator -T "$TITLE" -x "$RUSTY_TUI" "$WORK_DIR" &
else
    # Fallback: run in current terminal
    echo "No supported terminal emulator found. Running in current terminal..."
    "$RUSTY_TUI" "$WORK_DIR"
fi
