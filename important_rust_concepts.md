# Rust Macros and Other Core Concepts

This guide explains **macros in Rust** in depth, especially for someone who remembers **C macros** and wants to understand what is similar, what is different, and what else matters in Rust.

---

## 1. What is a macro in Rust?

A **macro** in Rust is a way to generate or transform code **before normal compilation finishes**.

You have already seen macros such as:

```rust
println!("Hello");
vec![1, 2, 3];
format!("x = {}", 10);
```

The `!` is the visual clue that this is a **macro call**, not a normal function call.

A macro is useful when the language needs something more flexible than an ordinary function.

Rust macros can:

* accept a variable number of arguments
* work with syntax, not just values
* generate repeated code
* reduce boilerplate
* enable domain-specific mini-syntaxes

---

## 2. Rust macros vs C macros

This is one of the most important comparisons.

### C macros

In C, macros are typically created with `#define`.

Example:

```c
#define SQUARE(x) x * x
```

Usage:

```c
int y = SQUARE(1 + 2);
```

This expands textually into:

```c
int y = 1 + 2 * 1 + 2;
```

That gives the wrong result because C macros are mostly **text substitution** done by the preprocessor.

That is why C macros can be dangerous:

* precedence bugs
* repeated evaluation bugs
* no real type checking
* hard-to-debug expansions

### Rust macros

Rust macros are **not** just dumb text substitution in the C sense.

Rust’s common declarative macros work on **token patterns** and integrate much more safely with the language.

That means:

* they understand Rust syntax better
* they are usually less error-prone than C macros
* they participate more cleanly in compilation
* they are still powerful, but in a more structured way

So the right mental model is:

* **C macro** → preprocessor text replacement
* **Rust macro** → syntax-aware code generation

They are not the same thing.

---

## 3. Why use a macro instead of a function?

A function works on values at runtime.

A macro works on code-like input during compilation.

### Function example

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

A function needs:

* a fixed signature
* normal Rust expressions as arguments
* runtime execution

### Macro example

```rust
println!("name = {}, age = {}", name, age);
```

Why can `println!` not just be a normal function?

Because it supports:

* a flexible number of arguments
* format string checking
* compile-time expansion into more detailed code

A normal function cannot mimic this exact syntax cleanly.

---

## 4. Common built-in macros you should know

These are the most important macros beginners use all the time.

## `println!`

Prints to standard output and adds a newline.

```rust
println!("Hello, Rust!");
println!("x = {}", 10);
```

## `print!`

Same as `println!`, but does not add a newline.

```rust
print!("Hello ");
print!("world");
```

## `eprintln!`

Prints to standard error.

```rust
eprintln!("Something went wrong");
```

## `format!`

Builds a `String` instead of printing.

```rust
let s = format!("name = {}", "JS");
```

## `dbg!`

Useful for debugging. Prints the expression and its value, then returns the value.

```rust
let x = 5;
let y = dbg!(x * 2);
```

## `vec!`

Creates a vector.

```rust
let nums = vec![1, 2, 3];
```

Without the macro, vector creation would be more verbose.

## `panic!`

Stops the program with an error message.

```rust
panic!("fatal error");
```

## `assert!`, `assert_eq!`, `assert_ne!`

Common in testing and debugging.

```rust
assert!(true);
assert_eq!(2 + 2, 4);
assert_ne!(1, 2);
```

## `include_str!` and `include_bytes!`

Include file contents at compile time.

```rust
let text = include_str!("data.txt");
```

---

## 5. What does the `!` mean?

In Rust, the `!` after a name means you are invoking a **macro**.

Example:

```rust
println!("hello");
```

This is different from a function call:

```rust
some_function();
```

So when you see `name!`, think:

> this is not an ordinary function; it is something expanded specially by the compiler.

---

## 6. Types of macros in Rust

There are two big categories you should know first.

## 6.1 Declarative macros (`macro_rules!`)

These are pattern-based macros. They are the easiest custom macros to start with.

They look like this:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}
```

Usage:

```rust
say_hello!();
```

### How to read this

* `macro_rules!` starts a macro definition
* `say_hello` is the macro name
* `()` means this pattern matches no input arguments
* `=>` means “expand into”
* the block after `=>` is the generated Rust code

---

## 6.2 Procedural macros

These are more advanced. They are written using Rust code that manipulates Rust syntax structures.

Common procedural macro kinds:

* custom `derive`
* attribute-like macros
* function-like procedural macros

Examples you have likely seen:

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
}
```

Here, `derive` is powered by procedural macros.

