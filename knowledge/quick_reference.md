# Rust Quick Reference

A concise reference for essential Rust concepts. Use this for quick lookups during learning.

## Ownership Rules

1. Each value has a variable that's its owner
2. Only one owner at a time
3. When owner goes out of scope, value is dropped

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 moved to s2, s1 no longer valid
```

## Borrowing Rules

1. One mutable reference OR any number of immutable references
2. References must always be valid
3. Can't mix mutable and immutable references

```rust
let mut s = String::from("hello");
let r1 = &s;        // OK: immutable borrow
let r2 = &s;        // OK: multiple immutable
// let r3 = &mut s; // ERROR: can't borrow as mutable while immutable borrows exist
```

## Common Types

| Type | Description | Example |
|------|-------------|---------|
| `i32`, `u32` | Integers | `let x: i32 = 5;` |
| `f64` | Float | `let y: f64 = 3.14;` |
| `bool` | Boolean | `let b: bool = true;` |
| `char` | Character | `let c: char = 'a';` |
| `&str` | String slice | `let s: &str = "hello";` |
| `String` | Owned string | `let s = String::from("hello");` |
| `Vec<T>` | Vector | `let v: Vec<i32> = vec![1, 2, 3];` |
| `Option<T>` | Optional value | `let x: Option<i32> = Some(5);` |
| `Result<T, E>` | Success or error | `let r: Result<i32, String> = Ok(5);` |

## Collections

### Vec
```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
// or
let v = vec![1, 2, 3];
```

### HashMap
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", "value");
let value = map.get("key");
```

### String
```rust
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');
```

## Error Handling

### Option
```rust
fn find(needle: i32) -> Option<i32> {
    if needle > 0 {
        Some(needle)
    } else {
        None
    }
}

// Using Option
match find(5) {
    Some(n) => println!("Found: {}", n),
    None => println!("Not found"),
}

// Or use if let
if let Some(n) = find(5) {
    println!("Found: {}", n);
}

// Or unwrap (panics if None)
let n = find(5).unwrap();
```

### Result
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Using Result with ?
fn process() -> Result<(), String> {
    let result = divide(10.0, 2.0)?; // returns Err early if error
    Ok(())
}
```

## Pattern Matching

```rust
match value {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),
    3..=10 => println!("three through ten"),
    _ => println!("something else"),
}

// if let for single pattern
if let Some(x) = optional_value {
    println!("{}", x);
}

// Destructuring
let (x, y) = (1, 2);

struct Point { x: i32, y: i32 }
let Point { x, y } = point;
```

## Traits

```rust
// Define trait
trait Summary {
    fn summarize(&self) -> String;
}

// Implement trait
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

// Trait bounds
fn print_summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}
```

## Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}
```

## Lifetimes

```rust
// Function with lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

## Smart Pointers

### Box - heap allocation
```rust
let b = Box::new(5);
```

### Rc - reference counting (single-threaded)
```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
```

### Arc - atomic reference counting (multi-threaded)
```rust
use std::sync::Arc;

let a = Arc::new(5);
let b = Arc::clone(&a);
```

### RefCell - interior mutability (single-threaded)
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;
```

### Mutex - mutual exclusion (multi-threaded)
```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num += 1;
}
```

## Iterators

```rust
let v = vec![1, 2, 3];

// Iterator methods
let result: Vec<_> = v.iter()
    .map(|x| x * 2)
    .filter(|x| x > &2)
    .collect();

// Common adapters
.map(|x| x * 2)        // Transform
.filter(|x| x > &5)    // Filter
.take(5)               // Take first n
.skip(3)               // Skip first n
.zip(other_iter)       // Combine
.enumerate()           // Add index
.chain(other_iter)     // Concat

// Common consumers
.collect()             // Collect to collection
.sum()                 // Sum values
.count()               // Count items
.any(|x| x > 5)        // Check if any match
.all(|x| x > 0)        // Check if all match
.find(|x| x > 5)       // Find first match
```

## Closures

```rust
// Closure syntax
let add = |x, y| x + y;

// Closure capturing
let x = 5;
let add_x = |y| x + y; // captures x

// Move closure
let s = String::from("hello");
let f = move || println!("{}", s); // takes ownership of s
```

## Async/Await

