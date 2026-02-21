use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Web search configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchConfig {
    /// Whether web search is enabled
    pub enabled: bool,
    /// Cache time-to-live in hours
    pub cache_ttl_hours: u32,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum number of search results to retrieve
    pub max_results: usize,
}

impl Default for WebSearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_ttl_hours: 24,
            timeout_seconds: 10,
            max_results: 5,
        }
    }
}

/// API keys configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    /// Optional Serper API key for web search
    pub serper: Option<String>,
}

impl Default for ApiKeys {
    fn default() -> Self {
        Self { serper: None }
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Web search configuration
    pub web_search: WebSearchConfig,
    /// API keys
    pub api_keys: ApiKeys,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web_search: WebSearchConfig::default(),
            api_keys: ApiKeys::default(),
        }
    }
}

impl Config {
    /// Get the path to the configuration file (~/.agent/config.toml)
    fn config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        Ok(home_dir.join(".agent").join("config.toml"))
    }

    /// Get the path to the .agent directory
    fn agent_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        Ok(home_dir.join(".agent"))
    }

    /// Load configuration from ~/.agent/config.toml
    /// If the file doesn't exist, returns default configuration
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        // If config file doesn't exist, return defaults
        if !config_path.exists() {
            println!("No config file found at {:?}, using defaults", config_path);
            return Ok(Self::default());
        }

        // Read and parse the config file
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;

        println!("Loaded configuration from {:?}", config_path);
        Ok(config)
    }

    /// Save configuration to ~/.agent/config.toml
    pub fn save(&self) -> Result<()> {
        let agent_dir = Self::agent_dir()?;
        let config_path = Self::config_path()?;

        // Create the .agent directory if it doesn't exist
        if !agent_dir.exists() {
            fs::create_dir_all(&agent_dir)
                .with_context(|| format!("Failed to create directory: {:?}", agent_dir))?;
            println!("Created directory: {:?}", agent_dir);
        }

        // Serialize the config to TOML
        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize configuration to TOML")?;

        // Write to file
        fs::write(&config_path, toml_string)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        println!("Saved configuration to {:?}", config_path);
        Ok(())
    }

    /// Create and save an example configuration file
    pub fn save_example() -> Result<()> {
        let config = Self::default();
        config.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.web_search.enabled);
        assert_eq!(config.web_search.cache_ttl_hours, 24);
        assert_eq!(config.web_search.timeout_seconds, 10);
        assert_eq!(config.web_search.max_results, 5);
        assert!(config.api_keys.serper.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("enabled"));
        assert!(toml_str.contains("cache_ttl_hours"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [web_search]
            enabled = false
            cache_ttl_hours = 48
            timeout_seconds = 20
            max_results = 10

            [api_keys]
            serper = "test-key-123"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(!config.web_search.enabled);
        assert_eq!(config.web_search.cache_ttl_hours, 48);
        assert_eq!(config.web_search.timeout_seconds, 20);
        assert_eq!(config.web_search.max_results, 10);
        assert_eq!(config.api_keys.serper, Some("test-key-123".to_string()));
    }
}
