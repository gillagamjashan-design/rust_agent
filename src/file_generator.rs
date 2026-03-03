use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::knowledge::database::KnowledgeDatabase;

/// Detects if a Claude response contains code that should be saved to a file
pub struct FileCreationDetector {
    // Matches @filepath followed by code block: @src/main.rs\n```rust ... ```
    at_filepath_regex: Regex,
    // Matches ```rust ... ``` or ```toml ... ``` code blocks
    code_block_regex: Regex,
    // Matches comments like "// src/main.rs" or "# Cargo.toml"
    filename_hint_regex: Regex,
    // Extract struct/enum/trait names
    type_name_regex: Regex,
}

impl FileCreationDetector {
    pub fn new() -> Self {
        Self {
            // Matches @filepath followed by code block (with optional newlines)
            // Example: @src/main.rs\n```rust\ncode\n```
            at_filepath_regex: Regex::new(
                r"@([a-zA-Z0-9_./\-]+\.(?:rs|toml))\s*\n```(\w+)\n([\s\S]*?)```"
            ).unwrap(),
            // Matches ```rust ... ``` or ```toml ... ``` code blocks (fallback)
            code_block_regex: Regex::new(r"```(\w+)\n([\s\S]*?)```").unwrap(),
            // Matches comments like "// src/main.rs" or "# Cargo.toml"
            filename_hint_regex: Regex::new(r"(?:^//|^#)\s*([a-zA-Z0-9_./]+\.(rs|toml))").unwrap(),
            // Extract struct/enum/trait names
            type_name_regex: Regex::new(r"(?:struct|enum|trait)\s+([A-Z][a-zA-Z0-9_]*)").unwrap(),
        }
    }

    /// Detect file creation intent from user query
    pub fn should_create_files(&self, user_query: &str) -> bool {
        let query_lower = user_query.to_lowercase();

        // Keywords that indicate file creation intent
        let create_keywords = [
            "create", "make", "generate", "write", "add",
            "new project", "new file", "hello world", "build"
        ];

        create_keywords.iter().any(|kw| query_lower.contains(kw))
    }

    /// Extract code blocks from Claude response
    /// Priority: @filepath markers > comment hints > heuristics
    pub fn extract_code_blocks(&self, response: &str) -> Vec<CodeBlock> {
        let mut blocks = Vec::new();
        let mut used_positions: Vec<(usize, usize)> = Vec::new();

        // First pass: Look for @filepath markers (highest priority)
        for cap in self.at_filepath_regex.captures_iter(response) {
            let full_match = cap.get(0).unwrap();
            let filepath = cap.get(1).unwrap().as_str();
            let language = cap.get(2).unwrap().as_str();
            let code = cap.get(3).unwrap().as_str();

            used_positions.push((full_match.start(), full_match.end()));

            blocks.push(CodeBlock {
                language: language.to_string(),
                code: code.to_string(),
                filename: Some(PathBuf::from(filepath)),
            });
        }

        // Second pass: Find remaining code blocks without @filepath
        for cap in self.code_block_regex.captures_iter(response) {
            let full_match = cap.get(0).unwrap();

            // Skip if this code block was already captured by @filepath regex
            let overlaps = used_positions.iter().any(|(start, end)| {
                full_match.start() >= *start && full_match.start() < *end
            });

            if overlaps {
                continue;
            }

            let language = cap.get(1).unwrap().as_str();
            let code = cap.get(2).unwrap().as_str();

            // Fall back to heuristic filename detection
            let filename = self.infer_filename(code, language);

            blocks.push(CodeBlock {
                language: language.to_string(),
                code: code.to_string(),
                filename,
            });
        }

        blocks
    }

