# Learning Cycle

## Overview

The agent's learning system has been redesigned from sequential Q&A to efficient batch learning. This document describes the new training pipeline and continuous learning mechanisms.

---

## Training Pipeline (4-Stage Process)

### Stage 1: Bootstrap - Foundation Building (1-2 hours)

**Goal**: Establish comprehensive baseline Rust knowledge from official documentation.

#### Phase 1A: Document Loading

**Sources**:
1. The Rust Book (complete)
2. Rust Reference (language specification)
3. The Rustonomicon (unsafe Rust and advanced topics)
4. Rust API Guidelines (idiomatic patterns)
5. std library documentation (core APIs)

**Process**:
```
For each document:
1. Load markdown/HTML from filesystem
2. Parse into structured sections
3. Extract headings, code blocks, explanations
4. Build section hierarchy
```

**Output**:
- Structured document tree
- Code examples indexed
- Concept relationships mapped

#### Phase 1B: Concept Extraction

**What is extracted**:
- **Language rules** (ownership, borrowing, lifetimes)
- **Syntax patterns** (struct definitions, trait implementations)
- **Standard library APIs** (Vec, HashMap, Result, Option)
- **Best practices** (error handling, naming conventions)
- **Safety rules** (unsafe guidelines, memory safety guarantees)

**Process**:
```rust
fn extract_concepts(doc: &Document) -> Vec<Concept> {
    // 1. Identify concept markers
    //    - Section headings
    //    - "Rule:" statements
    //    - Code examples with explanations

    // 2. Parse relationships
    //    - "Building on..."
    //    - "Related to..."
    //    - Cross-references

    // 3. Categorize by complexity
    //    - Beginner (basic types, ownership)
    //    - Intermediate (traits, lifetimes)
    //    - Advanced (unsafe, macros, async)
}
```

**Output**: 10,000+ concepts stored in `data/knowledge/rust/`

#### Phase 1C: Concept Graph Building

**Purpose**: Map relationships between concepts for better retrieval.

**Graph structure**:
```
ownership
├── requires: "values", "variables"
├── enables: "memory safety"
├── related: "borrowing", "lifetimes"
└── conflicts_with: "garbage collection"
```

**Query benefits**:
- "What do I need to know before learning lifetimes?" → Prerequisites graph
- "What else is related to ownership?" → Related concepts graph
- "Why does Rust use ownership?" → Purpose/benefits graph

**Result**: Comprehensive Rust knowledge foundation (80% of language)

---

### Stage 2: Pattern Recognition - Real-World Learning (2-4 hours)

**Goal**: Learn how real Rust projects use concepts in practice.

#### Phase 2A: Crate Selection

**Top 50 crates analyzed**:
```
async/concurrency:  tokio, async-std, rayon
serialization:      serde, bincode, serde_json
web:                actix-web, axum, rocket
CLI:                clap, structopt
error handling:     anyhow, thiserror
collections:        hashbrown, smallvec
parsing:            nom, pest
crypto:             ring, rustls
...and 30+ more
```

**Selection criteria**:
- High download count (popular)
- Diverse categories (broad coverage)
- Idiomatic code (best practices)
- Well-documented (clear intent)

#### Phase 2B: Code Analysis

**What is analyzed**:
```rust
// Pattern: Builder pattern for complex initialization
pub struct ConfigBuilder {
    field1: Option<String>,
    field2: Option<i32>,
}

impl ConfigBuilder {
    pub fn new() -> Self { ... }
    pub fn field1(mut self, val: String) -> Self { ... }
    pub fn build(self) -> Result<Config, Error> { ... }
}

// Extracted pattern:
Pattern {
    id: "builder-pattern",
    category: "object-construction",
    use_case: "Complex initialization with many optional fields",
    code_template: "...",
    common_errors: ["forgetting required fields", "..."],
    fixes: ["add compile-time checks", "..."]
}
```

**Pattern categories**:
- **Construction**: builders, factories, new()
- **Ownership**: ownership transfer, clone patterns
- **Error handling**: Result propagation, custom errors
- **Concurrency**: thread safety, async patterns
- **FFI**: C interop, callback patterns
- **Macros**: declarative, procedural patterns

**Output**: 500+ patterns in `data/patterns/rust-patterns.db`

#### Phase 2C: Idiom Identification

**Rust idioms extracted**:
```rust
// Idiom: Newtype pattern for type safety
struct UserId(u64);
struct OrderId(u64);
// Now UserId ≠ OrderId at compile time

// Idiom: Using ? operator for error propagation
fn process() -> Result<T, E> {
    let x = might_fail()?;
    let y = also_might_fail()?;
    Ok(combine(x, y))
}

// Idiom: Iterator chains over explicit loops
data.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.process())
    .collect()
```

