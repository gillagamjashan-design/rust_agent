//! Test utilities for code block parsing tests

/// Creates a markdown code block with specified language and content
pub fn create_code_block(language: &str, content: &str) -> String {
    format!("```{}\n{}\n```", language, content)
}

/// Creates a markdown response with multiple code blocks
pub fn create_multi_block_response(blocks: Vec<(&str, &str)>) -> String {
    let mut response = String::new();

    for (i, (language, content)) in blocks.iter().enumerate() {
        if i > 0 {
            response.push_str("\n\nHere's another code block:\n\n");
        }
        response.push_str(&create_code_block(language, content));
    }

    response
}

/// Creates a code block with backticks in the content
pub fn create_code_block_with_nested_backticks(language: &str) -> String {
    let content = r#"fn main() {
    let code = "```rust
    fn nested() {}
    ```";
    println!("{}", code);
}"#;
    create_code_block(language, content)
}

/// Creates a malformed code block (missing closing fence)
pub fn create_malformed_code_block(language: &str, content: &str) -> String {
    format!("```{}\n{}\n", language, content) // Missing closing ```
}

/// Creates a response with text before and after code blocks
pub fn create_response_with_context(language: &str, content: &str) -> String {
    format!(
        "Here's the code you requested:\n\n{}\n\nThis code demonstrates the concept.",
        create_code_block(language, content)
    )
}

/// Creates a code block with no language specified
pub fn create_code_block_no_language(content: &str) -> String {
    format!("```\n{}\n```", content)
}

/// Creates a TOML code block
pub fn create_toml_block(content: &str) -> String {
    create_code_block("toml", content)
}

/// Creates a standard Cargo.toml content
pub fn sample_cargo_toml() -> &'static str {
    r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
"#
}

/// Creates a standard Rust main.rs content
pub fn sample_main_rs() -> &'static str {
    r#"fn main() {
    println!("Hello, world!");
}
"#
}

/// Creates Rust test code
pub fn sample_test_rs() -> &'static str {
    r#"#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
"#
}
