# Comprehensive Testing Implementation for Rust Agent Project

## Project Context

**Project:** Rusty - A Rust learning agent with SQLite knowledge database and GUI interface
**Current Version:** v12.0.2
**Language:** Rust (2021 edition)
**Architecture:** Two main components:
1. **Core library** (`rust_agent/`) - Knowledge database, Claude API integration, file operations
2. **GUI application** (`rusty_tui/`) - Native GUI built with egui/eframe

## Current State

### What Exists
- ✅ Working knowledge database system (SQLite with FTS5)
- ✅ Claude API integration via ClaudeProxy
- ✅ File operations with security validation
- ✅ Web search fallback (DuckDuckGo)
- ✅ GUI with chat interface
- ✅ Automatic file creation (v12.0.2)
- ✅ Worker thread for async operations

### What's Missing
- ❌ **No unit tests** for core library functions
- ❌ **No integration tests** for API interactions
- ❌ **No tests** for file operations security
- ❌ **No tests** for knowledge database queries
- ❌ **No tests** for code block parsing
- ❌ **No GUI tests** (would require complex setup)

## Testing Requirements

### Critical Areas That MUST Be Tested

#### 1. **Knowledge Database** (`src/knowledge/`)

**Files:**
- `src/knowledge/database.rs` - SQLite schema and connections
- `src/knowledge/loader.rs` - JSON loading
- `src/knowledge/query.rs` - FTS5 search

**Test Requirements:**
- [ ] Database initialization from JSON files
- [ ] FTS5 full-text search accuracy
- [ ] Query performance (< 50ms target)
- [ ] Concept/pattern/command retrieval
- [ ] Empty query handling
- [ ] Invalid JSON handling
- [ ] Database corruption recovery
- [ ] Concurrent query handling

**Test Data:**
- Use small subset of actual knowledge files
- Create mock JSON files for edge cases
- Test with malformed JSON
- Test with special characters in queries

#### 2. **File Operations** (`src/tools/file_operations.rs`)

**CRITICAL - Security must be validated!**

**Test Requirements:**
- [ ] **Security Tests:**
  - [ ] Reject absolute paths (`/etc/passwd`)
  - [ ] Reject directory traversal (`../../etc/passwd`)
  - [ ] Reject home directory expansion (`~/secret`)
  - [ ] Enforce workspace boundaries
  - [ ] Block special files (`/dev/null`, `/proc/`)

- [ ] **Functionality Tests:**
  - [ ] Create file in workspace
  - [ ] Modify existing file
  - [ ] Handle file already exists error
  - [ ] Handle permission denied error
  - [ ] Handle disk full error
  - [ ] Create nested directories
  - [ ] Validate file content matches input

**Test Setup:**
```rust
// Create temporary workspace for each test
use tempfile::TempDir;

#[test]
fn test_reject_absolute_paths() {
    let workspace = TempDir::new().unwrap();
    let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

    let result = file_ops.create_file("/etc/passwd", "malicious content");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Absolute paths not allowed"));
}
```

#### 3. **Code Block Parsing** (`src/tools/parse_code_blocks.rs`)

**Critical for automatic file creation!**

**Test Requirements:**
- [ ] Parse single code block with explicit filename
- [ ] Parse multiple code blocks
- [ ] Infer filename from context
- [ ] Handle code blocks without language tag
- [ ] Handle nested code blocks (in markdown)
- [ ] Extract correct content (no fence markers)
- [ ] Handle special characters in filenames
- [ ] Handle Unicode in code content

**Test Cases:**
```rust
#[test]
fn test_parse_single_code_block_with_filename() {
    let response = r#"
Here's a hello world program:

```rust:main.rs
fn main() {
    println!("Hello, world!");
}
```
"#;

    let blocks = parse_code_blocks(response, "create a hello world program");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].path, "main.rs");
    assert!(blocks[0].content.contains("Hello, world!"));
}

#[test]
fn test_infer_filename_from_query() {
    let response = r#"
```rust
fn main() {
    println!("Hello!");
}
```
"#;

    let blocks = parse_code_blocks(response, "create main.rs");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].path, "main.rs");
}
```

#### 4. **Web Search Client** (`src/web_search/duckduckgo.rs`)

