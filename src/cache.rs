use crate::web_search::SearchResponse;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SearchCache {
    cache_dir: PathBuf,
    ttl: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedEntry {
    response: SearchResponse,
    cached_at: DateTime<Utc>,
}

impl SearchCache {
    /// Create a new SearchCache instance
    ///
    /// # Arguments
    /// * `cache_dir` - Directory to store cache files (typically ~/.agent/cache/)
    /// * `ttl_hours` - Time-to-live for cached results in hours
    pub fn new(cache_dir: PathBuf, ttl_hours: i64) -> Result<Self> {
        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .context(format!("Failed to create cache directory: {:?}", cache_dir))?;
        }

        let ttl = Duration::hours(ttl_hours);

        Ok(Self { cache_dir, ttl })
    }

    /// Get a cached search result for a query
    ///
    /// Returns None if:
    /// - Cache entry doesn't exist
    /// - Cache entry is expired
    /// - Cache file is corrupted
    pub fn get(&self, query: &str) -> Option<SearchResponse> {
        let cache_file = self.cache_file_path(query);

        if !cache_file.exists() {
            return None;
        }

        // Check if cache is expired
        if self.is_expired(&cache_file) {
            // Delete expired cache file
            let _ = fs::remove_file(&cache_file);
            return None;
        }

        // Read and deserialize cache file
        match fs::read_to_string(&cache_file) {
            Ok(content) => match serde_json::from_str::<CachedEntry>(&content) {
                Ok(entry) => Some(entry.response),
                Err(_) => {
                    // Corrupted cache file, delete it
                    let _ = fs::remove_file(&cache_file);
                    None
                }
            },
            Err(_) => None,
        }
    }

    /// Save a search result to the cache
    pub fn set(&self, query: &str, response: SearchResponse) -> Result<()> {
        let cache_file = self.cache_file_path(query);

        let entry = CachedEntry {
            response,
            cached_at: Utc::now(),
        };

        let json = serde_json::to_string_pretty(&entry)
            .context("Failed to serialize cache entry")?;

        fs::write(&cache_file, json)
            .context(format!("Failed to write cache file: {:?}", cache_file))?;

        Ok(())
    }

    /// Generate a cache key (filename) for a query using SHA256 hash
    fn cache_key(&self, query: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Get the full path for a cache file
    fn cache_file_path(&self, query: &str) -> PathBuf {
        let hash = self.cache_key(query);
        self.cache_dir.join(format!("search_{}.json", hash))
    }

    /// Check if a cache file is expired based on TTL
    fn is_expired(&self, file_path: &Path) -> bool {
        // Get file metadata
        let metadata = match fs::metadata(file_path) {
            Ok(meta) => meta,
            Err(_) => return true, // If we can't read metadata, consider it expired
        };

        // Get file modification time
        let modified = match metadata.modified() {
            Ok(time) => time,
            Err(_) => return true,
        };

        // Convert to DateTime
        let modified_dt: DateTime<Utc> = modified.into();
        let now = Utc::now();

        // Check if the file is older than TTL
        now.signed_duration_since(modified_dt) > self.ttl
    }

    /// Clear all cached files
    pub fn clear(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            return Ok(());
        }

        // Read all files in cache directory
        let entries = fs::read_dir(&self.cache_dir)
            .context(format!("Failed to read cache directory: {:?}", self.cache_dir))?;

        let mut errors = Vec::new();

        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        // Only delete files that match our cache pattern
                        if let Some(filename) = path.file_name() {
                            if let Some(name) = filename.to_str() {
                                if name.starts_with("search_") && name.ends_with(".json") {
                                    if let Err(e) = fs::remove_file(&path) {
                                        errors.push(format!("Failed to delete {:?}: {}", path, e));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("Failed to read directory entry: {}", e));
                }
            }
        }

        if !errors.is_empty() {
            anyhow::bail!("Errors during cache clear: {}", errors.join(", "));
        }

        Ok(())
    }

    /// Get the number of cached entries
    pub fn count(&self) -> usize {
        if !self.cache_dir.exists() {
            return 0;
        }

        fs::read_dir(&self.cache_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().is_file()
                            && e.file_name()
                                .to_str()
                                .map(|s| s.starts_with("search_") && s.ends_with(".json"))
                                .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SearchResult;
    use std::thread;
    use std::time::Duration as StdDuration;

    fn create_test_cache() -> SearchCache {
        let temp_dir = std::env::temp_dir().join("test_cache");
        SearchCache::new(temp_dir, 1).unwrap()
    }

    fn create_test_response(query: &str) -> SearchResponse {
        use crate::web_search::{SearchProvider, SearchResult};

        SearchResponse::new(
            query.to_string(),
            vec![SearchResult {
                title: "Test Result".to_string(),
                url: "https://example.com".to_string(),
                snippet: "Test snippet".to_string(),
                source: SearchProvider::DuckDuckGo,
                timestamp: Utc::now(),
                relevance_score: 1.0,
            }],
            SearchProvider::DuckDuckGo,
        )
    }

    #[test]
    fn test_cache_set_and_get() {
        let cache = create_test_cache();
        let query = "test query";
        let response = create_test_response(query);

        // Set cache
        cache.set(query, response.clone()).unwrap();

        // Get cache
        let cached = cache.get(query);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().query, query);

        // Cleanup
        cache.clear().unwrap();
    }

    #[test]
    fn test_cache_miss() {
        let cache = create_test_cache();
        let result = cache.get("nonexistent query");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_key_consistency() {
        let cache = create_test_cache();
        let query = "test query";
        let key1 = cache.cache_key(query);
        let key2 = cache.cache_key(query);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cache_clear() {
        let cache = create_test_cache();

        // Add multiple entries
        cache.set("query1", create_test_response("query1")).unwrap();
        cache.set("query2", create_test_response("query2")).unwrap();

        assert_eq!(cache.count(), 2);

        // Clear cache
        cache.clear().unwrap();

        assert_eq!(cache.count(), 0);
    }

    #[test]
    fn test_cache_expiration() {
        // Create cache with 0 hour TTL (immediately expires)
        let temp_dir = std::env::temp_dir().join("test_cache_expiry");
        let cache = SearchCache::new(temp_dir, 0).unwrap();

        let query = "test query";
        let response = create_test_response(query);

        cache.set(query, response).unwrap();

        // Sleep briefly to ensure time passes
        thread::sleep(StdDuration::from_millis(100));

        // Should be expired and return None
        let result = cache.get(query);
        assert!(result.is_none());

        cache.clear().unwrap();
    }
}
