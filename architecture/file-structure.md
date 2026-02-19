# Project File Structure

## Directory Layout

```
rust_agent/
├── Cargo.toml                  # Project manifest
├── architecture/               # Architecture documentation
│   ├── system-overview.md
│   ├── components.md
│   ├── data-flow.md
│   ├── learning-cycle.md
│   └── file-structure.md
├── src/
│   ├── main.rs                # Entry point
│   ├── agents/                # Agent implementations
│   │   ├── mod.rs
│   │   ├── question_agent.rs  # Question generation agent
│   │   ├── answer_agent.rs    # Answer generation agent
│   │   └── learning_agent.rs  # Main learning agent
│   ├── knowledge/             # Knowledge management
│   │   ├── mod.rs
│   │   ├── parser.rs          # Q&A parsing
│   │   ├── storage.rs         # Knowledge base storage
│   │   └── patterns.rs        # Pattern recognition
│   ├── generator/             # Project generation
│   │   ├── mod.rs
│   │   ├── templates.rs       # Code templates
│   │   └── builder.rs         # Project builder
│   └── utils/                 # Utilities
│       ├── mod.rs
│       ├── file_io.rs         # File operations
│       └── config.rs          # Configuration
├── data/                      # Runtime data
│   ├── questions.txt          # Generated questions
│   ├── answers.txt            # Generated answers
│   ├── knowledge_base.json    # Learned knowledge
│   └── learning_state.json    # Current state
├── templates/                 # Project templates
│   ├── cli_app/
│   ├── web_service/
│   └── library/
└── tests/                     # Tests
    ├── integration/
    └── unit/
```

## Key File Descriptions

### Source Files

#### src/main.rs
- Entry point for the application
- Handles command-line arguments
- Orchestrates the learning cycle
- Provides user controls (start/stop)

#### src/agents/question_agent.rs
```rust
pub struct QuestionAgent {
    question_count: usize,
    topics_covered: Vec<String>,
    output_file: PathBuf,
}

impl QuestionAgent {
    pub fn generate_question(&mut self) -> Result<String>;
    pub fn write_question(&self, question: &str) -> Result<()>;
}
```

#### src/agents/answer_agent.rs
```rust
pub struct AnswerAgent {
    input_file: PathBuf,
    output_file: PathBuf,
}

impl AnswerAgent {
    pub fn read_questions(&self) -> Result<Vec<String>>;
    pub fn generate_answer(&self, question: &str) -> Result<String>;
    pub fn write_answer(&self, answer: &str) -> Result<()>;
}
```

#### src/agents/learning_agent.rs
```rust
pub struct LearningAgent {
    knowledge_base: KnowledgeBase,
    qa_pairs: Vec<QAPair>,
}

impl LearningAgent {
    pub fn process_qa_pair(&mut self, q: String, a: String) -> Result<()>;
    pub fn extract_patterns(&self) -> Vec<Pattern>;
    pub fn generate_project(&self, spec: &ProjectSpec) -> Result<Project>;
}
```

### Data Files

#### data/questions.txt
Plain text file with timestamped questions:
```
[2026-02-18 22:00:00] Q1: How do you handle command-line arguments in Rust?
[2026-02-18 22:00:15] Q2: What is the difference between String and &str?
```

#### data/answers.txt
Plain text file with detailed answers:
```
[2026-02-18 22:00:05] A1: Command-line arguments in Rust can be handled using:
1. std::env::args() for simple cases
2. clap crate for complex CLI applications
[Example code...]

[2026-02-18 22:00:20] A2: String is an owned, growable string type...
```

#### data/knowledge_base.json
Structured JSON containing all learned knowledge, patterns, and templates.

#### data/learning_state.json
Tracks current learning progress:
```json
{
  "iteration": 42,
  "qa_pairs_processed": 42,
  "last_question_time": "2026-02-18T22:30:00Z",
  "topics_covered": ["ownership", "traits", "modules"],
  "is_running": true
}
```

## Configuration

### Cargo.toml
```toml
[package]
name = "rust_agent"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
# ... other dependencies
```

## Generated Project Structure

When the Learning Agent creates a project, it follows this structure:
```
generated_project/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs or lib.rs
│   └── [additional modules]
├── tests/
└── examples/
```