**Test Requirements:**
- [ ] Parse DuckDuckGo HTML correctly
- [ ] Handle network errors gracefully
- [ ] Cache results correctly (SHA-256 keys)
- [ ] Respect cache expiry (7 days)
- [ ] URL encoding for special characters
- [ ] Handle empty search results
- [ ] Handle rate limiting
- [ ] Handle malformed HTML

**Test Setup:**
```rust
// Use mock HTTP server for testing
use mockito::mock;

#[tokio::test]
async fn test_search_returns_results() {
    let _m = mock("GET", "/html/")
        .with_status(200)
        .with_body(r#"<div class="result">Test result</div>"#)
        .create();

    let client = DuckDuckGoClient::new();
    let results = client.search("rust ownership").await.unwrap();
    assert!(!results.is_empty());
}
```

#### 5. **Cache System** (`src/cache.rs`)

**Test Requirements:**
- [ ] Generate consistent SHA-256 keys
- [ ] Save cache entries to disk
- [ ] Load cached entries
- [ ] Respect expiry time
- [ ] Handle cache directory missing
- [ ] Handle corrupted cache files
- [ ] Clean up expired entries

#### 6. **Knowledge Fetcher Tool** (`src/tools/knowledge_fetcher.rs`)

**Test Requirements:**
- [ ] Search returns relevant concepts
- [ ] Format results correctly
- [ ] Handle no results gracefully
- [ ] Confidence scoring works
- [ ] Multiple result types (concepts, patterns, commands)

### Optional Areas (Lower Priority)

#### 7. **Claude API Integration** (`src/claude_proxy.rs`)

**Note:** Requires running ClaudeProxyAPI, so use mocks

**Test Requirements:**
- [ ] Mock successful API responses
- [ ] Handle API errors (connection refused)
- [ ] Handle timeout errors
- [ ] Handle invalid JSON responses
- [ ] Handle rate limiting

#### 8. **GUI Components** (`rusty_tui/src/gui/`)

**Note:** GUI testing is complex - focus on logic only

**Test Requirements:**
- [ ] Message passing between threads
- [ ] Worker command handling
- [ ] File operation notifications
- [ ] Error state handling

## Implementation Plan

### Phase 1: Foundation (Priority 1)

**Files to create:**
```
tests/
├── common/
│   └── mod.rs          # Shared test utilities
├── knowledge_test.rs   # Database and query tests
├── file_ops_test.rs    # File operation security tests
└── parsing_test.rs     # Code block parsing tests
```

**Setup:**
```toml
# Add to Cargo.toml [dev-dependencies]
tempfile = "3.8"         # Temporary directories for file tests
mockito = "1.2"          # HTTP mocking for web search tests
proptest = "1.4"         # Property-based testing for edge cases
```

### Phase 2: Integration Tests (Priority 2)

**Files to create:**
```
tests/
├── integration_test.rs  # End-to-end workflow tests
└── cache_test.rs        # Cache system tests
```

### Phase 3: Advanced Testing (Priority 3)

**Files to create:**
```
tests/
├── web_search_test.rs   # Web search client tests
├── api_mock_test.rs     # Claude API mock tests
└── performance_test.rs  # Performance benchmarks
```

## Specific Test Implementation Examples

### Example 1: Knowledge Database Test

```rust
// tests/knowledge_test.rs
use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeQuery};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_database_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Initialize database
    let db = KnowledgeDatabase::new(&db_path).unwrap();

    // Verify tables exist
    let concept_count = db.count_concepts().unwrap();
    assert!(concept_count > 0, "Database should have concepts");
}

#[test]
fn test_fts5_search_ownership() {
    let db = setup_test_database();
    let query = KnowledgeQuery::new(db.clone());

    let results = query.search_concepts("ownership").unwrap();

    assert!(!results.is_empty(), "Should find ownership concept");
    assert!(results[0].title.contains("Ownership"));
}

#[test]
fn test_search_performance() {
    let db = setup_test_database();
    let query = KnowledgeQuery::new(db.clone());

    let start = std::time::Instant::now();
    let _ = query.search_concepts("lifetimes").unwrap();
    let duration = start.elapsed();

    assert!(duration.as_millis() < 50, "Query should complete in < 50ms");
}

fn setup_test_database() -> KnowledgeDatabase {
    // Create minimal test database with sample data
    // (implementation details)
}
```

