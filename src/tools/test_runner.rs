// Test runner - Execute and interpret tests

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct TestRunner;

impl TestRunner {
    /// Run cargo test
    pub fn run_tests(workspace_path: &Path, test_type: TestType) -> Result<TestResult> {
        let mut cmd = Command::new("cargo");
        cmd.arg("test").current_dir(workspace_path);

        match test_type {
            TestType::Unit => {
                cmd.arg("--lib");
            }
            TestType::Integration => {
                cmd.arg("--test");
            }
            TestType::All => {
                // Run all tests (default)
            }
        }

        let output = cmd.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let (passed, failed, total) = Self::parse_test_output(&stdout);

        Ok(TestResult {
            success: output.status.success(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            passed,
            failed,
            total,
            failing_tests: Self::extract_failing_tests(&stdout),
        })
    }

    /// Run a specific test
    pub fn run_specific_test(workspace_path: &Path, test_name: &str) -> Result<TestResult> {
        let output = Command::new("cargo")
            .arg("test")
            .arg(test_name)
            .current_dir(workspace_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let (passed, failed, total) = Self::parse_test_output(&stdout);

        Ok(TestResult {
            success: output.status.success(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            passed,
            failed,
            total,
            failing_tests: Self::extract_failing_tests(&stdout),
        })
    }

    /// Parse test output to extract statistics
    fn parse_test_output(output: &str) -> (usize, usize, usize) {
        let mut passed = 0;
        let mut failed = 0;

        for line in output.lines() {
            if line.contains("test result:") {
                // Parse line like: "test result: ok. 10 passed; 0 failed; 0 ignored"
                let parts: Vec<&str> = line.split_whitespace().collect();

                for (i, part) in parts.iter().enumerate() {
                    if part == &"passed;" && i > 0 {
                        if let Ok(n) = parts[i - 1].parse::<usize>() {
                            passed = n;
                        }
                    }
                    if part == &"failed;" && i > 0 {
                        if let Ok(n) = parts[i - 1].parse::<usize>() {
                            failed = n;
                        }
                    }
                }
            }
        }

        let total = passed + failed;
        (passed, failed, total)
    }

    /// Extract failing test names
    fn extract_failing_tests(output: &str) -> Vec<String> {
        let mut failing = Vec::new();

        for line in output.lines() {
            if line.starts_with("test ") && line.contains("... FAILED") {
                if let Some(test_name) = line.split_whitespace().nth(1) {
                    failing.push(test_name.to_string());
                }
            }
        }

        failing
    }

    /// Generate test report
    pub fn generate_report(result: &TestResult) -> String {
        let mut report = String::new();

        report.push_str(&format!("Test Results\n"));
        report.push_str(&format!("============\n\n"));
        report.push_str(&format!("Total:  {}\n", result.total));
        report.push_str(&format!("Passed: {}\n", result.passed));
        report.push_str(&format!("Failed: {}\n\n", result.failed));

        if !result.failing_tests.is_empty() {
            report.push_str("Failing tests:\n");
            for test in &result.failing_tests {
                report.push_str(&format!("  - {}\n", test));
            }
        }

        report
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestType {
    Unit,
    Integration,
    All,
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub passed: usize,
    pub failed: usize,
    pub total: usize,
    pub failing_tests: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_output() {
        let output = r#"
running 10 tests
test test1 ... ok
test test2 ... FAILED

failures:

    test2

test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
"#;

        let (passed, failed, total) = TestRunner::parse_test_output(output);
        assert_eq!(passed, 9);
        assert_eq!(failed, 1);
        assert_eq!(total, 10);
    }

    #[test]
    fn test_extract_failing_tests() {
        let output = r#"
test test1 ... ok
test test2 ... FAILED
test test3 ... ok
test test4 ... FAILED
"#;

        let failing = TestRunner::extract_failing_tests(output);
        assert_eq!(failing.len(), 2);
        assert_eq!(failing[0], "test2");
        assert_eq!(failing[1], "test4");
    }
}
