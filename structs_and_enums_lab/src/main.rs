// ================================================================
// RUST LAB: STRUCTS + ENUMS
// This file is written as a teaching lab, so the comments are long
// and detailed on purpose.
//
// Run it with:
//     rustc main.rs
//     ./main
//
// This lab teaches:
// 1. What a struct is
// 2. What an enum is
// 3. Why they exist
// 4. How to create them
// 5. How to use methods with impl
// 6. How pattern matching works with match
// 7. How enums can store different kinds of data
// 8. The difference between tuple structs and named-field structs
// 9. Why Option<T> is an enum
// 10. Why Result<T, E> is an enum
// ================================================================

// ------------------------------------------------
// A STRUCT
// ------------------------------------------------
// A struct is used to GROUP RELATED DATA together.
//
// Think of a struct like a "custom type" that you design yourself.
//
// Example:
// A person has a name, age, and whether they are active.
// These values belong together, so a struct is a great fit.
//
// Structs are used when:
// - You want to represent a thing/object/entity
// - That thing has multiple properties
// - Those properties belong together logically
//
// Real-world examples:
// - User { id, email, active }
// - Product { name, price, stock }
// - Point { x, y }
// - Car { brand, model, year }
//
// Syntax:
// struct StructName {
//     field_name: Type,
//     another_field: Type,
// }
//
// Below is a named-field struct called User.
// "pub" is NOT needed here because everything is in one file
// and we are not making a library.
// ------------------------------------------------

// ================================================================
// 🔷 WHAT IS #[derive(...)] ?
// ================================================================
//
// In Rust, many common functionalities are implemented via "traits".
//
// A TRAIT is like a capability or behavior.
// Examples:
// - Debug   → allows printing with {:?}
// - Clone   → allows making a copy
// - Copy    → allows simple bitwise copy (no ownership move)
//
// Writing these manually is tedious.
//
// So Rust provides:
//     #[derive(Trait1, Trait2, ...)]
//
// This tells the compiler:
// 👉 "Automatically generate code for these traits"
//
// Example below:
// - Debug → so we can print using {:?}
// - Clone → so we can duplicate values
//
// IMPORTANT:
// Derive only works when it's safe for Rust to auto-generate logic.
//
// ================================================================
#[derive(Debug, Clone)]
struct User {
    // String is a heap-allocated, growable UTF-8 string.
    // It OWNS its data (important for ownership model).
    name: String,

    // Primitive types like u32 are stored on the stack.
    age: u32,

    // bool = true or false
    active: bool,
}

// ================================================================
// 🔷 impl BLOCK
// ================================================================
//
// impl = implementation block
//
// This is where we define behavior for a type.
//
// Think:
// struct = data
// impl   = behavior
//
// Together they form a complete abstraction.
//
// ================================================================

// ------------------------------------------------
// IMPLEMENTATION BLOCK FOR STRUCT
// ------------------------------------------------
// "impl" lets us attach functions to a type.
//
// Functions inside impl are often called "methods"
// when they take self, &self, or &mut self.
//
// Why use impl?
// - To keep behavior close to the data
// - To make code organized
// - To model "what this type can do"
//
// There are 2 common styles:
//
// 1. Associated function
//    - Does not take self
//    - Often used like a constructor
//    - Example: User::new(...)
//
// 2. Method
//    - Takes self, &self, or &mut self
//    - Called on an instance
//    - Example: user.describe()
//
// &self means:
// "borrow this value immutably"
// So we can read it, but not change it.
//
// &mut self means:
// "borrow this value mutably"
// So we can change fields.
//
// self means:
// "take ownership of the value"
// ------------------------------------------------
impl User {
    // Associated function: acts like a constructor.
    // Rust does not have "constructors" in the same special way as some languages,
    // but this pattern is extremely common.
    fn new(name: &str, age: u32, active: bool) -> Self {
        Self {
            name: name.to_string(),
            age,
            active,
        }
    }

