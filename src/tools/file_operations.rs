use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileOperations {
    workspace_root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileOperation {
    pub operation_type: OperationType,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Create,
    Modify,
    Delete,
}

impl FileOperations {
    /// Create new FileOperations instance
    /// workspace_root: Base directory for file operations (default: current dir)
    pub fn new(workspace_root: Option<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.unwrap_or_else(|| PathBuf::from(".")),
        }
    }

    /// Create a new file with the given content
    /// Returns: Result with success message or error
    pub fn create_file(&self, path: &str, content: &str) -> Result<String> {
        let full_path = self.resolve_path(path)?;

        // Check if file already exists
        if full_path.exists() {
            return Err(anyhow::anyhow!(
                "File already exists: {}. Use MODIFY_FILE to update it.",
                path
            ));
        }

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directories for {}", path))?;
        }

        // Write file
        fs::write(&full_path, content)
            .context(format!("Failed to write file {}", path))?;

        Ok(format!("✅ Created file: {}", path))
    }

    /// Modify an existing file
    /// For now, this overwrites the file. Future: support patches/diffs
    pub fn modify_file(&self, path: &str, new_content: &str) -> Result<String> {
        let full_path = self.resolve_path(path)?;

        // Check if file exists
        if !full_path.exists() {
            return Err(anyhow::anyhow!(
                "File does not exist: {}. Use CREATE_FILE to create it.",
                path
            ));
        }

        // Backup original (optional safety feature)
        let backup_path = full_path.with_extension("bak");
        fs::copy(&full_path, &backup_path)
            .context("Failed to create backup")?;

        // Write new content
        fs::write(&full_path, new_content)
            .context(format!("Failed to modify file {}", path))?;

        Ok(format!("✅ Modified file: {} (backup: {}.bak)", path, path))
    }

    /// Read a file's contents
    pub fn read_file(&self, path: &str) -> Result<String> {
        let full_path = self.resolve_path(path)?;
        fs::read_to_string(&full_path)
            .context(format!("Failed to read file {}", path))
    }

    /// Delete a file
    pub fn delete_file(&self, path: &str) -> Result<String> {
        let full_path = self.resolve_path(path)?;

        if !full_path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {}", path));
        }

        fs::remove_file(&full_path)
            .context(format!("Failed to delete file {}", path))?;

        Ok(format!("✅ Deleted file: {}", path))
    }

    /// Resolve relative path to absolute path within workspace
    /// Prevents directory traversal attacks (../..)
    fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let path = Path::new(path);

        // Prevent absolute paths (security)
        if path.is_absolute() {
            return Err(anyhow::anyhow!(
                "Absolute paths not allowed. Use relative paths within workspace."
            ));
        }

        // Build full path
        let full_path = self.workspace_root.join(path);

        // Normalize path - resolve . and .. while preserving root components
        let mut normalized = PathBuf::new();
        for component in full_path.components() {
            match component {
                std::path::Component::ParentDir => {
                    if !normalized.pop() {
                        // Tried to go above root - traversal attempt
                        return Err(anyhow::anyhow!(
                            "Path escapes workspace: {}",
                            path.display()
                        ));
                    }
                }
                std::path::Component::CurDir => {
                    // Skip current dir markers
                }
                component => {
                    // Preserve Prefix, RootDir, and Normal components
                    normalized.push(component);
                }
            }
        }

        // Normalize workspace_root the same way for comparison
        let mut normalized_workspace = PathBuf::new();
        for component in self.workspace_root.components() {
            match component {
                std::path::Component::CurDir => {}
                component => normalized_workspace.push(component),
            }
        }

        if !normalized.starts_with(&normalized_workspace) {
            return Err(anyhow::anyhow!(
                "Path escapes workspace: {}",
                path.display()
            ));
        }

        Ok(full_path)
    }
}

/// Parse Claude's response for code blocks (automatic detection)
/// Returns: Vec of FileOperation
pub fn parse_code_blocks(response: &str, user_query: &str) -> Vec<FileOperation> {
    let mut operations = Vec::new();

    // Pattern: ```language\ncode\n```
    // Use a simple state machine to parse code blocks
    let lines: Vec<&str> = response.lines().collect();
    let mut i = 0;
    let mut block_index = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a code block start
        if line.starts_with("```") {
            let language = if line.len() > 3 {
                line[3..].trim()
            } else {
                "txt"
            };

            // Collect code until closing ```
            let mut code_lines = Vec::new();
            i += 1;

            while i < lines.len() && !lines[i].trim().starts_with("```") {
                code_lines.push(lines[i]);
                i += 1;
            }

            if !code_lines.is_empty() {
                let content = code_lines.join("\n");
                let filename = infer_filename(user_query, &content, language, block_index);

                operations.push(FileOperation {
                    operation_type: OperationType::Create,
                    path: filename,
                    content,
                });

                block_index += 1;
            }
        }

        i += 1;
    }

    operations
}

