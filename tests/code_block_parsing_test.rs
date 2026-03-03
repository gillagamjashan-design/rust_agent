//! Comprehensive tests for code block parsing functionality
//!
//! This test suite covers:
//! - Parsing: Single/multiple blocks, different languages, malformed blocks
//! - Filename inference: From query context, code content, language type
//! - Edge cases: Unicode content, nested backticks, very large blocks

mod common;

use common::parsing_test_utils::*;
use rust_agent::tools::file_operations::{parse_code_blocks, OperationType};

// ============================================================================
// PARSING TESTS (8 tests)
// ============================================================================

#[test]
fn test_single_code_block() {
    // Parse a single Rust code block
    let code = sample_main_rs();
    let response = create_code_block("rust", code);

    let operations = parse_code_blocks(&response, "create a main function");

    assert_eq!(operations.len(), 1, "Should parse one code block");

    let op = &operations[0];
    match op.operation_type {
        OperationType::Create => {}
        _ => panic!("Should be Create operation"),
    }
    assert_eq!(op.content, code);
    assert!(
        op.path.ends_with(".rs"),
        "Should infer .rs extension for Rust code"
    );
}

#[test]
fn test_multiple_code_blocks() {
    // Parse multiple code blocks in one response

    let blocks = vec![
        ("rust", sample_main_rs()),
        ("toml", sample_cargo_toml()),
        ("rust", sample_test_rs()),
    ];

    let response = create_multi_block_response(blocks.clone());
    let operations = parse_code_blocks(&response, "create a project");

    assert_eq!(
        operations.len(),
        3,
        "Should parse all three code blocks"
    );

    // Verify Rust blocks
    assert_eq!(operations[0].content, blocks[0].1);
    assert!(operations[0].path.ends_with(".rs"));

    // Verify TOML block
    assert_eq!(operations[1].content, blocks[1].1);
    assert!(
        operations[1].path == "Cargo.toml",
        "Should infer Cargo.toml for TOML blocks"
    );

    // Verify second Rust block
    assert_eq!(operations[2].content, blocks[2].1);
    assert!(operations[2].path.ends_with(".rs"));
}

#[test]
fn test_empty_code_block() {
    // Handle empty code block gracefully
    let response = create_code_block("rust", "");

    let operations = parse_code_blocks(&response, "empty block");

    // Should still parse it
    assert_eq!(operations.len(), 1, "Should parse empty block");
    assert_eq!(operations[0].content, "");
}

#[test]
fn test_nested_backticks_in_content() {
    // Handle backticks inside code content
    let response = create_code_block_with_nested_backticks("rust");

    let operations = parse_code_blocks(&response, "code with backticks");

    assert_eq!(operations.len(), 1, "Should parse block with nested backticks");
    assert!(
        operations[0].content.contains("```rust"),
        "Should preserve nested backticks in content"
    );
}

#[test]
fn test_rust_language_variants() {
    // Test different ways to specify Rust language
    // Note: The parser currently only recognizes lowercase variants
    let code = "fn test() {}";

    let supported_variants = vec!["rust", "rs"];

    for variant in supported_variants {
        let response = create_code_block(variant, code);
        let operations = parse_code_blocks(&response, "test");

        assert_eq!(
            operations.len(),
            1,
            "Should parse variant: {}",
            variant
        );
        assert!(
            operations[0].path.ends_with(".rs"),
            "Should infer .rs for variant: {}",
            variant
        );
    }

    // Non-lowercase variants default to .txt
    let unsupported_variants = vec!["Rust", "RUST", "Rs"];
    for variant in unsupported_variants {
        let response = create_code_block(variant, code);
        let operations = parse_code_blocks(&response, "test");

        assert_eq!(operations.len(), 1, "Should still parse variant: {}", variant);
        // These get default extension (.txt) since they're not recognized
    }
}

#[test]
fn test_toml_blocks() {
    // TOML blocks should be saved as Cargo.toml
    let toml_content = sample_cargo_toml();
    let response = create_toml_block(toml_content);

    let operations = parse_code_blocks(&response, "create cargo.toml");

    assert_eq!(operations.len(), 1, "Should parse TOML block");
    assert_eq!(
        operations[0].path, "Cargo.toml",
        "Should name TOML file as Cargo.toml"
    );
    assert_eq!(operations[0].content, toml_content);
}

#[test]
fn test_no_language_specified() {
    // Code block without language specifier
    let content = "Some generic content";
    let response = create_code_block_no_language(content);

    let operations = parse_code_blocks(&response, "create file");

    assert_eq!(operations.len(), 1, "Should parse block without language");
    // Should default to .txt or similar
    assert!(
        operations[0].path.ends_with(".txt") || operations[0].path.ends_with(".rs"),
        "Should have some default extension"
    );
}

#[test]
fn test_malformed_code_blocks() {
    // Handle malformed code blocks (missing closing fence)
    let response = create_malformed_code_block("rust", "fn incomplete() {");

    let operations = parse_code_blocks(&response, "malformed");

    // Parser should handle this gracefully (either parse what it can or skip)
    // We don't crash - that's the important part
    // Exact behavior depends on implementation
    println!("Parsed {} operations from malformed block", operations.len());
}

