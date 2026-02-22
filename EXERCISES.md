# Rust Teaching Protocol - Exercise System

This document describes the comprehensive exercise system designed to teach Rust programming through progressive broken code challenges.

## Overview

The exercise system consists of 25 carefully crafted exercises across 5 stages, each focusing on specific Rust concepts. Every exercise contains intentionally broken code that produces compiler errors, helping learners understand Rust's ownership system and borrow checker.

## Architecture

### Core Components

1. **exercises.rs** - Core module containing:
   - `Exercise` struct - Individual exercise definition
   - `Curriculum` - Collection of all exercises and stages
   - `Stage` - Learning stage with advancement criteria

2. **exercise_integration.rs** - Integration layer providing:
   - `ExerciseManager` - Manages learning progress
   - `ExerciseProgress` - Tracks individual exercise attempts
   - `LearningProgress` - Overall learner progress

3. **curriculum.json** - JSON configuration with:
   - Stage definitions and metadata
   - Learning objectives
   - Progression rules
   - Resources and tips

## Exercise Structure

Each exercise includes:

```rust
Exercise {
    id: String,              // Unique identifier (e.g., "stage1_ex1")
    title: String,           // Human-readable title
    description: String,     // What the exercise teaches
    stage: u8,              // Which stage (1-5)
    broken_code: String,    // Code with intentional errors
    expected_error: String, // What error message to expect
    error_code: String,     // Compiler error code (E0XXX)
    constraints: Vec,       // What solutions cannot use
    hints: Vec,             // Progressive hints
    concepts: Vec,          // Concepts covered
    difficulty: Difficulty, // Beginner/Intermediate/Advanced/Expert
    working_solution: String, // Correct implementation
}
```

## Five Learning Stages

### Stage 1: Foundation - Ownership & Memory
**Focus:** Core ownership concepts, move semantics, stack vs heap

**Exercises:**
1. Basic Ownership Transfer (E0382)
2. Stack vs Heap - Copy Trait (E0382)
3. Function Return Ownership (E0382)
4. Vector Ownership in Loops (E0382)
5. Partial Moves in Structs (E0382)

**Concepts:**
- Ownership transfer
- Move vs Copy semantics
- Stack and heap allocation
- Drop trait
- Partial moves

### Stage 2: Borrowing & Lifetimes
**Focus:** References, borrowing rules, lifetime annotations

**Exercises:**
1. Multiple Mutable References (E0499)
2. Mixing Mutable and Immutable References (E0502)
3. Dangling References (E0106)
4. Lifetime Annotations in Structs (E0106)
5. Multiple Lifetime Parameters (E0106)

**Concepts:**
- Mutable vs immutable borrowing
- Borrow checker rules
- Lifetime annotations
- Non-lexical lifetimes
- Dangling reference prevention

### Stage 3: Patterns & Abstractions
**Focus:** Traits, generics, iterators, closures

**Exercises:**
1. Implementing Basic Traits (E0277)
2. Generic Functions with Trait Bounds (E0369)
3. Iterator Trait Implementation (E0277)
4. Closure Ownership Issues (E0502)
5. Advanced Iterator Chains (E0282)

**Concepts:**
- Trait implementation
- Generic programming
- Trait bounds
- Iterator patterns
- Closure capture modes

### Stage 4: Advanced Memory Management
**Focus:** Smart pointers, interior mutability, reference cycles

**Exercises:**
1. Reference Counting with Rc (E0382)
2. Interior Mutability with RefCell (E0596)
3. Combining Rc and RefCell (E0594)
4. Detecting Reference Cycles (LEAK)
5. Custom Smart Pointer with Deref (E0308)

**Concepts:**
- Rc for shared ownership
- RefCell for interior mutability
- Weak references
- Deref trait
- Memory leak prevention

### Stage 5: Systems Programming
**Focus:** Concurrency, thread safety, unsafe code

**Exercises:**
1. Basic Thread Safety (E0373)
2. Multiple Thread Synchronization (E0382)
3. Send and Sync Trait Bounds (E0277)
4. Deadlock Prevention (DEADLOCK)
5. Unsafe Code Contracts (UNDEFINED)

**Concepts:**
- Thread creation
- Arc + Mutex patterns
- Send and Sync traits
- Deadlock prevention
- Unsafe code contracts

## Usage Examples

### Creating the Curriculum

```rust
use rust_agent::exercises::Curriculum;

let curriculum = Curriculum::new();

// Get all exercises for Stage 1
let stage1_exercises = curriculum.get_exercises_for_stage(1);

// Get a specific exercise
let exercise = curriculum.get_exercise("stage1_ex1").unwrap();

println!("Title: {}", exercise.title);
println!("Broken Code:\n{}", exercise.broken_code);
```

### Managing Learning Progress

