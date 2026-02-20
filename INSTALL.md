# Installation Guide

## Quick Install

Install the agent to your PATH:

```bash
./install.sh
```

This will:
- Build the release binary
- Install it to `~/.local/bin/agent`
- Set up data directory at `~/.agent/data/`
- Copy existing knowledge base if available

## Usage

Once installed, you can run the agent from anywhere:

```bash
# Learning mode - continuously learns from CLIProxyAPI
agent

# Interactive mode - use learned knowledge as programming assistant
agent --interactive
```

## Data Location

All agent data is stored in `~/.agent/data/`:
- `knowledge_base.json` - The agent's learned knowledge
- `questions.txt` - Generated questions
- `answers.txt` - Generated answers

This means your agent's brain persists across updates and is accessible from anywhere on your system.

## Updating

When new features are added or the agent is improved:

```bash
./update.sh
```

This will:
- Rebuild the agent with latest code
- Reinstall to `~/.local/bin/agent`
- Preserve your existing knowledge base

Your agent's brain (`~/.agent/data/knowledge_base.json`) is never deleted during updates!

## PATH Setup

If `~/.local/bin` is not in your PATH, add this to your `~/.bashrc` or `~/.zshrc`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then reload your shell:

```bash
source ~/.bashrc  # or source ~/.zshrc
```

## Uninstall

To remove the agent:

```bash
rm ~/.local/bin/agent
rm -rf ~/.agent
```

## Requirements

- Rust toolchain (for building)
- CLIProxyAPI running on localhost:8317
- Claude Max subscription (for learning mode)
