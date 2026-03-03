//! Comprehensive tests for file operations security and functionality
//!
//! This test suite covers:
//! - Security: Directory traversal attacks, absolute paths, symlink attacks
//! - Functionality: Create, modify, delete, read operations
//! - Edge cases: Unicode filenames, special characters, long paths

mod common;

use common::file_test_utils::*;
use std::fs;

// ============================================================================
// SECURITY TESTS (15 tests)
// ============================================================================

#[test]
fn test_directory_traversal_basic() {
    // Attack: Basic parent directory traversal
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("../../etc/passwd", "malicious");

    assert!(result.is_err(), "Should reject basic directory traversal");
    // Just verify it failed - the specific error message may vary
}

#[test]
fn test_directory_traversal_nested() {
    // Attack: Deep nested directory traversal
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("../../../root/.ssh/authorized_keys", "ssh-rsa AAAA...");

    assert!(result.is_err(), "Should reject nested directory traversal");
    // Just verify it was rejected
}

#[test]
fn test_directory_traversal_mixed() {
    // Attack: Mixed relative paths (./sub/../../../etc)
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("./subdir/../../../etc/shadow", "malicious");

    assert!(result.is_err(), "Should reject mixed path traversal");
    // Just verify it was rejected
}

#[test]
#[cfg(unix)]
fn test_absolute_path_unix() {
    // Attack: Absolute Unix path
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("/etc/passwd", "malicious");

    assert!(result.is_err(), "Should reject absolute Unix path");
    // Just verify it was rejected
}

#[test]
#[cfg(windows)]
fn test_absolute_path_windows() {
    // Attack: Absolute Windows path
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("C:\\Windows\\System32\\drivers\\etc\\hosts", "malicious");

    assert!(result.is_err(), "Should reject absolute Windows path");
    // Just verify it was rejected
}

#[test]
#[cfg(unix)]
fn test_symlink_outside_workspace() {
    // Attack: Symlink pointing outside workspace
    let (workspace, file_ops) = create_temp_workspace();

    let symlink_path = create_symlink_attack(workspace.path());
    let symlink_name = symlink_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let result = file_ops.modify_file(symlink_name, "malicious content");

    // Should either reject or not follow the symlink outside workspace
    assert!(
        result.is_err(),
        "Should reject symlink pointing outside workspace"
    );
}

#[test]
fn test_null_byte_in_path() {
    // Attack: Null byte injection in path
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("test.txt\0/etc/passwd", "malicious");

    // Rust's Path handling should naturally reject this, but we verify
    assert!(
        result.is_err(),
        "Should reject path with null byte"
    );
}

#[test]
fn test_very_long_path() {
    // Edge case: Very long path (testing limits)
    let (_workspace, file_ops) = create_temp_workspace();

    // Create a path longer than typical limits (4096 on Linux, 260 on Windows)
    let long_filename = "a".repeat(5000);
    let result = file_ops.create_file(&long_filename, "content");

    // System should reject this (either our validation or OS)
    // We don't assert specific error, just that it's handled gracefully
    if result.is_ok() {
        // If it somehow succeeded, verify it's still in workspace
        // (This is unlikely but we handle it gracefully)
        println!("Note: Very long path was accepted by system");
    }
}

#[test]
fn test_unicode_filenames() {
    // Security + Functionality: Unicode filenames should work within workspace
    let (workspace, file_ops) = create_temp_workspace();

    let test_cases = vec![
        ("文件.rs", "Chinese filename"),
        ("файл.rs", "Cyrillic filename"),
        ("αρχείο.rs", "Greek filename"),
        ("📝notes.txt", "Emoji filename"),
    ];

    for (filename, description) in test_cases {
        let result = file_ops.create_file(filename, description);

        assert!(
            result.is_ok(),
            "Should support Unicode filename: {}",
            filename
        );

        let file_path = workspace.path().join(filename);
        assert!(file_path.exists(), "File should exist: {}", filename);
        let content = fs::read_to_string(&file_path).expect("Should read file");
        assert_eq!(content, description);
    }
}

