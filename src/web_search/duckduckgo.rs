use super::types::{SearchProvider, SearchQuery, SearchResponse, SearchResult};
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// DuckDuckGo Instant Answer API response structures
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DuckDuckGoResponse {
    /// Main abstract text (if available)
    #[serde(default)]
    abstract_text: String,
    /// Abstract source URL
    #[serde(default)]
    abstract_url: String,
    /// Definition text
    #[serde(default)]
    definition: String,
    /// Definition source URL
    #[serde(default)]
    definition_url: String,
    /// Related topics
    #[serde(default)]
    related_topics: Vec<RelatedTopic>,
    /// Answer (direct answer to query)
    #[serde(default)]
    answer: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RelatedTopic {
    /// First URL (link to topic)
    #[serde(default)]
    first_url: String,
    /// Text description
    #[serde(default)]
    text: String,
    /// Optional nested topics
    #[serde(default)]
    topics: Vec<RelatedTopic>,
}

/// Client for searching via DuckDuckGo Instant Answer API
pub struct DuckDuckGoClient {
    client: Client,
    timeout: Duration,
}

impl DuckDuckGoClient {
    /// Create a new DuckDuckGo client with default timeout (10 seconds)
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("rust_agent/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            timeout: Duration::from_secs(10),
        })
    }

    /// Create a new DuckDuckGo client with custom timeout
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("rust_agent/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, timeout })
    }

    /// Search using DuckDuckGo Instant Answer API
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json",
            urlencoding::encode(&query.query)
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to DuckDuckGo API")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "DuckDuckGo API returned error status: {}",
                response.status()
            );
        }

        let ddg_response: DuckDuckGoResponse = response
            .json()
            .await
            .context("Failed to parse DuckDuckGo API response")?;

        let results = self.parse_response(ddg_response, query.max_results);

        Ok(SearchResponse::new(
            query.query.clone(),
            results,
            SearchProvider::DuckDuckGo,
        ))
    }

    /// Parse DuckDuckGo API response into SearchResult vector
    fn parse_response(
        &self,
        response: DuckDuckGoResponse,
        max_results: usize,
    ) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let timestamp = Utc::now();

        // Add abstract as first result if available
        if !response.abstract_text.is_empty() && !response.abstract_url.is_empty() {
            results.push(SearchResult {
                title: "Abstract".to_string(),
                url: response.abstract_url.clone(),
                snippet: response.abstract_text.clone(),
                source: SearchProvider::DuckDuckGo,
                timestamp,
                relevance_score: 1.0,
            });
        }

        // Add definition as result if available
        if !response.definition.is_empty() && !response.definition_url.is_empty() {
            results.push(SearchResult {
                title: "Definition".to_string(),
                url: response.definition_url.clone(),
                snippet: response.definition.clone(),
                source: SearchProvider::DuckDuckGo,
                timestamp,
                relevance_score: 0.95,
            });
        }

        // Add direct answer if available
        if !response.answer.is_empty() {
            results.push(SearchResult {
                title: "Answer".to_string(),
                url: String::new(),
                snippet: response.answer.clone(),
                source: SearchProvider::DuckDuckGo,
                timestamp,
                relevance_score: 0.9,
            });
        }

        // Parse related topics
        let mut related_results = self.parse_related_topics(&response.related_topics, timestamp);
        results.append(&mut related_results);

        // Limit results to max_results
        results.truncate(max_results);

        results
    }

    /// Recursively parse related topics
    fn parse_related_topics(
        &self,
        topics: &[RelatedTopic],
        timestamp: chrono::DateTime<Utc>,
    ) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let mut relevance_score = 0.85;

        for topic in topics {
            // If there are nested topics, parse them recursively
            if !topic.topics.is_empty() {
                let mut nested = self.parse_related_topics(&topic.topics, timestamp);
                results.append(&mut nested);
            } else if !topic.text.is_empty() {
                // Extract title and snippet from text
                let (title, snippet) = self.split_topic_text(&topic.text);

                results.push(SearchResult {
                    title,
                    url: topic.first_url.clone(),
                    snippet,
                    source: SearchProvider::DuckDuckGo,
                    timestamp,
                    relevance_score,
                });

                // Decrease relevance score for subsequent results
                relevance_score = (relevance_score - 0.05).max(0.5);
            }
        }

        results
    }

    /// Split topic text into title and snippet
    /// DuckDuckGo formats text as "Title - Snippet" or just "Text"
    fn split_topic_text(&self, text: &str) -> (String, String) {
        if let Some(pos) = text.find(" - ") {
            let title = text[..pos].trim().to_string();
            let snippet = text[pos + 3..].trim().to_string();
            (title, snippet)
        } else {
            // If no separator, use first part as title and full text as snippet
            let title = text
                .split_whitespace()
                .take(5)
                .collect::<Vec<_>>()
                .join(" ");
            (title, text.to_string())
        }
    }

    /// Filter results to only include Rust documentation links
    pub fn filter_rust_docs(results: Vec<SearchResult>) -> Vec<SearchResult> {
        results
            .into_iter()
            .filter(|r| {
                r.url.contains("doc.rust-lang.org")
                    || r.url.contains("rust-lang.org")
                    || r.url.contains("docs.rs")
                    || r.snippet.to_lowercase().contains("rust")
                    || r.title.to_lowercase().contains("rust")
            })
            .collect()
    }
}

impl Default for DuckDuckGoClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default DuckDuckGo client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_duckduckgo_search() {
        let client = DuckDuckGoClient::new().unwrap();
        let query = SearchQuery::new("Rust programming language");

        let response = client.search(&query).await;

        // Should not error even if no results
        assert!(response.is_ok());
    }

    #[test]
    fn test_split_topic_text() {
        let client = DuckDuckGoClient::new().unwrap();

        let (title, snippet) = client.split_topic_text("Rust Language - A systems programming language");
        assert_eq!(title, "Rust Language");
        assert_eq!(snippet, "A systems programming language");

        let (title, snippet) = client.split_topic_text("Just some text");
        assert_eq!(title, "Just some text");
        assert_eq!(snippet, "Just some text");
    }
}
