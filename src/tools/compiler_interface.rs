// Compiler interface - Interact with rustc

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct CompilerInterface;

impl CompilerInterface {
    /// Compile a Rust file
    pub fn compile(file_path: &Path) -> Result<CompileResult> {
        let output = Command::new("rustc")
            .arg(file_path)
            .arg("--error-format=json")
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(CompileResult {
            success: output.status.success(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            errors: Self::parse_errors(&stderr),
        })
    }

    /// Run cargo check
    pub fn cargo_check(workspace_path: &Path) -> Result<CompileResult> {
        let output = Command::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(workspace_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(CompileResult {
            success: output.status.success(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            errors: Self::parse_errors(&stdout),
        })
    }

    /// Run cargo build
    pub fn cargo_build(workspace_path: &Path, release: bool) -> Result<CompileResult> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build").current_dir(workspace_path);

        if release {
            cmd.arg("--release");
        }

        let output = cmd.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(CompileResult {
            success: output.status.success(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            errors: Self::parse_errors(&stderr),
        })
    }

    /// Run clippy
    pub fn clippy(workspace_path: &Path) -> Result<ClippyResult> {
        let output = Command::new("cargo")
            .arg("clippy")
            .arg("--message-format=json")
            .current_dir(workspace_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        Ok(ClippyResult {
            success: output.status.success(),
            lints: Self::parse_lints(&stdout),
        })
    }

    /// Run rustfmt
    pub fn format(file_path: &Path) -> Result<bool> {
        let output = Command::new("rustfmt").arg(file_path).output()?;

        Ok(output.status.success())
    }

    /// Parse compiler errors from output
    fn parse_errors(output: &str) -> Vec<CompilerError> {
        let mut errors = Vec::new();

        for line in output.lines() {
            // Simple error extraction (would be more sophisticated in production)
            if line.contains("error[E") || line.contains("error:") {
                errors.push(CompilerError {
                    message: line.to_string(),
                    file: None,
                    line_number: None,
                });
            }
        }

        errors
    }

    /// Parse clippy lints from output
    fn parse_lints(output: &str) -> Vec<ClippyLint> {
        let mut lints = Vec::new();

        for line in output.lines() {
            if line.contains("warning: ") {
                lints.push(ClippyLint {
                    lint_name: "unknown".to_string(),
                    message: line.to_string(),
                    file: None,
                    line_number: None,
                });
            }
        }

        lints
    }
}

/// Compile result
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub errors: Vec<CompilerError>,
}

/// Compiler error
#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub file: Option<String>,
    pub line_number: Option<usize>,
}

/// Clippy result
#[derive(Debug, Clone)]
pub struct ClippyResult {
    pub success: bool,
    pub lints: Vec<ClippyLint>,
}

/// Clippy lint
#[derive(Debug, Clone)]
pub struct ClippyLint {
    pub lint_name: String,
    pub message: String,
    pub file: Option<String>,
    pub line_number: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_errors() {
        let output = r#"
error[E0382]: use of moved value: `s`
  --> src/main.rs:5:20
error: aborting due to previous error
"#;

        let errors = CompilerInterface::parse_errors(output);
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("E0382"));
    }
}