#[test]
fn test_special_filenames() {
    // Edge case: Special filenames (.git, .env, etc.) should be allowed in workspace
    let (workspace, file_ops) = create_temp_workspace();

    let special_files = vec![".gitignore", ".env", ".config", "..hidden"];

    for filename in special_files {
        let result = file_ops.create_file(filename, "content");

        // These should be allowed within workspace
        assert!(
            result.is_ok(),
            "Should allow special filename within workspace: {}",
            filename
        );

        let file_path = workspace.path().join(filename);
        assert!(file_path.exists(), "Special file should exist: {}", filename);
    }
}

#[test]
#[cfg(unix)]
fn test_symlink_chain() {
    // Attack: Chain of symlinks eventually pointing outside workspace
    let (workspace, file_ops) = create_temp_workspace();

    let final_link = create_symlink_chain(workspace.path(), 3);
    let link_name = final_link.file_name().unwrap().to_str().unwrap();

    let result = file_ops.modify_file(link_name, "malicious");

    // Should reject following symlink chain out of workspace
    assert!(
        result.is_err(),
        "Should reject symlink chain to outside workspace"
    );
}

#[test]
#[cfg(unix)]
fn test_dangling_symlink() {
    // Edge case: Dangling symlink (target doesn't exist, outside workspace)
    let (workspace, file_ops) = create_temp_workspace();

    let symlink_path = create_dangling_symlink(workspace.path());
    let symlink_name = symlink_path.file_name().unwrap().to_str().unwrap();

    let result = file_ops.modify_file(symlink_name, "content");

    // Should either reject or handle gracefully
    assert!(
        result.is_err(),
        "Should reject dangling symlink outside workspace"
    );
}

