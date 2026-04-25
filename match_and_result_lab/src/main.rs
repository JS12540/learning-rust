// ============================================================================
// RUST LAB 5: MATCH + PATTERN MATCHING
// ============================================================================
//
// This file is a full learning lab for:
//
// 1. match
// 2. pattern matching
// 3. enum matching
// 4. destructuring structs
// 5. destructuring tuples
// 6. matching Option and Result
// 7. match guards
// 8. ranges in patterns
// 9. if let
// 10. while let
// 11. _ and .. patterns
//
// This file is intentionally very long and heavily commented.
// The goal is not to be short.
// The goal is to explain WHAT is happening, WHY it works, and
// WHICH Rust keywords/concepts are involved.
//
// ----------------------------------------------------------------------------
// HOW TO RUN
// ----------------------------------------------------------------------------
//
// If using cargo project:
//     cargo run
//
// If using rustc directly:
//     rustc main.rs
//     ./main
//
// ============================================================================



// ============================================================================
// SECTION 1: #[derive(...)] AGAIN
// ============================================================================
//
// In Rust, traits define shared behavior.
//
// Example traits:
// - Debug  -> allows printing with {:?}
// - Clone  -> allows making an explicit duplicate
// - Copy   -> allows cheap stack copying for small simple types
// - PartialEq -> allows == comparison
//
// Writing these traits manually is possible, but repetitive.
//
// So Rust lets us use:
//
//     #[derive(Debug, Clone, Copy, PartialEq)]
//
// This asks the compiler to automatically generate trait implementations
// when possible.
//
// WHY WE USE THESE HERE:
// - Debug: so we can print values with {:?}
// - Clone: so we can duplicate values when needed
// - Copy: so small types like enums can be copied automatically
// - PartialEq: so we can compare with ==
//
// Note:
// Copy is only possible when all fields inside the type are also Copy.
// String is NOT Copy, so structs containing String cannot derive Copy.
// ============================================================================



// ============================================================================
// SECTION 2: SIMPLE ENUM FOR BASIC MATCHING
// ============================================================================
//
// enum = one value that can be one of several variants.
//
// This is perfect for match because match shines when a value
// can be "one of several possibilities".
//
// Here Direction can be:
// - North
// - South
// - East
// - West
//
// Only one variant exists at a time.
//
// We derive Debug, Clone, Copy so the enum is easy to print and reuse.
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}



// ============================================================================
// SECTION 3: ENUM WITH DATA
// ============================================================================
//
// One of Rust's biggest strengths:
//
// enum variants can carry data.
//
// This makes enums much more powerful than simple "labels".
//
// For example, a message in an app can be:
// - Quit
// - Move to some x/y coordinates
// - Write some text
// - Change color with RGB values
//
// Each variant can carry different data.
//
// Pattern matching is what lets us open and inspect that data safely.
//
// ============================================================================

#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}



// ============================================================================
// SECTION 4: STRUCT FOR DESTRUCTURING
// ============================================================================
//
// match does not only work on enums.
// It also works on structs, tuples, primitive values, references, ranges,
// and more.
//
// A struct groups related fields together.
//
// Later we will destructure this struct in match.
//
// ============================================================================

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}



// ============================================================================
// SECTION 5: ANOTHER STRUCT
// ============================================================================
//
// This one is small and useful for geometric examples.
//
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}



// ============================================================================
// SECTION 6: WHAT IS "match"?
// ============================================================================
//
// match is a control flow expression in Rust.
//
// KEYWORD:
//     match
//
// SYNTAX:
//
//     match value {
//         pattern1 => expression1,
//         pattern2 => expression2,
//         pattern3 => expression3,
//     }
//
// IMPORTANT IDEAS:
//
// 1. match compares a value against patterns
// 2. the first matching arm runs
// 3. match must be EXHAUSTIVE
// 4. match is an EXPRESSION, not just a statement
//
// "Exhaustive" means:
// Rust forces you to handle every possible case.
//
// This is one reason Rust is safe.
// You cannot accidentally ignore a possible enum variant unless you use
// a catch-all pattern like _.
//
// ============================================================================



// ============================================================================
// SECTION 7: SIMPLE VALUE MATCHING
// ============================================================================
//
// Here we match on an integer.
//
// Pattern examples:
// - exact value: 0
// - exact value: 1
// - wildcard: _
//
// "_" means "anything else".
// It is called the wildcard pattern.
//
// ============================================================================

