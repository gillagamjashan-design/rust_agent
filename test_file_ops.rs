// Standalone test for file operations
use std::path::PathBuf;

// Import the module we created
mod tools {
    pub mod file_operations;
}

use tools::file_operations::{parse_code_blocks, infer_filename};

fn main() {
    println!("Testing file operations...\n");

    // Test 1: Parse code blocks
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

    println!("Test 1: Parse code blocks");
    let operations = parse_code_blocks(response, "create a hello world program");
    println!("Found {} code blocks", operations.len());

    for (i, op) in operations.iter().enumerate() {
        println!("  Block {}: {} ({} bytes)", i + 1, op.path, op.content.len());
    }

    assert_eq!(operations.len(), 2, "Should find 2 code blocks");
    assert_eq!(operations[0].path, "main.rs", "First should be main.rs");
    assert_eq!(operations[1].path, "Cargo.toml", "Second should be Cargo.toml");
    println!("✅ Test 1 passed\n");

    // Test 2: Infer filename from query
    println!("Test 2: Infer filename from user query");
    let content = "pub fn hello() {}";
    let filename = infer_filename("create lib.rs with a hello function", content, "rust", 0);
    println!("  Inferred: {}", filename);
    assert_eq!(filename, "lib.rs", "Should infer lib.rs from query");
    println!("✅ Test 2 passed\n");

    // Test 3: Infer filename from content
    println!("Test 3: Infer filename from content");
    let content = "fn main() {\n    println!(\"Hello\");\n}";
    let filename = infer_filename("create a program", content, "rust", 0);
    println!("  Inferred: {}", filename);
    assert_eq!(filename, "main.rs", "Should infer main.rs from fn main()");
    println!("✅ Test 3 passed\n");

    // Test 4: Multiple code blocks
    println!("Test 4: Multiple code blocks in one response");
    let multi_response = r#"
First, the library:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

And the tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```
"#;

    let ops = parse_code_blocks(multi_response, "create a math library");
    println!("  Found {} code blocks", ops.len());
    for (i, op) in ops.iter().enumerate() {
        println!("    Block {}: {}", i + 1, op.path);
    }
    assert_eq!(ops.len(), 2, "Should find 2 code blocks");
    println!("✅ Test 4 passed\n");

    println!("🎉 All tests passed!");
}
