// ownership_borrowing_lab.rs
//
// This file is a guided lab for one of the MOST IMPORTANT Rust topics:
//
// 1. Ownership
// 2. Borrowing
// 3. References
// 4. Mutable vs immutable borrowing
// 5. Why moves happen
// 6. Why Rust prevents dangling references
// 7. String vs &str in ownership context
//
// ------------------------------------------------------------
// WHY THIS TOPIC MATTERS
// ------------------------------------------------------------
//
// In many languages, memory is managed by:
//
// - garbage collector (Java, Python, Go to some extent)
// - manual free/delete (C/C++)
//
// Rust uses a different approach:
// it uses OWNERSHIP rules checked at compile time.
//
// This gives two huge benefits:
//
// 1. memory safety
// 2. no garbage collector pause
//
// That is one reason Rust is fast like C/C++ but much safer.
//
// ------------------------------------------------------------
// THE 3 CORE OWNERSHIP RULES
// ------------------------------------------------------------
//
// Rule 1:
// Every value in Rust has a variable that is its OWNER.
//
// Rule 2:
// There can only be ONE owner at a time.
//
// Rule 3:
// When the owner goes out of scope, the value is dropped (cleaned up).
//
// These rules sound simple, but they drive a lot of Rust behavior.
//
// ------------------------------------------------------------
// IMPORTANT MENTAL MODEL
// ------------------------------------------------------------
//
// Think of heap data like a house key.
//
// - Only one person owns the key at a time.
// - You can lend the key temporarily (borrowing).
// - When the owner leaves forever, the house is closed and cleaned up.
//
// Rust tracks this at compile time.
//
// ------------------------------------------------------------
// STACK VS HEAP (simple intuition)
// ------------------------------------------------------------
//
// You do NOT need full systems-level detail immediately,
// but this basic idea helps:
//
// STACK:
// - fixed size data
// - fast
// - stored directly
//
// HEAP:
// - dynamically sized / runtime-sized data
// - accessed indirectly
// - needs memory management
//
// Example:
// - i32 is simple stack data
// - String usually owns heap data
//
// Ownership issues become more visible with heap-allocated data,
// especially String, Vec, Box, etc.