fn basic_number_match(n: i32) {
    println!("\n--- basic_number_match({}) ---", n);

    match n {
        0 => println!("n is zero"),
        1 => println!("n is one"),
        2 => println!("n is two"),
        _ => println!("n is something else"),
    }
}



// ============================================================================
// SECTION 8: MATCH IS AN EXPRESSION
// ============================================================================
//
// In many languages, switch is only for control flow.
//
// In Rust, match returns a value.
//
// That means you can do:
//
//     let result = match value {
//         ...
//     };
//
// Every arm should produce a compatible type.
//
// Here every arm returns &str.
//
// ============================================================================

fn number_to_word(n: i32) -> &'static str {
    let word = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many or unknown",
    };

    word
}



// ============================================================================
// SECTION 9: MATCHING ENUMS
// ============================================================================
//
// This is one of the most common uses of match.
//
// Since Direction can be exactly one of its variants,
// match is the natural tool.
//
// Notice there is no "_" arm here.
// Why?
// Because we are listing every possible Direction variant.
// This is exhaustive.
//
// If we forget one variant, Rust will not compile.
//
// ============================================================================

fn print_direction(direction: Direction) {
    println!("\n--- print_direction({:?}) ---", direction);

    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
    }
}



// ============================================================================
// SECTION 10: MATCHING ENUMS WITH DATA
// ============================================================================
//
// When enum variants carry data, patterns can "destructure" that data.
//
// DESTRUCTURE means:
// break apart a value into its internal pieces.
//
// For example:
//
// Message::Move { x, y }
//
// means:
// "If the value is Message::Move, bind its x field to local variable x
// and bind its y field to local variable y."
//
// Message::Write(text)
//
// means:
// "If the value is Write, bind the inner String to text."
//
// ============================================================================

fn handle_message(msg: Message) {
    println!("\n--- handle_message({:?}) ---", msg);

    match msg {
        Message::Quit => {
            println!("Quit variant: stop the program or close the screen");
        }

        Message::Move { x, y } => {
            println!("Move variant: x = {}, y = {}", x, y);
        }

        Message::Write(text) => {
            println!("Write variant: text = {}", text);
        }

        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor variant: red={}, green={}, blue={}", r, g, b);
        }
    }
}



// ============================================================================
// SECTION 11: MATCHING STRUCTS
// ============================================================================
//
// match can destructure a struct by field names.
//
// Pattern:
//
//     Person { name, age, active }
//
// This means:
// - match a Person
// - bind its name field to local variable name
// - bind its age field to local variable age
// - bind its active field to local variable active
//
// Since name is a String and we take `person` by value here,
// ownership of the String moves into the match arm.
//
// That is okay because we are consuming person in this function.
//
// ============================================================================

fn describe_person(person: Person) {
    println!("\n--- describe_person({:?}) ---", person);

    match person {
        Person { name, age, active } => {
            println!(
                "Person destructured -> name: {}, age: {}, active: {}",
                name, age, active
            );
        }
    }
}



// ============================================================================
// SECTION 12: MATCHING ONLY SOME STRUCT FIELDS
// ============================================================================
//
// Sometimes you do not care about every field.
//
// Rust provides:
//
//     ..
//
// Called the "rest" pattern.
//
// It means:
// "ignore the remaining fields"
//
// Example:
//
//     Point { x: 0, .. }
//
// means:
// - match a Point
// - require x to be 0
// - ignore all other fields
//
// ============================================================================

fn classify_point(point: Point) {
    println!("\n--- classify_point({:?}) ---", point);

    match point {
        Point { x: 0, y: 0 } => println!("Point is at the origin"),
        Point { x: 0, y } => println!("Point is on the Y axis at y = {}", y),
        Point { x, y: 0 } => println!("Point is on the X axis at x = {}", x),
        Point { x, y } => println!("General point at ({}, {})", x, y),
    }
}



// ============================================================================
// SECTION 13: MATCHING TUPLES
// ============================================================================
//
// Tuples are fixed-size collections where position matters.
//
// Example tuple type:
//     (i32, i32)
//
// Pattern matching can destructure tuples.
//
// ============================================================================

fn describe_pair(pair: (i32, i32)) {
    println!("\n--- describe_pair({:?}) ---", pair);

    match pair {
        (0, 0) => println!("Both values are zero"),
        (0, y) => println!("First is zero, second is {}", y),
        (x, 0) => println!("Second is zero, first is {}", x),
        (x, y) => println!("General pair: ({}, {})", x, y),
    }
}