```rust
// Async function
async fn fetch_data() -> String {
    // async work
    String::from("data")
}

// Await
let result = fetch_data().await;

// Tokio runtime
#[tokio::main]
async fn main() {
    let result = fetch_data().await;
}
```

## Common Macros

```rust
println!("Hello, {}!", name);       // Print with newline
print!("No newline");               // Print without newline
format!("Format {}", value);        // Create formatted string
vec![1, 2, 3];                      // Create vector
panic!("Error message");            // Panic with message
assert!(condition);                 // Assert condition
assert_eq!(left, right);            // Assert equality
```

## Cargo Commands

```bash
cargo new project_name              # Create new project
cargo build                         # Build project
cargo build --release               # Build with optimizations
cargo run                           # Build and run
cargo test                          # Run tests
cargo check                         # Check without building
cargo clippy                        # Lint code
cargo fmt                           # Format code
cargo doc --open                    # Generate and open docs
cargo add <crate>                   # Add dependency
```

## Common Attributes

```rust
#[derive(Debug)]                    // Auto-implement Debug
#[derive(Clone)]                    // Auto-implement Clone
#[derive(PartialEq)]                // Auto-implement equality
#[allow(dead_code)]                 // Suppress warning
#[cfg(test)]                        // Compile only for tests
#[test]                             // Mark as test function
```

## Module System

```rust
// Declare module
mod my_module {
    pub fn public_function() {}
    fn private_function() {}
}

// Use items
use std::collections::HashMap;
use std::io::{self, Write};

// Re-export
pub use self::my_module::public_function;
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This should panic");
    }
}
```

## Common Patterns

### Builder Pattern
```rust
let server = ServerBuilder::new("localhost")
    .port(3000)
    .timeout(Duration::from_secs(30))
    .build();
```

### RAII Pattern
```rust
{
    let file = File::create("data.txt")?;
    // use file
} // file automatically closed here
```

### Entry API
```rust
let mut map = HashMap::new();
*map.entry("key").or_insert(0) += 1;
```

### Error Context
```rust
use anyhow::{Context, Result};

fs::read_to_string("file.txt")
    .context("Failed to read file")?;
```

## When to Use What

### Vec vs Array
- `Vec<T>` - dynamic size, heap-allocated
- `[T; N]` - fixed size, stack-allocated

### String vs &str
- `String` - owned, mutable, heap-allocated
- `&str` - borrowed, immutable, can be stack or heap

### Rc vs Arc
- `Rc<T>` - single-threaded reference counting
- `Arc<T>` - thread-safe reference counting

### RefCell vs Mutex
- `RefCell<T>` - single-threaded interior mutability
- `Mutex<T>` - thread-safe interior mutability

### Box vs Rc vs Arc
- `Box<T>` - single ownership, heap allocation
- `Rc<T>` - multiple ownership, single-threaded
- `Arc<T>` - multiple ownership, thread-safe

## Common Compiler Errors

| Error | Meaning | Fix |
|-------|---------|-----|
| E0382 | Use of moved value | Clone or use references |
| E0499 | Cannot borrow as mutable more than once | Use only one mutable borrow |
| E0502 | Cannot borrow as mutable because also borrowed as immutable | Separate borrows or use interior mutability |
| E0106 | Missing lifetime specifier | Add explicit lifetime annotations |
| E0277 | Trait bound not satisfied | Implement required trait or add constraint |

## Memory Management

- **Stack**: Fast, fixed size, automatically managed
  - Primitives, fixed arrays, structs of stack types
- **Heap**: Flexible size, manual management (via RAII)
  - String, Vec, Box, Rc, Arc
- **No garbage collector**: Memory freed when owner goes out of scope
- **No manual free**: Drop trait handles cleanup automatically

## Tips

1. **Use borrowing by default**: `&T` instead of `T`
2. **Clone when necessary**: Use `.clone()` explicitly
3. **Read compiler errors**: They're usually helpful
4. **Use `?` for error propagation**: Cleaner than `match`
5. **Prefer iterators**: More idiomatic than loops
6. **Use `clippy`**: Catches common mistakes
7. **Format with `rustfmt`**: Consistent style
8. **Write tests**: `#[test]` functions in `#[cfg(test)]` modules

## Resources

- Official Book: https://doc.rust-lang.org/book/
- Error Index: https://doc.rust-lang.org/error-index.html
- Standard Library: https://doc.rust-lang.org/std/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Crates.io: https://crates.io/