fn main() {
    println!("==================================================");
    println!("OWNERSHIP + BORROWING LAB");
    println!("==================================================");

    // ============================================================
    // 1. SCOPE
    // ============================================================
    //
    // Scope means the region of code where a variable is valid.
    //
    // When a variable goes out of scope, Rust automatically drops it.
    //
    // For simple types this is not dramatic.
    // For heap-owning types like String, this is very important.

    {
        let x = 5;
        println!("\n1) Scope example: x = {}", x);
    }
    // x is no longer valid here because it went out of scope.

    // If we tried:
    // println!("{}", x);
    //
    // Rust would give a compile error.


    // ============================================================
    // 2. OWNERSHIP WITH STRING
    // ============================================================
    //
    // String is a very common example because it owns heap memory.
    //
    // Example:
    // - the variable itself may live on the stack
    // - the text contents live on the heap
    //
    // When the owner goes out of scope, Rust frees the heap memory.

    let s1 = String::from("hello");
    println!("\n2) s1 owns the String: {}", s1);

    // At this point:
    // - s1 is the owner
    // - when s1 goes out of scope, the String is dropped


    // ============================================================
    // 3. MOVE, NOT COPY
    // ============================================================
    //
    // This is where Rust feels surprising at first.
    //
    // If we assign a String to another variable:
    //
    //     let s2 = s1;
    //
    // many beginners expect a deep copy.
    //
    // But Rust does NOT automatically deep copy heap data here.
    // Instead, ownership MOVES.

    let s2 = s1;

    println!("\n3) Ownership moved from s1 to s2");
    println!("   s2 = {}", s2);

    // IMPORTANT:
    // We can use s2, because s2 is now the owner.
    //
    // But we CANNOT use s1 anymore.
    //
    // This would fail:
    //
    // println!("{}", s1); Nothing will be printed
    //
    // Why?
    //
    // Because if both s1 and s2 were allowed to think they own
    // the same heap memory, then when both went out of scope,
    // Rust would try to free the same memory twice.
    //
    // That would be a double-free bug.
    //
    // So Rust prevents that by making the assignment a MOVE.


    // ============================================================
    // 4. COPY TYPES
    // ============================================================
    //
    // Not all assignments move ownership.
    //
    // Some simple types implement the Copy trait.
    //
    // Examples:
    // - integers
    // - booleans
    // - chars
    // - tuples of Copy types
    //
    // These are cheap to duplicate, so Rust copies them.

    let a = 10;
    let b = a;

    println!("\n4) Copy types");
    println!("   a = {}", a);
    println!("   b = {}", b);

    // Here both a and b are valid.
    //
    // Why no move?
    //
    // Because i32 is Copy.
    //
    // You can think:
    // - Copy type => duplicate bits cheaply
    // - Non-Copy owner type => move ownership unless cloned


    // ============================================================
    // 5. CLONE FOR DEEP COPY
    // ============================================================
    //
    // If we WANT an actual duplicate of heap data,
    // we ask for it explicitly with clone().
    //
    // This is important:
    //
    // Rust wants expensive duplication to be explicit, not hidden.

    let original = String::from("Rust");
    let copied = original.clone();

    println!("\n5) Clone for deep copy");
    println!("   original = {}", original);
    println!("   copied   = {}", copied);

    // Here both variables are valid because clone() created a new heap allocation.
    //
    // So:
    //
    // let b = a;         // maybe move, maybe copy (depends on type)
    // let b = a.clone(); // explicit deep-ish duplicate when supported


    // ============================================================
    // 6. OWNERSHIP AND FUNCTIONS
    // ============================================================
    //
    // Passing a value into a function follows the same ownership rules as assignment.
    //
    // If you pass a String by value, ownership moves into the function.

    let name = String::from("Jay");
    take_ownership(name);

    // name is no longer valid here.
    //
    // This would fail:
    // println!("{}", name); Nothing printed
    //
    // Because ownership moved into take_ownership().


    // For Copy types, passing to a function copies the value.

    let number = 42;
    makes_copy(number);

    println!("\n6) number is still valid after function call: {}", number);

    // Because i32 is Copy, the function gets a copy, not the only ownership.


    // ============================================================
    // 7. RETURNING OWNERSHIP
    // ============================================================
    //
    // A function can return ownership back to the caller.

    let returned = give_ownership();
    println!("\n7) Function returned ownership: {}", returned);

    let temp = String::from("temporary");
    let temp = take_and_give_back(temp);
    println!("   Ownership moved into function and returned back: {}", temp);


    // ============================================================
    // 8. BORROWING WITH REFERENCES
    // ============================================================
    //
    // Ownership is powerful, but if every function took ownership,
    // code would become inconvenient fast.
    //
    // So Rust lets us BORROW values.
    //
    // Borrowing means:
    // - temporary access
    // - without becoming owner
    //
    // References are written with &
    //
    // Example:
    //     &String
    //
    // means:
    // "a reference to a String"

    let text = String::from("borrow me");
    let len = calculate_length(&text);

    println!("\n8) Borrowing with immutable reference");
    println!("   text = {}", text);
    println!("   length = {}", len);

    // Important:
    // text is still valid after the function call.
    //
    // Why?
    //
    // Because calculate_length() borrowed text.
    // It did NOT take ownership.


    // ============================================================
    // 9. IMMUTABLE REFERENCES
    // ============================================================
    //
    // An immutable reference lets code READ the value,
    // but not modify it.

    let city = String::from("London");
    print_string_ref(&city);

    println!("\n9) city still available after immutable borrow: {}", city);


    // ============================================================
    // 10. MUTABLE REFERENCES
    // ============================================================
    //
    // If a function needs to modify the value,
    // it needs a mutable reference: &mut T
    //
    // This means:
    // - borrower may change the value
    // - Rust enforces stricter rules for safety

    let mut language = String::from("Rust");
    append_text(&mut language);

    println!("\n10) Mutable borrow changed the String");
    println!("    language = {}", language);

    // We needed:
    //
    // let mut language = ...
    //
    // because the original binding must be mutable,
    // otherwise we cannot borrow it as mutable.


    // ============================================================
    // 11. BORROWING RULES
    // ============================================================
    //
    // This is one of the most important safety rules in Rust:
    //
    // At any given time, you can have EITHER:
    //
    // - any number of immutable references
    // OR
    // - exactly one mutable reference
    //
    // But not both at the same time in overlapping use.
    //
    // Why?
    //
    // Because if one part of code can mutate while another part
    // is reading at the same time, that can cause bugs and races.

    let book = String::from("The Rust Book");

    let r1 = &book;
    let r2 = &book;

    println!("\n11) Multiple immutable borrows are allowed");
    println!("    r1 = {}", r1);
    println!("    r2 = {}", r2);

    // This is fine because both are read-only.
    //
    // But if we tried to also create a mutable reference while
    // r1 and r2 are still being used, that would be an error.


    let mut car = String::from("BMW");
    let r3 = &mut car;
    r3.push_str(" M3");

    println!("\n    One mutable borrow is allowed");
    println!("    r3 = {}", r3);

    // While r3 exists as the active mutable reference,
    // you cannot also create other references that overlap improperly.


    // ============================================================
    // 12. NON-OVERLAPPING BORROWS
    // ============================================================
    //
    // Rust is smarter than many beginners expect.
    //
    // It checks actual usage scope, not just visual scope in many cases.

    let mut framework = String::from("Actix");

    let first_read = &framework;
    println!("\n12) first_read = {}", first_read);

    // first_read is no longer used after the println above,
    // so now a mutable borrow is allowed.

    let write_ref = &mut framework;
    write_ref.push_str(" Web");
    println!("    write_ref = {}", write_ref);

    // This works because the immutable borrow is finished
    // before the mutable borrow starts being used.


    // ============================================================
    // 13. DANGLING REFERENCES
    // ============================================================
    //
    // A dangling reference means:
    // a reference points to memory that has already been freed.
    //
    // In C/C++, dangling pointers are a classic source of bugs.
    //
    // Rust prevents this at compile time.

    let safe_ref_example = no_dangle();
    println!("\n13) No dangling reference: {}", safe_ref_example);

    // See the no_dangle() function below for explanation.


    // ============================================================
    // 14. STRING vs &str
    // ============================================================
    //
    // This is a very common beginner confusion.
    //
    // String:
    // - owned
    // - growable
    // - heap allocated
    //
    // &str:
    // - borrowed string slice
    // - does not own
    //
    // A string literal like "hello" is a &str.

    let literal: &str = "I am a string slice";
    let owned: String = String::from("I am an owned String");

    println!("\n14) String vs &str");
    println!("    literal (&str) = {}", literal);
    println!("    owned (String) = {}", owned);

    // Many functions prefer &str when they only need to read text,
    // because it is more flexible and avoids unnecessary ownership transfer.

    print_str(literal);
    print_str(&owned);
    //
    // Why does &owned work?
    //
    // Because a String can be borrowed as a string slice.


    // ============================================================
    // 15. BORROWING PART OF A STRING (SLICES)
    // ============================================================
    //
    // A slice is a reference to part of a collection.
    //
    // String slices are extremely common.

    let sentence = String::from("hello world");
    let hello = &sentence[0..5];
    let world = &sentence[6..11];

    println!("\n15) Slices");
    println!("    hello = {}", hello);
    println!("    world = {}", world);

    // hello and world are &str slices.
    // They borrow parts of sentence.
    // They do not own separate copies.


    // ============================================================
    // 16. FIRST WORD EXAMPLE
    // ============================================================
    //
    // This is a classic Rust example.
    //
    // We return a slice instead of copying the word into a new String.
    // That means:
    // - efficient
    // - borrowed view into original string

    let phrase = String::from("ownership matters");
    let word = first_word(&phrase);

    println!("\n16) First word function");
    println!("    phrase = {}", phrase);
    println!("    first word = {}", word);


    // ============================================================
    // 17. WHY OWNERSHIP EXISTS
    // ============================================================
    //
    // Rust’s rules may feel strict, but they solve real problems:
    //
    // - no double free
    // - no use-after-free
    // - no null-based ownership mistakes in the same way
    // - safer concurrency
    // - fewer memory bugs
    //
    // So when Rust complains, it is usually protecting you from
    // a class of bugs that other languages often allow.


    // ============================================================
    // 18. PRACTICAL GUIDELINES
    // ============================================================
    //
    // Beginner-friendly rules:
    //
    // 1. If a function only needs to READ data, pass by reference.
    // 2. If a function needs to MODIFY data, use &mut.
    // 3. If a function needs to OWN the data, take it by value.
    // 4. Use clone() only when you truly need another owned copy.
    // 5. Prefer &str over String for read-only string parameters when possible.
    //
    // This is not 100% of advanced Rust, but it is a strong start.


    // ============================================================
    // 19. END SUMMARY
    // ============================================================

    println!("\n==================================================");
    println!("SUMMARY");
    println!("==================================================");
    println!("Ownership = who is responsible for the value");
    println!("Move       = ownership transfers");
    println!("Copy       = cheap duplicate for simple types");
    println!("Clone      = explicit duplicate for owned data");
    println!("Borrow     = temporary access without ownership");
    println!("&T         = immutable reference");
    println!("&mut T     = mutable reference");
    println!("Slices     = borrowed views into data");
    println!("Rust prevents dangling references and double free");
    println!("==================================================");
}