### Example 2: File Operations Security Test

```rust
// tests/file_ops_test.rs
use rust_agent::tools::FileOperations;
use tempfile::TempDir;

#[test]
fn test_reject_absolute_path() {
    let workspace = TempDir::new().unwrap();
    let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

    let result = file_ops.create_file("/etc/passwd", "bad");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Absolute paths"));
}

#[test]
fn test_reject_directory_traversal() {
    let workspace = TempDir::new().unwrap();
    let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

    let result = file_ops.create_file("../../etc/passwd", "bad");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("traversal"));
}

#[test]
fn test_create_file_in_workspace() {
    let workspace = TempDir::new().unwrap();
    let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

    let result = file_ops.create_file("test.rs", "fn main() {}");
    assert!(result.is_ok());

    let file_path = workspace.path().join("test.rs");
    assert!(file_path.exists());

    let content = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(content, "fn main() {}");
}

#[test]
fn test_modify_existing_file() {
    let workspace = TempDir::new().unwrap();
    let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

    // Create initial file
    file_ops.create_file("test.rs", "old content").unwrap();

    // Modify it
    let result = file_ops.modify_file("test.rs", "new content");
    assert!(result.is_ok());

    let content = std::fs::read_to_string(workspace.path().join("test.rs")).unwrap();
    assert_eq!(content, "new content");
}
```

### Example 3: Code Block Parsing Test

```rust
// tests/parsing_test.rs
use rust_agent::tools::parse_code_blocks;

#[test]
fn test_parse_single_block_explicit_filename() {
    let response = r#"
Here's your code:

```rust:main.rs
fn main() {
    println!("Hello!");
}
```
"#;

    let blocks = parse_code_blocks(response, "create hello world");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].path, "main.rs");
    assert!(blocks[0].content.contains("println!"));
    assert!(!blocks[0].content.contains("```"));
}

#[test]
fn test_parse_multiple_blocks() {
    let response = r#"
Create two files:

```rust:main.rs
fn main() {}
```

```rust:lib.rs
pub fn helper() {}
```
"#;

    let blocks = parse_code_blocks(response, "create library");
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].path, "main.rs");
    assert_eq!(blocks[1].path, "lib.rs");
}

#[test]
fn test_infer_filename_from_query() {
    let response = "```rust\nfn main() {}\n```";

    let blocks = parse_code_blocks(response, "create main.rs with hello world");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].path, "main.rs");
}

#[test]
fn test_handle_no_code_blocks() {
    let response = "This is just text without code.";

    let blocks = parse_code_blocks(response, "explain ownership");
    assert_eq!(blocks.len(), 0);
}

#[test]
fn test_unicode_in_code() {
    let response = r#"
```rust:test.rs
fn main() {
    println!("Hello 世界 🦀");
}
```
"#;

    let blocks = parse_code_blocks(response, "unicode test");
    assert!(blocks[0].content.contains("世界"));
    assert!(blocks[0].content.contains("🦀"));
}
```

## Test Coverage Goals

**Minimum Coverage Targets:**
- Core library: **80%** line coverage
- File operations: **95%** line coverage (security critical)
- Knowledge database: **70%** line coverage
- Web search: **60%** line coverage (network dependent)

**Tools:**
```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage
```

## Running Tests

### Commands to implement:

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test file_ops_test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_reject_absolute_path

# Run in parallel
cargo test -- --test-threads=4

# Run with coverage
cargo tarpaulin --out Html
```

## Success Criteria

Tests are considered complete when:

- [x] All critical security tests pass
- [x] Code block parsing handles all formats
- [x] Knowledge database queries work correctly
- [x] File operations are secure and functional
- [x] Cache system works reliably
- [x] Tests run in < 30 seconds total
- [x] Coverage >= 75% for core library
- [x] No flaky tests (all deterministic)
- [x] CI/CD ready (can run in GitHub Actions)

## Edge Cases to Test