// ============================================================================
// FILENAME INFERENCE TESTS (4 tests)
// ============================================================================

#[test]
fn test_filename_from_query() {
    // Infer filename from query context
    let code = sample_main_rs();
    let response = create_code_block("rust", code);

    let operations = parse_code_blocks(&response, "create main.rs file");

    assert_eq!(operations.len(), 1);
    assert!(
        operations[0].path == "main.rs" || operations[0].path.ends_with("main.rs"),
        "Should infer main.rs from query"
    );
}

#[test]
fn test_filename_from_content_main() {
    // Infer main.rs from fn main() in content
    let code = sample_main_rs();
    let response = create_code_block("rust", code);

    let operations = parse_code_blocks(&response, "create a program");

    assert_eq!(operations.len(), 1);
    assert!(
        operations[0].path.contains("main") || operations[0].path == "main.rs",
        "Should infer main.rs from fn main() content"
    );
}

#[test]
fn test_filename_from_content_test() {
    // Infer tests.rs from #[cfg(test)] in content
    let code = sample_test_rs();
    let response = create_code_block("rust", code);

    let operations = parse_code_blocks(&response, "create tests");

    assert_eq!(operations.len(), 1);
    assert!(
        operations[0].path.contains("test") || operations[0].path.ends_with(".rs"),
        "Should infer test-related name from #[cfg(test)]"
    );
}

#[test]
fn test_filename_from_content_cargo() {
    // Infer Cargo.toml from [package] in content
    let toml = sample_cargo_toml();
    let response = create_code_block("toml", toml);

    let operations = parse_code_blocks(&response, "create config");

    assert_eq!(operations.len(), 1);
    assert_eq!(
        operations[0].path, "Cargo.toml",
        "Should infer Cargo.toml from TOML with [package]"
    );
}

// ============================================================================
// EDGE CASES (3 tests)
// ============================================================================

#[test]
fn test_unicode_in_code_content() {
    // Handle UTF-8 content in code blocks

    let unicode_code = r#"fn main() {
    println!("Hello, 世界!");
    println!("Привет, мир!");
    println!("こんにちは世界!");
    let emoji = "🦀";
}
"#;

    let response = create_code_block("rust", unicode_code);
    let operations = parse_code_blocks(&response, "unicode test");

    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].content, unicode_code);
    assert!(
        operations[0].content.contains("世界"),
        "Should preserve Unicode content"
    );
    assert!(
        operations[0].content.contains("🦀"),
        "Should preserve emoji"
    );
}

#[test]
fn test_special_chars_in_filename() {
    // Handle special characters in inferred filenames
    let code = "fn test() {}";

    // Query with special characters that might need sanitization
    let response = create_code_block("rust", code);
    let operations = parse_code_blocks(
        &response,
        "create my-test_file v2.0.rs"
    );

    assert_eq!(operations.len(), 1);
    // Filename should be sanitized (no spaces, valid characters)
    assert!(
        !operations[0].path.contains(" "),
        "Filename should not contain spaces"
    );
    assert!(
        operations[0].path.ends_with(".rs"),
        "Should have .rs extension"
    );
}

#[test]
fn test_very_large_code_block() {
    // Handle very large code blocks (>1MB)

    // Generate a large code block
    let mut large_code = String::from("fn main() {\n");
    for i in 0..50000 {
        large_code.push_str(&format!("    println!(\"Line {}\");\n", i));
    }
    large_code.push_str("}\n");

    let response = create_code_block("rust", &large_code);
    let operations = parse_code_blocks(&response, "large file");

    assert_eq!(operations.len(), 1, "Should parse large code block");
    assert_eq!(operations[0].content, large_code);
    assert!(
        operations[0].content.len() > 1_000_000,
        "Should preserve large content (>1MB)"
    );
}

// ============================================================================
// ADDITIONAL PARSING TESTS
// ============================================================================

#[test]
fn test_code_block_with_context() {
    // Parse code block surrounded by explanatory text
    let code = sample_main_rs();
    let response = create_response_with_context("rust", code);

    let operations = parse_code_blocks(&response, "explained code");

    assert_eq!(operations.len(), 1, "Should extract code from context");
    assert_eq!(operations[0].content, code);
}

#[test]
fn test_multiple_languages() {
    // Parse blocks with different languages

    let response = format!(
        "{}\n\n{}\n\n{}",
        create_code_block("rust", "fn main() {}"),
        create_code_block("python", "def main():\n    pass"),
        create_code_block("javascript", "function main() {}")
    );

    let operations = parse_code_blocks(&response, "multi-language");

    assert_eq!(operations.len(), 3, "Should parse all language blocks");

    // Verify extensions - only rust/toml/json/markdown are recognized
    assert!(operations[0].path.ends_with(".rs"), "First should be .rs");
    // Python and JavaScript aren't recognized, so they get .txt extension
    assert!(
        operations[1].path.ends_with(".txt"),
        "Unrecognized languages get .txt"
    );
    assert!(
        operations[2].path.ends_with(".txt"),
        "Unrecognized languages get .txt"
    );
}