These are extremely useful, but much more advanced than `macro_rules!`.

For a first-pass understanding, focus on declarative macros first.

---

## 7. First custom macro example

```rust
macro_rules! greet {
    () => {
        println!("Hello from macro");
    };
}

fn main() {
    greet!();
}
```

### What happens?

When the compiler processes `greet!()`, it expands it into:

```rust
println!("Hello from macro");
```

So the macro saves you from writing repeated code manually.

---

## 8. Macro with arguments

```rust
macro_rules! greet_name {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet_name!("JS");
    greet_name!("Rust learner");
}
```

### What does `$name:expr` mean?

This is the core pattern system.

* `$name` means “capture this part and name it `name`”
* `expr` means “it must be a Rust expression”

So these all qualify as expressions:

```rust
greet_name!("JS");
greet_name!(1 + 2);
greet_name!(some_variable);
```

---

## 9. Common pattern fragment specifiers in `macro_rules!`

These matter a lot.

## `expr`

Matches a Rust expression.

```rust
($x:expr)
```

## `ident`

Matches an identifier, such as a variable or function name.

```rust
($name:ident)
```

## `ty`

Matches a type.

```rust
($t:ty)
```

## `item`

Matches an item such as a function, struct, module, etc.

```rust
($i:item)
```

## `stmt`

Matches a statement.

## `block`

Matches a block like `{ ... }`.

## `pat`

Matches a pattern.

## `tt`

Matches a token tree. This is more general and flexible.

You do not need to memorize them all immediately, but knowing `expr`, `ident`, `ty`, and `tt` helps a lot.

---

## 10. Repetition in macros

Macros become especially powerful when you can accept repeated input.

Example:

```rust
macro_rules! print_all {
    ($($x:expr),*) => {
        $(println!("{}", $x);)*
    };
}
```

Usage:

```rust
print_all!(10, 20, 30);
```

### How to read this

`$($x:expr),*` means:

* capture zero or more expressions
* separate them by commas

And this part:

```rust
$(println!("{}", $x);)*
```

means:

* repeat this code for each captured expression

So the expansion behaves like:

```rust
println!("{}", 10);
println!("{}", 20);
println!("{}", 30);
```

This kind of pattern is one of the most useful things in declarative macros.

---

## 11. Another useful custom macro example

```rust
macro_rules! make_function {
    ($name:ident) => {
        fn $name() {
            println!("You called {:?}", stringify!($name));
        }
    };
}

make_function!(hello);
make_function!(goodbye);

fn main() {
    hello();
    goodbye();
}
```

### What is happening here?

* `($name:ident)` captures a function name
* the macro generates a function definition using that identifier

This shows that macros can generate not just expressions, but whole Rust items like functions.

### What is `stringify!`?

`stringify!` is another macro that turns tokens into a string literal.

So `stringify!(hello)` becomes something like `"hello"`.

---

## 12. Hygiene in Rust macros

Rust macros have a concept called **hygiene**, which helps avoid some accidental name collisions.

This is one reason Rust macros are safer than classic C macros.

You do not need a deep formal definition yet. The practical idea is:

> Rust tries to keep macro-generated names and surrounding code from accidentally interfering in confusing ways.

C-style textual replacement does much less to protect you.

---

## 13. When should you use a macro?

Use a macro when you need one or more of these:

* variable argument syntax
* code generation
* boilerplate reduction
* syntax-level abstraction
* domain-specific syntax

Do **not** use a macro just because it seems clever.

Prefer a normal function when a function is enough.

### Good rule of thumb

Use a function if you are transforming values.

Use a macro if you are transforming or generating code-like structure.

---

## 14. Why beginners should be careful with macros

Macros are powerful, but they can also make code harder to read if overused.

Problems with bad macro usage:

* confusing generated behavior
* harder error messages
* more complex debugging
* harder onboarding for new readers

That is why Rust codebases often use macros selectively, not everywhere.

---

# Other Important Rust Concepts You Should Learn

Macros are important, but they are only one part of Rust. The concepts below matter even more for real Rust programming.

---

## 15. Ownership

Ownership is the biggest Rust idea.

Rust uses ownership to manage memory safely **without a garbage collector**.

### Main rule

Each value in Rust has an **owner**.

When the owner goes out of scope, the value is cleaned up.

Example:

```rust
fn main() {
    let s = String::from("hello");
}
```

When `main` ends, `s` goes out of scope, and the `String` memory is freed.

### Why this matters

Rust avoids many common bugs:

* double free
* use after free
* dangling pointers

---

## 16. Move semantics