/// Infer filename from context
fn infer_filename(user_query: &str, content: &str, language: &str, index: usize) -> String {
    // Check if user specified filename in query
    let query_lower = user_query.to_lowercase();

    // Pattern: "create main.rs", "write to lib.rs", "make test.rs", etc.
    let filename_patterns = [
        r"main\.rs",
        r"lib\.rs",
        r"tests?\.rs",
        r"mod\.rs",
        r"cargo\.toml",
        r"\w+\.rs",
        r"\w+\.toml",
        r"\w+\.json",
        r"\w+\.md",
    ];

    for pattern in &filename_patterns {
        if let Some(matched) = query_lower.match_indices(pattern).next() {
            let start = matched.0;
            let end = start + matched.1.len();
            if end <= query_lower.len() {
                return query_lower[start..end].to_string();
            }
        }
    }

    // Check if content looks like Cargo.toml
    if content.contains("[package]") && content.contains("name =") {
        return "Cargo.toml".to_string();
    }

    // Check if content has specific patterns
    if content.contains("fn main()") {
        return "main.rs".to_string();
    }
    if content.contains("#[cfg(test)]") || content.contains("mod tests") {
        return "tests.rs".to_string();
    }
    if content.contains("pub mod") || content.contains("pub use") {
        return "lib.rs".to_string();
    }

    // Default: use language + index
    let extension = match language {
        "rust" | "rs" => "rs",
        "toml" => "toml",
        "json" => "json",
        "markdown" | "md" => "md",
        _ => "txt",
    };

    if index == 0 {
        format!("file.{}", extension)
    } else {
        format!("file{}.{}", index + 1, extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code_blocks() {
        let response = r#"Here's a hello world program:

```rust
fn main() {
    println!("Hello, World!");
}
```

And here's the Cargo.toml:

```toml
[package]
name = "hello"
version = "0.1.0"
```
"#;

        let operations = parse_code_blocks(response, "create a hello world program");

        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].path, "main.rs");
        assert!(operations[0].content.contains("fn main()"));
        assert_eq!(operations[1].path, "Cargo.toml");
        assert!(operations[1].content.contains("[package]"));
    }

    #[test]
    fn test_infer_filename_from_query() {
        let content = "pub fn hello() {}";
        let filename = infer_filename("create lib.rs with a hello function", content, "rust", 0);
        assert_eq!(filename, "lib.rs");
    }

    #[test]
    fn test_infer_filename_from_content() {
        let content = "fn main() {\n    println!(\"Hello\");\n}";
        let filename = infer_filename("create a program", content, "rust", 0);
        assert_eq!(filename, "main.rs");
    }

    #[test]
    fn test_directory_traversal_blocked() {
        use std::env;
        let temp_dir = env::temp_dir().join("test_workspace");
        let file_ops = FileOperations::new(Some(temp_dir));

        // Test various traversal attempts
        let result = file_ops.create_file("../../etc/passwd", "hacked");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("escapes workspace"));
    }

    #[test]
    fn test_absolute_paths_blocked() {
        use std::env;
        let temp_dir = env::temp_dir().join("test_workspace");
        let file_ops = FileOperations::new(Some(temp_dir));

        let result = file_ops.create_file("/etc/passwd", "hacked");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Absolute paths not allowed"));
    }

    #[test]
    fn test_simple_file_creation() {
        use std::env;
        let temp_dir = env::temp_dir().join("test_workspace_simple");
        fs::create_dir_all(&temp_dir).unwrap();
        let file_ops = FileOperations::new(Some(temp_dir.clone()));

        // This should work - simple file in workspace root
        let result = file_ops.create_file("test.txt", "Hello, World!");
        assert!(result.is_ok());

        // Verify file was created
        let file_path = temp_dir.join("test.txt");
        assert!(file_path.exists());

        // Clean up
        fs::remove_file(file_path).ok();
        fs::remove_dir(temp_dir).ok();
    }

    #[test]
    fn test_subdirectory_file_creation() {
        use std::env;
        let temp_dir = env::temp_dir().join("test_workspace_subdir");
        fs::create_dir_all(&temp_dir).unwrap();
        let file_ops = FileOperations::new(Some(temp_dir.clone()));

        // This should work - file in subdirectory
        let result = file_ops.create_file("src/main.rs", "fn main() {}");
        assert!(result.is_ok());

        // Verify file was created
        let file_path = temp_dir.join("src/main.rs");
        assert!(file_path.exists());

        // Clean up
        fs::remove_file(file_path).ok();
        fs::remove_dir(temp_dir.join("src")).ok();
        fs::remove_dir(temp_dir).ok();
    }
}
