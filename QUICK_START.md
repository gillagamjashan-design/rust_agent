# Quick Start - Rusty Agent

## One-Command Build and Run

```bash
cd /workspace/jashan/rust_agent
./run-all.sh
```

That's it! This script will:

1. ✅ Check Rust installation
2. ✅ Build core library (`rust_agent`)
3. ✅ Build GUI application (`rusty`)
4. ✅ Install to `~/.local/bin/rusty`
5. ✅ Verify knowledge files
6. ✅ Initialize database directory
7. ✅ Check ClaudeProxyAPI status
8. ✅ Run verification tests
9. ✅ Give you options to run

## What Happens When You Run It

```
🦀 Rusty Agent - Build & Setup Script
======================================

▶ Step 1: Checking Rust installation...
✓ Rust is installed: rustc 1.xx.x

▶ Step 2: Checking cargo...
✓ Cargo is installed: cargo 1.xx.x

▶ Step 3: Building core library (rust_agent)...
ℹ Building in release mode...
   Compiling rust_agent v0.1.0
✓ Core library built successfully

▶ Step 4: Building GUI application (rusty)...
ℹ This may take 4-5 minutes on first build...
   Compiling rusty v1.0.0
✓ GUI application built successfully
✓ Binary created: target/release/rusty (21M)

▶ Step 5: Installing rusty to PATH...
✓ Installed rusty to ~/.local/bin/rusty

▶ Step 6: Verifying knowledge files...
✓ Found 7 knowledge JSON files

▶ Step 7: Checking knowledge database...
✓ Database directory created: ~/.agent/data
ℹ Knowledge database will be initialized on first run

▶ Step 8: Checking ClaudeProxyAPI...
✓ ClaudeProxyAPI is running on localhost:8317

▶ Step 9: Running verification...
✓ All verification tests passed

======================================
🎉 Build & Installation Complete!
======================================

What would you like to do?
  1) Run rusty now
  2) Start ClaudeProxyAPI and run rusty
  3) Exit (run manually later)

Enter choice [1-3]:
```

## After Installation

### Run from any directory:

```bash
cd ~/my_rust_project
rusty
```

### Expected output:

```
🚀 Rusty GUI starting...
📂 Database path: "~/.agent/data/knowledge.db"
✅ Worker thread spawned
🎨 Initializing GUI...
📂 File workspace: "/home/user/my_rust_project"  ← Your current directory!
📚 13 concepts, 18 patterns loaded
```

### GUI window opens with:

- Welcome message showing workspace path
- Input box at bottom
- Chat area in center

### Try it out:

```
What is ownership?
Create a hello world program
Show me the builder pattern
```

When you ask for file creation:
```
📨 Received query: Create a hello world program
🤖 Querying Claude API...
✅ Got response (234 chars)
📝 Found 1 code blocks, creating files automatically...
🔨 Creating file: main.rs (45 bytes)
✅ Created file: main.rs
```

**File appears immediately in your current directory!** ✅
**No confirmation dialog needed!** ⚡

## Automatic File Creation (v12.0.2)

Files are created **automatically** without confirmation:

- ✅ Files created immediately when code is generated
- ✅ No dialog boxes or manual approval needed
- ✅ Files created in current directory (shown in terminal)
- ✅ Workspace shown in terminal: `📂 File workspace: "/your/dir"`
- ✅ File details shown: `🔨 Creating file: main.rs (45 bytes)`
- ✅ Clear notifications in GUI: "✅ Created file: main.rs"
- ✅ If file exists, automatically modifies it instead

## Troubleshooting

### If ClaudeProxyAPI is not running:

```bash
./start_cliproxyapi.sh
```

Or use the run-all.sh option 2.

### If rusty command not found:

```bash
# Use full path
~/.local/bin/rusty

# Or add to PATH
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### To rebuild from scratch:

```bash
cargo clean
cd rusty_tui && cargo clean && cd ..
./run-all.sh
```

## Manual Build (if needed)

If you prefer to build manually:

```bash
# Build core library
cargo build --release

# Build GUI
cd rusty_tui
CARGO_HOME=/tmp/.cargo cargo build --release
cd ..

# Run
./rusty_tui/target/release/rusty
```

## What Gets Installed

- **Binary**: `~/.local/bin/rusty` (21MB)
- **Database**: `~/.agent/data/knowledge.db` (created on first run)
- **Cache**: `~/.agent/cache/` (web search cache)

## Commands

Once rusty is running:

- `/help` - Show available commands
- `/search <query>` - Search knowledge base
- `/stats` - Show database statistics
- `/clear` - Clear chat history
- `/quit` - Exit

## That's It!

Just run `./run-all.sh` and you're ready to go! 🦀🚀
