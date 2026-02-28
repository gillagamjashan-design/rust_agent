# Testing Guide for Rusty GUI v12.0.0

## Quick Test

```bash
cd /workspace/jashan/rust_agent
./rusty_tui/target/release/rusty
```

---

## Expected Terminal Output

When you run the application, you should see:

```
🦀 Rusty - Rust Learning Agent v12.0.0
=====================================
✅ Knowledge database found
🚀 Starting GUI...

🚀 Rusty GUI starting...
📂 Database path: "/home/jashan/.agent/data/knowledge.db"
✅ Worker thread spawned
🎨 Initializing GUI...
```

---

## Expected GUI Window

### Window Properties
- **Title**: "Rusty 🦀 - Rust Learning Agent"
- **Size**: 900x700 pixels
- **Minimum size**: 600x400 pixels

### Layout Sections

#### 1. Header (Top)
```
🦀 Rusty    Rust Learning Agent                    13 concepts, 18 patterns loaded
─────────────────────────────────────────────────────────────────────────────────
```

#### 2. Chat Area (Middle - SHOULD BE VISIBLE)
```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│ [23:30:45] System:                                                      │
│ 🦀 Welcome to Rusty - Your Rust Learning Agent!                        │
│                                                                         │
│ I'm here to help you learn Rust programming. I have instant access to: │
│ • Rust core concepts (ownership, lifetimes, traits)                    │
│ • Design patterns and idioms                                           │
│ • Cargo commands and toolchain usage                                   │
│ • Async/concurrency examples                                           │
│                                                                         │
│ 💬 Ask me anything! For example:                                        │
│ • "What is ownership?"                                                  │
│ • "How do I use cargo test?"                                            │
│ • "Show me the builder pattern"                                         │
│                                                                         │
│ 📋 Type /help to see available commands                                 │
│ 🔍 Type /search <topic> to search the knowledge base                    │
│                                                                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

#### 3. Input Area (Bottom)
```
─────────────────────────────────────────────────────────────────────────────────
[Type your message or /help for commands...              ] [ Send ]
```

---

## Manual Tests

### Test 1: Welcome Message Visibility ✅
**Steps**:
1. Run the application
2. Look at the GUI window

**Expected**:
- Large chat area with welcome message fully visible
- Message is not cut off
- All bullet points are readable

**Status**: ✅ FIXED - Chat area now has proper height allocation

---

### Test 2: Type and Send Message
**Steps**:
1. Click in the input field
2. Type: "What is ownership?"
3. Press Enter

**Expected**:
- Your message appears in chat area
- Input field clears
- Spinner shows "Agent is thinking..."
- Agent response appears (if API is running)

**Terminal output**:
```
📨 Received query: What is ownership?
📚 Found 3 knowledge results
🤖 Querying Claude API...
✅ Got response (450 chars)
```

---

### Test 3: Help Command
**Steps**:
1. Type: `/help`
2. Press Enter

**Expected**:
- Command list appears showing:
  - /help - Show help message
  - /search <query> - Search knowledge database
  - /stats - Show database statistics
  - /web <query> - Force web search
  - /clear - Clear chat history
  - /quit - Exit application

---

### Test 4: Search Command
**Steps**:
1. Type: `/search ownership`
2. Press Enter

**Expected**:
- Search results appear showing:
  - 📚 Concepts: Ownership System
  - 🔧 Patterns: (related patterns)
  - Related topics

---

### Test 5: Error Handling (API Down)
**Steps**:
1. Make sure ClaudeProxyAPI is NOT running
2. Type: "What is Rust?"
3. Press Enter

**Expected**:
- Friendly error message:
  ```
  Sorry, I couldn't connect to the AI service.

  Error: Connection refused

  💡 Make sure ClaudeProxyAPI is running:
  • Check: curl http://localhost:8317/
  • Start: ./start_cliproxyapi.sh

  🔍 You can still search the knowledge base with /search <query>
  ```

**Terminal output**:
```
📨 Received query: What is Rust?
ℹ️  No knowledge results, using Claude only
🤖 Querying Claude API...
❌ Claude API error: Connection refused
```

---

### Test 6: Stats Command
**Steps**:
1. Type: `/stats`
2. Press Enter

**Expected**:
```
Knowledge Database Statistics:

📚 Concepts: 13
🔧 Patterns: 18
⚙️  Commands: 22

Database location: ~/.agent/data/knowledge.db
Query performance: <50ms average
```

---

### Test 7: Clear Command
**Steps**:
1. Have some chat history
2. Type: `/clear`
3. Press Enter

**Expected**:
- All chat history is cleared
- Only "Chat history cleared." message remains

---

### Test 8: Multiple Messages
**Steps**:
1. Ask several questions in a row
2. Observe chat scrolling

**Expected**:
- Messages stack vertically with 15px spacing
- Chat area scrolls automatically to show latest
- Scroll to top to see history

---

## Verification Checklist

Before considering the fix complete, verify:

- [ ] Binary exists at `rusty_tui/target/release/rusty`
- [ ] Binary is 20 MB in size
- [ ] GUI window opens without errors
- [ ] **Chat area is visible (not collapsed to 0 height)**
- [ ] **Welcome message is fully visible**
- [ ] Header shows knowledge stats
- [ ] Input field is responsive
- [ ] Enter key sends messages
- [ ] /help command works
- [ ] /search command works
- [ ] Error messages are helpful
- [ ] Terminal shows debug logging
- [ ] Chat scrolls with many messages

---

## Troubleshooting

### GUI doesn't open
```bash
# Check if DISPLAY is set
echo $DISPLAY

# Try with explicit display
DISPLAY=:0 ./rusty_tui/target/release/rusty
```

### Chat area is still empty
```bash
# Check if welcome message is created
RUST_LOG=debug ./rusty_tui/target/release/rusty 2>&1 | grep -i welcome

# Should show: Message created with content starting with "🦀 Welcome"
```

### Knowledge database missing
```bash
# Check if it exists
ls -lh ~/.agent/data/knowledge.db

# If missing, it will be created on first run
# Should take 1-2 seconds
```

### API not responding
```bash
# Check if ClaudeProxyAPI is running
curl http://localhost:8317/health

# Start it if needed
./start_cliproxyapi.sh

# The GUI will still work for /search commands without API
```

---

## Success Criteria

The fix is successful if:

1. ✅ Chat area is visible (height > 200px)
2. ✅ Welcome message displays immediately
3. ✅ All welcome text is readable (not cut off)
4. ✅ Input field works and responds to Enter
5. ✅ Messages appear in chat when sent
6. ✅ Commands (/help, /search) work
7. ✅ Error messages are helpful
8. ✅ Chat scrolls properly

---

**All tests should pass!** The critical fix ensures the chat area has proper height allocation, making it visible from startup.
