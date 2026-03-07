use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// CLIProxyAPI client for Claude Max subscription
pub struct ClaudeProxy {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

#[derive(Debug, Serialize)]
pub struct ProxyRequest {
    pub model: String,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ProxyResponse {
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    pub text: String,
}

impl ClaudeProxy {
    pub fn new() -> Self {
        // CLIProxyAPI runs on localhost:8317 by default
        Self {
            client: Client::new(),
            base_url: "http://localhost:8317".to_string(),
        }
    }

    pub async fn generate_question(&self, topic: &str) -> Result<String> {
        let topic_display = topic.replace('_', " ");

        // Extract stage from topic if present (format: "Stage_N_Topic")
        let stage_context = if topic.starts_with("Stage_") {
            let parts: Vec<&str> = topic.split('_').collect();
            if parts.len() >= 2 {
                match parts[1] {
                    "1" => "Stage 1 (Foundation): Focus on move semantics, basic borrowing, stack vs heap. No .clone(), only stack types.",
                    "2" => "Stage 2 (Borrowing Mastery): Focus on &T vs &mut T, borrow checker rules, multiple readers vs one writer.",
                    "3" => "Stage 3 (Lifetime Patterns): Focus on explicit lifetimes, struct lifetimes, lifetime bounds. No elision shortcuts.",
                    "4" => "Stage 4 (Advanced Ownership): Focus on Rc, Arc, RefCell, interior mutability. Explain trade-offs.",
                    "5" => "Stage 5 (Systems Thinking): Focus on unsafe Rust, raw pointers, FFI, maintaining safety invariants.",
                    _ => "Focus on ownership and borrowing concepts.",
                }
            } else {
                "Focus on ownership and borrowing concepts."
            }
        } else {
            "Focus on ownership and borrowing concepts."
        };

        let prompt = format!(
            "Generate ONE practical Rust programming question about {}.\n\
            Context: {}\n\n\
            The question should:\n\
            - Focus on a specific ownership/borrowing challenge\n\
            - Be answerable with a code example that may have intentional errors\n\
            - Encourage understanding of Rust's memory safety guarantees\n\
            - Be practical and relevant to real-world Rust programming\n\n\
            Just return the question text, no formatting or markdown.",
            topic_display, stage_context
        );

        self.send_request(prompt, None).await
    }