### File Operations
- Empty filename
- Filename with spaces
- Filename with special chars (!, @, #, $)
- Very long filenames (> 255 chars)
- Null bytes in content
- Binary content
- Empty content
- Very large files (> 1MB)
- Disk full scenario
- Permission denied scenario

### Code Parsing
- Code blocks without language
- Nested code blocks
- Inline code vs blocks
- Mixed languages in response
- Markdown in code comments
- Escaped backticks in code

### Database Queries
- Empty query string
- Very long queries
- Special characters (!@#$%^&*)
- SQL injection attempts
- Unicode queries
- Multiple word queries
- Partial word matches

## Property-Based Testing

Use `proptest` for generating random test cases:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_no_path_traversal(filename in "[a-zA-Z0-9_-]{1,50}\\.rs") {
        let workspace = TempDir::new().unwrap();
        let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

        // Should succeed for valid filenames
        let result = file_ops.create_file(&filename, "content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_reject_invalid_paths(path in ".*\\.\\..*") {
        let workspace = TempDir::new().unwrap();
        let file_ops = FileOperations::new(Some(workspace.path().to_path_buf()));

        // Should fail for paths with ..
        let result = file_ops.create_file(&path, "content");
        assert!(result.is_err());
    }
}
```

## Documentation Requirements

Each test should include:
```rust
/// Tests that absolute paths are rejected by FileOperations.
///
/// # Security
/// This is a critical security test. Absolute paths could allow
/// file creation outside the workspace boundary.
///
/// # Test Case
/// - Input: "/etc/passwd"
/// - Expected: Error with "Absolute paths not allowed"
#[test]
fn test_reject_absolute_path() {
    // ... test implementation
}
```

## Current Project Files Reference

**Key files that need testing:**
- `src/knowledge/database.rs` - 312 lines
- `src/knowledge/query.rs` - 156 lines
- `src/tools/file_operations.rs` - 234 lines
- `src/tools/parse_code_blocks.rs` - (need to check)
- `src/web_search/duckduckgo.rs` - (need to check)
- `src/cache.rs` - (need to check)

## Instructions for Implementation

1. **Start with security tests first** - File operations security is critical
2. **Use TempDir for all file tests** - Never use real filesystem
3. **Make tests deterministic** - No network calls, use mocks
4. **Add test utilities** - Create helper functions in `tests/common/mod.rs`
5. **Document each test** - Explain what it tests and why
6. **Run tests frequently** - After each implementation
7. **Check coverage** - Aim for 75%+ on critical code
8. **Make CI/CD ready** - Tests should work in any environment

## Testing Checklist

- [ ] Create `tests/` directory structure
- [ ] Add dev-dependencies to Cargo.toml
- [ ] Implement file operations security tests
- [ ] Implement code block parsing tests
- [ ] Implement knowledge database tests
- [ ] Implement cache system tests
- [ ] Implement web search tests (with mocks)
- [ ] Add test utilities in common module
- [ ] Document all tests with doc comments
- [ ] Run coverage report
- [ ] Fix any failing tests
- [ ] Optimize slow tests
- [ ] Add CI/CD configuration (.github/workflows/test.yml)

## Expected Output

After implementation, running `cargo test` should show:

```
running 45 tests
test file_ops_test::test_reject_absolute_path ... ok
test file_ops_test::test_reject_directory_traversal ... ok
test file_ops_test::test_create_file_in_workspace ... ok
test file_ops_test::test_modify_existing_file ... ok
test parsing_test::test_parse_single_block_explicit_filename ... ok
test parsing_test::test_parse_multiple_blocks ... ok
test parsing_test::test_infer_filename_from_query ... ok
test knowledge_test::test_database_initialization ... ok
test knowledge_test::test_fts5_search_ownership ... ok
test knowledge_test::test_search_performance ... ok
... (35 more tests)

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Summary

**Implement comprehensive testing for the Rust agent project with focus on:**
1. **Security** (file operations) - CRITICAL
2. **Functionality** (code parsing, database) - HIGH PRIORITY
3. **Integration** (API, cache, web search) - MEDIUM PRIORITY
4. **Performance** (query speed, benchmarks) - NICE TO HAVE

**Start with file operations security tests and code block parsing tests as these are critical for the v12.0.2 automatic file creation feature.**
