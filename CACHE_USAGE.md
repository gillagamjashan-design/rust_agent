# Search Cache System Usage

## Overview

The `SearchCache` is a file-based caching system for web search results. It stores cached results as individual JSON files in the `~/.agent/cache/` directory, with each file named using a SHA256 hash of the query.

## Features

- **File-based storage**: Each search result is stored as a separate JSON file
- **TTL (Time-To-Live)**: Cached results expire after a configurable duration
- **SHA256 hashing**: Query strings are hashed to create unique, collision-resistant filenames
- **Automatic cleanup**: Expired and corrupted cache files are automatically deleted
- **Error handling**: Gracefully handles corrupted files and I/O errors

## Usage Example

```rust
use std::path::PathBuf;
use rust_agent::cache::SearchCache;
use rust_agent::web_search::{SearchResponse, SearchResult, SearchProvider};

// Create a cache with 24-hour TTL
let cache_dir = PathBuf::from("/home/user/.agent/cache");
let cache = SearchCache::new(cache_dir, 24)?;

// Create a search response
let response = SearchResponse::new(
    "rust programming".to_string(),
    vec![
        SearchResult {
            title: "The Rust Programming Language".to_string(),
            url: "https://www.rust-lang.org/".to_string(),
            snippet: "A language empowering everyone...".to_string(),
            source: SearchProvider::DuckDuckGo,
            timestamp: chrono::Utc::now(),
            relevance_score: 1.0,
        }
    ],
    SearchProvider::DuckDuckGo,
);

// Save to cache
cache.set("rust programming", response)?;

// Retrieve from cache
if let Some(cached_response) = cache.get("rust programming") {
    println!("Found cached results: {}", cached_response.results.len());
}

// Clear all cached entries
cache.clear()?;

// Get count of cached entries
let count = cache.count();
println!("Cached entries: {}", count);
```

## API Reference

### `SearchCache::new(cache_dir: PathBuf, ttl_hours: i64) -> Result<Self>`

Creates a new cache instance. Automatically creates the cache directory if it doesn't exist.

**Parameters:**
- `cache_dir`: Path to the directory where cache files will be stored (typically `~/.agent/cache/`)
- `ttl_hours`: Time-to-live for cached results in hours

**Returns:** `Result<SearchCache>` - The cache instance or an error

### `cache.get(query: &str) -> Option<SearchResponse>`

Retrieves a cached search result for the given query.

**Returns:**
- `Some(SearchResponse)` if cached result exists and is not expired
- `None` if no cached result exists, result is expired, or file is corrupted

**Side effects:** Automatically deletes expired or corrupted cache files

### `cache.set(query: &str, response: SearchResponse) -> Result<()>`

Saves a search result to the cache.

**Parameters:**
- `query`: The search query string
- `response`: The search response to cache

**Returns:** `Result<()>` - Success or error

### `cache.clear() -> Result<()>`

Deletes all cached search result files.

**Returns:** `Result<()>` - Success or error

### `cache.count() -> usize`

Returns the number of cached entries.

**Returns:** `usize` - Number of cache files

## Cache File Format

Cache files are stored as JSON with the following structure:

```json
{
  "response": {
    "query": "rust programming",
    "results": [
      {
        "title": "The Rust Programming Language",
        "url": "https://www.rust-lang.org/",
        "snippet": "A language empowering everyone...",
        "source": "DuckDuckGo",
        "timestamp": "2026-02-20T12:00:00Z",
        "relevance_score": 1.0
      }
    ],
    "provider": "DuckDuckGo",
    "total_results": 1,
    "timestamp": "2026-02-20T12:00:00Z"
  },
  "cached_at": "2026-02-20T12:00:00Z"
}
```

**File naming:** `search_<sha256_hash>.json`

Example: `search_a3c5f1e2b4d8c9a1f3e5b7d9c1a3f5e7b9d1c3a5f7e9b1d3c5a7f9e1b3d5c7a9.json`

## Implementation Details

### SHA256 Hashing

Queries are hashed using SHA256 to create unique filenames:
- Ensures no filename collisions
- Creates consistent hashes for the same query
- Generates filesystem-safe filenames

### TTL and Expiration

The cache uses file modification time to determine expiration:
1. When retrieving a cached result, the file's modification time is checked
2. If `(current_time - modification_time) > TTL`, the file is considered expired
3. Expired files are automatically deleted

### Error Handling

The cache handles several error cases gracefully:
- **Corrupted files**: If a cache file cannot be deserialized, it's deleted and `None` is returned
- **Missing directories**: The cache directory is automatically created if it doesn't exist
- **I/O errors**: All I/O operations return `Result` types for proper error handling
- **Permission errors**: File operations that fail due to permissions are logged but don't crash the application

## Dependencies

The cache system uses the following crates (already in Cargo.toml):
- `serde_json`: JSON serialization/deserialization
- `sha2`: SHA256 hashing for cache keys
- `chrono`: Timestamp handling and expiration checking
- `anyhow`: Error handling with context

## Testing

The module includes comprehensive tests:
- `test_cache_set_and_get`: Verifies basic cache operations
- `test_cache_miss`: Tests cache miss scenarios
- `test_cache_key_consistency`: Ensures hash consistency
- `test_cache_clear`: Tests clearing all cached entries
- `test_cache_expiration`: Verifies TTL-based expiration

Run tests with:
```bash
cargo test cache
```

## Integration with Interactive Agent

The cache is automatically used by the InteractiveAgent:

```rust
let cache = SearchCache::new(
    cache_dir,
    config.web_search.cache_ttl_hours as i64,
).expect("Failed to create search cache");
```

When a search is performed, the agent:
1. Checks the cache first
2. If found and valid, returns cached results
3. If not found or expired, performs a new search
4. Stores the new results in the cache