// ============================================================================
// SECTION 14: NESTED PATTERN MATCHING
// ============================================================================
//
// Patterns can be nested.
//
// Here we match a tuple whose second value is a Direction.
//
// This shows that Rust patterns compose very naturally.
//
// ============================================================================

fn nested_patterns(input: (i32, Direction)) {
    println!("\n--- nested_patterns({:?}) ---", input);

    match input {
        (0, Direction::North) => println!("Zero and North"),
        (0, dir) => println!("Zero and some other direction: {:?}", dir),
        (steps, Direction::East) => println!("{} steps to the East", steps),
        (steps, dir) => println!("{} steps toward {:?}", steps, dir),
    }
}



// ============================================================================
// SECTION 15: MATCHING Option<T>
// ============================================================================
//
// Option is one of the most important enums in Rust.
//
// Rough idea:
//
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// Why it exists:
// Instead of null, Rust makes "absence of value" explicit.
//
// Pattern matching is the standard way to handle Option.
//
// ============================================================================

fn print_optional_number(value: Option<i32>) {
    println!("\n--- print_optional_number({:?}) ---", value);

    match value {
        Some(number) => println!("We have a number: {}", number),
        None => println!("There is no number"),
    }
}



// ============================================================================
// SECTION 16: MATCHING Result<T, E>
// ============================================================================
//
// Result is another core enum in Rust.
//
// Rough idea:
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//
// It is used for operations that can succeed or fail.
//
// match is a natural way to handle both possibilities.
//
// ============================================================================

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn print_division_result(result: Result<f64, String>) {
    println!("\n--- print_division_result({:?}) ---", result);

    match result {
        Ok(value) => println!("Division succeeded: {}", value),
        Err(message) => println!("Division failed: {}", message),
    }
}



// ============================================================================
// SECTION 17: WILDCARD PATTERN "_"
// ============================================================================
//
// "_" means:
// "I do not care about this value"
//
// This is useful when:
// - you do not need the value
// - you just need to cover remaining cases
//
// Example:
//
//     match n {
//         0 => ...
//         _ => ...
//     }
//
// "_" does NOT bind a variable.
// It simply ignores the matched value.
//
// ============================================================================

fn wildcard_examples(n: i32) {
    println!("\n--- wildcard_examples({}) ---", n);

    match n {
        1 => println!("Exactly one"),
        _ => println!("Anything other than one"),
    }
}



// ============================================================================
// SECTION 18: NAMED BINDINGS INSIDE PATTERNS
// ============================================================================
//
// When a pattern matches, parts of the value can be bound to variables.
//
// Example:
//     Some(x)
//
// If Option is Some(42), then x becomes 42.
//
// This is often called "binding".
//
// ============================================================================

fn binding_example(value: Option<i32>) {
    println!("\n--- binding_example({:?}) ---", value);

    match value {
        Some(x) => println!("Matched Some, bound inner value to x = {}", x),
        None => println!("Matched None"),
    }
}



// ============================================================================
// SECTION 19: MATCHING RANGES
// ============================================================================
//
// Rust allows ranges in patterns.
//
// Common forms:
// - 1..=5  -> inclusive range from 1 through 5
//
// IMPORTANT:
// In patterns, ranges are mostly used with char and numeric types.
//
// ============================================================================

fn classify_age(age: u32) {
    println!("\n--- classify_age({}) ---", age);

    match age {
        0..=12 => println!("Child"),
        13..=17 => println!("Teen"),
        18..=64 => println!("Adult"),
        _ => println!("Senior"),
    }
}



// ============================================================================
// SECTION 20: MATCH GUARDS
// ============================================================================
//
// A match guard is an extra condition after a pattern.
//
// Syntax:
//
//     pattern if condition => ...
//
// This lets you say:
// "Match this pattern, but only if some additional rule is true"
//
// Example:
//     Some(x) if x > 10 => ...
//
// Pattern must match first.
// Then the guard condition is checked.
//
// ============================================================================

fn match_guard_example(value: Option<i32>) {
    println!("\n--- match_guard_example({:?}) ---", value);

    match value {
        Some(x) if x < 0 => println!("Negative number: {}", x),
        Some(0) => println!("Exactly zero"),
        Some(x) if x > 0 && x <= 10 => println!("Positive small number: {}", x),
        Some(x) => println!("Positive bigger number: {}", x),
        None => println!("No number present"),
    }
}



// ============================================================================
// SECTION 21: MATCHING MULTIPLE PATTERNS WITH |
// ============================================================================
//
// "|" means "or" in pattern matching.
//
// Example:
//
//     1 | 2 | 3 => ...
//
// This means:
// match if the value is 1 or 2 or 3.
//
// ============================================================================

