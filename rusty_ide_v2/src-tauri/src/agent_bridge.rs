use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Errors that can occur during agent bridge operations
#[derive(Error, Debug)]
pub enum AgentBridgeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Response timeout after {0:?}")]
    Timeout(Duration),

    #[error("File watcher error: {0}")]
    Watcher(#[from] notify::Error),

    #[error("Agent directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("No response available")]
    NoResponse,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, AgentBridgeError>;

/// Context sent to the agent for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub timestamp: String,
    pub workspace_path: String,
    pub current_file: Option<String>,
    pub current_code: Option<String>,
    pub files: Vec<String>,
    pub ide_source: String,
    pub query: String,
}

impl AgentContext {
    /// Create a new agent context with the given query
    pub fn new(query: String) -> Self {
        Self {
            timestamp: get_timestamp(),
            workspace_path: String::new(),
            current_file: None,
            current_code: None,
            files: Vec::new(),
            ide_source: String::new(),
            query,
        }
    }

    /// Builder pattern for workspace path
    pub fn with_workspace(mut self, path: String) -> Self {
        self.workspace_path = path;
        self
    }

    /// Builder pattern for current file
    pub fn with_current_file(mut self, file: String, code: String) -> Self {
        self.current_file = Some(file);
        self.current_code = Some(code);
        self
    }

    /// Builder pattern for file list
    pub fn with_files(mut self, files: Vec<String>) -> Self {
        self.files = files;
        self
    }

    /// Builder pattern for IDE source
    pub fn with_ide_source(mut self, source: String) -> Self {
        self.ide_source = source;
        self
    }
}

/// A suggestion for code changes from the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSuggestion {
    pub file: String,
    pub code: String,
    pub language: String,
    pub description: String,
}

/// Response received from the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub timestamp: String,
    pub response_text: String,
    pub code_suggestions: Vec<CodeSuggestion>,
    pub apply_changes: bool,
}

impl AgentResponse {
    /// Check if this response has code suggestions
    pub fn has_suggestions(&self) -> bool {
        !self.code_suggestions.is_empty()
    }

    /// Get suggestions for a specific file
    pub fn suggestions_for_file(&self, file: &str) -> Vec<&CodeSuggestion> {
        self.code_suggestions
            .iter()
            .filter(|s| s.file == file)
            .collect()
    }
}

/// File-based communication bridge with the external agent
pub struct AgentBridge {
    agent_dir: PathBuf,
    request_path: PathBuf,
    response_path: PathBuf,
    watcher: Arc<Mutex<Option<RecommendedWatcher>>>,
    receiver: Arc<Mutex<Option<Receiver<notify::Result<Event>>>>>,
}

impl AgentBridge {
    /// Create a new agent bridge
    ///
    /// This initializes the agent directory at ~/.rusty/agent/ and sets up
    /// the request/response file paths.
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| AgentBridgeError::DirectoryNotFound("Home directory not found".to_string()))?;

        let agent_dir = home.join(".rusty").join("agent");

        // Create agent directory if it doesn't exist
        if !agent_dir.exists() {
            fs::create_dir_all(&agent_dir)?;
        }

        let request_path = agent_dir.join("request.json");
        let response_path = agent_dir.join("response.json");

        Ok(Self {
            agent_dir,
            request_path,
            response_path,
            watcher: Arc::new(Mutex::new(None)),
            receiver: Arc::new(Mutex::new(None)),
        })
    }

    /// Send a request to the agent
    ///
    /// This writes the request to request.json and removes any existing response.json
    pub fn send_request(&self, context: AgentContext) -> Result<()> {
        // Clear any existing response
        if self.response_path.exists() {
            fs::remove_file(&self.response_path)?;
        }

        // Serialize and write request
        let json = serde_json::to_string_pretty(&context)?;
        fs::write(&self.request_path, json)?;

        Ok(())
    }

    /// Wait for a response from the agent with a timeout
    ///
    /// This blocks until either:
    /// - A response is received (returns Some(response))
    /// - The timeout expires (returns Err(Timeout))
    pub fn wait_for_response(&self, timeout: Duration) -> Result<AgentResponse> {
        let start = std::time::Instant::now();

        // Start file watcher
        self.start_watcher()?;

        loop {
            // Check if response file exists
            if let Some(response) = self.check_response()? {
                // Stop watcher and return response
                self.stop_watcher()?;
                return Ok(response);
            }

            // Check for events from file watcher
            if let Some(receiver) = self.receiver.lock().unwrap().as_ref() {
                if let Ok(event_result) = receiver.try_recv() {
                    if let Ok(event) = event_result {
                        // Check if the event is for our response file
                        if event.paths.iter().any(|p| p == &self.response_path) {
                            // Give the file system a moment to complete the write
                            std::thread::sleep(Duration::from_millis(100));

                            if let Some(response) = self.check_response()? {
                                self.stop_watcher()?;
                                return Ok(response);
                            }
                        }
                    }
                }
            }

            // Check timeout
            if start.elapsed() >= timeout {
                self.stop_watcher()?;
                return Err(AgentBridgeError::Timeout(timeout));
            }

            // Sleep briefly to avoid busy waiting
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Check if a response is ready (non-blocking)
    ///
    /// Returns Some(response) if available, None otherwise
    pub fn check_response(&self) -> Result<Option<AgentResponse>> {
        if !self.response_path.exists() {
            return Ok(None);
        }

        // Read and parse response
        let content = fs::read_to_string(&self.response_path)?;

        if content.trim().is_empty() {
            return Ok(None);
        }

        let response: AgentResponse = serde_json::from_str(&content)
            .map_err(|e| AgentBridgeError::InvalidResponse(e.to_string()))?;

        // Clean up response file after reading
        fs::remove_file(&self.response_path)?;

        Ok(Some(response))
    }

    /// Clear request and response files
    pub fn clear(&self) -> Result<()> {
        if self.request_path.exists() {
            fs::remove_file(&self.request_path)?;
        }

        if self.response_path.exists() {
            fs::remove_file(&self.response_path)?;
        }

        Ok(())
    }

    /// Get the IDE source code for self-awareness
    ///
    /// This reads all Rust source files in the src-tauri/src directory
    pub fn get_ide_source(&self) -> Result<String> {
        let mut source = String::new();

        // Get the src-tauri/src directory
        // Navigate up from the binary location to find src
        let current_exe = std::env::current_exe()?;
        let mut src_dir = current_exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("src-tauri").join("src"));

        // If that doesn't work, try relative path
        if src_dir.is_none() || !src_dir.as_ref().unwrap().exists() {
            src_dir = Some(PathBuf::from("src"));
        }

        if let Some(dir) = src_dir {
            if dir.exists() {
                self.collect_rust_files(&dir, &mut source)?;
            }
        }

        if source.is_empty() {
            source = "// IDE source code not available".to_string();
        }

        Ok(source)
    }

    /// Get the agent directory path
    pub fn agent_dir(&self) -> &Path {
        &self.agent_dir
    }

    /// Get the request file path
    pub fn request_path(&self) -> &Path {
        &self.request_path
    }

    /// Get the response file path
    pub fn response_path(&self) -> &Path {
        &self.response_path
    }

    // Private helper methods

    fn start_watcher(&self) -> Result<()> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(&self.agent_dir, RecursiveMode::NonRecursive)?;

        *self.watcher.lock().unwrap() = Some(watcher);
        *self.receiver.lock().unwrap() = Some(rx);

        Ok(())
    }

    fn stop_watcher(&self) -> Result<()> {
        *self.watcher.lock().unwrap() = None;
        *self.receiver.lock().unwrap() = None;
        Ok(())
    }

    fn collect_rust_files(&self, dir: &Path, source: &mut String) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                source.push_str(&format!("\n// File: {}\n", file_name));

                match fs::read_to_string(&path) {
                    Ok(content) => source.push_str(&content),
                    Err(e) => source.push_str(&format!("// Error reading file: {}\n", e)),
                }

                source.push_str("\n");
            } else if path.is_dir() {
                // Recursively process subdirectories
                self.collect_rust_files(&path, source)?;
            }
        }

        Ok(())
    }
}