// ============================================================
// FUNCTIONS USED IN THE LAB
// ============================================================

// This function TAKES ownership of the String.
// After the caller passes the String here, the caller loses it.
fn take_ownership(s: String) {
    println!("\n6) take_ownership received: {}", s);

    // When this function ends, s goes out of scope and is dropped.
    //
    // That means the String memory is cleaned up here.
}

// This function takes an i32.
// Since i32 is Copy, the caller keeps its own valid version.
fn makes_copy(x: i32) {
    println!("   makes_copy received: {}", x);
}

// This function CREATES a String and returns ownership to the caller.
fn give_ownership() -> String {
    let s = String::from("I came from a function");
    s
    // No semicolon:
    // this final expression returns the String.
    //
    // Ownership moves from this function to the caller.
}

// This takes ownership, then returns ownership back.
fn take_and_give_back(s: String) -> String {
    println!("   Function got ownership of: {}", s);
    s
}

// Borrowing example:
// this function only reads the String,
// so it takes an immutable reference.
fn calculate_length(s: &String) -> usize {
    s.len()
    // This returns the length, while s remains owned by caller.
}

// Another immutable borrow example.
fn print_string_ref(s: &String) {
    println!("   print_string_ref sees: {}", s);
    // Cannot modify s because it is only &String, not &mut String.
}

// Mutable borrow example.
fn append_text(s: &mut String) {
    s.push_str(" language");
    // Because s is mutable reference, we can modify the original String.
}

// This function returns a String, not a reference.
// Returning a local reference would be dangerous.
fn no_dangle() -> String {
    let s = String::from("safe owned return");
    s
    // If we tried to return &s, that would fail,
    // because s would be dropped when the function ends.
    //
    // Rust prevents that compile-time bug.
}

// More flexible string-reading function.
// It takes &str instead of &String.
//
// This is often better API design for read-only text.
fn print_str(s: &str) {
    println!("   print_str sees: {}", s);
}

// Return the first word as a string slice.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // We iterate through bytes looking for a space character.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space found, return the whole string as slice.
    &s[..]
}