**Result**: 200+ idiomatic patterns recognized

---

### Stage 3: Error Database - Learning from Mistakes (1 hour)

**Goal**: Build comprehensive error knowledge for debugging.

#### Phase 3A: Compiler Error Collection

**Source**: Rust error index (https://doc.rust-lang.org/error-index.html)

**Errors cataloged** (1000+ total):
```
E0382: Use of moved value
E0502: Cannot borrow as mutable because it is also borrowed as immutable
E0499: Cannot borrow as mutable more than once
E0597: Borrowed value does not live long enough
E0308: Mismatched types
...
```

**For each error**:
```rust
ErrorPattern {
    error_code: "E0382",
    title: "Use of moved value",
    explanation: "This error occurs when an attempt is made to use a variable after its ownership has been moved...",
    example_code: "
        let s = String::from(\"hello\");
        let x = s;  // s moved here
        println!(\"{}\", s);  // ERROR: s no longer valid
    ",
    fix: "
        Option 1: Clone before move: let x = s.clone();
        Option 2: Borrow instead: let x = &s;
        Option 3: Move later: use s after x is done
    ",
    related_errors: ["E0507", "E0505"],
}
```

#### Phase 3B: Clippy Lint Collection

**Source**: Clippy lint list (https://rust-lang.github.io/rust-clippy/master/)

**Lint categories**:
- **Correctness**: Code likely to be wrong
- **Suspicious**: Code likely to be buggy
- **Style**: Code style issues
- **Complexity**: Unnecessarily complex code
- **Perf**: Performance issues
- **Pedantic**: Overly strict lints

**Example**:
```rust
LintPattern {
    lint: "clippy::needless_return",
    level: "style",
    explanation: "Return keyword is unnecessary at end of block",
    example_bad: "
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    ",
    example_good: "
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    ",
    auto_fix: true,
}
```

**Output**: Complete error + lint database

---

### Stage 4: Continuous Learning - Ongoing Improvement

**Goal**: Learn from actual usage and adapt knowledge.

#### Active Learning Loop

```
User Interaction
    │
    v
┌──────────────────────┐
│ Log Interaction      │
│ - Query              │
│ - Response           │
│ - User reaction      │
└──────────┬───────────┘
           │
           v
┌──────────────────────┐
│ Analyze Feedback     │
│ - Was answer helpful?│
│ - Did code work?     │
│ - User corrections?  │
└──────────┬───────────┘
           │
           v
┌──────────────────────┐
│ Update Knowledge     │
│ - Reinforce success  │
│ - Fix failures       │
│ - Add new patterns   │
└──────────┬───────────┘
           │
           v
┌──────────────────────┐
│ Weekly Batch Update  │
│ - Aggregate learnings│
│ - Update databases   │
│ - Re-train embeddings│
└──────────────────────┘
```

#### Learning Event Types

**1. User Correction**
```rust
Event::UserCorrection {
    agent_response: "Use `.to_string()`",
    user_correction: "Actually `.into()` is more idiomatic here",
    context: "Converting &str to String",
}

// Action: Mark .to_string() pattern as suboptimal
//         Promote .into() pattern in this context
```

**2. Successful Pattern**
```rust
Event::SuccessfulPattern {
    pattern: "builder-pattern",
    context: "User built complex Config struct",
    code_compiled: true,
    user_satisfied: true,
}

// Action: Reinforce builder-pattern confidence score
```

**3. New Error Encountered**
```rust
Event::UnknownError {
    error: "E0786: `…` cannot be used for generic arguments",
    context: "User tried X.method::<...>()",
    resolution: "Helped user understand",
}

// Action: Add to error database if not present
//         Update explanation if present but unhelpful
```

**4. Code Generated Successfully**
```rust
Event::CodeGenerated {
    spec: "CLI app with subcommands",
    patterns_used: ["clap-derive", "result-propagation"],
    compiled_first_try: true,
}

// Action: Reinforce used patterns
//         Log successful generation strategy
```

#### Weekly Knowledge Update Process

**Sunday night automation**:
```bash
1. Aggregate week's learning events
2. Identify:
   - Most helpful patterns (use more)
   - Unhelpful patterns (fix or remove)
   - Common user questions (add to knowledge)
   - New Rust features (update docs)
3. Re-run pattern extraction on new data
4. Update vector embeddings
5. Backup old knowledge (versioning)
6. Deploy new knowledge
```

---

## Knowledge Verification

### Automated Testing

**Concept Tests**:
```rust
#[test]
fn test_ownership_understanding() {
    let query = "What happens when you move a value?";
    let answer = agent.query(query);

    assert!(answer.contains("ownership transfer"));
    assert!(answer.contains("original binding invalid"));
    assert!(answer.contains("moved value"));
}
```

**Pattern Tests**:
```rust
#[test]
fn test_builder_pattern_recognition() {
    let code = "/* builder pattern code */";
    let patterns = agent.identify_patterns(code);

    assert!(patterns.contains(&"builder-pattern"));
}
```

**Error Tests**:
```rust
#[test]
fn test_error_explanation() {
    let error = "error[E0382]: use of moved value: `s`";
    let explanation = agent.explain_error(error);

    assert!(explanation.contains("ownership"));
    assert!(explanation.contains("moved"));
    assert!(explanation.solutions.len() >= 2);
}
```

### Quality Metrics

**Track continuously**:
```rust
struct QualityMetrics {
    answer_accuracy: f64,        // Target: >95%
    response_time: Duration,     // Target: <100ms
    code_compile_rate: f64,      // Target: >90%
    false_positive_rate: f64,    // Target: <5%
    user_satisfaction: f64,      // Target: >95%
}
```

**Monthly review**:
- Compare metrics to targets
- Identify weak areas
- Plan improvements
- Update training data

---

## Learning Progress Tracking

### Metrics Logged

```json
{
  "date": "2026-02-23",
  "stage": "continuous",
  "concepts_total": 10234,
  "patterns_total": 547,
  "errors_cataloged": 1089,
  "conversations_logged": 423,
  "average_quality_score": 0.94,
  "areas_of_strength": [
    "ownership",
    "error handling",
    "async programming"
  ],
  "areas_for_improvement": [
    "macro development",
    "unsafe code patterns"
  ]
}
```

### Knowledge Growth Over Time

```
Week 1:  Bootstrap complete - 10,000 concepts
Week 2:  Pattern library - 500 patterns
Week 3:  Error database - 1,000 errors
Week 4:  First user interactions - 20 conversations
Week 8:  Continuous learning - 200 conversations
Week 12: Refined knowledge - 12,000 concepts, 650 patterns
```

---

## Comparison: Old vs New Learning

| Aspect | Old (Q&A) | New (Batch) |
|--------|-----------|-------------|
| **Initial training** | Weeks | 4 hours |
| **Knowledge coverage** | ~10% of Rust | ~80% of Rust |
| **Pattern library** | Sequential discovery | 500+ from day 1 |
| **Error database** | Learn as encountered | 1,000+ from start |
| **Cost** | High (continuous API calls) | Low (one-time) |
| **Consistency** | Variable quality | Spec-compliant |
| **Update mechanism** | Manual retraining | Automated weekly |
| **Context retention** | Limited | Full concept graph |

---

## Future Enhancements

### Planned Improvements

1. **Rust Version Tracking**
   - Detect new Rust releases
   - Auto-update documentation
   - Flag deprecated patterns

2. **Crate Ecosystem Tracking**
   - Monitor popular crate updates
   - Update patterns when APIs change
   - Suggest modern alternatives

3. **User-Specific Learning**
   - Track each user's skill level
   - Adapt explanation complexity
   - Personalized suggestions

4. **Code Review Learning**
   - Learn from code review feedback
   - Identify project-specific patterns
   - Build organization style guides

5. **Cross-Language Patterns**
   - Recognize patterns from other languages
   - Explain Rust equivalents
   - Migration assistance

---

## Success Criteria

### Knowledge Completeness
- ✅ 100% of Rust Book concepts
- ✅ 100% of Rust Reference rules
- ✅ 500+ Rust patterns cataloged
- ✅ 1,000+ error patterns with solutions
- ✅ 200+ Git workflows
- ✅ 500+ Linux commands

### Performance
- ✅ <100ms average query response
- ✅ >95% accuracy on known patterns
- ✅ >90% code compiles first try
- ✅ <5% false positive rate
- ✅ 4-hour bootstrap time

### User Experience
- ✅ Helpful 95%+ of interactions
- ✅ Clear, understandable explanations
- ✅ Correct permission model (ask before modifying)
- ✅ Educational (teaches while helping)
- ✅ Spec-compliant suggestions

---

**Result**: An efficient, comprehensive, self-improving Rust agent that learns in hours instead of weeks, maintains high quality through continuous improvement, and provides consistent spec-compliant assistance.