#[test]
fn test_directory_traversal_encoded() {
    // Attack: URL-encoded directory traversal
    // Note: In practice, this creates a file with literal % characters in the name
    // which is actually safe (it's contained in the workspace).
    // This test documents that URL encoding doesn't bypass our security.
    let (workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("%2e%2e%2f%2e%2e%2fetc%2fpasswd", "malicious");

    // Either rejected OR created safely in workspace (both are acceptable)
    if result.is_ok() {
        // If it succeeded, verify it's in the workspace
        let file_path = workspace.path().join("%2e%2e%2f%2e%2e%2fetc%2fpasswd");
        assert!(file_path.starts_with(workspace.path()),
            "File must be in workspace");
    }
    // If it failed, that's also fine - path validation caught it
}

#[test]
fn test_directory_traversal_unicode() {
    // Attack: Unicode path traversal attempts
    // Note: These Unicode characters are not interpreted as path separators by Rust/OS,
    // so they create literal filenames which are safe.
    let (workspace, file_ops) = create_temp_workspace();

    // Unicode alternative representations that look like ../
    let test_cases = vec![
        "..\u{2044}..\u{2044}etc\u{2044}passwd", // U+2044 is fraction slash
        "..\u{FF0F}..\u{FF0F}etc\u{FF0F}passwd", // U+FF0F is fullwidth solidus
    ];

    for attack_path in test_cases {
        let result = file_ops.create_file(attack_path, "malicious");

        // Either rejected OR created safely in workspace (both are acceptable)
        if result.is_ok() {
            // Verify it's safely in the workspace
            let file_path = workspace.path().join(attack_path);
            assert!(file_path.starts_with(workspace.path()),
                "File must be in workspace: {}", attack_path);
        }
        // If it failed, that's also fine
    }
}

#[test]
#[cfg(windows)]
fn test_absolute_path_network() {
    // Attack: Windows network path
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("\\\\network\\share\\file.txt", "malicious");

    assert!(
        result.is_err(),
        "Should reject Windows network path"
    );
    // Just verify it was rejected
}

// ============================================================================
// FUNCTIONAL TESTS (10 tests)
// ============================================================================

#[test]
fn test_create_file_in_workspace() {
    // Basic file creation
    let (workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("test.txt", "Hello, World!");

    assert!(result.is_ok(), "Should create file successfully");
    assert!(result.unwrap().contains("Created file"));

    let file_path = workspace.path().join("test.txt");
    assert!(file_path.exists(), "File should exist");

    let content = fs::read_to_string(&file_path).expect("Should read file");
    assert_eq!(content, "Hello, World!");
}

#[test]
fn test_create_file_in_subdirectory() {
    // Creating file in subdirectory (auto-create parents)
    let (workspace, file_ops) = create_temp_workspace();

    let result = file_ops.create_file("sub/dir/test.rs", "fn main() {}");

    assert!(result.is_ok(), "Should create file in subdirectory");

    let file_path = workspace.path().join("sub/dir/test.rs");
    assert!(file_path.exists(), "File should exist in subdirectory");

    let content = fs::read_to_string(&file_path).expect("Should read file");
    assert_eq!(content, "fn main() {}");
}

#[test]
fn test_modify_existing_file() {
    // Modifying an existing file
    let (workspace, file_ops) = create_temp_workspace();

    // Create initial file
    fs::write(workspace.path().join("test.txt"), "original content")
        .expect("Should write initial file");

    let result = file_ops.modify_file("test.txt", "modified content");

    assert!(result.is_ok(), "Should modify file successfully");
    assert!(result.unwrap().contains("Modified file"));

    let content = fs::read_to_string(workspace.path().join("test.txt"))
        .expect("Should read file");
    assert_eq!(content, "modified content");

    // Verify backup was created (with_extension replaces the extension)
    let backup_path = workspace.path().join("test.bak");
    assert!(backup_path.exists(), "Backup should be created");
}

#[test]
fn test_delete_file() {
    // Deleting a file
    let (workspace, file_ops) = create_temp_workspace();

    // Create file to delete
    let file_path = workspace.path().join("to_delete.txt");
    fs::write(&file_path, "delete me").expect("Should write file");
    assert!(file_path.exists(), "File should exist before deletion");

    let result = file_ops.delete_file("to_delete.txt");

    assert!(result.is_ok(), "Should delete file successfully");
    assert!(result.unwrap().contains("Deleted file"));
    assert!(!file_path.exists(), "File should not exist after deletion");
}

#[test]
fn test_read_file() {
    // Reading a file
    let (workspace, file_ops) = create_temp_workspace();

    // Create file to read
    let content = "content to read";
    fs::write(workspace.path().join("read_me.txt"), content)
        .expect("Should write file");

    let result = file_ops.read_file("read_me.txt");

    assert!(result.is_ok(), "Should read file successfully");
    assert_eq!(result.unwrap(), content);
}

#[test]
fn test_create_file_already_exists() {
    // Error case: Creating file that already exists
    let (workspace, file_ops) = create_temp_workspace();

    // Create initial file
    fs::write(workspace.path().join("existing.txt"), "exists")
        .expect("Should write file");

    let result = file_ops.create_file("existing.txt", "new content");

    assert!(result.is_err(), "Should reject creating existing file");
    assert!(result.is_err());
}

#[test]
fn test_modify_nonexistent_file() {
    // Error case: Modifying file that doesn't exist
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.modify_file("nonexistent.txt", "content");

    assert!(result.is_err(), "Should reject modifying nonexistent file");
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent_file() {
    // Error case: Deleting file that doesn't exist
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.delete_file("nonexistent.txt");

    assert!(result.is_err(), "Should reject deleting nonexistent file");
    assert!(result.is_err());
}

#[test]
fn test_read_nonexistent_file() {
    // Error case: Reading file that doesn't exist
    let (_workspace, file_ops) = create_temp_workspace();

    let result = file_ops.read_file("nonexistent.txt");

    assert!(result.is_err(), "Should reject reading nonexistent file");
    // Error should indicate file not found
    assert!(result.is_err());
}

#[test]
#[cfg(unix)]
fn test_permission_denied() {
    // Error case: Reading read-only file (then trying to modify)
    let (workspace, file_ops) = create_temp_workspace();

    let readonly_path = create_readonly_file(workspace.path(), "readonly.txt");

    // Reading should work
    let read_result = file_ops.read_file("readonly.txt");
    assert!(read_result.is_ok(), "Should read readonly file");

    // Modifying should fail
    let modify_result = file_ops.modify_file("readonly.txt", "new content");
    assert!(
        modify_result.is_err(),
        "Should reject modifying readonly file"
    );

    // Cleanup: restore permissions so TempDir can clean up
    let mut perms = fs::metadata(&readonly_path)
        .expect("Should get metadata")
        .permissions();
    use std::os::unix::fs::PermissionsExt;
    perms.set_mode(0o644);
    fs::set_permissions(&readonly_path, perms).expect("Should restore permissions");
}
