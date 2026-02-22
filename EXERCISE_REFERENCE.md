# Quick Reference: All Exercises

This is a quick reference guide for all 25 exercises in the Rust teaching curriculum.

## Stage 1: Foundation - Ownership & Memory

### Exercise 1.1: Basic Ownership Transfer
- **ID:** `stage1_ex1`
- **Error:** E0382 - borrow of moved value
- **Concept:** Basic move semantics
- **Difficulty:** Beginner
- **Key Learning:** Values are moved by default for non-Copy types

### Exercise 1.2: Stack vs Heap - Copy Trait
- **ID:** `stage1_ex2`
- **Error:** E0382 - borrow of moved value
- **Concept:** Copy vs Move types
- **Difficulty:** Beginner
- **Key Learning:** Stack types implement Copy, heap types don't

### Exercise 1.3: Function Return Ownership
- **ID:** `stage1_ex3`
- **Error:** E0382 - borrow of moved value
- **Concept:** Ownership through function calls
- **Difficulty:** Beginner
- **Key Learning:** Functions can return ownership

### Exercise 1.4: Vector Ownership in Loops
- **ID:** `stage1_ex4`
- **Error:** E0382 - borrow of moved value
- **Concept:** Iterating without moving
- **Difficulty:** Intermediate
- **Key Learning:** Use `&vec` to iterate by reference

### Exercise 1.5: Partial Moves in Structs
- **ID:** `stage1_ex5`
- **Error:** E0382 - borrow of partially moved value
- **Concept:** Partial moves
- **Difficulty:** Intermediate
- **Key Learning:** Moving one field makes the whole struct unusable

---

## Stage 2: Borrowing & Lifetimes

### Exercise 2.1: Multiple Mutable References
- **ID:** `stage2_ex1`
- **Error:** E0499 - cannot borrow as mutable more than once
- **Concept:** Mutable borrowing rules
- **Difficulty:** Beginner
- **Key Learning:** Only one mutable reference at a time

### Exercise 2.2: Mixing Mutable and Immutable References
- **ID:** `stage2_ex2`
- **Error:** E0502 - cannot borrow as mutable/immutable
- **Concept:** Borrowing exclusivity
- **Difficulty:** Beginner
- **Key Learning:** Mutable and immutable borrows are exclusive

### Exercise 2.3: Dangling References
- **ID:** `stage2_ex3`
- **Error:** E0106 - missing lifetime specifier
- **Concept:** Reference validity
- **Difficulty:** Beginner
- **Key Learning:** Cannot return reference to local variable

### Exercise 2.4: Lifetime Annotations in Structs
- **ID:** `stage2_ex4`
- **Error:** E0106 - missing lifetime specifier
- **Concept:** Struct lifetimes
- **Difficulty:** Intermediate
- **Key Learning:** Structs with references need lifetime parameters

### Exercise 2.5: Multiple Lifetime Parameters
- **ID:** `stage2_ex5`
- **Error:** E0106 - missing lifetime specifier
- **Concept:** Function lifetimes
- **Difficulty:** Intermediate
- **Key Learning:** Return lifetime tied to input lifetimes

---

## Stage 3: Patterns & Abstractions

### Exercise 3.1: Implementing Basic Traits
- **ID:** `stage3_ex1`
- **Error:** E0277 - trait not implemented
- **Concept:** Trait implementation
- **Difficulty:** Beginner
- **Key Learning:** Implement Display for custom formatting

### Exercise 3.2: Generic Functions with Trait Bounds
- **ID:** `stage3_ex2`
- **Error:** E0369 - operation not supported
- **Concept:** Trait bounds
- **Difficulty:** Intermediate
- **Key Learning:** Generic types need trait bounds for operations

### Exercise 3.3: Iterator Trait Implementation
- **ID:** `stage3_ex3`
- **Error:** E0277 - not an iterator
- **Concept:** Custom iterators
- **Difficulty:** Intermediate
- **Key Learning:** Implement Iterator trait with next() method

### Exercise 3.4: Closure Ownership Issues
- **ID:** `stage3_ex4`
- **Error:** E0502 - cannot borrow
- **Concept:** Closure captures
- **Difficulty:** Intermediate
- **Key Learning:** Closures capture their environment

### Exercise 3.5: Advanced Iterator Chains
- **ID:** `stage3_ex5`
- **Error:** E0282 - type annotations needed
- **Concept:** Iterator consumption
- **Difficulty:** Intermediate
- **Key Learning:** Iterators are lazy, use collect() to consume

---

## Stage 4: Advanced Memory Management

### Exercise 4.1: Reference Counting with Rc
- **ID:** `stage4_ex1`
- **Error:** E0382 - use of moved value
- **Concept:** Shared ownership
- **Difficulty:** Advanced
- **Key Learning:** Use Rc for multiple owners

### Exercise 4.2: Interior Mutability with RefCell
- **ID:** `stage4_ex2`
- **Error:** E0596 - cannot borrow as mutable
- **Concept:** Interior mutability
- **Difficulty:** Advanced
- **Key Learning:** RefCell allows mutation through immutable reference

### Exercise 4.3: Combining Rc and RefCell
- **ID:** `stage4_ex3`
- **Error:** E0594 - cannot assign to Rc
- **Concept:** Shared mutable state
- **Difficulty:** Advanced
- **Key Learning:** Combine Rc<RefCell<T>> for shared mutable data

