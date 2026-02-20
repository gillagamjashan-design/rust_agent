# Self-Learning Rust Agent

A self-learning agent system that uses **CLIProxyAPI** to convert your Claude Max subscription into teacher agents, while YOUR local Rust agent learns by reading files.

## Overview

This system uses **CLIProxyAPI** - a local proxy that turns your Claude Max subscription into an API endpoint, allowing you to use it without additional API costs!

### Components

1. **Question Agent** 🤔 - Uses CLIProxyAPI (Claude Max) to generate programming questions
2. **Answer Agent** 💡 - Uses CLIProxyAPI (Claude Max) to provide detailed answers
3. **Learning Agent** 🧠 - YOUR local Rust agent that learns by reading files (NO API!)

## Why CLIProxyAPI?

- ✅ **Use your existing Claude Max subscription** - No additional API costs
- ✅ **Local proxy server** - Fast, secure, OAuth authenticated
- ✅ **OpenAI-compatible API** - Works like standard APIs
- ✅ **Simple setup** - Just start the proxy and run

## Quick Start

### 1. Install and Start CLIProxyAPI

```bash
# Clone CLIProxyAPI
git clone https://github.com/anthropics/cliproxyapi.git
cd cliproxyapi

# Install and start
npm install
npm start
```

This starts the proxy on `http://localhost:8000`

### 2. Run the Learning Agent

```bash
cd /workspace/jashan/rust_agent

# Build
CARGO_HOME=../.cargo cargo build

# Run
CARGO_HOME=../.cargo cargo run
```

## How It Works

```
Question Agent → CLIProxyAPI (localhost:8000) → Claude Max → questions.txt
                                                                    ↓
Answer Agent → CLIProxyAPI (localhost:8000) → Claude Max → answers.txt
                                                                    ↓
YOUR Learning Agent → Reads both files → Stores in knowledge_base.json
```

### The Learning Cycle

1. **Question Agent** calls CLIProxyAPI to generate programming questions
2. **Answer Agent** calls CLIProxyAPI to generate detailed answers with code
3. **YOUR Learning Agent** monitors both files and learns WITHOUT any API calls
4. Knowledge stored in `data/knowledge_base.json` (YOUR agent's brain!)

## Topics Covered

Your agent learns about:
- **Linux** - Commands, permissions, processes, file operations
- **Git** - Version control, branches, merges, rebases
- **GitHub CLI** - Repo management, PRs, issues, releases
- **Bash** - Scripting, variables, loops, functions
- **Networking** - curl, wget, ssh, scp, rsync
- **Docker** - Containers, images, volumes, Dockerfile
- **System** - Monitoring, services, logs
- **Packages** - apt, npm, cargo, pip

## Setup Details

See [CLIPROXYAPI_SETUP.md](CLIPROXYAPI_SETUP.md) for detailed setup instructions.

## Output Files

The system creates:
- `data/questions.txt` - Generated questions
- `data/answers.txt` - Detailed answers with code examples
- `data/knowledge_base.json` - YOUR agent's learned knowledge

## Previous Versions

- **v1.0.0** - Architecture documentation
- **v2.0.0** - Rust Book learning system
- **v3.0.0** - Spawned Claude Code agents (deprecated)
- **v4.0.0** - CLIProxyAPI system (current) ⭐

## Benefits Over Previous Versions

**v3.0.0 (Spawned Agents):**
- ❌ Limited by Claude Code session limits
- ❌ Couldn't run long sessions
- ❌ Complex spawning logic

**v4.0.0 (CLIProxyAPI):**
- ✅ Use your Claude Max subscription directly
- ✅ No session limits
- ✅ Simple HTTP API calls
- ✅ Works like any API but uses Max

## Troubleshooting

### CLIProxyAPI Not Running

```
Error: CLIProxyAPI not running on localhost:8000
```

**Solution:** Start CLIProxyAPI first:
```bash
cd cliproxyapi
npm start
```

### Authentication Issues

1. Stop the proxy
2. Clear browser cookies for Claude
3. Restart and re-authenticate

## Architecture

Detailed architecture docs in `architecture/` directory:
- [System Overview](architecture/system-overview.md)
- [Components](architecture/components.md)
- [Data Flow](architecture/data-flow.md)

## Development Status

**Current:** v4.0.0 - CLIProxyAPI Integration

✅ **Working:**
- CLIProxyAPI integration
- Question generation via Claude Max
- Answer generation via Claude Max
- Local learning agent (YOUR agent)
- Knowledge storage without APIs

## License

MIT License

## Acknowledgments

Built with:
- Rust 🦀
- CLIProxyAPI (Anthropic)
- Claude Max subscription
- Claude Sonnet 4.5

---

**Your agent learns from Claude Max without API costs!** 🎓✨
