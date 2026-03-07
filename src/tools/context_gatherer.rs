use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context as AnyhowContext};

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
    pub code: Option<String>, // Error code like "E0425"
}

#[derive(Debug, Clone)]
pub struct FileWithContent {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct CompileErrorContext {
    pub compiler_errors: Vec<CompilerError>,
    pub affected_files: Vec<FileWithContent>,
    pub cargo_toml: String,
    pub raw_output: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeIssueContext {
    pub user_description: String,
    pub relevant_files: Vec<FileWithContent>,
    pub cargo_toml: String,
}

#[derive(Debug, Clone)]
pub struct FeatureContext {
    pub relevant_files: Vec<FileWithContent>,
    pub project_structure: Vec<String>,
    pub cargo_toml: String,
}

pub struct ContextGatherer;

impl ContextGatherer {
    /// Gather compile error context by running cargo check
    pub fn gather_compile_error_context(workspace: &Path) -> Result<CompileErrorContext> {
        // Run cargo check to get errors
        let output = Command::new("cargo")
            .arg("check")
            .arg("--message-format=short")
            .current_dir(workspace)
            .output()
            .context("Failed to run cargo check")?;

        let raw_output = String::from_utf8_lossy(&output.stderr).to_string();

        // Parse errors from output
        let compiler_errors = Self::parse_cargo_errors(&raw_output);

        // Read affected files
        let affected_files = Self::read_affected_files(workspace, &compiler_errors)?;

        // Read Cargo.toml
        let cargo_toml = Self::read_cargo_toml(workspace)?;

        Ok(CompileErrorContext {
            compiler_errors,
            affected_files,
            cargo_toml,
            raw_output,
        })
    }

    /// Gather runtime issue context by identifying relevant files
    pub fn gather_runtime_issue_context(
        workspace: &Path,
        query: &str,
    ) -> Result<RuntimeIssueContext> {
        // Identify relevant files based on query
        let relevant_files = Self::identify_relevant_files(workspace, query)?;

        // Read Cargo.toml
        let cargo_toml = Self::read_cargo_toml(workspace)?;

        Ok(RuntimeIssueContext {
            user_description: query.to_string(),
            relevant_files,
            cargo_toml,
        })
    }

    /// Gather feature context by reading project structure
    pub fn gather_feature_context(
        workspace: &Path,
        query: &str,
    ) -> Result<FeatureContext> {
        // Identify relevant files
        let relevant_files = Self::identify_relevant_files(workspace, query)?;

        // Get project structure
        let project_structure = Self::get_project_structure(workspace)?;

        // Read Cargo.toml
        let cargo_toml = Self::read_cargo_toml(workspace)?;

        Ok(FeatureContext {
            relevant_files,
            project_structure,
            cargo_toml,
        })
    }

    /// Parse cargo check errors from output
    fn parse_cargo_errors(output: &str) -> Vec<CompilerError> {
        let mut errors = Vec::new();

        for line in output.lines() {
            // Parse lines like: "src/main.rs:10:5: error: cannot find value `foo`"
            if line.contains("error") || line.contains("warning") {
                if let Some(error) = Self::parse_error_line(line) {
                    errors.push(error);
                }
            }
        }

        errors
    }

    fn parse_error_line(line: &str) -> Option<CompilerError> {
        // Try to parse format: "file:line:col: error[E0425]: message"
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() >= 4 {
            let file = parts[0].trim().to_string();
            let line_num = parts[1].trim().parse::<usize>().ok();
            let col_num = parts[2].trim().parse::<usize>().ok();

            // Extract error message
            let message_part = parts[3..].join(":");
            let (code, message) = Self::extract_error_code(&message_part);

            return Some(CompilerError {
                file,
                line: line_num,
                column: col_num,
                message,
                code,
            });
        }

        None
    }

    fn extract_error_code(message: &str) -> (Option<String>, String) {
        // Look for error codes like "error[E0425]:"
        if let Some(start) = message.find("error[") {
            if let Some(end) = message[start..].find(']') {
                let code = message[start + 6..start + end].to_string();
                let msg = message[start + end + 2..].trim().to_string();
                return (Some(code), msg);
            }
        }
        (None, message.trim().to_string())
    }

    /// Read files affected by compiler errors
    fn read_affected_files(
        workspace: &Path,
        errors: &[CompilerError],
    ) -> Result<Vec<FileWithContent>> {
        let mut files = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();

        for error in errors {
            let file_path = workspace.join(&error.file);

            if seen_paths.contains(&error.file) {
                continue;
            }
            seen_paths.insert(error.file.clone());

            if file_path.exists() {
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => {
                        files.push(FileWithContent {
                            path: error.file.clone(),
                            content,
                        });
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to read {}: {}", error.file, e);
                    }
                }
            }
        }

