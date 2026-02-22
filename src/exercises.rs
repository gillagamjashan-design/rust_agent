use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub description: String,
    pub stage: u8,
    pub broken_code: String,
    pub expected_error: String,
    pub error_code: String, // E0XXX
    pub constraints: Vec<String>,
    pub hints: Vec<String>,
    pub concepts: Vec<String>,
    pub difficulty: Difficulty,
    pub working_solution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    pub stages: Vec<Stage>,
    pub exercises: HashMap<String, Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub concepts: Vec<String>,
    pub exercise_ids: Vec<String>,
    pub advancement_criteria: AdvancementCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancementCriteria {
    pub min_exercises_completed: usize,
    pub required_concepts: Vec<String>,
    pub description: String,
}

impl Curriculum {
    pub fn new() -> Self {
        let mut curriculum = Self {
            stages: Vec::new(),
            exercises: HashMap::new(),
        };

        curriculum.initialize_stages();
        curriculum.initialize_exercises();
        curriculum
    }

    fn initialize_stages(&mut self) {
        self.stages = vec![
            Stage {
                id: 1,
                name: "Foundation - Ownership & Memory".to_string(),
                description: "Learn the core concepts of Rust's ownership system, move semantics, and memory management.".to_string(),
                concepts: vec![
                    "ownership".to_string(),
                    "move semantics".to_string(),
                    "copy trait".to_string(),
                    "stack vs heap".to_string(),
                    "drop trait".to_string(),
                ],
                exercise_ids: vec![
                    "stage1_ex1".to_string(),
                    "stage1_ex2".to_string(),
                    "stage1_ex3".to_string(),
                    "stage1_ex4".to_string(),
                    "stage1_ex5".to_string(),
                ],
                advancement_criteria: AdvancementCriteria {
                    min_exercises_completed: 4,
                    required_concepts: vec!["ownership".to_string(), "move semantics".to_string()],
                    description: "Complete at least 4 exercises and demonstrate understanding of ownership and move semantics.".to_string(),
                },
            },
            Stage {
                id: 2,
                name: "Borrowing & Lifetimes".to_string(),
                description: "Master references, borrowing rules, and lifetime annotations.".to_string(),
                concepts: vec![
                    "references".to_string(),
                    "mutable borrowing".to_string(),
                    "immutable borrowing".to_string(),
                    "lifetime annotations".to_string(),
                    "borrow checker".to_string(),
                ],
                exercise_ids: vec![
                    "stage2_ex1".to_string(),
                    "stage2_ex2".to_string(),
                    "stage2_ex3".to_string(),
                    "stage2_ex4".to_string(),
                    "stage2_ex5".to_string(),
                ],
                advancement_criteria: AdvancementCriteria {
                    min_exercises_completed: 4,
                    required_concepts: vec!["references".to_string(), "lifetime annotations".to_string()],
                    description: "Complete at least 4 exercises and understand borrowing rules and basic lifetimes.".to_string(),
                },
            },
            Stage {
                id: 3,
                name: "Patterns & Abstractions".to_string(),
                description: "Learn traits, generics, and common Rust patterns.".to_string(),
                concepts: vec![
                    "traits".to_string(),
                    "generics".to_string(),
                    "iterators".to_string(),
                    "closures".to_string(),
                    "pattern matching".to_string(),
                ],
                exercise_ids: vec![
                    "stage3_ex1".to_string(),
                    "stage3_ex2".to_string(),
                    "stage3_ex3".to_string(),
                    "stage3_ex4".to_string(),
                    "stage3_ex5".to_string(),
                ],
                advancement_criteria: AdvancementCriteria {
                    min_exercises_completed: 4,
                    required_concepts: vec!["traits".to_string(), "generics".to_string()],
                    description: "Complete at least 4 exercises and demonstrate trait implementation and generic programming.".to_string(),
                },
            },
            Stage {
                id: 4,
                name: "Advanced Memory Management".to_string(),
                description: "Master smart pointers, interior mutability, and advanced patterns.".to_string(),
                concepts: vec![
                    "Rc".to_string(),
                    "Arc".to_string(),
                    "RefCell".to_string(),
                    "Box".to_string(),
                    "reference cycles".to_string(),
                ],
                exercise_ids: vec![
                    "stage4_ex1".to_string(),
                    "stage4_ex2".to_string(),
                    "stage4_ex3".to_string(),
                    "stage4_ex4".to_string(),
                    "stage4_ex5".to_string(),
                ],
                advancement_criteria: AdvancementCriteria {
                    min_exercises_completed: 4,
                    required_concepts: vec!["Rc".to_string(), "RefCell".to_string()],
                    description: "Complete at least 4 exercises and understand smart pointers and interior mutability.".to_string(),
                },
            },
            Stage {
                id: 5,
                name: "Systems Programming".to_string(),
                description: "Learn concurrent programming, unsafe code, and systems-level concepts.".to_string(),
                concepts: vec![
                    "threads".to_string(),
                    "Arc + Mutex".to_string(),
                    "Send + Sync".to_string(),
                    "unsafe code".to_string(),
                    "FFI".to_string(),
                ],
                exercise_ids: vec![
                    "stage5_ex1".to_string(),
                    "stage5_ex2".to_string(),
                    "stage5_ex3".to_string(),
                    "stage5_ex4".to_string(),
                    "stage5_ex5".to_string(),
                ],
                advancement_criteria: AdvancementCriteria {
                    min_exercises_completed: 4,
                    required_concepts: vec!["threads".to_string(), "Arc + Mutex".to_string()],
                    description: "Complete at least 4 exercises and demonstrate concurrent programming skills.".to_string(),
                },
            },
        ];
    }