    // Method that only reads data.
    // ------------------------------------------------------------
    // METHOD WITH &self
    // ------------------------------------------------------------
    //
    // &self means:
    // - Borrow the struct immutably
    // - Can READ but NOT modify
    //
    fn describe(&self) {
        println!("User => name: {}, age: {}, active: {}", self.name, self.age, self.active);
    }

    // Method that changes data.
    // ------------------------------------------------------------
    // METHOD WITH &mut self
    // ------------------------------------------------------------
    //
    // &mut self means:
    // - Borrow mutably
    // - Can MODIFY fields
    //
    fn birthday(&mut self) {
        self.age += 1;
    }

    // Another method that changes a field.
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Method returning a value.
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

// ================================================================
// 🔷 TUPLE STRUCT
// ================================================================
//
// Difference from normal struct:
// - No named fields
// - Access using index (.0, .1, .2)
//
// When to use?
// - Lightweight grouping
// - Names not important
//
// Copy trait:
// - Allows implicit copying (stack copy)
// - Only works when all fields are Copy types
//
// ================================================================

// ------------------------------------------------
// A TUPLE STRUCT
// ------------------------------------------------
// Rust also supports tuple structs.
//
// A tuple struct groups values together, but fields are accessed by position
// instead of by name.
//
// Example:
// struct Color(u8, u8, u8);
//
// Why use tuple structs?
// - When names are not very important
// - When you want a distinct type
// - When you want something lightweight
//
// Example use cases:
// - RGB color
// - Coordinates
// - Wrapper type around another value
//
// Access fields using .0, .1, .2 etc.
// ------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

// ------------------------------------------------
// A UNIT-LIKE STRUCT
// ------------------------------------------------
// A unit-like struct has no fields at all.
//
// Why does this exist?
// It is useful when:
// - You want a type with no data
// - You want to mark something
// - You want behavior without stored fields
//
// It is less common for beginners, but good to know.
// ------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct Logger;

// ------------------------------------------------
// ENUM
// ------------------------------------------------
// An enum is used when a value can be ONE OF SEVERAL POSSIBILITIES.
//
// "enum" means enumeration: a list of possible variants.
//
// This is different from a struct.
//
// Struct:
// - One thing with many fields
// - "all fields exist together"
//
// Enum:
// - One thing that can be one variant at a time
// - "only one choice is active at a time"
//
// Example ideas for enums:
// - Direction: North, South, East, West
// - TrafficLight: Red, Yellow, Green
// - PaymentStatus: Pending, Paid, Failed
// - Shape: Circle, Rectangle, Triangle
//
// Enums are extremely important in Rust.
// Rust uses enums everywhere, including:
// - Option<T>
// - Result<T, E>
//
// Below is a simple enum with no extra data.
// ------------------------------------------------
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ------------------------------------------------
// ENUM WITH DATA INSIDE VARIANTS
// ------------------------------------------------
// This is where Rust enums become very powerful.
//
// Each variant can carry different data.
//
// Example:
// A message in a game or app might be:
//
// - Quit
// - Move with x and y
// - Write some text
// - Change color with RGB values
//
// A single enum can represent ALL those possibilities.
//
// This is much more powerful than simple enums in many other languages.
// ------------------------------------------------
#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// ------------------------------------------------
// IMPLEMENTATION FOR ENUM
// ------------------------------------------------
// Enums can also have methods using impl.
// This shows that enums are full types, not just labels.
// ------------------------------------------------
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Message::Quit => the program or action should stop.");
            }
            Message::Move { x, y } => {
                println!("Message::Move => move to coordinates x={}, y={}", x, y);
            }
            Message::Write(text) => {
                println!("Message::Write => text is: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Message::ChangeColor => rgb({}, {}, {})", r, g, b);
            }
        }
    }
}

