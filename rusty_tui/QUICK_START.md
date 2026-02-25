# Rusty GUI - Quick Start Guide

## 🚀 Run the Application

```bash
# From rusty_tui directory:
./target/release/rusty

# Or if not built yet:
CARGO_HOME=/tmp/cargo-home cargo build --release
./target/release/rusty
```

## 💬 Using the Chat Interface

1. **Type your message** in the input box at the bottom
2. **Press Enter** to send (the Enter key works properly!)
3. **Wait for response** - A spinner shows while the agent is thinking
4. **Scroll through history** - Previous messages are saved and scrollable

## ⌨️ Available Commands

Type these in the chat:

- `/help` - Show all available commands
- `/search <query>` - Search the knowledge database
- `/stats` - Show database statistics
- `/clear` - Clear chat history
- `/quit` - Show goodbye message (close window to exit)

## 🎨 Interface Features

- **Tokyo Night Theme** - Beautiful dark color scheme
- **Color-coded messages:**
  - User (you): Cyan
  - Agent: Green
  - System: Yellow
  - Errors: Red
- **Auto-scroll** - Automatically scrolls to latest message
- **Loading indicator** - Spinner shows during API calls
- **Timestamps** - Each message shows when it was sent

## 🔧 Troubleshooting

**Window doesn't open:**
- Check that you're running the correct binary
- Make sure you have a display (GUI environment)

**Agent doesn't respond:**
- Check that Claude API is configured properly
- Verify knowledge database exists at `~/.agent/data/knowledge.db`

**Enter key doesn't work:**
- This should be fixed in v12.0.0
- Make sure you're running the latest build
- Try clicking the "Send" button instead

## 📝 Tips

- Ask questions naturally: "What is ownership in Rust?"
- Request code: "Write a TCP server"
- Search knowledge: "/search lifetimes"
- Get help anytime: "/help"

Enjoy learning Rust! 🦀
