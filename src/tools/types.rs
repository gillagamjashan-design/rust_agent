// Tool types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlashCommand {
    Agent {
        task: String,
        context: Option<String>,
    },
    Generate {
        spec: String,
        output_file: PathBuf,
    },
    Refactor {
        target: PathBuf,
        pattern: String,
    },
    Test {
        test_type: TestType,
    },
    Debug {
        error: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    All,
}
