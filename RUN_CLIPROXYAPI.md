# How to Run CLIProxyAPI

CLIProxyAPI is required for the agent to learn. It acts as a proxy to Claude Max subscription.

## Prerequisites

- CLIProxyAPI installed at `/workspace/jashan/cliproxyapi`
- Claude Max subscription
- Port 8317 available

## Running CLIProxyAPI

### Option 1: Run in background (Recommended)

```bash
cd /workspace/jashan/cliproxyapi
nohup ./cli-proxy-api > cliproxyapi.log 2>&1 &
```

### Option 2: Run in foreground (for debugging)

```bash
cd /workspace/jashan/cliproxyapi
./cli-proxy-api
```

## Verify it's running

```bash
# Check if process is running
ps aux | grep cli-proxy-api | grep -v grep

# Check if API is responding
curl http://localhost:8317/health
```

## Stop CLIProxyAPI

```bash
pkill cli-proxy-api
```

## Configuration

The agent expects:
- **URL**: `http://localhost:8317`
- **Auth Header**: `Authorization: Bearer rust-agent-key-123`

## Once CLIProxyAPI is running

Start the learning agent:

```bash
# Run agent in learning mode
agent

# Or in background
nohup agent > /tmp/agent.log 2>&1 &
```

## What the agent will learn

The agent is now configured to learn **Rust programming ONLY**:

- Rust Basics
- Ownership and Borrowing
- Lifetimes
- Traits
- Error Handling
- Async/Await
- Macros
- Cargo
- Collections
- Pattern Matching
- Generics
- Testing
- Concurrency
- Smart Pointers
- Iterators
- Modules

**No other programming languages or topics will be learned.**
