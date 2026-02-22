# Rusty IDE Agent Bridge - Quick Start Guide

## Overview

This guide will help you get the Agent Bridge up and running in minutes.

## Prerequisites

1. Rust toolchain (1.70+)
2. The external agent (`rust_agent`) installed and in PATH
3. Basic dependencies:
   ```bash
   sudo apt-get install jq inotify-tools
   ```

## Installation

### 1. Add to Your Tauri Project

The agent bridge is already integrated in the `src-tauri` directory. To use it in a new project:

```bash
# Copy the agent bridge module
cp src-tauri/src/agent_bridge.rs your-project/src-tauri/src/

# Copy the wrapper script
cp src-tauri/src/agent_wrapper.sh your-project/src-tauri/src/
chmod +x your-project/src-tauri/src/agent_wrapper.sh
```

### 2. Add Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
notify = "6.1"
dirs = "5.0"
```

### 3. Build

```bash
cd src-tauri
cargo build
```

## Quick Test

### Terminal 1: Start the Agent Wrapper

```bash
cd src-tauri/src
./agent_wrapper.sh --daemon
```

You should see:
```
[2026-02-21 10:30:00] [INFO] === Rusty IDE Agent Wrapper ===
[2026-02-21 10:30:00] [INFO] Starting agent wrapper in daemon mode
[2026-02-21 10:30:00] [INFO] Monitoring directory: /home/user/.rusty/agent
```

### Terminal 2: Send a Test Request

```bash
# Create a test request
cat > ~/.rusty/agent/request.json << 'EOF'
{
  "timestamp": "2026-02-21T10:30:00.000Z",
  "workspace_path": "/tmp/test",
  "current_file": null,
  "current_code": null,
  "files": [],
  "ide_source": "",
  "query": "What is Rust?"
}
