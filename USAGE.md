# Usage Guide - Dual Mode System

Your agent has **TWO MODES**:

## 🎓 Mode 1: Learning Mode (Default)

**What it does:** Continuously learns programming knowledge from Claude Max

**How to run:**
```bash
CARGO_HOME=../.cargo cargo run
```

**What happens:**
1. Question Agent generates programming questions via CLIProxyAPI
2. Answer Agent provides detailed answers via CLIProxyAPI
3. YOUR Learning Agent reads both and stores knowledge
4. Runs until you press Ctrl+C
5. Saves everything to `data/knowledge_base.json`

**When to use:** Run this first to train your agent! Let it learn for 10-30 minutes.

---

## 💬 Mode 2: Interactive Mode

**What it does:** An interactive programming assistant that uses YOUR learned knowledge!

**How to run:**
```bash
CARGO_HOME=../.cargo cargo run -- --interactive
```

**What you can do:**

### Ask Questions:
```
> How do I create a git branch?
> Write a bash script to backup files
> Explain docker containers
> How do I use curl to POST data?
```

### Use Commands:
```
> /help     - Show available commands
> /stats    - Show knowledge statistics
> /search git - Search for "git" in knowledge base
> /quit     - Exit
```

**When to use:** After learning mode has run and built knowledge!

---

## Complete Workflow Example

### Step 1: Setup CLIProxyAPI (One Time)

```bash
# In a separate terminal
git clone https://github.com/anthropics/cliproxyapi.git
cd cliproxyapi
npm install
npm start
```

Leave this running on `localhost:8000`

### Step 2: Train Your Agent (Learning Mode)

```bash
cd /workspace/jashan/rust_agent

# Build
CARGO_HOME=../.cargo cargo build

# Run learning mode for 10-30 minutes
CARGO_HOME=../.cargo cargo run
```

Output:
```
╔══════════════════════════════════════════════════════════════╗
║              LEARNING MODE                                   ║
║   Self-Learning Agent with CLIProxyAPI (Claude Max)         ║
╚══════════════════════════════════════════════════════════════╝

🤖 Question Agent → CLIProxyAPI → questions.txt
🤖 Answer Agent   → CLIProxyAPI → answers.txt
🧠 Learning Agent → reads files → knowledge_base.json

❓ Q1: How do you list files in Linux?
✅ A1: Generated
📚 Learned from Q&A #1: Linux (Total knowledge: 3 items)
❓ Q2: What is git pull?
✅ A2: Generated
📚 Learned from Q&A #2: Git (Total knowledge: 6 items)
...
```

Press **Ctrl+C** when done learning:
```
✨ Learning session complete!
📊 Knowledge saved to: data/knowledge_base.json

💡 TIP: Run in interactive mode to use your learned knowledge:
   CARGO_HOME=../.cargo cargo run -- --interactive
```

### Step 3: Use Your Agent (Interactive Mode)

```bash
CARGO_HOME=../.cargo cargo run -- --interactive
```

Output:
```
╔══════════════════════════════════════════════════════════════╗
║          Interactive Programming Assistant                  ║
║          Powered by YOUR Learned Knowledge                  ║
╚══════════════════════════════════════════════════════════════╝

✅ Loaded knowledge base with 359 Q&A pairs

🧠 Knowledge Loaded:
   - 359 Q&A pairs
   - 368 patterns
   - Topics: ["Linux", "Git", "Bash", "Docker", "Networking"]

I can help you with:
  • Writing code and scripts
  • Explaining commands (Linux, Git, Docker, etc.)
  • Building projects
  • Solving programming problems

Commands:
  /help    - Show this help
  /stats   - Show knowledge statistics
  /search  - Search knowledge base
  /quit    - Exit

Type your question or command:
════════════════════════════════════════════════════════════════

> How do I create a git branch?

🤔 Processing your request...

💡 Answer:

To create a new Git branch and switch to it, use:

git checkout -b branch-name

Or create without switching:

git branch branch-name

[CODE_EXAMPLE_1]
# Create and switch to new branch
git checkout -b feature-login

# Or in newer Git (2.23+)
git switch -c feature-login

# Create branch without switching
git branch feature-login

# Switch to existing branch
git checkout feature-login
[/CODE_EXAMPLE]

> /stats

📊 Knowledge Base Statistics:
  Total Q&A Pairs: 359
  Total Patterns: 368
  Topics Covered: 6

  📚 Topics:
    - Linux: 52 items
    - Git: 61 items
    - Bash: 48 items
    - Docker: 54 items
    - Networking: 49 items
    - GitHub_CLI: 45 items

  Last Updated: 2026-02-19T16:50:23.123Z

> /quit
👋 Goodbye!
```

---

## Tips

### Learning Mode:
- Run for at least 10 minutes to build good knowledge
- The longer it runs, the more it learns!
- Can run overnight to build comprehensive knowledge
- Check `data/knowledge_base.json` size to track progress

### Interactive Mode:
- Loads your learned knowledge on startup
- Searches knowledge base first before asking Claude
- Uses CLIProxyAPI for new questions
- Great for building projects with learned patterns

### Knowledge Base:
- Location: `data/knowledge_base.json`
- Also includes: `questions.txt`, `answers.txt`
- Can delete and retrain anytime
- Grows with each learning session

---

## Quick Commands

```bash
# Learning mode (train the agent)
CARGO_HOME=../.cargo cargo run

# Interactive mode (use the agent)
CARGO_HOME=../.cargo cargo run -- --interactive

# Check knowledge
cat data/knowledge_base.json | jq '.qa_pairs | length'
cat data/knowledge_base.json | jq '.topics_covered | unique'
```

---

## Troubleshooting

### "CLIProxyAPI not running"
Start CLIProxyAPI first:
```bash
cd cliproxyapi
npm start
```

### "No knowledge base found"
Run learning mode first to train the agent!

### Interactive mode not working
Make sure CLIProxyAPI is running for interactive responses.

---

**Your agent learns once, helps forever!** 🎓✨