### Exercise 4.4: Detecting Reference Cycles
- **ID:** `stage4_ex4`
- **Error:** LEAK - reference cycle
- **Concept:** Memory leaks
- **Difficulty:** Advanced
- **Key Learning:** Use Weak to break cycles

### Exercise 4.5: Custom Smart Pointer with Deref
- **ID:** `stage4_ex5`
- **Error:** E0308 - type mismatch
- **Concept:** Deref coercion
- **Difficulty:** Advanced
- **Key Learning:** Implement Deref for automatic dereferencing

---

## Stage 5: Systems Programming

### Exercise 5.1: Basic Thread Safety
- **ID:** `stage5_ex1`
- **Error:** E0373 - closure may outlive
- **Concept:** Thread spawning
- **Difficulty:** Advanced
- **Key Learning:** Use Arc<Mutex<T>> for shared state across threads

### Exercise 5.2: Multiple Thread Synchronization
- **ID:** `stage5_ex2`
- **Error:** E0382 - use of moved value
- **Concept:** Thread synchronization
- **Difficulty:** Advanced
- **Key Learning:** Clone Arc before moving into each thread

### Exercise 5.3: Send and Sync Trait Bounds
- **ID:** `stage5_ex3`
- **Error:** E0277 - not Send
- **Concept:** Thread safety traits
- **Difficulty:** Intermediate
- **Key Learning:** Arc is Send+Sync, Rc is not

### Exercise 5.4: Deadlock Prevention
- **ID:** `stage5_ex4`
- **Error:** DEADLOCK - potential deadlock
- **Concept:** Lock ordering
- **Difficulty:** Expert
- **Key Learning:** Always acquire locks in the same order

### Exercise 5.5: Unsafe Code Contracts
- **ID:** `stage5_ex5`
- **Error:** UNDEFINED - undefined behavior
- **Concept:** Unsafe invariants
- **Difficulty:** Expert
- **Key Learning:** Respect aliasing rules even in unsafe code

---

## Common Error Codes Reference

| Error Code | Description | Exercises |
|------------|-------------|-----------|
| E0106 | Missing lifetime specifier | 3 |
| E0277 | Trait bound not satisfied | 4 |
| E0282 | Type annotations needed | 1 |
| E0308 | Mismatched types | 1 |
| E0369 | Binary operation not supported | 1 |
| E0373 | Closure may outlive function | 1 |
| E0382 | Use of moved value | 7 |
| E0499 | Multiple mutable borrows | 1 |
| E0502 | Mixed mutable/immutable borrows | 3 |
| E0594 | Cannot assign to Rc | 1 |
| E0596 | Cannot borrow as mutable | 1 |

## Concepts Progression

### Basic → Intermediate → Advanced

1. **Ownership**
   - Basic move (1.1) → Function ownership (1.3) → Partial moves (1.5)

2. **Borrowing**
   - Mutable borrows (2.1) → Mixed borrows (2.2) → Lifetimes (2.5)

3. **Abstractions**
   - Basic traits (3.1) → Trait bounds (3.2) → Custom iterators (3.3)

4. **Smart Pointers**
   - Rc basics (4.1) → RefCell (4.2) → Combined patterns (4.3)

5. **Concurrency**
   - Basic threads (5.1) → Synchronization (5.2) → Advanced patterns (5.4)

## Study Tips

### For Each Exercise:

1. **Read the broken code** carefully before looking at hints
2. **Try to fix it** yourself first
3. **Read compiler error** - it's very helpful
4. **Use hints** progressively if stuck
5. **Compare your solution** to the working solution
6. **Understand why** the fix works

### General Strategy:

- **Stage 1-2:** Focus on understanding what the compiler is telling you
- **Stage 3:** Start thinking about design patterns
- **Stage 4:** Understand when to use each smart pointer
- **Stage 5:** Think about thread safety from the start

### Common Pitfalls:

- **Stage 1:** Trying to use values after they've been moved
- **Stage 2:** Fighting the borrow checker instead of understanding it
- **Stage 3:** Over-complicating solutions with unnecessary generics
- **Stage 4:** Using Rc when Arc is needed (or vice versa)
- **Stage 5:** Not considering race conditions and deadlocks

## Time Estimates

| Stage | Total Time | Per Exercise |
|-------|------------|--------------|
| 1 | 2 hours | 20-30 min |
| 2 | 2.5 hours | 25-35 min |
| 3 | 2.5 hours | 25-35 min |
| 4 | 3.5 hours | 35-45 min |
| 5 | 3.5 hours | 35-50 min |
| **Total** | **14 hours** | **Average: 33 min** |

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Error Index](https://doc.rust-lang.org/error-index.html) - Detailed error explanations
- [Rustlings](https://github.com/rust-lang/rustlings) - More practice exercises
- [Rust Playground](https://play.rust-lang.org/) - Try code online

## Quick Commands

```bash
# Run the demo
cargo run --example exercises_demo

# Run learning integration example
cargo run --example learning_with_exercises

# Test the exercises module
cargo test exercises

# Test exercise integration
cargo test exercise_integration

# Generate curriculum JSON
# (already included in examples/exercises_demo.rs)
```