    fn initialize_exercises(&mut self) {
        // STAGE 1: FOUNDATION - OWNERSHIP & MEMORY

        self.exercises.insert(
            "stage1_ex1".to_string(),
            Exercise {
                id: "stage1_ex1".to_string(),
                title: "Basic Ownership Transfer".to_string(),
                description: "Fix the code that tries to use a value after it has been moved.".to_string(),
                stage: 1,
                broken_code: r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1: {}", s1); // Error: value borrowed here after move
    println!("s2: {}", s2);
}"#.to_string(),
                expected_error: "borrow of moved value: `s1`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Cannot use .clone()".to_string(),
                    "Must understand move semantics".to_string(),
                ],
                hints: vec![
                    "When a String is assigned to another variable, ownership is transferred".to_string(),
                    "After the move, the original variable is no longer valid".to_string(),
                    "Try using s2 for both print statements, or use a reference".to_string(),
                ],
                concepts: vec!["ownership".to_string(), "move semantics".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 is no longer valid after the move
    println!("s2: {}", s2);
    println!("s2 again: {}", s2);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage1_ex2".to_string(),
            Exercise {
                id: "stage1_ex2".to_string(),
                title: "Stack vs Heap - Copy Trait".to_string(),
                description: "Understand why integers behave differently from Strings in ownership.".to_string(),
                stage: 1,
                broken_code: r#"fn take_ownership(s: String) {
    println!("Took ownership: {}", s);
}

fn main() {
    let x = 5;
    let s = String::from("hello");

    take_ownership(s);
    println!("After function call: {}", s); // Error!

    // Why does this work with x but not s?
}"#.to_string(),
                expected_error: "borrow of moved value: `s`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Explain the difference between Copy and Move types".to_string(),
                ],
                hints: vec![
                    "i32 implements the Copy trait, String does not".to_string(),
                    "Copy types are stored on the stack and are cheap to copy".to_string(),
                    "Heap-allocated types like String are moved to avoid expensive copies".to_string(),
                ],
                concepts: vec!["copy trait".to_string(), "stack vs heap".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn take_ownership(s: String) {
    println!("Took ownership: {}", s);
}

fn main() {
    let x = 5; // i32 implements Copy
    let s = String::from("hello");

    take_ownership(s);
    // s is moved, cannot use it here

    // x can still be used because i32 is Copy
    println!("x is still valid: {}", x);

    // To use s, we need to create a new String or use a reference
    let s2 = String::from("world");
    println!("New string: {}", s2);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage1_ex3".to_string(),
            Exercise {
                id: "stage1_ex3".to_string(),
                title: "Function Return Ownership".to_string(),
                description: "Fix ownership transfer in function returns.".to_string(),
                stage: 1,
                broken_code: r#"fn create_string() -> String {
    let s = String::from("hello");
    s
}

fn append_world(s: String) {
    let result = format!("{} world", s);
    // What happens to result here?
}

fn main() {
    let s = create_string();
    append_world(s);
    println!("Final string: {}", s); // Error!
}"#.to_string(),
                expected_error: "borrow of moved value: `s`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Must return the modified string from append_world".to_string(),
                ],
                hints: vec![
                    "When s is passed to append_world, ownership is transferred".to_string(),
                    "append_world should return the String to transfer ownership back".to_string(),
                    "Values are dropped when they go out of scope unless returned".to_string(),
                ],
                concepts: vec!["ownership".to_string(), "drop trait".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn create_string() -> String {
    let s = String::from("hello");
    s // ownership transferred to caller
}

fn append_world(s: String) -> String {
    let result = format!("{} world", s);
    result // return ownership to caller
}

fn main() {
    let s = create_string();
    let s = append_world(s); // take back ownership
    println!("Final string: {}", s);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage1_ex4".to_string(),
            Exercise {
                id: "stage1_ex4".to_string(),
                title: "Vector Ownership in Loops".to_string(),
                description: "Fix ownership issues when iterating over a vector.".to_string(),
                stage: 1,
                broken_code: r#"fn main() {
    let names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];

    for name in names {
        println!("Hello, {}", name);
    }

    println!("Number of names: {}", names.len()); // Error!
}"#.to_string(),
                expected_error: "borrow of moved value: `names`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Must iterate without moving the vector".to_string(),
                ],
                hints: vec![
                    "for name in names moves the entire vector".to_string(),
                    "Use &names to iterate by reference".to_string(),
                    "for name in &names creates references to elements".to_string(),
                ],
                concepts: vec!["ownership".to_string(), "move semantics".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"fn main() {
    let names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];

    // Iterate by reference to avoid moving
    for name in &names {
        println!("Hello, {}", name);
    }

    // names is still valid
    println!("Number of names: {}", names.len());
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage1_ex5".to_string(),
            Exercise {
                id: "stage1_ex5".to_string(),
                title: "Partial Moves in Structs".to_string(),
                description: "Understand partial moves and how they affect struct usage.".to_string(),
                stage: 1,
                broken_code: r#"struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let name = person.name; // Partial move
    println!("Name: {}", name);
    println!("Person: {} is {} years old", person.name, person.age); // Error!
}"#.to_string(),
                expected_error: "borrow of partially moved value: `person`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Understand partial moves".to_string(),
                    "Cannot use struct after partial move of non-Copy field".to_string(),
                ],
                hints: vec![
                    "Moving person.name makes person partially moved".to_string(),
                    "age is Copy, but name is not, so person cannot be used as a whole".to_string(),
                    "Use a reference to name instead of moving it".to_string(),
                ],
                concepts: vec!["ownership".to_string(), "move semantics".to_string(), "copy trait".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Use a reference instead of moving
    let name = &person.name;
    println!("Name: {}", name);
    println!("Person: {} is {} years old", person.name, person.age);
}"#.to_string(),
            },
        );

        // STAGE 2: BORROWING & LIFETIMES

        self.exercises.insert(
            "stage2_ex1".to_string(),
            Exercise {
                id: "stage2_ex1".to_string(),
                title: "Multiple Mutable References".to_string(),
                description: "Fix the classic multiple mutable borrow error.".to_string(),
                stage: 2,
                broken_code: r#"fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // Error!

    r1.push_str(" world");
    r2.push_str("!");

    println!("{}", s);
}"#.to_string(),
                expected_error: "cannot borrow `s` as mutable more than once at a time".to_string(),
                error_code: "E0499".to_string(),
                constraints: vec![
                    "Can only have one mutable reference at a time".to_string(),
                ],
                hints: vec![
                    "Rust allows only one mutable reference to prevent data races".to_string(),
                    "Use the reference, let it go out of scope, then create another".to_string(),
                    "Or restructure the code to use only one mutable reference".to_string(),
                ],
                concepts: vec!["mutable borrowing".to_string(), "borrow checker".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn main() {
    let mut s = String::from("hello");

    // Use the mutable reference in a scope
    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1 goes out of scope

    // Now we can create another mutable reference
    {
        let r2 = &mut s;
        r2.push_str("!");
    }

    println!("{}", s);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage2_ex2".to_string(),
            Exercise {
                id: "stage2_ex2".to_string(),
                title: "Mixing Mutable and Immutable References".to_string(),
                description: "Fix errors when mixing mutable and immutable borrows.".to_string(),
                stage: 2,
                broken_code: r#"fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // Error!

    println!("{}, {}", r1, r2);
    r3.push_str(" world");
    println!("{}", r3);
}"#.to_string(),
                expected_error: "cannot borrow `s` as mutable because it is also borrowed as immutable".to_string(),
                error_code: "E0502".to_string(),
                constraints: vec![
                    "Cannot have mutable reference while immutable references exist".to_string(),
                ],
                hints: vec![
                    "Immutable references must not be active when creating a mutable one".to_string(),
                    "A reference's scope ends at its last use (Non-Lexical Lifetimes)".to_string(),
                    "Move the mutable borrow after the last use of immutable borrows".to_string(),
                ],
                concepts: vec!["mutable borrowing".to_string(), "immutable borrowing".to_string(), "borrow checker".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // Now this is fine
    r3.push_str(" world");
    println!("{}", r3);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage2_ex3".to_string(),
            Exercise {
                id: "stage2_ex3".to_string(),
                title: "Dangling References".to_string(),
                description: "Fix a function that returns a reference to a local variable.".to_string(),
                stage: 2,
                broken_code: r#"fn dangle() -> &String {
    let s = String::from("hello");
    &s // Error: returns a reference to data owned by this function
}

fn main() {
    let reference = dangle();
    println!("{}", reference);
}"#.to_string(),
                expected_error: "missing lifetime specifier".to_string(),
                error_code: "E0106".to_string(),
                constraints: vec![
                    "Cannot return reference to local variable".to_string(),
                ],
                hints: vec![
                    "s goes out of scope at the end of the function and is dropped".to_string(),
                    "The reference would point to deallocated memory".to_string(),
                    "Return the String itself, transferring ownership to the caller".to_string(),
                ],
                concepts: vec!["references".to_string(), "lifetime annotations".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"fn no_dangle() -> String {
    let s = String::from("hello");
    s // Transfer ownership to caller
}

fn main() {
    let string = no_dangle();
    println!("{}", string);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage2_ex4".to_string(),
            Exercise {
                id: "stage2_ex4".to_string(),
                title: "Lifetime Annotations in Structs".to_string(),
                description: "Fix lifetime issues in a struct holding references.".to_string(),
                stage: 2,
                broken_code: r#"struct Excerpt {
    part: &str, // Error: missing lifetime specifier
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = Excerpt {
        part: first_sentence,
    };

    println!("{}", excerpt.part);
}"#.to_string(),
                expected_error: "missing lifetime specifier".to_string(),
                error_code: "E0106".to_string(),
                constraints: vec![
                    "Must add lifetime annotations".to_string(),
                ],
                hints: vec![
                    "Structs holding references need lifetime parameters".to_string(),
                    "The lifetime annotation tells Rust how long the reference is valid".to_string(),
                    "Use 'a as the lifetime parameter: struct Excerpt<'a>".to_string(),
                ],
                concepts: vec!["lifetime annotations".to_string(), "references".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = Excerpt {
        part: first_sentence,
    };

    println!("{}", excerpt.part);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage2_ex5".to_string(),
            Exercise {
                id: "stage2_ex5".to_string(),
                title: "Multiple Lifetime Parameters".to_string(),
                description: "Fix a function with multiple lifetime parameters.".to_string(),
                stage: 2,
                broken_code: r#"fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // Error!
}"#.to_string(),
                expected_error: "missing lifetime specifier".to_string(),
                error_code: "E0106".to_string(),
                constraints: vec![
                    "Must add lifetime annotations to function signature".to_string(),
                ],
                hints: vec![
                    "The return type needs a lifetime parameter".to_string(),
                    "Use the same lifetime for both parameters and return type".to_string(),
                    "The lifetime of the return value is tied to the shorter of the two inputs".to_string(),
                ],
                concepts: vec!["lifetime annotations".to_string(), "references".to_string(), "borrow checker".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // Note: Both strings must live as long as result is used
}"#.to_string(),
            },
        );

        // STAGE 3: PATTERNS & ABSTRACTIONS

        self.exercises.insert(
            "stage3_ex1".to_string(),
            Exercise {
                id: "stage3_ex1".to_string(),
                title: "Implementing Basic Traits".to_string(),
                description: "Implement the Display trait for a custom struct.".to_string(),
                stage: 3,
                broken_code: r#"use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Point: {}", p); // Error: Point doesn't implement Display
}"#.to_string(),
                expected_error: "`Point` doesn't implement `std::fmt::Display`".to_string(),
                error_code: "E0277".to_string(),
                constraints: vec![
                    "Must implement the Display trait".to_string(),
                ],
                hints: vec![
                    "Implement fmt::Display for Point".to_string(),
                    "The fmt method writes to a formatter".to_string(),
                    "Use write! macro to format the output".to_string(),
                ],
                concepts: vec!["traits".to_string(), "trait implementation".to_string()],
                difficulty: Difficulty::Beginner,
                working_solution: r#"use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Point: {}", p);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage3_ex2".to_string(),
            Exercise {
                id: "stage3_ex2".to_string(),
                title: "Generic Functions with Trait Bounds".to_string(),
                description: "Fix a generic function that needs trait bounds.".to_string(),
                stage: 3,
                broken_code: r#"fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { // Error: binary operation `>` cannot be applied to type `&T`
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
}"#.to_string(),
                expected_error: "binary operation `>` cannot be applied to type `&T`".to_string(),
                error_code: "E0369".to_string(),
                constraints: vec![
                    "Must add trait bounds to T".to_string(),
                ],
                hints: vec![
                    "T needs to implement PartialOrd to use > operator".to_string(),
                    "Add trait bound: where T: PartialOrd".to_string(),
                    "Alternatively use T: PartialOrd in the angle brackets".to_string(),
                ],
                concepts: vec!["generics".to_string(), "traits".to_string(), "trait bounds".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"fn largest<T: PartialOrd>(list: &[T]) -> &T {
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
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage3_ex3".to_string(),
            Exercise {
                id: "stage3_ex3".to_string(),
                title: "Iterator Trait Implementation".to_string(),
                description: "Implement a custom iterator.".to_string(),
                stage: 3,
                broken_code: r#"struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Error: Missing Iterator implementation

fn main() {
    let counter = Counter::new();

    for num in counter { // Error: Counter doesn't implement Iterator
        println!("{}", num);
        if num >= 5 {
            break;
        }
    }
}"#.to_string(),
                expected_error: "`Counter` is not an iterator".to_string(),
                error_code: "E0277".to_string(),
                constraints: vec![
                    "Must implement Iterator trait".to_string(),
                ],
                hints: vec![
                    "Implement Iterator trait for Counter".to_string(),
                    "Define the Item associated type".to_string(),
                    "Implement the next() method".to_string(),
                ],
                concepts: vec!["iterators".to_string(), "traits".to_string(), "associated types".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new();

    for num in counter {
        println!("{}", num);
    }
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage3_ex4".to_string(),
            Exercise {
                id: "stage3_ex4".to_string(),
                title: "Closure Ownership Issues".to_string(),
                description: "Fix ownership issues with closures.".to_string(),
                stage: 3,
                broken_code: r#"fn main() {
    let mut list = vec![1, 2, 3];

    let mut add_to_list = || {
        list.push(4);
    };

    println!("List: {:?}", list); // Error: cannot borrow `list` as immutable
    add_to_list();
}"#.to_string(),
                expected_error: "cannot borrow `list` as immutable because it is also borrowed as mutable".to_string(),
                error_code: "E0502".to_string(),
                constraints: vec![
                    "Understand closure borrowing rules".to_string(),
                ],
                hints: vec![
                    "The closure captures &mut list".to_string(),
                    "Cannot use list while the closure exists".to_string(),
                    "Call the closure before trying to use list immutably".to_string(),
                ],
                concepts: vec!["closures".to_string(), "mutable borrowing".to_string(), "borrow checker".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"fn main() {
    let mut list = vec![1, 2, 3];

    let mut add_to_list = || {
        list.push(4);
    };

    add_to_list(); // Call closure first

    println!("List: {:?}", list); // Now this works
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage3_ex5".to_string(),
            Exercise {
                id: "stage3_ex5".to_string(),
                title: "Advanced Iterator Chains".to_string(),
                description: "Fix type inference issues in iterator chains.".to_string(),
                stage: 3,
                broken_code: r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2); // Error: type must be known

    println!("Result: {:?}", result);
}"#.to_string(),
                expected_error: "type annotations needed".to_string(),
                error_code: "E0282".to_string(),
                constraints: vec![
                    "Must consume the iterator".to_string(),
                ],
                hints: vec![
                    "Iterators are lazy and need to be consumed".to_string(),
                    "Use .collect() to consume the iterator".to_string(),
                    "You may need to annotate the type for collect()".to_string(),
                ],
                concepts: vec!["iterators".to_string(), "lazy evaluation".to_string(), "type inference".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect(); // Consume the iterator

    println!("Result: {:?}", result);
}"#.to_string(),
            },
        );

        // STAGE 4: ADVANCED MEMORY MANAGEMENT

        self.exercises.insert(
            "stage4_ex1".to_string(),
            Exercise {
                id: "stage4_ex1".to_string(),
                title: "Reference Counting with Rc".to_string(),
                description: "Fix multiple ownership issues using Rc.".to_string(),
                stage: 4,
                broken_code: r#"enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a)); // Error: a moved here
    let c = Cons(4, Box::new(a)); // Error: cannot move a again
}"#.to_string(),
                expected_error: "use of moved value: `a`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Must use Rc for shared ownership".to_string(),
                ],
                hints: vec![
                    "Box allows only one owner".to_string(),
                    "Use Rc (Reference Counted) for multiple owners".to_string(),
                    "Wrap the shared value in Rc and clone the Rc, not the data".to_string(),
                ],
                concepts: vec!["Rc".to_string(), "shared ownership".to_string(), "Box".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("Reference count of a: {}", Rc::strong_count(&a));
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage4_ex2".to_string(),
            Exercise {
                id: "stage4_ex2".to_string(),
                title: "Interior Mutability with RefCell".to_string(),
                description: "Fix mutability issues using RefCell.".to_string(),
                stage: 4,
                broken_code: r#"struct MockMessenger {
    sent_messages: Vec<String>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: vec![],
        }
    }

    fn send(&self, message: &str) {
        self.sent_messages.push(message.to_string()); // Error: cannot mutate
    }
}

fn main() {
    let messenger = MockMessenger::new();
    messenger.send("hello");
}"#.to_string(),
                expected_error: "cannot borrow `self.sent_messages` as mutable".to_string(),
                error_code: "E0596".to_string(),
                constraints: vec![
                    "Must use RefCell for interior mutability".to_string(),
                    "Cannot change the send signature to &mut self".to_string(),
                ],
                hints: vec![
                    "RefCell allows mutation through immutable references".to_string(),
                    "Wrap sent_messages in RefCell".to_string(),
                    "Use .borrow_mut() to get a mutable reference at runtime".to_string(),
                ],
                concepts: vec!["RefCell".to_string(), "interior mutability".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(message.to_string());
    }
}

fn main() {
    let messenger = MockMessenger::new();
    messenger.send("hello");
    println!("Sent: {:?}", messenger.sent_messages.borrow());
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage4_ex3".to_string(),
            Exercise {
                id: "stage4_ex3".to_string(),
                title: "Combining Rc and RefCell".to_string(),
                description: "Create shared mutable state with Rc<RefCell<T>>.".to_string(),
                stage: 4,
                broken_code: r#"use std::rc::Rc;

#[derive(Debug)]
struct SharedValue {
    value: i32,
}

fn main() {
    let shared = Rc::new(SharedValue { value: 5 });

    let shared_a = Rc::clone(&shared);
    let shared_b = Rc::clone(&shared);

    // Want to modify through shared_a
    shared_a.value += 10; // Error: cannot mutate through Rc

    println!("Value: {}", shared_b.value);
}"#.to_string(),
                expected_error: "cannot assign to data in an `Rc`".to_string(),
                error_code: "E0594".to_string(),
                constraints: vec![
                    "Must combine Rc and RefCell".to_string(),
                ],
                hints: vec![
                    "Rc alone provides shared ownership but not mutability".to_string(),
                    "Combine Rc<RefCell<T>> for shared mutable state".to_string(),
                    "Use borrow_mut() to mutate the inner value".to_string(),
                ],
                concepts: vec!["Rc".to_string(), "RefCell".to_string(), "interior mutability".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct SharedValue {
    value: i32,
}

fn main() {
    let shared = Rc::new(RefCell::new(SharedValue { value: 5 }));

    let shared_a = Rc::clone(&shared);
    let shared_b = Rc::clone(&shared);

    // Modify through shared_a
    shared_a.borrow_mut().value += 10;

    println!("Value: {}", shared_b.borrow().value);
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage4_ex4".to_string(),
            Exercise {
                id: "stage4_ex4".to_string(),
                title: "Detecting Reference Cycles".to_string(),
                description: "Identify and fix a reference cycle memory leak.".to_string(),
                stage: 4,
                broken_code: r#"use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Option<Rc<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(None),
        children: RefCell::new(vec![]),
    });

    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Some(Rc::clone(&parent))),
        children: RefCell::new(vec![]),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    // Memory leak! parent and child reference each other
    println!("Parent rc count: {}", Rc::strong_count(&parent));
}"#.to_string(),
                expected_error: "Reference cycle causing memory leak".to_string(),
                error_code: "LEAK".to_string(),
                constraints: vec![
                    "Must use Weak references to break the cycle".to_string(),
                ],
                hints: vec![
                    "Strong references (Rc) create reference cycles".to_string(),
                    "Use Weak for parent references to prevent cycles".to_string(),
                    "Weak doesn't prevent the value from being dropped".to_string(),
                ],
                concepts: vec!["Rc".to_string(), "reference cycles".to_string(), "Weak".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>, // Use Weak here
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(None),
        children: RefCell::new(vec![]),
    });

    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Some(Rc::downgrade(&parent))), // Downgrade to Weak
        children: RefCell::new(vec![]),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    println!("Parent rc count: {}", Rc::strong_count(&parent));
    println!("Child rc count: {}", Rc::strong_count(&child));
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage4_ex5".to_string(),
            Exercise {
                id: "stage4_ex5".to_string(),
                title: "Custom Smart Pointer with Deref".to_string(),
                description: "Implement Deref trait for a custom smart pointer.".to_string(),
                stage: 4,
                broken_code: r#"struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Error: expected &str, found &MyBox<String>
}"#.to_string(),
                expected_error: "expected `&str`, found `&MyBox<String>`".to_string(),
                error_code: "E0308".to_string(),
                constraints: vec![
                    "Must implement Deref trait".to_string(),
                ],
                hints: vec![
                    "Implement Deref to allow automatic dereferencing".to_string(),
                    "Deref coercion converts &MyBox<String> to &String to &str".to_string(),
                    "The deref method returns a reference to the inner value".to_string(),
                ],
                concepts: vec!["Box".to_string(), "Deref trait".to_string(), "deref coercion".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Deref coercion: &MyBox<String> -> &String -> &str
}"#.to_string(),
            },
        );

        // STAGE 5: SYSTEMS PROGRAMMING

        self.exercises.insert(
            "stage5_ex1".to_string(),
            Exercise {
                id: "stage5_ex1".to_string(),
                title: "Basic Thread Safety".to_string(),
                description: "Fix a data race in multi-threaded code.".to_string(),
                stage: 5,
                broken_code: r#"use std::thread;

fn main() {
    let mut counter = 0;

    let handle = thread::spawn(|| {
        counter += 1; // Error: cannot capture mutable reference
    });

    handle.join().unwrap();
    println!("Counter: {}", counter);
}"#.to_string(),
                expected_error: "closure may outlive the current function".to_string(),
                error_code: "E0373".to_string(),
                constraints: vec![
                    "Must use thread-safe primitives".to_string(),
                ],
                hints: vec![
                    "Cannot share mutable references across threads".to_string(),
                    "Use Arc<Mutex<T>> for shared mutable state".to_string(),
                    "Arc allows sharing, Mutex provides interior mutability".to_string(),
                ],
                concepts: vec!["threads".to_string(), "Arc + Mutex".to_string(), "Send + Sync".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut num = counter_clone.lock().unwrap();
        *num += 1;
    });

    handle.join().unwrap();
    println!("Counter: {}", *counter.lock().unwrap());
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage5_ex2".to_string(),
            Exercise {
                id: "stage5_ex2".to_string(),
                title: "Multiple Thread Synchronization".to_string(),
                description: "Coordinate multiple threads safely.".to_string(),
                stage: 5,
                broken_code: r#"use std::thread;
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut num = counter.lock().unwrap(); // Error: counter moved
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}"#.to_string(),
                expected_error: "use of moved value: `counter`".to_string(),
                error_code: "E0382".to_string(),
                constraints: vec![
                    "Must share Mutex across multiple threads".to_string(),
                ],
                hints: vec![
                    "Each thread needs its own Arc clone".to_string(),
                    "Wrap the Mutex in Arc for shared ownership".to_string(),
                    "Clone the Arc before moving into each thread closure".to_string(),
                ],
                concepts: vec!["threads".to_string(), "Arc + Mutex".to_string()],
                difficulty: Difficulty::Advanced,
                working_solution: r#"use std::thread;
use std::sync::{Arc, Mutex};

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
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage5_ex3".to_string(),
            Exercise {
                id: "stage5_ex3".to_string(),
                title: "Send and Sync Trait Bounds".to_string(),
                description: "Fix trait bound errors for thread safety.".to_string(),
                stage: 5,
                broken_code: r#"use std::thread;
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    let data_clone = Rc::clone(&data);

    thread::spawn(move || {
        println!("Data: {:?}", data_clone); // Error: Rc is not Send
    });
}"#.to_string(),
                expected_error: "`Rc<Vec<i32>>` cannot be sent between threads safely".to_string(),
                error_code: "E0277".to_string(),
                constraints: vec![
                    "Must use thread-safe types".to_string(),
                ],
                hints: vec![
                    "Rc is not thread-safe (doesn't implement Send)".to_string(),
                    "Use Arc instead of Rc for thread-safe reference counting".to_string(),
                    "Arc implements both Send and Sync".to_string(),
                ],
                concepts: vec!["Send + Sync".to_string(), "Arc".to_string(), "threads".to_string()],
                difficulty: Difficulty::Intermediate,
                working_solution: r#"use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("Data: {:?}", data_clone);
    });

    handle.join().unwrap();
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage5_ex4".to_string(),
            Exercise {
                id: "stage5_ex4".to_string(),
                title: "Deadlock Prevention".to_string(),
                description: "Identify and fix a potential deadlock.".to_string(),
                stage: 5,
                broken_code: r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap(); // Potential deadlock
    });

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        let _lock2 = r2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock1 = r1.lock().unwrap(); // Potential deadlock
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}"#.to_string(),
                expected_error: "Potential deadlock due to lock ordering".to_string(),
                error_code: "DEADLOCK".to_string(),
                constraints: vec![
                    "Must acquire locks in consistent order".to_string(),
                ],
                hints: vec![
                    "Deadlock occurs when threads acquire locks in different orders".to_string(),
                    "Always acquire locks in the same order across all threads".to_string(),
                    "Both threads should lock resource1 before resource2".to_string(),
                ],
                concepts: vec!["Arc + Mutex".to_string(), "deadlock".to_string(), "threads".to_string()],
                difficulty: Difficulty::Expert,
                working_solution: r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap();
    });

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        // Acquire locks in same order as thread1
        let _lock1 = r1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}"#.to_string(),
            },
        );

        self.exercises.insert(
            "stage5_ex5".to_string(),
            Exercise {
                id: "stage5_ex5".to_string(),
                title: "Unsafe Code Contracts".to_string(),
                description: "Fix unsafe code by maintaining invariants.".to_string(),
                stage: 5,
                broken_code: r#"fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r1 is: {}", *r1); // Undefined behavior!
    }
}"#.to_string(),
                expected_error: "Creating aliasing mutable and immutable raw pointers".to_string(),
                error_code: "UNDEFINED".to_string(),
                constraints: vec![
                    "Must maintain Rust's aliasing rules even in unsafe".to_string(),
                ],
                hints: vec![
                    "Even in unsafe, Rust's aliasing rules should be respected".to_string(),
                    "Don't create mutable and immutable pointers simultaneously".to_string(),
                    "Use only one type of pointer at a time".to_string(),
                ],
                concepts: vec!["unsafe code".to_string(), "raw pointers".to_string(), "undefined behavior".to_string()],
                difficulty: Difficulty::Expert,
                working_solution: r#"fn main() {
    let mut num = 5;

    // Use immutable pointer first
    {
        let r1 = &num as *const i32;
        unsafe {
            println!("r1 is: {}", *r1);
        }
    } // r1 no longer exists

    // Now use mutable pointer
    {
        let r2 = &mut num as *mut i32;
        unsafe {
            *r2 = 10;
        }
    } // r2 no longer exists

    println!("num is: {}", num);
}"#.to_string(),
            },
        );
    }

    pub fn get_exercise(&self, id: &str) -> Option<&Exercise> {
        self.exercises.get(id)
    }

    pub fn get_stage(&self, stage_id: u8) -> Option<&Stage> {
        self.stages.iter().find(|s| s.id == stage_id)
    }

    pub fn get_exercises_for_stage(&self, stage_id: u8) -> Vec<&Exercise> {
        if let Some(stage) = self.get_stage(stage_id) {
            stage
                .exercise_ids
                .iter()
                .filter_map(|id| self.exercises.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn save_to_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let curriculum = serde_json::from_str(&json)?;
        Ok(curriculum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curriculum_creation() {
        let curriculum = Curriculum::new();
        assert_eq!(curriculum.stages.len(), 5);
        assert!(curriculum.exercises.len() >= 25);
    }

    #[test]
    fn test_stage_exercises() {
        let curriculum = Curriculum::new();
        let stage1_exercises = curriculum.get_exercises_for_stage(1);
        assert_eq!(stage1_exercises.len(), 5);
    }

    #[test]
    fn test_exercise_retrieval() {
        let curriculum = Curriculum::new();
        let exercise = curriculum.get_exercise("stage1_ex1");
        assert!(exercise.is_some());
        assert_eq!(exercise.unwrap().stage, 1);
    }
}
