# Rust Teaching Protocol - Exercise System

A comprehensive, progressive exercise system for teaching Rust through broken code challenges.

## 🎯 Quick Start

```bash
# Run the demo to see all exercises
cargo run --example exercises_demo

# Run the learning integration example
cargo run --example learning_with_exercises

# Run tests
cargo test exercises
```

## 📚 What's Included

### 25 Progressive Exercises Across 5 Stages

1. **Stage 1: Foundation** - Ownership & Memory (5 exercises)
2. **Stage 2: Borrowing** - References & Lifetimes (5 exercises)
3. **Stage 3: Patterns** - Traits & Generics (5 exercises)
4. **Stage 4: Advanced** - Smart Pointers (5 exercises)
5. **Stage 5: Systems** - Concurrency & Unsafe (5 exercises)

### Each Exercise Contains

- ❌ **Broken Code** - Intentionally fails to compile
- 🔍 **Error Code** - Specific compiler error (E0XXX)
- 💡 **Progressive Hints** - 3-4 hints to guide learning
- ⚠️ **Constraints** - What solutions cannot use
- ✅ **Working Solution** - Correct, idiomatic Rust
- 🏷️ **Concepts** - Tagged learning objectives

## 📖 Documentation

| File | Description |
|------|-------------|
| [EXERCISES.md](EXERCISES.md) | Complete documentation and usage guide |
| [EXERCISE_REFERENCE.md](EXERCISE_REFERENCE.md) | Quick reference for all 25 exercises |
| [INTEGRATION_GUIDE.md](INTEGRATION_GUIDE.md) | How to integrate with learning agent |
| [EXERCISES_SUMMARY.md](EXERCISES_SUMMARY.md) | Implementation summary |
| [EXERCISE_DIAGRAM.txt](EXERCISE_DIAGRAM.txt) | Visual architecture diagrams |

## 🚀 Usage Examples

### Get Next Exercise

```rust
use rust_agent::exercise_integration::ExerciseManager;
use std::path::PathBuf;

let progress_file = PathBuf::from("progress.json");
let manager = ExerciseManager::new(progress_file);

if let Some(exercise) = manager.get_next_exercise() {
    println!("Title: {}", exercise.title);
    println!("Broken Code:\n{}", exercise.broken_code);
    println!("Expected Error: {}", exercise.error_code);
}
```

### Track Progress

```rust
// Record a successful attempt
manager.record_attempt(
    "stage1_ex1",
    "student's solution".to_string(),
    true  // success
);

// Check if ready to advance
if manager.can_advance() {
    manager.advance_stage()?;
}

// Generate progress report
println!("{}", manager.generate_learning_report());
```

### Browse the Curriculum

```rust
use rust_agent::exercises::Curriculum;

let curriculum = Curriculum::new();

// Get all exercises for a stage
let stage1 = curriculum.get_exercises_for_stage(1);

for exercise in stage1 {
    println!("{}: {} ({:?})",
        exercise.id,
        exercise.title,
        exercise.difficulty
    );
}
```

## 📊 Statistics

- **Total Exercises:** 25
- **Total Stages:** 5
- **Error Codes Covered:** 14 different compiler errors
- **Difficulty Levels:** Beginner (8), Intermediate (10), Advanced (6), Expert (2)
- **Estimated Study Time:** 14 hours total
- **Code Size:** 55 KB exercises.rs + 11 KB integration

## 🎓 Learning Path

```
Stage 1 (Foundation)     Stage 2 (Borrowing)    Stage 3 (Patterns)
  Ownership       ──►      References    ──►      Traits
  Move Semantics  ──►      Lifetimes     ──►      Generics
  Copy Trait      ──►      Borrow Rules  ──►      Iterators
                                                       │
                                                       │
                                                       ▼
Stage 5 (Systems)        Stage 4 (Advanced)
  Threads         ◄──      Smart Pointers
  Arc + Mutex     ◄──      Rc / RefCell
  Send + Sync     ◄──      Weak / Cycles
```

## 🔧 Error Codes Covered

| Code | Description | Count |
|------|-------------|-------|
| E0382 | Use of moved value | 7 |
| E0277 | Trait not implemented | 4 |
| E0106 | Missing lifetime specifier | 3 |
| E0502 | Cannot borrow as mutable/immutable | 3 |
| E0499 | Multiple mutable borrows | 1 |
| E0596 | Cannot borrow as mutable | 1 |
| E0594 | Cannot assign to Rc | 1 |
| E0369 | Binary operation not supported | 1 |
| E0282 | Type annotations needed | 1 |
| E0308 | Type mismatch | 1 |
| E0373 | Closure may outlive function | 1 |
| LEAK | Memory leak (reference cycle) | 1 |
| DEADLOCK | Potential deadlock | 1 |
| UNDEFINED | Undefined behavior | 1 |