Some types are moved when assigned.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

This does **not** copy the string contents by default.

Instead, ownership moves from `s1` to `s2`.

After that, `s1` is no longer valid.

Why? Because if both were allowed to free the same memory, that would be unsafe.

For cheap simple types like integers, Rust usually copies instead.

```rust
let a = 5;
let b = a;
```

This is fine because `i32` implements `Copy`.

---

## 17. Borrowing

Borrowing lets you use a value without taking ownership.

### Immutable borrow

```rust
fn print_len(s: &String) {
    println!("{}", s.len());
}
```

Here `&String` means “borrow a reference to a String”.

The function can read it, but does not own it.

### Mutable borrow

```rust
fn add_world(s: &mut String) {
    s.push_str(" world");
}
```

This lets the function modify the value.

### Core borrowing rule

At a given time, you can have either:

* many immutable references, or
* one mutable reference

This prevents data races and many mutation bugs.

---

## 18. References: `&` and `&mut`

These appear everywhere in Rust.

* `&T` means shared immutable reference
* `&mut T` means exclusive mutable reference

This is not just syntax noise. It is central to how Rust enforces safety.

---

## 19. `String` vs `&str`

This confuses almost every beginner.

## `String`

Owned, heap-allocated, growable string.

```rust
let s = String::from("hello");
```

## `&str`

Borrowed string slice.

```rust
let s = "hello";
```

A string literal is usually a `&'static str`.

Simple mental model:

* `String` → owns text
* `&str` → borrows text

---

## 20. Lifetimes

Lifetimes describe how long references remain valid.

Rust often infers lifetimes automatically.

You usually feel lifetimes most when returning references from functions.

Example shape:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

This says the returned reference is valid for at least the lifetime `'a` shared by the inputs.

Beginners often find lifetimes difficult, but they become more understandable once ownership and borrowing feel natural.

---

## 21. Structs and methods

Rust does not have classes in the classic Java/C++ sense, but it has something close enough for many uses.

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn greet(&self) {
        println!("Hi, I am {}", self.name);
    }
}
```

Think of this as:

* `struct` → data
* `impl` → behavior

---

## 22. Traits

Traits are one of Rust’s most important abstraction tools.

A trait defines shared behavior.

```rust
trait Speak {
    fn speak(&self);
}
```

Different types can implement it:

```rust
struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}
```

Traits are Rust’s main answer to:

* interfaces
* shared behavior
* polymorphism

---

## 23. Generics

Generics let you write code once for many types.

```rust
fn identity<T>(x: T) -> T {
    x
}
```

This works with different types because `T` is a placeholder type.

Generics are heavily used with traits.

```rust
fn show<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}
```

This says `T` can be any type that implements `Debug`.

---

## 24. Enums

Rust enums are much more powerful than plain C-style enums.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This means one value can be one of several structured variants.

Enums are heavily used for modeling state and errors.

---

## 25. Pattern matching

Rust’s `match` is a major language feature.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example use:

```rust
fn describe(x: Option<i32>) {
    match x {
        Some(v) => println!("value = {}", v),
        None => println!("no value"),
    }
}
```

Pattern matching is much more expressive than many languages’ `switch` statements.

---

## 26. `Option<T>` and `Result<T, E>`

These are extremely important.

## `Option<T>`

Represents an optional value.

* `Some(value)`
* `None`

This avoids null-pointer style mistakes.

## `Result<T, E>`

Represents success or failure.

* `Ok(value)`
* `Err(error)`

This is Rust’s main error-handling style.

Example:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

---

## 27. The `?` operator

This operator makes error propagation concise.

```rust
fn read_name() -> Result<String, std::io::Error> {
    let s = std::fs::read_to_string("name.txt")?;
    Ok(s)
}
```

If the read fails, the error is returned immediately.

If it succeeds, execution continues.

This is one of Rust’s most useful ergonomic features.

---

## 28. Modules and crates

These help organize code.

## Crate

A crate is a Rust compilation unit.

* binary crate → executable
* library crate → reusable package

## Module

A module helps structure code within a crate.

Example:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

---

## 29. `pub` and visibility

Items are private by default.

If you want to expose something outside a module, use `pub`.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Rust encourages explicit interfaces.

---

## 30. Iterators

Rust uses iterators a lot instead of raw index loops everywhere.

Example:

```rust
let nums = vec![1, 2, 3];
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

This style is concise, composable, and safe.

Closures appear often with iterators.

---

## 31. Closures

Closures are anonymous functions that can capture their environment.

```rust
let extra = 10;
let add_extra = |x| x + extra;
```

