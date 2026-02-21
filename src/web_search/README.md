# Web Search Module

This module provides web search capabilities for the Rust learning agent using various search providers.

## Components

### Types (`types.rs`)

Core data structures for web search:

- **`SearchResult`**: Represents a single search result with:
  - `title`: Title of the result
  - `url`: URL of the resource
  - `snippet`: Summary of the content
  - `source`: Which provider returned this result
  - `timestamp`: When the result was fetched
  - `relevance_score`: Score from 0.0 to 1.0

- **`SearchProvider`**: Supported search providers
  - `DuckDuckGo`: Free instant answer API
  - `Serper`: (Future) Google search API
  - `Scraped`: (Future) Direct web scraping

- **`SearchQuery`**: Query parameters
  - `query`: Search query string
  - `max_results`: Maximum number of results to return
  - `provider`: Which search provider to use

- **`SearchResponse`**: Complete search response
  - `query`: Original query
  - `results`: Vector of search results
  - `provider`: Provider used
  - `total_results`: Number of results returned
  - `timestamp`: When the search was performed

### DuckDuckGo Client (`duckduckgo.rs`)

Implements search using the DuckDuckGo Instant Answer API.

#### Features

- **Free API**: No API key required
- **Instant Answers**: Returns direct answers, abstracts, and definitions
- **Related Topics**: Extracts related topics from the API response
- **Rust Documentation**: Can filter results for Rust-specific documentation
- **Timeout**: Configurable timeout (default: 10 seconds)
- **Error Handling**: Returns empty results instead of errors when no results found

#### API Response Structure

The DuckDuckGo API returns:
- **Abstract**: Main abstract text and URL (highest relevance)
- **Definition**: Definition text and URL (0.95 relevance)
- **Answer**: Direct answer to the query (0.9 relevance)
- **RelatedTopics**: Array of related topics with URLs (0.85-0.5 relevance)

#### Usage Example

```rust
use rust_agent::web_search::{DuckDuckGoClient, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create client
    let client = DuckDuckGoClient::new()?;

    // Create query
    let query = SearchQuery::new("Rust programming language")
        .with_max_results(5);

    // Execute search
    let response = client.search(&query).await?;

    // Process results
    for result in response.results {
        println!("{}: {}", result.title, result.url);
        println!("  {}", result.snippet);
    }

    Ok(())
}
```

#### Filtering Rust Documentation

```rust
let response = client.search(&query).await?;
let rust_docs = DuckDuckGoClient::filter_rust_docs(response.results);
```

This filters results to only include:
- doc.rust-lang.org
- rust-lang.org
- docs.rs
- Results mentioning "rust" in title or snippet

## Testing

Run the example to test the DuckDuckGo client:

```bash
CARGO_HOME=/workspace/jashan/.cargo cargo run --example test_web_search
```

## Future Enhancements

1. **Serper Provider**: Integration with Google search via Serper API
2. **Web Scraping**: Direct scraping of documentation sites
3. **Caching**: Cache search results to reduce API calls
4. **Ranking**: Improved relevance scoring based on content analysis
5. **Multi-Provider**: Query multiple providers and merge results

## Dependencies

- `reqwest`: HTTP client for API calls
- `serde`: JSON serialization/deserialization
- `chrono`: Timestamp handling
- `anyhow`: Error handling
- `urlencoding`: URL encoding for query parameters
- `tokio`: Async runtime

## API Limits

DuckDuckGo Instant Answer API:
- No API key required
- No hard rate limits
- Best effort service (no SLA)
- May not return results for all queries
- Focus on instant answers vs comprehensive search results