impl Default for AgentBridge {
    fn default() -> Self {
        Self::new().expect("Failed to create default AgentBridge")
    }
}

// Thread-safe implementation
unsafe impl Send for AgentBridge {}
unsafe impl Sync for AgentBridge {}

/// Get current timestamp in ISO 8601 format
fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Simple ISO 8601 format
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();

    // Convert to datetime (simplified, not handling leap seconds)
    let days = secs / 86400;
    let day_secs = secs % 86400;
    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;
    let seconds = day_secs % 60;

    // Simplified date calculation (assuming days since epoch)
    let years = days / 365;
    let year = 1970 + years;
    let day_of_year = days % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        year, month, day, hours, minutes, seconds, nanos
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_bridge_creation() {
        let bridge = AgentBridge::new();
        assert!(bridge.is_ok());

        let bridge = bridge.unwrap();
        assert!(bridge.agent_dir().exists());
    }

    #[test]
    fn test_agent_context_builder() {
        let context = AgentContext::new("Test query".to_string())
            .with_workspace("/test/workspace".to_string())
            .with_current_file("main.rs".to_string(), "fn main() {}".to_string())
            .with_files(vec!["main.rs".to_string(), "lib.rs".to_string()]);

        assert_eq!(context.query, "Test query");
        assert_eq!(context.workspace_path, "/test/workspace");
        assert_eq!(context.current_file, Some("main.rs".to_string()));
        assert_eq!(context.files.len(), 2);
    }

    #[test]
    fn test_send_request() {
        let bridge = AgentBridge::new().unwrap();
        let context = AgentContext::new("Test query".to_string());

        let result = bridge.send_request(context);
        assert!(result.is_ok());
        assert!(bridge.request_path().exists());

        // Clean up
        let _ = bridge.clear();
    }

    #[test]
    fn test_clear() {
        let bridge = AgentBridge::new().unwrap();
        let context = AgentContext::new("Test query".to_string());

        let _ = bridge.send_request(context);
        assert!(bridge.request_path().exists());

        let result = bridge.clear();
        assert!(result.is_ok());
        assert!(!bridge.request_path().exists());
    }

    #[test]
    fn test_timestamp_format() {
        let ts = get_timestamp();
        assert!(ts.contains("T"));
        assert!(ts.contains("Z"));
        assert!(ts.len() > 20);
    }

    #[test]
    fn test_agent_response_methods() {
        let response = AgentResponse {
            timestamp: get_timestamp(),
            response_text: "Test response".to_string(),
            code_suggestions: vec![
                CodeSuggestion {
                    file: "main.rs".to_string(),
                    code: "fn main() {}".to_string(),
                    language: "rust".to_string(),
                    description: "Test".to_string(),
                },
            ],
            apply_changes: true,
        };

        assert!(response.has_suggestions());
        let suggestions = response.suggestions_for_file("main.rs");
        assert_eq!(suggestions.len(), 1);
    }
}
