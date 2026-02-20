# CLIProxyAPI Setup Guide

This project uses **CLIProxyAPI** to convert your Claude Max subscription into a local API endpoint.

## What is CLIProxyAPI?

CLIProxyAPI is a local proxy server that:
- Authenticates with your Claude account using OAuth
- Exposes an API endpoint (default: `http://localhost:8000`)
- Allows your Max subscription to be used like an API
- No additional API costs - uses your existing Max subscription

## Setup Instructions

### 1. Install CLIProxyAPI

```bash
# Clone the CLIProxyAPI repository
git clone https://github.com/anthropics/cliproxyapi.git
cd cliproxyapi

# Install dependencies
npm install

# Or with yarn
yarn install
```

### 2. Start the Proxy Server

```bash
# Start CLIProxyAPI (runs on http://localhost:8000)
npm start

# Or
yarn start
```

The proxy will:
1. Open your browser for Claude authentication
2. Handle OAuth flow
3. Start serving requests on `localhost:8000`

### 3. Run the Self-Learning Agent

In a separate terminal:

```bash
cd /workspace/jashan/rust_agent

# Build the project
CARGO_HOME=../.cargo cargo build

# Run the system
CARGO_HOME=../.cargo cargo run
```

## How It Works

```
┌─────────────────┐
│ Question Agent  │──┐
└─────────────────┘  │
                     ├──> CLIProxyAPI ──> Claude Max ──> Your Account
┌─────────────────┐  │    (localhost:8000)
│ Answer Agent    │──┘
└─────────────────┘

┌─────────────────┐
│ Learning Agent  │──> Reads Q&A files ──> Stores Knowledge
└─────────────────┘
```

## System Flow

1. **Question Agent** → Calls CLIProxyAPI → Gets question from Claude Max
2. **Answer Agent** → Calls CLIProxyAPI → Gets answer from Claude Max
3. **Learning Agent** → Reads both files → Builds knowledge base

## Benefits

- ✅ Uses your existing Claude Max subscription
- ✅ No additional API costs
- ✅ Local proxy - fast and secure
- ✅ OAuth authentication
- ✅ Compatible with OpenAI API format

## Troubleshooting

### Proxy Not Running

If you get connection errors:

```
Error: CLIProxyAPI not running on localhost:8000
```

Make sure CLIProxyAPI is started:
```bash
cd cliproxyapi
npm start
```

### Authentication Issues

If authentication fails:
1. Stop the proxy
2. Clear browser cookies for Claude
3. Restart proxy and re-authenticate

### Port Already in Use

If port 8000 is taken, configure a different port:

```bash
# In cliproxyapi
PORT=8001 npm start
```

Then update `src/claude_proxy.rs`:
```rust
base_url: "http://localhost:8001".to_string(),
```

## Alternative: Custom Proxy URL

To use a different proxy URL:

```rust
// In src/main.rs
let proxy = ClaudeProxy::with_custom_url("http://localhost:8001".to_string());
```

## Resources

- CLIProxyAPI: https://github.com/anthropics/cliproxyapi
- Claude Max: https://claude.ai/
- Documentation: See README for more details
