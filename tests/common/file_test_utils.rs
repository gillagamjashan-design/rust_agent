//! Test utilities for file operations testing

use rust_agent::tools::file_operations::FileOperations;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Creates a temporary workspace and FileOperations instance for testing
pub fn create_temp_workspace() -> (TempDir, FileOperations) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_ops = FileOperations::new(Some(temp_dir.path().to_path_buf()));
    (temp_dir, file_ops)
}

/// Asserts that a Result contains an error with a message matching the pattern
pub fn assert_error_contains<T: std::fmt::Debug>(
    result: Result<T, anyhow::Error>,
    pattern: &str,
) {
    match result {
        Ok(val) => panic!(
            "Expected error containing '{}', but got Ok({:?})",
            pattern, val
        ),
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains(pattern),
                "Error message '{}' does not contain '{}'",
                error_msg,
                pattern
            );
        }
    }
}

/// Creates a symlink attack scenario (symlink pointing outside workspace)
/// Returns the path to the created symlink
#[cfg(unix)]
pub fn create_symlink_attack(workspace: &Path) -> PathBuf {
    use std::os::unix::fs::symlink;

    let symlink_path = workspace.join("evil_link");
    let target = Path::new("/etc/passwd");

    symlink(target, &symlink_path).expect("Failed to create symlink");
    symlink_path
}

/// Creates a symlink attack scenario (Windows version)
#[cfg(windows)]
pub fn create_symlink_attack(workspace: &Path) -> PathBuf {
    use std::os::windows::fs::symlink_file;

    let symlink_path = workspace.join("evil_link");
    let target = Path::new("C:\\Windows\\System32\\config\\SAM");

    symlink_file(target, &symlink_path).expect("Failed to create symlink");
    symlink_path
}

/// Creates a chain of symlinks, with the final one pointing outside workspace
#[cfg(unix)]
pub fn create_symlink_chain(workspace: &Path, chain_length: usize) -> PathBuf {
    use std::os::unix::fs::symlink;

    let mut current_path = workspace.join("link_0");
    let final_target = Path::new("/etc/passwd");

    for i in 1..chain_length {
        let next_path = workspace.join(format!("link_{}", i));
        symlink(&current_path, &next_path).expect("Failed to create symlink");
        current_path = next_path;
    }

    // Final link points outside workspace
    let final_link = workspace.join(format!("link_{}", chain_length));
    symlink(final_target, &final_link).expect("Failed to create final symlink");

    final_link
}

/// Creates a dangling symlink (target doesn't exist, and is outside workspace)
#[cfg(unix)]
pub fn create_dangling_symlink(workspace: &Path) -> PathBuf {
    use std::os::unix::fs::symlink;

    let symlink_path = workspace.join("dangling_link");
    let target = Path::new("/nonexistent/file/outside/workspace");

    symlink(target, &symlink_path).expect("Failed to create dangling symlink");
    symlink_path
}

/// Creates a read-only file for permission testing
pub fn create_readonly_file(workspace: &Path, filename: &str) -> PathBuf {
    let file_path = workspace.join(filename);
    fs::write(&file_path, "readonly content").expect("Failed to write file");

    let mut perms = fs::metadata(&file_path)
        .expect("Failed to get metadata")
        .permissions();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0o444);
    }

    #[cfg(windows)]
    {
        perms.set_readonly(true);
    }

    fs::set_permissions(&file_path, perms).expect("Failed to set permissions");
    file_path
}