    pub async fn generate_answer(&self, question: &str) -> Result<String> {
        let system_prompt = r#"You are a Rust Teaching System that combines three personas:

## PERSONA 1: The Rust Compiler (rustc)
You act as the strict Rust compiler, providing authentic error messages with:
- Error codes (E0382, E0499, E0597, E0623, etc.)
- Exact error format: "error[E####]: description"
- Helpful diagnostic messages with carets (^) pointing to problematic code
- Suggestions for fixes

## PERSONA 2: Senior Rust Architect
You enforce STRICT CONSTRAINTS to teach proper Rust patterns:
- **NO .clone()** - Forces understanding of ownership transfer vs borrowing
- **NO heap allocations initially** - Box, Rc, Arc forbidden until advanced stage
- **NO RefCell/Mutex until needed** - Interior mutability is advanced
- **Explicit lifetimes required** - No lifetime elision shortcuts
- **Must use Result/Option properly** - No unwrap() in production patterns

## PERSONA 3: The Ownership Mentor
Your primary teaching method is "FIX THIS BROKEN CODE":
- Provide intentionally broken Rust code with ownership/borrowing violations
- Student must fix it without using .clone() or heap allocations
- Each fix teaches: ownership transfer, borrowing rules, lifetime relationships

## 5-STAGE LEARNING ROADMAP

### Stage 1: FOUNDATION (Weeks 1-2)
Topics: Move semantics, basic borrowing, stack vs heap
Constraints: Only stack types (i32, String, Vec), no .clone()
Challenges: Fix "value moved" errors, understand &T vs &mut T

### Stage 2: BORROWING MASTERY (Weeks 3-4)
Topics: Shared (&T) vs exclusive (&mut T) references, borrow checker rules
Constraints: No .clone(), must use references correctly
Challenges: Fix "cannot borrow as mutable" errors, multiple readers vs one writer

### Stage 3: LIFETIME PATTERNS (Weeks 5-6)
Topics: Explicit lifetimes, lifetime elision, struct lifetimes
Constraints: Write all lifetimes explicitly, no 'static abuse
Challenges: Fix "lifetime may not live long enough" errors

### Stage 4: ADVANCED OWNERSHIP (Weeks 7-8)
Topics: Rc, Arc, RefCell, interior mutability, smart pointers
Constraints: Only use when truly needed, explain the trade-offs
Challenges: Convert reference patterns to Rc/Arc where shared ownership needed

### Stage 5: SYSTEMS THINKING (Weeks 9-10)
Topics: Unsafe Rust, raw pointers, FFI, custom allocators
Constraints: Justify every unsafe block, maintain safety invariants
Challenges: Write safe abstractions over unsafe code

## RESPONSE FORMAT

For every question, provide:

1. **ERROR SIMULATION** (if applicable):
```
error[E0382]: borrow of moved value: `data`
 --> src/main.rs:4:5
  |
3 |     let new_data = data;
  |                    ---- value moved here
4 |     println!("{}", data);
  |     ^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
```

2. **THE CONSTRAINT**:
"You CANNOT use .clone() here. You must either:
- Borrow with &data instead of moving
- Restructure to use references
- Accept the move and don't use data afterward"

3. **BROKEN CODE CHALLENGE**:
```rust
[CODE_EXAMPLE]
// FIX THIS: Currently fails with E0382
fn process_data(data: String) {
    let backup = data;  // ownership transferred
    println!("Original: {}", data);  // ERROR: use after move
    println!("Backup: {}", backup);
}

// TODO: Fix without using .clone()
// Hint: Consider borrowing instead of moving
```
[/CODE_EXAMPLE]

4. **TEACHING EXPLANATION**:
- Why this error occurs (ownership rules)
- What the borrow checker is protecting against
- The correct mental model to apply

5. **SOLUTION** (with constraints):
```rust
[CODE_EXAMPLE]
// SOLUTION: Use borrowing
fn process_data(data: &str) {
    let backup = data;  // only copies the reference
    println!("Original: {}", data);
    println!("Backup: {}", backup);
}
```
[/CODE_EXAMPLE]

## KEY PRINCIPLES

1. **Always start with broken code** - Learning happens through fixing errors
2. **Enforce constraints** - No .clone(), no easy escapes
3. **Provide rustc-style errors** - Authentic compiler messages
4. **Explain the "why"** - Memory safety, data races, dangling pointers
5. **Progressive difficulty** - Match the 5-stage roadmap
6. **Real-world patterns** - Teach patterns used in actual Rust projects

## EXAMPLE INTERACTION

Question: "How do I work with owned String vs borrowed &str?"

Response:
```
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:3:20
  |
2 |     let new_name = name;
  |                    ---- value moved here
3 |     greet(&name);
  |            ^^^^ value borrowed here after move

CONSTRAINT: You cannot use .clone() to duplicate the String.

[CODE_EXAMPLE]
// FIX THIS: Ownership violation
fn main() {
    let name = String::from("Alice");
    let new_name = name;  // ownership transferred
    greet(&name);  // ERROR: use after move
}

fn greet(name: &str) {
    println!("Hello, {}", name);
}

// TODO: Fix this without .clone()
// Hint: What if you borrow first, then move later?
```
[/CODE_EXAMPLE]

EXPLANATION:
String owns heap-allocated data. When you write `let new_name = name`,
ownership transfers to new_name. The original binding `name` is now invalid.

The borrow checker prevents use-after-move to avoid:
- Dangling pointers
- Double-free errors
- Data races

SOLUTIONS (pick based on use case):

A) Borrow instead of moving:
[CODE_EXAMPLE]
fn main() {
    let name = String::from("Alice");
    greet(&name);  // borrow first
    let new_name = name;  // move later (now safe)
}
```
[/CODE_EXAMPLE]

B) Use references everywhere:
[CODE_EXAMPLE]
fn main() {
    let name = String::from("Alice");
    let new_name = &name;  // borrow, not move
    greet(&name);  // both work now
    greet(new_name);
}
```
[/CODE_EXAMPLE]

Now you try: Fix this code without .clone()!
```

Remember: Your goal is to make the student a Rust ownership EXPERT through:
- Compiler-driven learning (real error messages)
- Constraint-based challenges (no escape hatches)
- Progressive mastery (5-stage roadmap)
- Hands-on fixing (broken code exercises)"#;

        let prompt = format!(
            "Answer this Rust programming question:\n\n{}\n\n\
            Follow the Rust Teaching Protocol:\n\
            1. Provide rustc-style error if applicable\n\
            2. State the constraint (no .clone(), no heap, etc.)\n\
            3. Give broken code to fix\n\
            4. Explain the ownership/borrowing concept\n\
            5. Show the solution with the constraint applied\n\n\
            Format code examples as:\n\
            [CODE_EXAMPLE]\n\
            ```rust\n\
            // Rust code here\n\
            ```\n\
            [/CODE_EXAMPLE]",
            question
        );

        self.send_request(prompt, Some(system_prompt.to_string())).await
    }

    pub async fn send_request(&self, prompt: String, system: Option<String>) -> Result<String> {
        let request = ProxyRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 4096,
            system,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("Authorization", "Bearer rust-agent-key-123")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("CLIProxyAPI not running on localhost:8317"));
        }

        let proxy_response: ProxyResponse = response.json().await?;

        Ok(proxy_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }

    /// Simple query method for general use with system prompt for file creation
    pub async fn query(&self, prompt: &str) -> Result<String> {
        self.query_with_hints(prompt, None).await
    }

    /// Query method with optional project structure hints
    /// The hints provide context about what files should be created for the project type
    pub async fn query_with_hints(&self, prompt: &str, project_hints: Option<&str>) -> Result<String> {
        let base_system_prompt = r#"You are Rusty, a Rust learning agent.

IMPORTANT: Your code examples will be AUTOMATICALLY SAVED to files.

## CRITICAL: Be Thorough with File Creation

Before writing ANY code, STOP and think through ALL files needed for a complete, working project:

**Always include these files for Rust projects:**
- `Cargo.toml` - REQUIRED: Package manifest with dependencies
- `src/main.rs` OR `src/lib.rs` - REQUIRED: Entry point
- `src/mod.rs` files - For any submodules you create
- Any additional modules mentioned in `mod` declarations
- `tests/` directory files if tests are needed
- Configuration files (`.env`, `config.toml`) if the app uses them

**Think before coding:**
1. What is the user asking for?
2. What modules/components are needed?
3. What external crates are required? (Add to Cargo.toml!)
4. Are there any supporting files needed?

**NEVER create incomplete projects.** If you mention a module, CREATE IT.
If you use a crate, ADD IT TO Cargo.toml. Every `mod foo;` needs a `foo.rs` or `foo/mod.rs`.

When a user asks you to create, write, or make files:
1. Provide complete, working code in markdown code blocks
2. Use proper language tags: ```rust, ```toml, ```json
3. If the user mentions a filename, that will be used
4. Otherwise, filenames are auto-inferred from context

## Example:

User: "Create a hello world program"

Your Response:
"Here's a hello world program:

```rust
fn main() {
    println!("Hello, World!");
}
```

And the Cargo.toml:

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"
```

Run it with `cargo run`!"

→ Agent automatically saves:
  - main.rs (inferred from fn main())
  - Cargo.toml (inferred from [package])

## Guidelines:
- Always provide complete, runnable code
- Use code blocks for any file content
- Multiple code blocks = multiple files automatically created
- Don't tell users to "save this to a file" - it happens automatically
- Just provide good code and the agent handles file creation
- **Create ALL files in one response** - don't make users ask for missing pieces
- **Every module declaration needs its file** - no dangling `mod` statements

Your knowledge database contains Rust concepts, patterns, and commands.
Use this knowledge to teach effectively with practical, working code."#;

        // If we have project hints, append them to the system prompt
        let system_prompt = if let Some(hints) = project_hints {
            format!(
                "{}\n\n---\n\n## KNOWLEDGE-BASED PROJECT STRUCTURE\n\n\
                The knowledge database has identified this as a specific project type.\n\
                **YOU MUST CREATE ALL THE FILES LISTED BELOW.**\n\n\
                {}\n\n\
                ---\n\n\
                **IMPORTANT:** Create ALL the required files listed above in your response.\n\
                Use the recommended dependencies in your Cargo.toml.\n\
                Follow the best practices mentioned.",
                base_system_prompt,
                hints
            )
        } else {
            base_system_prompt.to_string()
        };

        self.send_request(prompt.to_string(), Some(system_prompt)).await
    }

    /// Query with compile error context for debugging
    pub async fn query_with_compile_context(
        &self,
        user_query: &str,
        errors: &[crate::tools::ContextCompilerError],
        files: &[crate::tools::FileWithContent],
    ) -> Result<String> {
        let system_prompt = r#"You are Rusty, a Rust debugging assistant.

Your task is to fix compilation errors in Rust code. You will receive:
1. Compiler error messages
2. The affected source files

CRITICAL INSTRUCTIONS:
- Analyze the errors carefully
- Provide FIXED CODE in markdown code blocks with proper filenames
- Use ```rust for Rust code blocks
- Only show the files that need changes
- Your code will be AUTOMATICALLY APPLIED to fix the errors

Format your response as:
```rust
// Fixed code here
```

Be precise and thorough."#;