fn multiple_patterns(n: i32) {
    println!("\n--- multiple_patterns({}) ---", n);

    match n {
        1 | 2 | 3 => println!("One, two, or three"),
        4 | 5 => println!("Four or five"),
        _ => println!("Something else"),
    }
}



// ============================================================================
// SECTION 22: @ BINDING
// ============================================================================
//
// Rust has a very useful pattern feature:
//
//     name @ pattern
//
// This means:
// - check that the value matches the pattern
// - also bind the whole matched value to a variable
//
// Example:
//     value @ 1..=5
//
// This means:
// if value is within 1..=5, bind that exact value to `value`.
//
// ============================================================================

fn at_binding_example(n: i32) {
    println!("\n--- at_binding_example({}) ---", n);

    match n {
        small @ 1..=5 => println!("Small number matched by range, actual value = {}", small),
        medium @ 6..=10 => println!("Medium number, actual value = {}", medium),
        other => println!("Other number = {}", other),
    }
}



// ============================================================================
// SECTION 23: IGNORING PARTS OF A TUPLE WITH "_"
// ============================================================================
//
// Sometimes you want only part of a value.
//
// Example:
//     (x, _)
//
// means:
// - bind first element to x
// - ignore second element
//
// ============================================================================

fn first_value_only(pair: (i32, i32)) {
    println!("\n--- first_value_only({:?}) ---", pair);

    match pair {
        (x, _) => println!("First value is {}, second is ignored", x),
    }
}



// ============================================================================
// SECTION 24: USING ".." IN TUPLES
// ============================================================================
//
// ".." can also mean "ignore the rest".
//
// Example with a 5-element tuple:
//     (first, .., last)
//
// This binds the first and last values and ignores the middle.
//
// ============================================================================

fn tuple_rest_example(values: (i32, i32, i32, i32, i32)) {
    println!("\n--- tuple_rest_example({:?}) ---", values);

    match values {
        (first, .., last) => {
            println!("First = {}, last = {}, middle values ignored", first, last);
        }
    }
}



// ============================================================================
// SECTION 25: if let
// ============================================================================
//
// if let is a shorter form used when you only care about ONE pattern
// and do not need a full match.
//
// Full match:
//
//     match value {
//         Some(x) => ...
//         None => ...
//     }
//
// If you only care about Some(x), you can write:
//
//     if let Some(x) = value {
//         ...
//     }
//
// KEYWORDS:
// - if
// - let
//
// WHY IT EXISTS:
// It reduces boilerplate when a full exhaustive match would be too much.
//
// ============================================================================

fn if_let_example(value: Option<i32>) {
    println!("\n--- if_let_example({:?}) ---", value);

    if let Some(x) = value {
        println!("if let matched Some(x), x = {}", x);
    } else {
        println!("if let did not match Some(x)");
    }
}



// ============================================================================
// SECTION 26: while let
// ============================================================================
//
// while let is like:
// "keep looping while this pattern continues to match"
//
// Very common for:
// - repeatedly popping from a vector
// - iterating custom state machines
// - consuming optional values in loops
//
// Here we use Vec::pop(), which returns Option<T>:
// - Some(last_item) if vector still has values
// - None if vector is empty
//
// ============================================================================

fn while_let_example() {
    println!("\n--- while_let_example() ---");

    let mut stack = vec![10, 20, 30, 40];

    while let Some(top) = stack.pop() {
        println!("Popped value = {}", top);
    }

    println!("Stack is now empty");
}



// ============================================================================
// SECTION 27: match WITH REFERENCES
// ============================================================================
//
// Rust values can be matched by value or by reference.
//
// Here input is:
//
//     &Option<i32>
//
// That means "a shared reference to an Option<i32>"
//
// Pattern:
//
//     Some(x)
//
// works because pattern matching can handle references in many cases
// through match ergonomics.
//
// To keep this beginner-friendly, the main point is:
// Rust can match borrowed values too.
//
// ============================================================================

fn match_reference_example(input: &Option<i32>) {
    println!("\n--- match_reference_example({:?}) ---", input);

    match input {
        Some(x) => println!("Borrowed Option contains {}", x),
        None => println!("Borrowed Option is None"),
    }
}



// ============================================================================
// SECTION 28: DESCRIBING Message WITH MORE SPECIFIC PATTERNS
// ============================================================================
//
// Pattern matching is powerful because you can match exact variants,
// exact field values, ranges, and fallbacks.
//
// Here we match specific interesting shapes of Message.
//
// ============================================================================