        Ok(files)
    }

    /// Identify relevant files based on query keywords
    fn identify_relevant_files(workspace: &Path, query: &str) -> Result<Vec<FileWithContent>> {
        let query_lower = query.to_lowercase();
        let mut files = Vec::new();

        // Map keywords to directories/files
        let search_patterns = if query_lower.contains("gui")
            || query_lower.contains("button")
            || query_lower.contains("layout")
            || query_lower.contains("ui") {
            vec!["src/gui/*.rs", "rusty_tui/src/gui/*.rs"]
        } else if query_lower.contains("api") || query_lower.contains("endpoint") {
            vec!["src/api/*.rs", "src/*.rs"]
        } else if query_lower.contains("database") || query_lower.contains("db") {
            vec!["src/db/*.rs", "src/*.rs"]
        } else {
            // Default: read main source files
            vec!["src/*.rs", "rusty_tui/src/*.rs"]
        };

        for pattern in search_patterns {
            if let Ok(entries) = Self::glob_files(workspace, pattern) {
                for entry in entries {
                    if let Ok(content) = std::fs::read_to_string(&entry) {
                        files.push(FileWithContent {
                            path: entry.strip_prefix(workspace)
                                .unwrap_or(&entry)
                                .to_string_lossy()
                                .to_string(),
                            content,
                        });
                    }
                }
            }
        }

        Ok(files)
    }

    fn glob_files(workspace: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
        let pattern_path = workspace.join(pattern);
        let pattern_str = pattern_path.to_string_lossy();

        let mut paths = Vec::new();
        for entry in glob::glob(&pattern_str).context("Failed to glob pattern")? {
            if let Ok(path) = entry {
                if path.is_file() {
                    paths.push(path);
                }
            }
        }
        Ok(paths)
    }

    fn get_project_structure(workspace: &Path) -> Result<Vec<String>> {
        let mut structure = Vec::new();

        if let Ok(entries) = std::fs::read_dir(workspace) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    structure.push(name);
                }
            }
        }

        Ok(structure)
    }

    pub fn read_cargo_toml(workspace: &Path) -> Result<String> {
        let cargo_path = workspace.join("Cargo.toml");
        std::fs::read_to_string(cargo_path)
            .context("Failed to read Cargo.toml")
    }

    /// Gather ALL project files for complete context using recursive globs
    pub fn gather_full_workspace_context(workspace: &Path) -> Result<Vec<FileWithContent>> {
        let mut files = Vec::new();

        // Use RECURSIVE glob patterns to find ALL files
        let patterns = vec![
            "**/*.rs",        // All Rust files recursively
            "**/Cargo.toml",  // All Cargo files
            "**/*.toml",      // All config files
        ];

        for pattern in patterns {
            let pattern_path = workspace.join(pattern);
            if let Ok(entries) = glob::glob(&pattern_path.to_string_lossy().to_string()) {
                for entry in entries.flatten() {
                    if entry.is_file() {
                        // Skip target directory
                        if entry.to_string_lossy().contains("/target/") {
                            continue;
                        }
                        // Skip files larger than 100KB to avoid token explosion
                        if let Ok(metadata) = std::fs::metadata(&entry) {
                            if metadata.len() > 100_000 {
                                eprintln!("⚠️  Skipping large file: {}", entry.display());
                                continue;
                            }
                        }
                        if let Ok(content) = std::fs::read_to_string(&entry) {
                            files.push(FileWithContent {
                                path: entry.strip_prefix(workspace)
                                    .unwrap_or(&entry)
                                    .to_string_lossy()
                                    .to_string(),
                                content,
                            });
                        }
                    }
                }
            }
        }

        eprintln!("📂 Gathered {} files from workspace", files.len());
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_line() {
        let line = "src/main.rs:10:5: error[E0425]: cannot find value `foo` in this scope";
        let error = ContextGatherer::parse_error_line(line).unwrap();

        assert_eq!(error.file, "src/main.rs");
        assert_eq!(error.line, Some(10));
        assert_eq!(error.column, Some(5));
        assert_eq!(error.code, Some("E0425".to_string()));
        assert!(error.message.contains("cannot find value"));
    }

    #[test]
    fn test_extract_error_code() {
        let msg = " error[E0425]: cannot find value `foo`";
        let (code, message) = ContextGatherer::extract_error_code(msg);

        assert_eq!(code, Some("E0425".to_string()));
        assert_eq!(message, "cannot find value `foo`");
    }
}