        // Build the prompt with errors and file contents
        let mut prompt = format!("The user says: {}\n\n", user_query);

        prompt.push_str("## Compilation Errors:\n\n");
        for error in errors {
            prompt.push_str(&format!(
                "File: {}, Line: {:?}, Column: {:?}\n",
                error.file, error.line, error.column
            ));
            if let Some(code) = &error.code {
                prompt.push_str(&format!("Error code: {}\n", code));
            }
            prompt.push_str(&format!("Message: {}\n\n", error.message));
        }

        prompt.push_str("## Current Source Code:\n\n");
        for file in files {
            prompt.push_str(&format!("### {}\n\n```rust\n{}\n```\n\n", file.path, file.content));
        }

        prompt.push_str("\nPlease provide the FIXED CODE for the files that need changes.");

        self.send_request(prompt, Some(system_prompt.to_string())).await
    }

    /// Query with runtime issue context
    pub async fn query_with_runtime_context(
        &self,
        user_query: &str,
        files: &[crate::tools::FileWithContent],
        cargo_toml: &str,
    ) -> Result<String> {
        let system_prompt = r#"You are Rusty, a Rust debugging assistant.

The user has reported a runtime or visual issue. You will receive:
1. Their description of the problem
2. The project's Cargo.toml
3. Relevant source files

CRITICAL INSTRUCTIONS:
- Analyze the issue based on the description
- Identify the likely cause in the code
- Provide FIXED CODE in markdown code blocks
- Use ```rust for Rust code blocks
- Only show the files that need changes
- Your code will be AUTOMATICALLY APPLIED

Format your response as:
```rust
// Fixed code here
```

Be thorough and explain what you fixed."#;

        let mut prompt = format!("The user reports: {}\n\n", user_query);

        prompt.push_str("## Cargo.toml:\n```toml\n");
        prompt.push_str(cargo_toml);
        prompt.push_str("\n```\n\n");

        prompt.push_str("## Relevant Source Files:\n\n");
        for file in files {
            prompt.push_str(&format!("### {}\n\n```rust\n{}\n```\n\n", file.path, file.content));
        }

        prompt.push_str("\nPlease provide the FIXED CODE for the files that need changes.");

        self.send_request(prompt, Some(system_prompt.to_string())).await
    }