fn advanced_message_match(msg: Message) {
    println!("\n--- advanced_message_match({:?}) ---", msg);

    match msg {
        Message::Quit => println!("Quit command received"),

        Message::Move { x: 0, y: 0 } => {
            println!("Move command to origin");
        }

        Message::Move { x, y: 0 } => {
            println!("Move command along X axis only, x = {}", x);
        }

        Message::Move { x: 0, y } => {
            println!("Move command along Y axis only, y = {}", y);
        }

        Message::Move { x, y } => {
            println!("General move command to ({}, {})", x, y);
        }

        Message::Write(text) if text.is_empty() => {
            println!("Write command with empty string");
        }

        Message::Write(text) => {
            println!("Write command with text: {}", text);
        }

        Message::ChangeColor(255, 0, 0) => {
            println!("Change color to pure red");
        }

        Message::ChangeColor(r, g, b) => {
            println!("Change color to rgb({}, {}, {})", r, g, b);
        }
    }
}



// ============================================================================
// SECTION 29: MATCHING char VALUES
// ============================================================================
//
// char in Rust represents a Unicode scalar value.
//
// Pattern matching on char is common.
//
// ============================================================================

fn classify_char(c: char) {
    println!("\n--- classify_char({}) ---", c);

    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Lowercase vowel"),
        'A' | 'E' | 'I' | 'O' | 'U' => println!("Uppercase vowel"),
        '0'..='9' => println!("Digit"),
        _ => println!("Other character"),
    }
}



// ============================================================================
// SECTION 30: WHY match IS SO IMPORTANT IN RUST
// ============================================================================
//
// match is central in Rust because:
//
// 1. Rust uses enums heavily
// 2. Option and Result are enums
// 3. Rust encourages explicit handling of cases
// 4. exhaustive matching prevents forgotten branches
// 5. pattern matching makes code expressive and safe
//
// Instead of scattered if/else logic, Rust often encourages
// describing all valid shapes of data directly.
//
// ============================================================================



// ============================================================================
// SECTION 31: MAIN
// ============================================================================
//
// This is where the program starts.
// We call all the example functions one by one.
//
// ============================================================================

