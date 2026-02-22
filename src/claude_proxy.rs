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
}
