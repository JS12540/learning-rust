// print_lab.rs
//
// Run with:
//     cargo run
//
// This file demonstrates:
// - Different print macros
// - Formatting styles
// - Debug printing
// - Advanced formatting options
//
// IMPORTANT:
// All print functions in Rust are MACROS (note the !)
// Example: println!
//
// Why macros?
// Because they allow flexible argument handling and compile-time checks.

fn main() {

    // ============================================================
    // BASIC PRINTING
    // ============================================================

    // println! prints with a newline at the end
    println!("Hello, Rust!");

    // print! does NOT add a newline
    print!("This is ");
    print!("on the same line\n"); // manually adding newline

    // eprintln! prints to standard error (useful for errors/logging)
    eprintln!("This is an error message!");



    // ============================================================
    // BASIC FORMAT PLACEHOLDERS
    // ============================================================

    // {} is a placeholder
    println!("Hello, {}!", "JS");

    let name = "JS";
    let age = 25;

    // Multiple placeholders
    println!("Name: {}, Age: {}", name, age);

    // Order matters
    println!("Age: {}, Name: {}", age, name);



    // ============================================================
    // POSITIONAL ARGUMENTS
    // ============================================================

    // You can specify positions manually
    println!("{0} is learning {1}, and {0} likes it!", "JS", "Rust");

    // {0} refers to first argument
    // {1} refers to second argument



    // ============================================================
    // NAMED ARGUMENTS (Closest to Python f-strings)
    // ============================================================

    println!(
        "Name: {name}, Age: {age}",
        name = "JS",
        age = 25
    );

    // Cleaner way (modern Rust automatically captures variables)
    println!("Name: {name}, Age: {age}");

    // This is the closest thing to Python f-strings:
    //
    // Python:
    //     f"Name: {name}, Age: {age}"
    //
    // Rust:
    //     println!("Name: {name}, Age: {age}");



    // ============================================================
    // DEBUG PRINTING
    // ============================================================

    let numbers = vec![1, 2, 3, 4];

    // {:?} = debug format
    println!("Numbers: {:?}", numbers);

    // {:#?} = pretty debug (multi-line, readable)
    println!("Pretty debug:\n{:#?}", numbers);

    // NOTE:
    // Not all types support {:?}
    // They must implement the Debug trait.
    //
    // Most standard types already do.



    // ============================================================
    // DISPLAY VS DEBUG
    // ============================================================

    let x = 10;

    println!("Display: {}", x);   // uses Display trait
    println!("Debug: {:?}", x);   // uses Debug trait

    // Difference:
    // - Display = user-friendly output
    // - Debug = developer-friendly output



    // ============================================================
    // FORMATTING NUMBERS
    // ============================================================

    let pi = 3.1415926535;

    // Limit decimal places (precision)
    println!("Pi (2 decimal places): {:.2}", pi);

    // Width + alignment
    println!("Right aligned: {:>10}", 42);
    println!("Left aligned:  {:<10}", 42);
    println!("Center aligned:{:^10}", 42);

    // Fill with custom character
    println!("Padded with zeros: {:0>5}", 42);
    println!("Custom padding: {:*^10}", 42);

    // Explanation:
    //
    // {:>10}
    // > means right align
    // 10 means total width = 10
    //
    // {:0>5}
    // 0 = padding character
    // > = right align
    // 5 = width



    // ============================================================
    // NUMBER BASE FORMATTING
    // ============================================================

    let num = 255;

    println!("Binary: {:b}", num);   // base 2
    println!("Octal: {:o}", num);   // base 8
    println!("Hex: {:x}", num);     // base 16 (lowercase)
    println!("Hex: {:X}", num);     // base 16 (uppercase)



    // ============================================================
    // FORMAT! (STRING BUILDER)
    // ============================================================

    // format! does NOT print
    // It returns a String

    let s = format!("Name: {}, Age: {}", name, age);

    println!("Formatted string stored in variable:");
    println!("{}", s);

    // Think of format! like:
    // Python:
    //     s = f"Name: {name}, Age: {age}"



    // ============================================================
    // ESCAPING CHARACTERS
    // ============================================================

    println!("This is a newline:\nNext line");
    println!("This is a tab:\tTabbed");

    // To print {}
    println!("Use double braces: {{}}");



    // ============================================================
    // MULTI-LINE STRINGS
    // ============================================================

    let multi_line = "
        This is
        a multi-line
        string in Rust
    ";

    println!("{}", multi_line);



    // ============================================================
    // PRINTING STRUCTS (IMPORTANT CONCEPT)
    // ============================================================

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: "JS".to_string(),
        age: 25,
    };

    // This works because of #[derive(Debug)]
    println!("Person: {:?}", p);

    // Pretty print
    println!("Pretty Person:\n{:#?}", p);



    // ============================================================
    // INLINE EXPRESSIONS IN PRINT
    // ============================================================

    let a = 5;
    let b = 3;

    println!("Sum of {a} and {b} is {}", a + b);

    // You can mix inline variables and expressions.



    // ============================================================
    // CONDITIONAL FORMATTING
    // ============================================================

    let status = true;

    println!(
        "Status: {}",
        if status { "Active" } else { "Inactive" }
    );



    // ============================================================
    // FINAL SUMMARY PRINT
    // ============================================================

    println!("\n--- SUMMARY ---");
    println!("Use println! for printing with newline");
    println!("Use format! to build strings");
    println!("Use {{}} for display, {{:?}} for debug");
    println!("Use {{name}} style for clean formatting (like f-strings)");
}