## 📂 File Structure

```
src/
├── exercises.rs              # Core exercise definitions (55 KB)
├── exercise_integration.rs   # Progress tracking (11 KB)
└── lib.rs                    # Module exports

examples/
├── exercises_demo.rs         # Curriculum demonstration
└── learning_with_exercises.rs # Integration example

tests/
└── exercises_test.rs         # Comprehensive tests (25+ tests)

Documentation/
├── curriculum.json           # Curriculum configuration
├── EXERCISES.md              # Main documentation
├── EXERCISE_REFERENCE.md     # Quick reference
├── INTEGRATION_GUIDE.md      # Integration instructions
├── EXERCISES_SUMMARY.md      # Implementation summary
├── EXERCISE_DIAGRAM.txt      # Visual diagrams
└── README_EXERCISES.md       # This file
```

## 🎯 Example Exercise

**Exercise 2.1: Multiple Mutable References**

```rust
// Broken Code (produces E0499)
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // Error!

    r1.push_str(" world");
    r2.push_str("!");

    println!("{}", s);
}

// Error: cannot borrow `s` as mutable more than once at a time
```

**Hints:**
1. Rust allows only one mutable reference to prevent data races
2. Use the reference, let it go out of scope, then create another
3. Or restructure the code to use only one mutable reference

**Working Solution:**
```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1 goes out of scope

    {
        let r2 = &mut s;
        r2.push_str("!");
    }

    println!("{}", s);
}
```

## 🔄 Integration with Learning Agent

The exercise system integrates with the existing learning agent:

1. **Exercise Delivery** - Present broken code challenges
2. **Progress Tracking** - Track attempts and completions
3. **Knowledge Base** - Feed successful patterns into KB
4. **Adaptive Learning** - Adjust difficulty based on performance
5. **Concept Mapping** - Link exercises to Q&A learning

See [INTEGRATION_GUIDE.md](INTEGRATION_GUIDE.md) for details.

## ✅ Tests

Run the comprehensive test suite:

```bash
# Run all exercise tests
cargo test exercises

# Run specific test
cargo test test_curriculum_has_25_exercises

# Run with output
cargo test exercises -- --nocapture
```

Tests cover:
- Curriculum structure validation
- Exercise content completeness
- Progress tracking functionality
- Stage advancement logic
- File persistence
- Error handling

## 🌟 Features

- ✅ 25 complete exercises with broken and working code
- ✅ Progressive difficulty across 5 stages
- ✅ Comprehensive error code coverage
- ✅ Detailed hints and constraints
- ✅ Progress tracking and persistence
- ✅ Stage advancement criteria
- ✅ Integration-ready with learning agent
- ✅ Extensive documentation
- ✅ Comprehensive test coverage
- ✅ JSON configuration support

## 🎯 Learning Objectives

By completing all exercises, learners will:

1. **Understand ownership** - Know when values move vs copy
2. **Master borrowing** - Use references correctly
3. **Work with lifetimes** - Annotate and understand lifetime bounds
4. **Implement traits** - Create custom trait implementations
5. **Use generics** - Write generic functions with proper bounds
6. **Handle smart pointers** - Choose and use Rc, Arc, Box, RefCell
7. **Write concurrent code** - Use threads safely with Arc + Mutex
8. **Understand unsafe** - Know when and how to use unsafe code

## 📝 Adding New Exercises

To add more exercises:

1. Add exercise definition in `src/exercises.rs`
2. Update stage `exercise_ids` vector
3. Add metadata to `curriculum.json`
4. Update documentation
5. Add tests in `tests/exercises_test.rs`

See [EXERCISES.md](EXERCISES.md) for detailed instructions.

## 🤝 Contributing

Contributions welcome! Areas for expansion:

- More exercises per stage (target 10-15 per stage)
- Challenge exercises (extra difficult)
- Real-world scenario exercises
- Performance optimization exercises
- Async/await exercises (new Stage 6)
- Macro metaprogramming exercises

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust Error Index](https://doc.rust-lang.org/error-index.html) - Error explanations
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Additional practice

## 📄 License

Part of the rust_agent project (same license).

---

**Ready to learn Rust?** Start with `cargo run --example exercises_demo` and dive into the exercises!