```rust
use rust_agent::exercise_integration::ExerciseManager;
use std::path::PathBuf;

let progress_file = PathBuf::from("progress.json");
let mut manager = ExerciseManager::new(progress_file);

// Get next exercise
if let Some(exercise) = manager.get_next_exercise() {
    println!("Next: {}", exercise.title);

    // Save exercise to file
    let output_dir = PathBuf::from("./exercises");
    manager.save_exercise_to_file(&exercise.id, &output_dir).ok();
}

// Record an attempt
manager.record_attempt(
    "stage1_ex1",
    "fn main() { /* student's code */ }".to_string(),
    true  // success
);

// Check if ready to advance
if manager.can_advance() {
    match manager.advance_stage() {
        Ok(new_stage) => println!("Advanced to stage {}", new_stage),
        Err(e) => println!("Cannot advance: {}", e),
    }
}

// Get progress summary
println!("{}", manager.get_progress_summary());
```

### Running the Demo

```bash
cargo run --example exercises_demo
```

## Advancement Criteria

Each stage has specific criteria for advancement:

- **Minimum exercises completed:** 4 out of 5
- **Required concepts:** Must demonstrate understanding of key concepts
- **Success rate:** Varies by stage (70-80%)

Example for Stage 1:
```json
{
  "min_exercises_completed": 4,
  "required_concepts": ["ownership", "move semantics"],
  "description": "Complete at least 4 exercises and demonstrate understanding of ownership and move semantics."
}
```

## Error Code Coverage

The curriculum covers these Rust compiler errors:

- **E0106** - Missing lifetime specifier (3 exercises)
- **E0277** - Trait bound not satisfied (4 exercises)
- **E0282** - Type annotations needed (1 exercise)
- **E0308** - Mismatched types (1 exercise)
- **E0369** - Binary operation not supported (1 exercise)
- **E0373** - Closure may outlive function (1 exercise)
- **E0382** - Use of moved value (7 exercises)
- **E0499** - Cannot borrow as mutable more than once (1 exercise)
- **E0502** - Cannot borrow as mutable/immutable (3 exercises)
- **E0594** - Cannot assign to Rc (1 exercise)
- **E0596** - Cannot borrow as mutable (1 exercise)
- **DEADLOCK** - Deadlock scenario (1 exercise)
- **LEAK** - Memory leak from reference cycle (1 exercise)
- **UNDEFINED** - Undefined behavior in unsafe code (1 exercise)

## Integration with Learning Agent

The exercise system integrates with the learning agent through:

1. **Question Generation:** Exercises can generate questions about broken code
2. **Progress Tracking:** Learning agent tracks which exercises are completed
3. **Adaptive Learning:** Next exercise selected based on progress
4. **Concept Reinforcement:** Related concepts from previous stages are reviewed

### Example Integration

```rust
// In the learning agent
let exercise_manager = ExerciseManager::new(progress_file);

// When student is ready for an exercise
if let Some(exercise) = exercise_manager.get_next_exercise() {
    // Present the broken code
    let question = format!(
        "Fix this code that produces {}:\n\n{}",
        exercise.error_code,
        exercise.broken_code
    );

    // Send to question agent
    question_agent.ask(question, exercise.stage).await?;
}
```

## File Structure

```
rust_agent/
├── src/
│   ├── exercises.rs              # Core exercise definitions
│   ├── exercise_integration.rs   # Progress tracking
│   └── lib.rs                    # Module exports
├── examples/
│   └── exercises_demo.rs         # Usage demonstration
├── curriculum.json               # Curriculum configuration
├── EXERCISES.md                  # This file
└── progress.json                 # Learning progress (generated)
```

## Best Practices

### For Learners

1. **Read error messages carefully** - Rust's compiler errors are educational
2. **Use hints progressively** - Try solving before looking at hints
3. **Understand, don't memorize** - Focus on why the code is broken
4. **Experiment** - Modify the code to see different errors
5. **Review solutions** - Compare your solution to the working one

### For Teachers/Integrators

1. **Start simple** - Always begin with Stage 1
2. **Don't skip stages** - Each builds on previous knowledge
3. **Allow revisiting** - Students should review earlier stages
4. **Track progress** - Use ExerciseManager for persistence
5. **Provide context** - Explain why each concept matters

## Future Enhancements

Potential additions to the exercise system:

1. **More exercises per stage** - Expand to 10-15 per stage
2. **Challenge exercises** - Extra-difficult optional exercises
3. **Real-world scenarios** - Exercises based on actual codebases
4. **Performance exercises** - Focus on optimization
5. **Async/await exercises** - Future Stage 6
6. **Macro exercises** - Advanced metaprogramming
7. **FFI exercises** - Calling C code from Rust
8. **Interactive mode** - REPL-style learning
9. **Automated testing** - Verify solutions automatically
10. **Hints system** - Progressive hint revelation

## Testing

Run the test suite:

```bash
# Test exercise creation and retrieval
cargo test -p rust_agent exercises

# Test progress tracking
cargo test -p rust_agent exercise_integration

# Run all tests
cargo test
```

## Contributing

To add new exercises:

1. Add exercise to `initialize_exercises()` in `exercises.rs`
2. Update stage `exercise_ids` list
3. Add metadata to `curriculum.json`
4. Test with `cargo test`
5. Update this documentation

## License

This exercise system is part of the rust_agent project and follows the same license.

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust Error Index](https://doc.rust-lang.org/error-index.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