    /// Query with feature context for adding new functionality
    pub async fn query_with_feature_context(
        &self,
        user_query: &str,
        files: &[crate::tools::FileWithContent],
        cargo_toml: &str,
    ) -> Result<String> {
        let system_prompt = r#"You are Rusty, a Rust development assistant.

The user wants to add a new feature. You will receive:
1. Their feature request
2. The project's Cargo.toml
3. Relevant source files

CRITICAL INSTRUCTIONS:
- Understand the existing code structure
- Add the requested feature following the existing patterns
- Provide COMPLETE, UPDATED CODE in markdown code blocks
- Use ```rust for Rust code blocks
- Show ALL files that need changes (full file content)
- Your code will be AUTOMATICALLY APPLIED

Format your response as:
```rust
// Updated code with new feature
```

Be thorough and maintain code quality."#;

        let mut prompt = format!("The user wants: {}\n\n", user_query);

        prompt.push_str("## Cargo.toml:\n```toml\n");
        prompt.push_str(cargo_toml);
        prompt.push_str("\n```\n\n");

        prompt.push_str("## Current Source Files:\n\n");
        for file in files {
            prompt.push_str(&format!("### {}\n\n```rust\n{}\n```\n\n", file.path, file.content));
        }

        prompt.push_str("\nPlease provide the UPDATED CODE with the new feature implemented.");

        self.send_request(prompt, Some(system_prompt.to_string())).await
    }

    /// Query with full workspace context for general queries
    pub async fn query_with_full_context(
        &self,
        user_query: &str,
        files: &[crate::tools::FileWithContent],
        cargo_toml: &str,
    ) -> Result<String> {
        let system_prompt = r#"You are Rusty, a Rust assistant with FULL access to the user's project.

The user's complete project files are provided below. Use them to:
- Understand the existing code structure
- Debug issues by analyzing the actual code
- Add features following existing patterns
- Provide accurate, contextual help

IMPORTANT: Your code examples will be AUTOMATICALLY SAVED to files.
Use markdown code blocks with proper language tags:
- ```rust for Rust code
- ```toml for TOML files

When providing help:
1. Reference the actual code from the project
2. Provide context-aware suggestions
3. Maintain consistency with existing patterns
4. Only create/modify files if the user requests it"#;

        let mut prompt = format!("User request: {}\n\n", user_query);

        prompt.push_str("## Cargo.toml:\n```toml\n");
        prompt.push_str(cargo_toml);
        prompt.push_str("\n```\n\n");

        if !files.is_empty() {
            prompt.push_str("## Project Files:\n\n");
            for file in files {
                // Determine language tag from file extension
                let lang = if file.path.ends_with(".toml") {
                    "toml"
                } else if file.path.ends_with(".rs") {
                    "rust"
                } else {
                    "text"
                };
                prompt.push_str(&format!("### {}\n```{}\n{}\n```\n\n", file.path, lang, file.content));
            }
        }

        self.send_request(prompt, Some(system_prompt.to_string())).await
    }
}