// ------------------------------------------------
// ANOTHER ENUM: SHAPE
// ------------------------------------------------
// This enum shows different variants each carrying different data.
//
// Why is enum useful here?
// Because a shape can be:
// - Circle with radius
// - Rectangle with width and height
// - Square with side
//
// It would be awkward to force all shapes into one struct with many
// maybe-unused fields.
// Enum models this naturally.
// ------------------------------------------------
#[derive(Debug, Clone, Copy)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Square(side) => side * side,
        }
    }

    fn describe(&self) {
        match self {
            Shape::Circle { radius } => {
                println!("This is a Circle with radius {}", radius);
            }
            Shape::Rectangle { width, height } => {
                println!("This is a Rectangle with width {} and height {}", width, height);
            }
            Shape::Square(side) => {
                println!("This is a Square with side {}", side);
            }
        }
    }
}

// ------------------------------------------------
// OPTION<T>
// ------------------------------------------------
// Option<T> is one of the most important enums in Rust.
//
// It is defined in the standard library roughly like this:
//
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// It means:
// - Some(value) => there is a value
// - None        => there is no value
//
// Why is this useful?
// It prevents null-related bugs.
//
// Instead of using null, Rust forces you to handle the possibility
// that a value may be missing.
//
// Example use cases:
// - Searching for an item that may not exist
// - Returning an optional result
// - Optional config values
// ------------------------------------------------
fn find_user_by_name(name: &str) -> Option<User> {
    if name == "Alice" {
        Some(User::new("Alice", 25, true))
    } else {
        None
    }
}

// ------------------------------------------------
// RESULT<T, E>
// ------------------------------------------------
// Result<T, E> is another core Rust enum.
//
// Roughly:
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//
// It means:
// - Ok(value)  => success
// - Err(error) => failure
//
// Why is this useful?
// Rust encourages handling errors explicitly instead of hiding them.
//
// Example use cases:
// - Parsing text into a number
// - Reading files
// - Network operations
// - Any operation that might fail
// ------------------------------------------------
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// ------------------------------------------------
// MATCH
// ------------------------------------------------
// "match" is one of the most important control flow features in Rust.
//
// It is heavily used with enums.
//
// Why?
// Because enums represent "one of many possibilities"
// and match lets us handle each possibility.
//
// Rust requires match to be exhaustive.
// That means you must cover every possible variant.
//
// This makes code safer because you cannot accidentally forget a case.
//
// Example structure:
//
// match value {
//     Pattern1 => ...,
//     Pattern2 => ...,
//     _ => ...,
// }
//
// "_" means "anything else".
// ------------------------------------------------
fn print_direction(direction: Direction) {
    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}

// ------------------------------------------------
// MATCH WITH OPTION
// ------------------------------------------------
fn print_optional_number(value: Option<i32>) {
    match value {
        Some(number) => println!("We got a number: {}", number),
        None => println!("There is no number"),
    }
}

// ------------------------------------------------
// MATCH WITH RESULT
// ------------------------------------------------
fn print_division_result(result: Result<f64, String>) {
    match result {
        Ok(value) => println!("Division succeeded: {}", value),
        Err(error_message) => println!("Division failed: {}", error_message),
    }
}