This is more flexible than a plain function because it can use nearby variables.

Closures are very common with iterators, callbacks, and async-style code.

---

## 32. Traits vs inheritance

Rust does not use classical inheritance the same way Java or C++ does.

Instead, Rust emphasizes:

* structs for data
* traits for shared behavior
* composition over inheritance

This is a major conceptual difference if you come from OOP-heavy languages.

---

## 33. Static vs dynamic dispatch

Rust supports both.

## Static dispatch

Uses generics and is usually resolved at compile time.

```rust
fn speak<T: Speak>(x: T) {
    x.speak();
}
```

## Dynamic dispatch

Uses trait objects.

```rust
fn speak_dyn(x: &dyn Speak) {
    x.speak();
}
```

Static dispatch is often faster.
Dynamic dispatch is often more flexible.

---

## 34. `Copy`, `Clone`, and `Debug`

These traits matter a lot.

## `Copy`

Type can be copied trivially.

Examples: integers, booleans, chars.

## `Clone`

Type can be duplicated explicitly.

```rust
let s2 = s1.clone();
```

## `Debug`

Type can be printed with `{:?}`.

Often derived automatically:

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
}
```

---

## 35. `derive` macros

This connects back to macros.

When you write:

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
}
```

Rust is using procedural macros to automatically generate implementations for those traits.

So macros are not just `println!`. They are also deeply connected to how traits get auto-implemented.

---

## 36. Unsafe Rust

Rust has an `unsafe` keyword for operations the compiler cannot fully verify.

Examples include:

* raw pointer dereferencing
* calling certain foreign functions
* manual low-level memory operations

Important point:

* most Rust is safe Rust
* `unsafe` is allowed, but carefully restricted

This is how Rust supports systems programming without making everything dangerous by default.

---

## 37. Concurrency and thread safety

Rust is famous for making concurrency safer.

Its ownership and borrowing rules help prevent data races at compile time.

This is one reason Rust is valued in systems and backend programming.

---

## 38. Cargo

Cargo is Rust’s tool for:

* creating projects
* building code
* running programs
* managing dependencies
* running tests
* generating docs

Example:

```bash
cargo new hello_rust
cargo run
cargo test
```

Cargo is much more than just a package installer.

---

## 39. Why Rust feels hard at first

Rust front-loads correctness.

That means beginners often struggle early with:

* ownership
* borrowing
* lifetimes
* trait bounds
* type inference errors

But the payoff is that many bugs are stopped before runtime.

A good mindset is:

> Rust is not trying to annoy you; it is trying to force correctness early.

---

## 40. Practical learning order

A good order for learning core Rust is:

1. variables, mutability, basic types
2. functions and expressions
3. ownership
4. borrowing and references
5. structs and methods
6. enums and `match`
7. `Option` and `Result`
8. traits and generics
9. iterators and closures
10. modules and Cargo
11. macros
12. lifetimes in more depth
13. async and concurrency later

Macros are important, but ownership/borrowing are even more central to everyday Rust.

---

# Final summary

## What a macro is

A macro in Rust is a compile-time code generation or transformation mechanism.

## How it differs from C macros

C macros are mostly text substitution. Rust macros are more structured and syntax-aware.

## Why macros exist

They support flexible syntax, variable arguments, boilerplate reduction, and code generation.

## Most common macro you already use

* `println!`
* `format!`
* `vec!`
* `dbg!`
* `assert_eq!`
* `panic!`

## Most important non-macro Rust topics

* ownership
* borrowing
* references
* structs
* traits
* generics
* enums
* pattern matching
* `Option` and `Result`
* modules and Cargo

If you understand those well, macros become much easier to place in the bigger picture.

---

# Mini reference examples

## Simple declarative macro

```rust
macro_rules! greet {
    () => {
        println!("hello");
    };
}
```

## Macro with expression argument

```rust
macro_rules! square_print {
    ($x:expr) => {
        println!("{}", $x * $x);
    };
}
```

## Macro with repetition

```rust
macro_rules! show_all {
    ($($x:expr),*) => {
        $(println!("{}", $x);)*
    };
}
```

## Trait example

```rust
trait Speak {
    fn speak(&self);
}
```

## Result example

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

## Borrowing example

```rust
fn print_name(name: &String) {
    println!("{}", name);
}
```

---

# Suggested next labs

1. Ownership and borrowing lab
2. Traits and polymorphism lab
3. Enums, Option, Result, and match lab
4. Modules and Cargo project structure lab
5. Macros lab with many custom `macro_rules!` examples
