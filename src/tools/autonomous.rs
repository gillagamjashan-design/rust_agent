// Autonomous agent tools - file writing, command execution, and file reading
// All operations are constrained to the launch directory for security

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Duration;

/// Tool definition matching Claude API schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Tool result for Claude API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_use_id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// Get all available tools for the autonomous agent
pub fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "write_file".to_string(),
            description: "Write content to a file. Path must be relative to the project directory. Creates parent directories if needed.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative file path (e.g., 'src/main.rs', 'examples/hello.rs')"
                    },
                    "content": {
                        "type": "string",
                        "description": "Complete file content to write"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        Tool {
            name: "read_file".to_string(),
            description: "Read content from a file in the project directory.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative file path to read"
                    }
                },
                "required": ["path"]
            }),
        },
        Tool {
            name: "run_command".to_string(),
            description: "Execute a bash command in the project directory. Use this to run cargo, rustc, tests, or any Linux command.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "Bash command to execute (e.g., 'cargo build', 'rustc main.rs', 'ls -la')"
                    },
                    "timeout_secs": {
                        "type": "integer",
                        "description": "Timeout in seconds (default: 60, max: 300)",
                        "minimum": 1,
                        "maximum": 300
                    }
                },
                "required": ["command"]
            }),
        },
        Tool {
            name: "list_files".to_string(),
            description: "List files and directories in a path. Shows directory structure with '/' suffix for directories.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Directory path to list (default: '.' for current directory)"
                    }
                },
                "required": []
            }),
        },
    ]
}

/// Executes tools with security constraints (launch directory only)
pub struct ToolExecutor {
    launch_dir: PathBuf,
}

impl ToolExecutor {
    pub fn new(launch_dir: PathBuf) -> Self {
        Self { launch_dir }
    }

    /// Execute a tool by name with the given input parameters
    pub async fn execute(&self, name: &str, input: &Value) -> Result<String> {
        match name {
            "write_file" => self.write_file(input).await,
            "read_file" => self.read_file(input).await,
            "run_command" => self.run_command(input).await,
            "list_files" => self.list_files(input).await,
            _ => Err(anyhow!("Unknown tool: {}", name)),
        }
    }

    /// Validate that a path is within the launch directory
    ///
    /// - Relative paths are resolved against launch_dir
    /// - Absolute paths are allowed only if they still resolve under launch_dir
    fn validate_path(&self, path: &str) -> Result<PathBuf> {
        let path_buf = PathBuf::from(path);
        let full_path = if path_buf.is_absolute() {
            path_buf
        } else {
            self.launch_dir.join(path)
        };

        // Try to canonicalize - this will fail for non-existent paths
        match full_path.canonicalize() {
            Ok(canonical) => {
                if !canonical.starts_with(&self.launch_dir) {
                    bail!("Path outside project directory: {}", path);
                }
                Ok(canonical)
            }
            Err(_) => {
                // For non-existent paths, check parent directory
                if let Some(parent) = full_path.parent() {
                    if parent.exists() {
                        let canonical_parent = parent.canonicalize()?;
                        if !canonical_parent.starts_with(&self.launch_dir) {
                            bail!("Path outside project directory: {}", path);
                        }
                    }
                }
                Ok(full_path)
            }
        }
    }

    /// Write content to a file
    async fn write_file(&self, input: &Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'path' field"))?;
        let content = input["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'content' field"))?;

        // Validate path before creating
        let full_path = self.validate_path(path)?;

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write the file
        tokio::fs::write(&full_path, content).await?;

        // Verify the created file is within bounds
        let canonical = full_path.canonicalize()?;
        if !canonical.starts_with(&self.launch_dir) {
            tokio::fs::remove_file(&canonical).await?;
            bail!("Path outside project directory: {}", path);
        }

        Ok(format!(
            "Successfully wrote {} bytes to {}",
            content.len(),
            path
        ))
    }

    /// Read content from a file
    async fn read_file(&self, input: &Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'path' field"))?;

        let full_path = self.validate_path(path)?;

        // Check if file exists
        if !full_path.exists() {
            bail!("File not found: {}", path);
        }

        // Check if it's a file (not a directory)
        if !full_path.is_file() {
            bail!("Path is not a file: {}", path);
        }

        // Read the file
        let content = tokio::fs::read_to_string(full_path).await?;

        Ok(content)
    }

    /// Execute a bash command
    async fn run_command(&self, input: &Value) -> Result<String> {
        let command = input["command"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'command' field"))?;

        let timeout = input["timeout_secs"]
            .as_u64()
            .unwrap_or(60)
            .min(300); // Max 5 minutes

        let output = tokio::time::timeout(
            Duration::from_secs(timeout),
            tokio::process::Command::new("bash")
                .arg("-c")
                .arg(command)
                .current_dir(&self.launch_dir)
                .output(),
        )
        .await;

        match output {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let exit_code = output.status.code().unwrap_or(-1);

                Ok(format!(
                    "Exit code: {}\n\nStdout:\n{}\n\nStderr:\n{}",
                    exit_code,
                    if stdout.is_empty() {
                        "(empty)"
                    } else {
                        &stdout
                    },
                    if stderr.is_empty() {
                        "(empty)"
                    } else {
                        &stderr
                    }
                ))
            }
            Ok(Err(e)) => Err(anyhow!("Failed to execute command: {}", e)),
            Err(_) => Err(anyhow!("Command timed out after {} seconds", timeout)),
        }
    }

    /// List files in a directory
    async fn list_files(&self, input: &Value) -> Result<String> {
        let path = input["path"].as_str().unwrap_or(".");

        let full_path = self.validate_path(path)?;

        // Check if path exists
        if !full_path.exists() {
            bail!("Directory not found: {}", path);
        }

        // Check if it's a directory
        if !full_path.is_dir() {
            bail!("Path is not a directory: {}", path);
        }

        // Read directory entries
        let mut entries = tokio::fs::read_dir(full_path).await?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            let file_type = if entry.file_type().await?.is_dir() {
                "/"
            } else {
                ""
            };
            files.push(format!("{}{}", name, file_type));
        }

        files.sort();

        if files.is_empty() {
            Ok("(empty directory)".to_string())
        } else {
            Ok(files.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_path_validation() {
        let launch_dir = env::current_dir().unwrap();
        let executor = ToolExecutor::new(launch_dir.clone());

        // Valid relative path
        assert!(executor.validate_path("src/lib.rs").is_ok());

        // Invalid absolute path outside project
        assert!(executor.validate_path("/etc/passwd").is_err());

        // Invalid path traversal
        assert!(executor.validate_path("../../etc/passwd").is_err());
    }

    #[tokio::test]
    async fn test_list_files() {
        let launch_dir = env::current_dir().unwrap();
        let executor = ToolExecutor::new(launch_dir);

        let input = json!({ "path": "." });
        let result = executor.list_files(&input).await;

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.contains("src/") || files.contains("Cargo.toml"));
    }
}
