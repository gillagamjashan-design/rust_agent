# Configuration System

The Rust learning agent uses a TOML-based configuration system to manage settings for web search and API keys.

## Configuration File Location

The configuration file is located at: `~/.agent/config.toml`

If the file doesn't exist, the agent will use sensible defaults.

## Configuration Structure

### Web Search Settings

Controls how the agent performs web searches:

```toml
[web_search]
# Enable or disable web search functionality
enabled = true

# Cache time-to-live in hours (how long to keep cached search results)
cache_ttl_hours = 24

# Request timeout in seconds (maximum time to wait for search results)
timeout_seconds = 10

# Maximum number of search results to retrieve per query
max_results = 5
```

### API Keys

Optional API keys for external services:

```toml
[api_keys]
# Serper API key for web search (optional)
# Get your API key from https://serper.dev/
serper = "your-serper-api-key-here"
```

## Default Configuration

If no configuration file exists, these defaults are used:

- **Web Search Enabled**: Yes
- **Cache TTL**: 24 hours
- **Timeout**: 10 seconds
- **Max Results**: 5 results per query
- **API Keys**: None

## Usage in Code

### Loading Configuration

```rust
use rust_agent::config::Config;

// Load configuration (uses defaults if file doesn't exist)
let config = Config::load()?;

// Access settings
if config.web_search.enabled {
    println!("Web search is enabled");
    println!("Max results: {}", config.web_search.max_results);
}

// Check API keys
if let Some(api_key) = &config.api_keys.serper {
    println!("Serper API key is configured");
}
```

### Saving Configuration

```rust
use rust_agent::config::Config;

// Create a new configuration
let mut config = Config::default();

// Modify settings
config.web_search.enabled = false;
config.web_search.max_results = 10;

// Save to ~/.agent/config.toml
config.save()?;
```

### Creating Example Config

```rust
use rust_agent::config::Config;

// Create and save an example configuration file
Config::save_example()?;
```

## Directory Structure

The agent creates the following directory structure in your home directory:

```
~/.agent/
├── config.toml          # Configuration file
├── cache/               # Web search cache
│   └── search_*.json   # Cached search results
└── data/               # Learning data
    ├── questions.txt   # Generated questions
    ├── answers.txt     # Generated answers
    └── knowledge_base.json  # Learned knowledge
```

## Examples

### Running the Config Demo

```bash
CARGO_HOME=../.cargo cargo run --example config_demo
```

This will:
1. Load the configuration from `~/.agent/config.toml`
2. Display current settings
3. Show example configuration format

### Creating a Custom Configuration

1. Copy the example configuration:
   ```bash
   cp config.toml.example ~/.agent/config.toml
   ```

2. Edit the file with your preferred settings:
   ```bash
   nano ~/.agent/config.toml
   ```

3. Update the values as needed:
   ```toml
   [web_search]
   enabled = true
   cache_ttl_hours = 48  # Keep cache for 2 days
   timeout_seconds = 15   # Longer timeout
   max_results = 10       # More results

   [api_keys]
   serper = "your-actual-api-key"
   ```

### Disabling Web Search

To disable web search functionality, set `enabled = false`:

```toml
[web_search]
enabled = false
cache_ttl_hours = 24
timeout_seconds = 10
max_results = 5
```

## Error Handling

The configuration system handles errors gracefully:

- **Missing config file**: Uses default configuration
- **Invalid TOML syntax**: Returns error with details
- **Missing fields**: Uses default values for missing fields
- **Cannot create directory**: Returns error with details

## Testing

Run the tests for the configuration system:

```bash
CARGO_HOME=../.cargo cargo test config
```

## Cache Management

The web search cache is managed automatically based on the `cache_ttl_hours` setting:

- Results are cached in `~/.agent/cache/`
- Cache files are named using SHA256 hash of the query
- Expired cache entries are automatically deleted when accessed
- You can manually clear the cache using the `/cache clear` command in interactive mode

## Best Practices

1. **Keep API keys secure**: Never commit `config.toml` to version control
2. **Adjust cache TTL**: Lower values for frequently changing data, higher for stable resources
3. **Set appropriate timeouts**: Balance between responsiveness and reliability
4. **Limit max results**: More results = slower searches and higher API costs

## Troubleshooting

### Configuration not loading

Check if the file exists and has correct permissions:
```bash
ls -la ~/.agent/config.toml
```

### Invalid TOML syntax

Validate your TOML file:
```bash
cat ~/.agent/config.toml
```

Look for:
- Missing quotes around strings
- Incorrect section headers
- Typos in field names

### Permission errors

Ensure the `.agent` directory is writable:
```bash
chmod 755 ~/.agent
chmod 644 ~/.agent/config.toml
```
