// `use` keyword:
// Imports items (functions, traits, structs, etc.) from modules.
// Here we import `fmt` (formatting utilities) from standard library.
use std::fmt;

// `enum` keyword:
// Used to define a type that can be one of multiple variants.
// Think of it like a set of named options.

// `derive(Debug)`:
// Automatically implements Debug trait so we can print it using {:?}

// =============================================================
// SIMPLE LOGGER (custom)
// =============================================================
#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

// Function to log messages
fn log(level: LogLevel, message: &str) {
    // `{level:?}` means debug print
    println!("[{level:?}] {message}");
}

// =============================================================
// STRUCTS (custom data types)
// =============================================================

// `struct` keyword:
// Used to define custom data types (like objects)
#[derive(Debug)]
struct Person {
    name: String, // owned string
    age: u32,     // unsigned integer
}

// Tuple struct (no field names)
#[derive(Debug)]
struct Point(i32, i32);

// Unit-like struct (no data)
#[derive(Debug)]
struct UnitLike;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// =============================================================
// FUNCTIONS
// =============================================================

// Basic function
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// Function with return type
fn add(a: i32, b: i32) -> i32 {
    a + b // last line without `;` = return value
}


// Explicit return
fn square(num: i32) -> i32 {
    return num * num;
}

fn print_direction(direction: Direction){
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}

// Generic function
// `<T: fmt::Display>` means T must support printing
fn print_line<T: fmt::Display>(label: &str, value: T) {
    println!("{label}: {value}");
}

fn main() {
    log(LogLevel::Info, "Program started");

    // ----------------------------------------------------------
    // VARIABLES & DATA TYPES
    // ----------------------------------------------------------

    // `let` keyword: declare variable (immutable by default)
    let x: i32 = 10;

    // `mut` keyword: makes variable mutable (changeable)
    let mut y = 20;
    y += 5;

    // Basic types
    let int = 42;          // integer
    let float = 3.14;      // floating point
    let is_true = true;    // boolean
    let letter = 'A';      // char

    print_line("int", int);
    print_line("float", float);
    print_line("bool", is_true);
    print_line("char", letter);

    // ----------------------------------------------------------
    // TUPLES
    // ----------------------------------------------------------
    let tuple = ("Jay", 25, true);
    println!("Tuple: {:?}", tuple);

    // Access tuple
    println!("Name: {}", tuple.0);

    // ----------------------------------------------------------
    // ARRAYS
    // ----------------------------------------------------------
    let arr = [1, 2, 3, 4];
    println!("Array: {:?}", arr);

    // Fixed-size array
    let fixed: [i32; 3] = [10, 20, 30];

    // 2D array
    let matrix = [[1, 2], [3, 4]];

    // ----------------------------------------------------------
    // STRINGS
    // ----------------------------------------------------------

    // &str = string slice (immutable, borrowed)
    let s1 = "hello";

    // String = owned, mutable string
    let mut s2 = String::from("world");
    s2.push('!');

    println!("{} {}", s1, s2);

    // ----------------------------------------------------------
    // VECTORS (dynamic arrays)
    // ----------------------------------------------------------
    let mut vec = vec![1, 2, 3];
    vec.push(4);

    // ----------------------------------------------------------
    // REFERENCES (POINTERS)
    // ----------------------------------------------------------

    let val = 100;

    // `&` = reference (like pointer)
    let ref_val = &val;

    println!("ref points to: {}", *ref_val); // `*` dereference

    // Mutable reference
    let mut num = 5;
    let r = &mut num;
    *r += 1;

    // Raw pointer (unsafe)
    let raw: *const i32 = &val;
    println!("raw pointer: {:p}", raw);

    // ----------------------------------------------------------
    // STRUCT USAGE
    // ----------------------------------------------------------
    let person = Person {
        name: String::from("Jay"),
        age: 30,
    };

    println!("Person: {:?}", person);

    let point = Point(1, 2);
    println!("Point: {:?}", point);

    // ----------------------------------------------------------
    // PRINTING
    // ----------------------------------------------------------
    println!("Normal print");
    print!("Inline print -> ");
    println!("continued");
    eprintln!("Error print");

    // Formatting
    println!("Binary: {:b}", 10);
    println!("Hex: {:x}", 255);
    println!("Float: {:.2}", 3.14159);

    // ----------------------------------------------------------
    // CONDITIONS
    // ----------------------------------------------------------
    let marks = 75;

    if marks > 80 {
        println!("A grade");
    } else {
        println!("B grade");
    }

    // Match (like switch)
    let day = 1;
    match day {
        1 => println!("Monday"),
        _ => println!("Other"),
    }

    // ----------------------------------------------------------
    // LOOPS
    // ----------------------------------------------------------

    for i in 1..=3 {
        println!("for: {}", i);
    }

    let mut i = 0;
    while i < 2 {
        println!("while: {}", i);
        i += 1;
    }

    loop {
        println!("loop once");
        break;
    }

    // ----------------------------------------------------------
    // OWNERSHIP & BORROWING
    // ----------------------------------------------------------

    let s = String::from("hello");
    borrow(&s);

    println!("Still usable: {}", s);

    // ----------------------------------------------------------
    // OPTION (value or none)
    // ----------------------------------------------------------
    let some = Some(10);
    let none: Option<i32> = None;

    println!("Option: {:?}", some);
    println!("None: {:?}", none);

    // ----------------------------------------------------------
    // RESULT (error handling)
    // ----------------------------------------------------------
    let res = divide(10.0, 2.0);
    let bad = divide(10.0, 0.0);

    println!("Result ok: {:?}", res);
    println!("Result err: {:?}", bad);

    log(LogLevel::Info, "Program finished");
}

// Borrowing function
fn borrow(s: &String) {
    println!("Borrowed: {}", s);
}

// Result example
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Divide by zero"))
    } else {
        Ok(a / b)
    }
}

/*
KEYWORDS SUMMARY:

use      -> import modules
let      -> variable declaration
mut      -> mutable variable
fn       -> function
struct   -> custom data type
enum     -> multiple variants type
match    -> pattern matching
if/else  -> condition
loop     -> infinite loop
for      -> iteration
while    -> conditional loop
&        -> reference
*        -> dereference
Option   -> value or none
Result   -> success or error

*/
