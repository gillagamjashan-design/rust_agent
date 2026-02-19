use crate::types::Answer;
use anyhow::Result;
use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

pub struct AnswerAgent {
    input_file: PathBuf,
    output_file: PathBuf,
    last_processed_id: usize,
}

impl AnswerAgent {
    pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            input_file: input_path,
            output_file: output_path,
            last_processed_id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("💡 Answer Agent started - processing questions every 2 seconds");

        loop {
            if let Some((id, question_text, category)) = self.read_latest_question()? {
                if id > self.last_processed_id {
                    let answer = self.generate_answer(id, &question_text, &category);
                    self.save_answer(&answer)?;

                    println!("✅ A{}: Answered question about '{}'", id, category);

                    self.last_processed_id = id;
                }
            }

            sleep(Duration::from_secs(2)).await;
        }
    }

    fn read_latest_question(&self) -> Result<Option<(usize, String, String)>> {
        if !self.input_file.exists() {
            return Ok(None);
        }

        let file = File::open(&self.input_file)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        if let Some(last_line) = lines.last() {
            // Parse: [timestamp] Q<id>: <text> [<category>]
            if let Some(q_pos) = last_line.find("Q") {
                let after_q = &last_line[q_pos + 1..];
                if let Some(colon_pos) = after_q.find(':') {
                    let id_str = &after_q[..colon_pos];
                    if let Ok(id) = id_str.trim().parse::<usize>() {
                        let after_colon = &after_q[colon_pos + 1..];
                        if let Some(bracket_pos) = after_colon.rfind('[') {
                            let question_text = after_colon[..bracket_pos].trim().to_string();
                            let category = after_colon[bracket_pos + 1..]
                                .trim_end_matches(']')
                                .trim()
                                .to_string();
                            return Ok(Some((id, question_text, category)));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn generate_answer(&self, question_id: usize, question: &str, category: &str) -> Answer {
        let (answer_text, code_examples) = match category {
            "ownership" => self.answer_ownership(question),
            "borrowing" => self.answer_borrowing(question),
            "lifetimes" => self.answer_lifetimes(question),
            "traits" => self.answer_traits(question),
            "generics" => self.answer_generics(question),
            "error_handling" => self.answer_error_handling(question),
            "modules" => self.answer_modules(question),
            "cargo" => self.answer_cargo(question),
            "testing" => self.answer_testing(question),
            "macros" => self.answer_macros(question),
            "concurrency" => self.answer_concurrency(question),
            "smart_pointers" => self.answer_smart_pointers(question),
            "iterators" => self.answer_iterators(question),
            "closures" => self.answer_closures(question),
            "pattern_matching" => self.answer_pattern_matching(question),
            _ => (
                "Rust is a systems programming language focused on safety, speed, and concurrency.".to_string(),
                vec![]
            ),
        };

        Answer {
            question_id,
            text: answer_text,
            code_examples,
            timestamp: Utc::now(),
        }
    }

    fn save_answer(&self, answer: &Answer) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file)?;

        writeln!(
            file,
            "[{}] A{}: {}",
            answer.timestamp.format("%Y-%m-%d %H:%M:%S"),
            answer.question_id,
            answer.text
        )?;

        for (i, code) in answer.code_examples.iter().enumerate() {
            writeln!(file, "\n[CODE_EXAMPLE_{}]", i + 1)?;
            writeln!(file, "{}", code)?;
            writeln!(file, "[/CODE_EXAMPLE]")?;
        }

        writeln!(file)?;

        Ok(())
    }

    // Answer generators for different topics
    fn answer_ownership(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Ownership is Rust's most unique feature that enables memory safety without garbage collection. Each value has a single owner, and when the owner goes out of scope, the value is dropped. The three rules are: 1) Each value has one owner, 2) Only one owner at a time, 3) When owner goes out of scope, value is dropped.".to_string();

        let code = r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // Error: s1 no longer valid
    println!("{}", s2); // OK
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_borrowing(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Borrowing allows you to reference data without taking ownership. Immutable references (&T) allow reading, mutable references (&mut T) allow modification. You can have either multiple immutable references OR one mutable reference, but not both simultaneously. This prevents data races at compile time.".to_string();

        let code = r#"fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow
    let r2 = &s; // multiple immutable borrows OK
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // mutable borrow
    r3.push_str(" world");
    println!("{}", r3);
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_lifetimes(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Lifetimes are Rust's way of tracking how long references are valid. They prevent dangling references. Lifetime annotations describe relationships between references' lifetimes without changing them. The 'static lifetime means the reference can live for the entire program duration.".to_string();

        let code = r#"fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // result is valid here
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_traits(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Traits define shared behavior across types, similar to interfaces in other languages. Types implement traits to provide the required functionality. Traits enable polymorphism and generic programming in Rust. You can also use trait bounds to constrain generic types.".to_string();

        let code = r#"trait Speak {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says woof!", self.name)
    }
}

fn main() {
    let dog = Dog { name: "Rex".to_string() };
    println!("{}", dog.speak());
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_generics(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Generics allow writing flexible, reusable code that works with multiple types. Rust uses monomorphization - generating specialized code for each concrete type at compile time. This provides zero-cost abstraction. Generic types can be constrained with trait bounds to ensure they have required capabilities.".to_string();

        let code = r#"fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_error_handling(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Rust uses Result<T, E> for recoverable errors and panic! for unrecoverable errors. The ? operator propagates errors up the call stack. Option<T> handles nullable values. unwrap() panics on error, expect() panics with a custom message. Best practice is to handle errors explicitly rather than unwrapping.".to_string();

        let code = r#"use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("data.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_modules(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Modules organize code into logical units and control privacy. 'mod' declares modules, 'use' brings items into scope. By default, everything is private; use 'pub' to make items public. Files and folders create module hierarchy. mod.rs was traditionally used for module roots, but now you can use the folder name.rs.".to_string();

        let code = r#"// In src/lib.rs or src/main.rs
mod utils {
    pub fn helper() {
        println!("Helper function");
    }

    fn private_function() {
        println!("Private");
    }
}

// In another file
use crate::utils::helper;

fn main() {
    helper();
    // utils::private_function(); // Error: private
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_cargo(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Cargo is Rust's package manager and build system. It handles dependencies, compilation, running tests, and generating documentation. cargo build compiles your project, cargo run builds and executes it. Dependencies are specified in Cargo.toml. cargo publish uploads crates to crates.io. Workspaces allow managing multiple related packages.".to_string();

        let code = r#"# Cargo.toml example
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

# Common commands:
# cargo new my_project
# cargo build --release
# cargo test
# cargo doc --open"#.to_string();

        (answer, vec![code])
    }

    fn answer_testing(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Rust has built-in testing support. Unit tests live in the same file as the code with #[cfg(test)] module. Integration tests go in tests/ directory. Use #[test] attribute to mark test functions. assert!, assert_eq!, and assert_ne! macros verify conditions. cargo test runs all tests. Tests can access private functions.".to_string();

        let code = r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_macros(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Macros are code that writes code (metaprogramming). Declarative macros use pattern matching with macro_rules!. Procedural macros operate on the abstract syntax tree and include derive macros, attribute-like macros, and function-like macros. Macros are expanded at compile time. Use macros when you need to generate repetitive code or when functions can't express what you need.".to_string();

        let code = r#"// Declarative macro
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

// Derive macro usage
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let strings = vec_of_strings!["hello", "world"];
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_concurrency(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Rust prevents data races at compile time using ownership and type system. std::thread spawns threads. Arc (atomic reference counting) enables shared ownership across threads. Mutex provides mutual exclusion. Channels (mpsc) allow message passing between threads. Send trait marks types safe to transfer between threads, Sync marks types safe to reference from multiple threads.".to_string();

        let code = r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_smart_pointers(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Smart pointers are data structures that act like pointers but have additional metadata and capabilities. Box<T> allocates on heap. Rc<T> enables multiple ownership through reference counting (single-threaded). Arc<T> is thread-safe Rc. RefCell<T> enables interior mutability, allowing mutation of data even when immutable references exist. Weak<T> prevents reference cycles.".to_string();

        let code = r#"use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Box: heap allocation
    let b = Box::new(5);
    println!("b = {}", b);

    // Rc: multiple ownership
    let rc_a = Rc::new(5);
    let rc_b = Rc::clone(&rc_a);
    println!("Count: {}", Rc::strong_count(&rc_a));

    // RefCell: interior mutability
    let data = RefCell::new(5);
    *data.borrow_mut() += 1;
    println!("data = {}", data.borrow());
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_iterators(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Iterators are lazy - they don't do work until consumed. iter() borrows, iter_mut() mutably borrows, into_iter() takes ownership. Iterator trait requires next() method. Iterator adapters (map, filter, etc.) transform iterators. Consumers (collect, sum, etc.) produce values. Iterators are often more efficient than loops due to compiler optimizations.".to_string();

        let code = r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Iterator chain
    let sum: i32 = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .sum();

    println!("Sum: {}", sum);

    // Custom iterator
    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();

    println!("Evens: {:?}", evens);
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_closures(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Closures are anonymous functions that can capture their environment. Three traits define how closures capture: FnOnce (takes ownership), FnMut (mutable borrow), Fn (immutable borrow). Compiler infers which trait based on usage. Use move keyword to force taking ownership. Closures are commonly used with iterators and for callbacks.".to_string();

        let code = r#"fn main() {
    let x = 4;

    // Fn: immutable borrow
    let equal_to_x = |z| z == x;
    println!("Is 4 equal? {}", equal_to_x(4));

    // FnMut: mutable borrow
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("Count: {}", increment());

    // FnOnce: takes ownership
    let s = String::from("hello");
    let consume = move || {
        println!("{}", s);
    };
    consume();
}"#.to_string();

        (answer, vec![code])
    }

    fn answer_pattern_matching(&self, _question: &str) -> (String, Vec<String>) {
        let answer = "Pattern matching allows destructuring and matching values against patterns. match must be exhaustive. Patterns can match literals, variables, wildcards, ranges, structs, enums, and more. Match guards add additional conditions with if. if let and while let provide convenient shorthand for single patterns. @ binds matched values to variables.".to_string();

        let code = r#"enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) if text.len() > 10 => {
            println!("Long message: {}", text)
        }
        Message::Write(text) => println!("Short: {}", text),
    }
}

fn main() {
    let msg = Message::Write("Hello".to_string());
    process(msg);
}"#.to_string();

        (answer, vec![code])
    }
}