fn main() {
    println!("====================================================");
    println!("RUST LAB 5: MATCH + PATTERN MATCHING");
    println!("====================================================");

    // ------------------------------------------------------------------------
    // 1. Basic integer matching
    // ------------------------------------------------------------------------
    basic_number_match(0);
    basic_number_match(7);

    // ------------------------------------------------------------------------
    // 2. match as expression
    // ------------------------------------------------------------------------
    println!("\n--- number_to_word examples ---");
    println!("0 => {}", number_to_word(0));
    println!("3 => {}", number_to_word(3));
    println!("99 => {}", number_to_word(99));

    // ------------------------------------------------------------------------
    // 3. Enum matching
    // ------------------------------------------------------------------------
    print_direction(Direction::North);
    print_direction(Direction::South);
    print_direction(Direction::East);
    print_direction(Direction::West);

    // ------------------------------------------------------------------------
    // 4. Enum with data
    // ------------------------------------------------------------------------
    handle_message(Message::Quit);
    handle_message(Message::Move { x: 10, y: 20 });
    handle_message(Message::Write("hello from rust".to_string()));
    handle_message(Message::ChangeColor(10, 20, 30));

    // ------------------------------------------------------------------------
    // 5. Struct destructuring
    // ------------------------------------------------------------------------
    let person = Person {
        name: "Jay".to_string(),
        age: 28,
        active: true,
    };
    describe_person(person);

    // ------------------------------------------------------------------------
    // 6. Struct pattern matching with fields
    // ------------------------------------------------------------------------
    classify_point(Point { x: 0, y: 0 });
    classify_point(Point { x: 0, y: 7 });
    classify_point(Point { x: 9, y: 0 });
    classify_point(Point { x: 4, y: 5 });

    // ------------------------------------------------------------------------
    // 7. Tuple matching
    // ------------------------------------------------------------------------
    describe_pair((0, 0));
    describe_pair((0, 8));
    describe_pair((9, 0));
    describe_pair((3, 4));

    // ------------------------------------------------------------------------
    // 8. Nested patterns
    // ------------------------------------------------------------------------
    nested_patterns((0, Direction::North));
    nested_patterns((0, Direction::West));
    nested_patterns((15, Direction::East));
    nested_patterns((20, Direction::South));

    // ------------------------------------------------------------------------
    // 9. Option matching
    // ------------------------------------------------------------------------
    print_optional_number(Some(42));
    print_optional_number(None);

    // ------------------------------------------------------------------------
    // 10. Result matching
    // ------------------------------------------------------------------------
    let result1 = safe_divide(10.0, 2.0);
    let result2 = safe_divide(10.0, 0.0);
    print_division_result(result1);
    print_division_result(result2);

    // ------------------------------------------------------------------------
    // 11. Wildcard
    // ------------------------------------------------------------------------
    wildcard_examples(1);
    wildcard_examples(999);

    // ------------------------------------------------------------------------
    // 12. Binding values
    // ------------------------------------------------------------------------
    binding_example(Some(123));
    binding_example(None);

    // ------------------------------------------------------------------------
    // 13. Ranges
    // ------------------------------------------------------------------------
    classify_age(8);
    classify_age(15);
    classify_age(30);
    classify_age(80);

    // ------------------------------------------------------------------------
    // 14. Match guards
    // ------------------------------------------------------------------------
    match_guard_example(Some(-5));
    match_guard_example(Some(0));
    match_guard_example(Some(7));
    match_guard_example(Some(25));
    match_guard_example(None);

    // ------------------------------------------------------------------------
    // 15. Multiple patterns
    // ------------------------------------------------------------------------
    multiple_patterns(2);
    multiple_patterns(5);
    multiple_patterns(9);

    // ------------------------------------------------------------------------
    // 16. @ binding
    // ------------------------------------------------------------------------
    at_binding_example(4);
    at_binding_example(8);
    at_binding_example(100);

    // ------------------------------------------------------------------------
    // 17. Ignore values with _
    // ------------------------------------------------------------------------
    first_value_only((99, 123));

    // ------------------------------------------------------------------------
    // 18. Ignore middle with ..
    // ------------------------------------------------------------------------
    tuple_rest_example((1, 2, 3, 4, 5));

    // ------------------------------------------------------------------------
    // 19. if let
    // ------------------------------------------------------------------------
    if_let_example(Some(55));
    if_let_example(None);

    // ------------------------------------------------------------------------
    // 20. while let
    // ------------------------------------------------------------------------
    while_let_example();

    // ------------------------------------------------------------------------
    // 21. Match references
    // ------------------------------------------------------------------------
    let borrowed_some = Some(77);
    let borrowed_none: Option<i32> = None;
    match_reference_example(&borrowed_some);
    match_reference_example(&borrowed_none);

    // ------------------------------------------------------------------------
    // 22. Advanced message matching
    // ------------------------------------------------------------------------
    advanced_message_match(Message::Move { x: 0, y: 0 });
    advanced_message_match(Message::Move { x: 5, y: 0 });
    advanced_message_match(Message::Move { x: 0, y: 9 });
    advanced_message_match(Message::Move { x: 7, y: 8 });
    advanced_message_match(Message::Write("".to_string()));
    advanced_message_match(Message::Write("pattern matching is powerful".to_string()));
    advanced_message_match(Message::ChangeColor(255, 0, 0));
    advanced_message_match(Message::ChangeColor(12, 34, 56));

    // ------------------------------------------------------------------------
    // 23. Match chars
    // ------------------------------------------------------------------------
    classify_char('a');
    classify_char('U');
    classify_char('7');
    classify_char('#');

    println!("\n====================================================");
    println!("LAB COMPLETE");
    println!("====================================================");
}



// ============================================================================
// FINAL SUMMARY
// ============================================================================
//
// KEYWORDS COVERED:
//
// match
// - compares a value against patterns
// - exhaustive
// - returns a value because it is an expression
//
// if let
// - shorter form when only one pattern matters
//
// while let
// - keep looping while a pattern matches
//
// enum
// - one of many variants
//
// struct
// - groups related fields
//
// Some / None
// - variants of Option<T>
//
// Ok / Err
// - variants of Result<T, E>
//
// _
// - wildcard, ignore any value
//
// ..
// - ignore the rest
//
// |
// - multiple patterns ("or")
//
// if in a match arm
// - match guard
//
// @
// - bind the matched value while also checking a pattern
//
// WHY PATTERN MATCHING MATTERS:
//
// Pattern matching lets Rust code describe the SHAPE of data.
// That makes code:
// - safer
// - clearer
// - more explicit
// - easier to reason about
//
// Most importantly, match works beautifully with enums,
// and enums are everywhere in Rust.
//
// ============================================================================
