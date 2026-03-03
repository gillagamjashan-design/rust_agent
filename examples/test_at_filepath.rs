use rust_agent::file_generator::FileCreationDetector;

fn main() {
    let detector = FileCreationDetector::new();
    
    // Test response with @filepath markers
    let response = r#"I'll create a hello world program for you.

@src/main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```

This creates a simple program.

@Cargo.toml
```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"
```
"#;

    println!("=== Testing @filepath detection ===\n");
    let blocks = detector.extract_code_blocks(response);
    println!("Found {} code blocks:\n", blocks.len());
    
    for (i, block) in blocks.iter().enumerate() {
        println!("Block {}:", i + 1);
        println!("  Language: {}", block.language);
        println!("  Filename: {:?}", block.filename);
        println!("  Code: {}", block.code.lines().next().unwrap_or(""));
        println!();
    }
    
    if blocks.len() == 2 {
        println!("✅ @filepath detection working!");
    } else {
        println!("❌ Expected 2 blocks, got {}", blocks.len());
    }
}
