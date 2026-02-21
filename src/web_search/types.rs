use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Title of the search result
    pub title: String,
    /// URL of the resource
    pub url: String,
    /// Snippet or summary of the content
    pub snippet: String,
    /// Source of the search result (e.g., "DuckDuckGo")
    pub source: SearchProvider,
    /// Timestamp when the result was fetched
    pub timestamp: DateTime<Utc>,
    /// Relevance score (0.0 to 1.0)
    pub relevance_score: f32,
}

/// Supported search providers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchProvider {
    DuckDuckGo,
    Serper,
    Scraped,
}

impl std::fmt::Display for SearchProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchProvider::DuckDuckGo => write!(f, "DuckDuckGo"),
            SearchProvider::Serper => write!(f, "Serper"),
            SearchProvider::Scraped => write!(f, "Scraped"),
        }
    }
}

/// Query parameters for a search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search query string
    pub query: String,
    /// Maximum number of results to return
    pub max_results: usize,
    /// Preferred search provider
    pub provider: SearchProvider,
}

impl SearchQuery {
    /// Create a new search query with default settings
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            max_results: 10,
            provider: SearchProvider::DuckDuckGo,
        }
    }

    /// Set the maximum number of results
    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }

    /// Set the search provider
    pub fn with_provider(mut self, provider: SearchProvider) -> Self {
        self.provider = provider;
        self
    }
}

/// Response from a search operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The original query
    pub query: String,
    /// List of search results
    pub results: Vec<SearchResult>,
    /// Search provider used
    pub provider: SearchProvider,
    /// Total number of results found
    pub total_results: usize,
    /// Timestamp of the search
    pub timestamp: DateTime<Utc>,
}

impl SearchResponse {
    /// Create a new search response
    pub fn new(query: String, results: Vec<SearchResult>, provider: SearchProvider) -> Self {
        let total_results = results.len();
        Self {
            query,
            results,
            provider,
            total_results,
            timestamp: Utc::now(),
        }
    }

    /// Check if the response has any results
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get the number of results
    pub fn len(&self) -> usize {
        self.results.len()
    }
}