// ------------------------------------------------
// MAIN FUNCTION
// ------------------------------------------------
// This is where the program starts.
//
// We will create structs and enums, use methods,
// and pattern-match on values.
// ------------------------------------------------
fn main() {
    println!("================================================");
    println!("RUST LAB: STRUCTS + ENUMS");
    println!("================================================");

    // ============================================================
    // PART 1: USING A STRUCT
    // ============================================================
    println!("\n--- PART 1: STRUCTS ---");

    // Create a user directly using field names.
    // This is struct literal syntax.
    let user1 = User {
        name: String::from("John"),
        age: 30,
        active: true,
    };

    // Print with Debug formatting.
    // {:?} works because we added #[derive(Debug)] above.
    println!("user1 with Debug: {:?}", user1);

    // Use method that reads data.
    user1.describe();

    // Create another user using the associated function User::new.
    let mut user2 = User::new("Sara", 17, true);
    user2.describe();

    // Check method returning bool.
    println!("Is user2 an adult? {}", user2.is_adult());

    // Since user2 is mutable, we can change it.
    user2.birthday();
    println!("After birthday:");
    user2.describe();
    println!("Is user2 an adult now? {}", user2.is_adult());

    user2.deactivate();
    println!("After deactivate:");
    user2.describe();

    // ============================================================
    // PART 2: TUPLE STRUCT
    // ============================================================
    println!("\n--- PART 2: TUPLE STRUCT ---");

    let red = Color(255, 0, 0);

    // Access by position.
    println!("Color red as Debug: {:?}", red);
    println!("Red channel   = {}", red.0);
    println!("Green channel = {}", red.1);
    println!("Blue channel  = {}", red.2);

    // ============================================================
    // PART 3: UNIT-LIKE STRUCT
    // ============================================================
    println!("\n--- PART 3: UNIT-LIKE STRUCT ---");

    let logger = Logger;
    println!("Logger value: {:?}", logger);

    // ============================================================
    // PART 4: SIMPLE ENUM
    // ============================================================
    println!("\n--- PART 4: SIMPLE ENUM ---");

    let d1 = Direction::North;
    let d2 = Direction::West;

    print_direction(d1);
    print_direction(d2);

    // ============================================================
    // PART 5: ENUM WITH DATA
    // ============================================================
    println!("\n--- PART 5: ENUM WITH DATA ---");

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write("Hello from Rust enum".to_string());
    let msg4 = Message::ChangeColor(10, 20, 30);

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    // ============================================================
    // PART 6: SHAPE ENUM
    // ============================================================
    println!("\n--- PART 6: SHAPE ENUM ---");

    let circle = Shape::Circle { radius: 2.0 };
    let rectangle = Shape::Rectangle {
        width: 5.0,
        height: 3.0,
    };
    let square = Shape::Square(4.0);

    circle.describe();
    println!("Area = {}", circle.area());

    rectangle.describe();
    println!("Area = {}", rectangle.area());

    square.describe();
    println!("Area = {}", square.area());

    // ============================================================
    // PART 7: OPTION<T>
    // ============================================================
    println!("\n--- PART 7: OPTION<T> ---");

    let maybe_user1 = find_user_by_name("Alice");
    let maybe_user2 = find_user_by_name("Bob");

    match maybe_user1 {
        Some(user) => {
            println!("Found user:");
            user.describe();
        }
        None => {
            println!("User not found");
        }
    }

    match maybe_user2 {
        Some(user) => {
            println!("Found user:");
            user.describe();
        }
        None => {
            println!("User not found");
        }
    }

    // Another simple Option example
    let number1 = Some(42);
    let number2: Option<i32> = None;

    print_optional_number(number1);
    print_optional_number(number2);

    // ============================================================
    // PART 8: RESULT<T, E>
    // ============================================================
    println!("\n--- PART 8: RESULT<T, E> ---");

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    print_division_result(result1);
    print_division_result(result2);

    // ============================================================
    // PART 9: WHY STRUCTS AND ENUMS EXIST
    // ============================================================
    println!("\n--- PART 9: WHY THEY EXIST ---");

    println!("STRUCTS are used to model one thing with multiple related fields.");
    println!("ENUMS are used to model one value that can be one of multiple possibilities.");

    // Example thought:
    //
    // User is naturally a struct because:
    // - a user HAS a name
    // - a user HAS an age
    // - a user HAS an active status
    //
    // Direction is naturally an enum because:
    // - direction is one of North / South / East / West
    // - it is not all four at once
    //
    // Shape is a good enum because:
    // - a shape can be Circle OR Rectangle OR Square
    // - each variant may need different data

    // ============================================================
    // PART 10: STRUCT VS ENUM SUMMARY
    // ============================================================
    println!("\n--- PART 10: STRUCT VS ENUM SUMMARY ---");

    println!("Use STRUCT when:");
    println!("1. You are describing one object/entity");
    println!("2. That object has several fields");
    println!("3. All those fields belong together");

    println!("\nUse ENUM when:");
    println!("1. A value can be one of several cases");
    println!("2. Each case may be different");
    println!("3. You want pattern matching with match");

    println!("\nLab finished.");
}