    /// Infer filename from code content and language
    fn infer_filename(&self, code: &str, language: &str) -> Option<PathBuf> {
        // 1. Check for filename comments in code
        if let Some(cap) = self.filename_hint_regex.captures(code) {
            return Some(PathBuf::from(cap.get(1).unwrap().as_str()));
        }

        // 2. Use heuristics based on language and content
        match language {
            "rust" => {
                if code.contains("fn main()") {
                    Some(PathBuf::from("src/main.rs"))
                } else if code.contains("#[cfg(test)]") || code.contains("#[test]") {
                    Some(PathBuf::from("tests/integration_test.rs"))
                } else if code.contains("pub struct") || code.contains("pub enum") {
                    // Extract struct/enum name and use as filename
                    self.extract_type_name(code)
                        .map(|name| PathBuf::from(format!("src/{}.rs", to_snake_case(&name))))
                } else {
                    Some(PathBuf::from("src/lib.rs"))
                }
            }
            "toml" => {
                if code.contains("[package]") {
                    Some(PathBuf::from("Cargo.toml"))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Extract struct/enum/trait name from Rust code
    fn extract_type_name(&self, code: &str) -> Option<String> {
        self.type_name_regex.captures(code)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }
}

impl Default for FileCreationDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
    pub filename: Option<PathBuf>,
}

/// File creator - handles actual file I/O
pub struct FileCreator {
    workspace_root: PathBuf,
}

impl FileCreator {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    /// Create or append to a file with the given content
    pub fn create_or_append_file(&self, relative_path: &Path, content: &str) -> Result<(PathBuf, bool)> {
        let full_path = self.workspace_root.join(relative_path);

        // Create parent directories if they don't exist
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        // Check if file already exists
        let existed = full_path.exists();

        if existed {
            // Append mode: add separator and new content
            let mut file = OpenOptions::new()
                .append(true)
                .open(&full_path)
                .with_context(|| format!("Failed to open file for append: {:?}", full_path))?;

            writeln!(file, "\n// --- Auto-generated by Rusty agent ---\n")?;
            write!(file, "{}", content)?;

            Ok((full_path, true))  // true = appended
        } else {
            // Create new file
            fs::write(&full_path, content)
                .with_context(|| format!("Failed to write file: {:?}", full_path))?;

            Ok((full_path, false))  // false = created new
        }
    }

    /// Create multiple files from code blocks
    pub fn create_files(&self, blocks: Vec<CodeBlock>) -> Result<Vec<FileCreationResult>> {
        let mut results = Vec::new();

        for block in blocks {
            if let Some(filename) = block.filename {
                match self.create_or_append_file(&filename, &block.code) {
                    Ok((path, appended)) => {
                        results.push(FileCreationResult {
                            path,
                            appended,
                            error: None,
                        });
                    }
                    Err(e) => {
                        results.push(FileCreationResult {
                            path: self.workspace_root.join(&filename),
                            appended: false,
                            error: Some(e.to_string()),
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct FileCreationResult {
    pub path: PathBuf,
    pub appended: bool,
    pub error: Option<String>,
}

impl FileCreationResult {
    pub fn success(&self) -> bool {
        self.error.is_none()
    }
}

/// Database-driven file creator - uses file_templates table to determine paths
/// This automatically creates files based on templates without user confirmation
pub struct AutoFileCreator {
    workspace_root: PathBuf,
    db: KnowledgeDatabase,
    detector: FileCreationDetector,
}

impl AutoFileCreator {
    pub fn new(workspace_root: PathBuf, db: KnowledgeDatabase) -> Self {
        Self {
            workspace_root,
            db,
            detector: FileCreationDetector::new(),
        }
    }

    /// Automatically create files from Claude response using database templates
    /// No user confirmation required - files are created based on template matching
    pub fn auto_create_from_response(&self, user_query: &str, response: &str) -> Result<Vec<FileCreationResult>> {
        // Check if this looks like a file creation request
        if !self.detector.should_create_files(user_query) {
            return Ok(vec![]);
        }

        // Extract code blocks from response
        let mut blocks = self.detector.extract_code_blocks(response);

        // For blocks without explicit filenames, use database templates
        for block in &mut blocks {
            if block.filename.is_none() {
                // Query database for matching template based on code content
                if let Ok(Some(template)) = self.db.get_template_for_code(&block.code, &block.language) {
                    // Use template's default filename
                    let filename = self.resolve_template_filename(&template.default_filename, &block.code);
                    block.filename = Some(PathBuf::from(filename));
                }
            }
        }

        // Create the files
        let mut results = Vec::new();
        for block in blocks {
            if let Some(filename) = block.filename {
                match self.create_file(&filename, &block.code) {
                    Ok((path, appended)) => {
                        results.push(FileCreationResult {
                            path,
                            appended,
                            error: None,
                        });
                    }
                    Err(e) => {
                        results.push(FileCreationResult {
                            path: self.workspace_root.join(&filename),
                            appended: false,
                            error: Some(e.to_string()),
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    /// Resolve template filename, replacing variables like {{module_name}}
    fn resolve_template_filename(&self, template_filename: &str, code: &str) -> String {
        let mut filename = template_filename.to_string();

        // Replace {{module_name}} with extracted struct/enum name (snake_case)
        if filename.contains("{{module_name}}") {
            let type_regex = Regex::new(r"(?:struct|enum|trait)\s+([A-Z][a-zA-Z0-9_]*)").unwrap();
            if let Some(cap) = type_regex.captures(code) {
                let type_name = cap.get(1).unwrap().as_str();
                let module_name = to_snake_case(type_name);
                filename = filename.replace("{{module_name}}", &module_name);
            } else {
                // Default to "module" if no type name found
                filename = filename.replace("{{module_name}}", "module");
            }
        }

        // Replace {{struct_name}} with the actual struct name
        if filename.contains("{{struct_name}}") {
            let type_regex = Regex::new(r"struct\s+([A-Z][a-zA-Z0-9_]*)").unwrap();
            if let Some(cap) = type_regex.captures(code) {
                let struct_name = cap.get(1).unwrap().as_str();
                filename = filename.replace("{{struct_name}}", struct_name);
            }
        }

        filename
    }

    /// Create or append to a file
    fn create_file(&self, relative_path: &Path, content: &str) -> Result<(PathBuf, bool)> {
        let full_path = self.workspace_root.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        let existed = full_path.exists();

        if existed {
            // Append mode
            let mut file = OpenOptions::new()
                .append(true)
                .open(&full_path)
                .with_context(|| format!("Failed to open file: {:?}", full_path))?;

            writeln!(file, "\n// --- Auto-generated by Rusty agent ---\n")?;
            write!(file, "{}", content)?;

            Ok((full_path, true))
        } else {
            // Create new
            fs::write(&full_path, content)
                .with_context(|| format!("Failed to write file: {:?}", full_path))?;

            Ok((full_path, false))
        }
    }
}

/// Convert CamelCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_hello_world_intent() {
        let detector = FileCreationDetector::new();
        assert!(detector.should_create_files("Create a hello world program"));
        assert!(detector.should_create_files("make a new rust project"));
        assert!(!detector.should_create_files("What is ownership?"));
    }

    #[test]
    fn test_extract_code_blocks() {
        let detector = FileCreationDetector::new();
        let response = "Here's your program:\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n";
        let blocks = detector.extract_code_blocks(response);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language, "rust");
        assert!(blocks[0].code.contains("fn main"));
    }

    #[test]
    fn test_infer_main_rs() {
        let detector = FileCreationDetector::new();
        let code = "fn main() {\n    println!(\"test\");\n}\n";
        let filename = detector.infer_filename(code, "rust");
        assert_eq!(filename, Some(PathBuf::from("src/main.rs")));
    }

    #[test]
    fn test_infer_cargo_toml() {
        let detector = FileCreationDetector::new();
        let code = "[package]\nname = \"myapp\"\nversion = \"0.1.0\"\n";
        let filename = detector.infer_filename(code, "toml");
        assert_eq!(filename, Some(PathBuf::from("Cargo.toml")));
    }

    #[test]
    fn test_create_file() {
        let temp_dir = TempDir::new().unwrap();
        let creator = FileCreator::new(temp_dir.path().to_path_buf());

        let content = "fn main() {}\n";
        let (path, appended) = creator.create_or_append_file(Path::new("src/main.rs"), content).unwrap();

        assert!(path.exists());
        assert!(!appended);  // New file, not appended
        let read_content = fs::read_to_string(&path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_append_to_existing() {
        let temp_dir = TempDir::new().unwrap();
        let creator = FileCreator::new(temp_dir.path().to_path_buf());

        // Create first file
        creator.create_or_append_file(Path::new("src/main.rs"), "fn main() {}").unwrap();

        // Append to same file
        let (path, appended) = creator.create_or_append_file(Path::new("src/main.rs"), "fn test() {}").unwrap();

        assert!(appended);  // Should have appended
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("fn main"));
        assert!(content.contains("fn test"));
        assert!(content.contains("Auto-generated by Rusty agent"));
    }

    #[test]
    fn test_extract_struct_name() {
        let detector = FileCreationDetector::new();
        let code = "pub struct User {\n    name: String,\n}\n";
        let name = detector.extract_type_name(code);
        assert_eq!(name, Some("User".to_string()));
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("HttpServer"), "http_server");
        assert_eq!(to_snake_case("ConfigManager"), "config_manager");
    }
}
