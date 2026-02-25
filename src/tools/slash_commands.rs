// Slash commands - Agent capabilities

use super::types::*;
// use crate::orchestration::{WorkflowEngine, Workflow};  // TODO: Re-enable when orchestration module is added
use anyhow::Result;
use std::path::PathBuf;

pub struct SlashCommandExecutor {
    // workflow_engine: WorkflowEngine,  // TODO: Re-enable when orchestration module is added
}

impl SlashCommandExecutor {
    pub fn new() -> Self {
        Self {
            // workflow_engine: WorkflowEngine::new(),  // TODO: Re-enable when orchestration module is added
        }
    }

    /// Execute a slash command
    pub async fn execute(&mut self, command: &SlashCommand) -> Result<CommandResult> {
        match command {
            SlashCommand::Agent { task, context } => self.execute_agent(task, context.as_deref()).await,
            SlashCommand::Generate { spec, output_file } => self.execute_generate(spec, output_file).await,
            SlashCommand::Refactor { target, pattern } => self.execute_refactor(target, pattern).await,
            SlashCommand::Test { test_type } => self.execute_test(test_type).await,
            SlashCommand::Debug { error } => self.execute_debug(error.as_deref()).await,
        }
    }

    /// Parse slash command from user input
    pub fn parse(input: &str) -> Result<SlashCommand> {
        let input = input.trim();

        if !input.starts_with('/') {
            anyhow::bail!("Not a slash command");
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            anyhow::bail!("Empty command");
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "/agent" => {
                if args.is_empty() {
                    anyhow::bail!("/agent requires a task description");
                }
                Ok(SlashCommand::Agent {
                    task: args.join(" "),
                    context: None,
                })
            }
            "/generate" => {
                if args.is_empty() {
                    anyhow::bail!("/generate requires a specification");
                }
                Ok(SlashCommand::Generate {
                    spec: args.join(" "),
                    output_file: PathBuf::from("generated.rs"),
                })
            }
            "/refactor" => {
                if args.len() < 2 {
                    anyhow::bail!("/refactor requires <file> <pattern>");
                }
                Ok(SlashCommand::Refactor {
                    target: PathBuf::from(args[0]),
                    pattern: args[1..].join(" "),
                })
            }
            "/test" => {
                let test_type = if args.is_empty() {
                    TestType::All
                } else {
                    match args[0] {
                        "unit" => TestType::Unit,
                        "integration" => TestType::Integration,
                        _ => TestType::All,
                    }
                };
                Ok(SlashCommand::Test { test_type })
            }
            "/debug" => {
                Ok(SlashCommand::Debug {
                    error: if args.is_empty() {
                        None
                    } else {
                        Some(args.join(" "))
                    },
                })
            }
            _ => anyhow::bail!("Unknown command: {}", command),
        }
    }

    /// Execute /agent command
    async fn execute_agent(&mut self, task: &str, _context: Option<&str>) -> Result<CommandResult> {
        Ok(CommandResult {
            success: true,
            output: format!("Agent task: {}", task),
            details: None,
        })
    }

    /// Execute /generate command
    async fn execute_generate(&mut self, spec: &str, output_file: &PathBuf) -> Result<CommandResult> {
        // TODO: Re-implement when orchestration module is added
        Ok(CommandResult {
            success: true,
            output: format!("Generate command (spec: {}, output: {:?})", spec, output_file),
            details: Some("Workflow engine not yet implemented".to_string()),
        })
    }

    /// Execute /refactor command
    async fn execute_refactor(&mut self, target: &PathBuf, pattern: &str) -> Result<CommandResult> {
        // TODO: Re-implement when orchestration module is added
        Ok(CommandResult {
            success: true,
            output: format!("Refactor command (target: {:?}, pattern: {})", target, pattern),
            details: Some("Workflow engine not yet implemented".to_string()),
        })
    }

    /// Execute /test command
    async fn execute_test(&mut self, test_type: &TestType) -> Result<CommandResult> {
        let output = match test_type {
            TestType::Unit => "Running unit tests...".to_string(),
            TestType::Integration => "Running integration tests...".to_string(),
            TestType::All => "Running all tests...".to_string(),
        };

        Ok(CommandResult {
            success: true,
            output,
            details: None,
        })
    }

    /// Execute /debug command
    async fn execute_debug(&mut self, error: Option<&str>) -> Result<CommandResult> {
        let error_msg = error.unwrap_or("No error specified");

        // TODO: Re-implement when orchestration module is added
        Ok(CommandResult {
            success: true,
            output: format!("Debug command (error: {})", error_msg),
            details: Some("Workflow engine not yet implemented".to_string()),
        })
    }

    /// Get command help text
    pub fn help() -> String {
        r#"Available commands:
/agent <task>           - Spawn helper agent for task
/generate <spec>        - Generate code from specification
/refactor <file> <pattern> - Refactor code to pattern
/test [unit|integration|all] - Run tests
/debug [error]          - Debug error (or most recent)

Examples:
/agent implement user authentication
/generate struct User { name: String }
/refactor src/main.rs builder-pattern
/test unit
/debug E0382
"#
        .to_string()
    }
}

impl Default for SlashCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Command execution result
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub details: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_agent_command() {
        let cmd = SlashCommandExecutor::parse("/agent create a user struct").unwrap();
        match cmd {
            SlashCommand::Agent { task, .. } => {
                assert_eq!(task, "create a user struct");
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_generate_command() {
        let cmd = SlashCommandExecutor::parse("/generate struct User").unwrap();
        match cmd {
            SlashCommand::Generate { spec, .. } => {
                assert_eq!(spec, "struct User");
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_test_command() {
        let cmd = SlashCommandExecutor::parse("/test unit").unwrap();
        match cmd {
            SlashCommand::Test { test_type } => {
                assert!(matches!(test_type, TestType::Unit));
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_invalid_command() {
        let result = SlashCommandExecutor::parse("/invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_not_a_command() {
        let result = SlashCommandExecutor::parse("hello world");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_generate() {
        let mut executor = SlashCommandExecutor::new();

        let cmd = SlashCommand::Generate {
            spec: "test function".to_string(),
            output_file: PathBuf::from("test.rs"),
        };

        let result = executor.execute(&cmd).await.unwrap();
        assert!(!result.output.is_empty());
    }
}
