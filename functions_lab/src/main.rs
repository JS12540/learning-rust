// functions_lab.rs
//
// This file is written like a mini Rust notebook / lab.
// Read top to bottom and run with:
//
//     cargo run
//
// Rust is different from C/C++/Java/Python in a few important ways:
//
// 1. Rust cares a lot about safety and correctness.
// 2. Rust has OOP-like ideas, but does NOT use inheritance the same way as Java/C++.
// 3. Rust prefers composition + traits instead of class inheritance.
// 4. Rust uses expressions heavily, which is why "no semicolon" can matter a lot.
//
// ------------------------------------------------------------
// BIG IDEA: STATEMENTS VS EXPRESSIONS
// ------------------------------------------------------------
//
// In many languages, a line is just a statement that does something.
// In Rust, many things are *expressions*, meaning they produce a value.
//
// Example:
//
//     let x = 5;
//
// Here, `5` is an expression that evaluates to the value 5.
//
// Another example:
//
//     let y = {
//         let a = 2;
//         let b = 3;
//         a + b
//     };
//
// The block above returns 5 because the LAST LINE has no semicolon.
//
// Why?
// Because in Rust:
//
// - `a + b;`  -> statement, value is discarded
// - `a + b`   -> expression, value is returned from the block
//
// This idea is used everywhere in Rust, especially in functions.


// ============================================================
// BASIC FUNCTIONS
// ============================================================

// Simple function with no parameters and no return value.
//
// Syntax:
//
//     fn function_name() {
//         ...
//     }
//
// `fn` means "define a function".
//
// `greet` takes no input.
// It also does not return anything meaningful.
// In Rust, a function with no useful return returns the unit type: `()`.
//
// The unit type `()` is like "nothing useful", similar to `void` in C/C++/Java.
fn greet() {
    println!("Hello from Rust!");
    // This line ends with a semicolon because we are executing a statement.
    // We are not trying to return the value of println!.
    //
    // Also note: println! has an exclamation mark because it is a MACRO,
    // not a normal function.
    //
    // Macros in Rust are written like:
    //     something!(...)
    //
    // A macro can expand into code before compilation.
}


// Function with parameters and a return value.
//
// `a: i32` means:
// - parameter name = a
// - type = i32
//
// `i32` means a signed 32-bit integer.
//
// `-> i32` means the function returns an i32.
fn add(a: i32, b: i32) -> i32 {
    a + b
    // IMPORTANT:
    // There is NO semicolon here.
    //
    // Why?
    // Because `a + b` is the final expression of the function body.
    // Rust returns the value of the final expression automatically.
    //
    // If we wrote:
    //
    //     a + b;
    //
    // then the semicolon would turn it into a statement.
    // That means the value would be discarded, and the function
    // would effectively return `()`, which would be wrong here.
    //
    // So:
    //
    //     a + b     => returned
    //     a + b;    => not returned
}


// Same idea, but with explicit `return`.
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
    // This works too.
    //
    // Rust allows both styles:
    //
    // explicit:
    //     return value;
    //
    // implicit:
    //     value
    //
    // Idiomatic Rust usually prefers the final-expression style
    // when the logic is simple.
}


// ============================================================
// HIGHER-ORDER FUNCTIONS
// ============================================================
//
// A higher-order function is a function that:
//
// - takes another function as input, OR
// - returns a function/closure
//
// This is common in functional programming,
// but Rust supports it nicely too.


// This function accepts another function as a parameter.
//
// `f: fn(i32, i32) -> i32` means:
//
// f is a FUNCTION POINTER
// to a function that:
//
// - takes two i32 values
// - returns one i32
//
// So `add` and `subtract` both match this type.
//
// Think of it like:
// "Give me any function with this shape/signature, and I can call it."
fn apply_operation(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
    // Again, no semicolon because this is the final expression.
    //
    // We are calling whatever function was passed in,
    // using x and y as arguments.
    //
    // Example:
    //     apply_operation(add, 2, 3)
    //
    // becomes:
    //     add(2, 3)
}


// ============================================================
// CLOSURES
// ============================================================
//
// Closures are one of the most important concepts in Rust.
//
// A closure is like a small anonymous function.
// "Anonymous" means it does not need a normal named `fn` declaration.
//
// Example shape:
//
//     let my_closure = |x| x + 1;
//
// Closures are useful because:
// - they are short
// - they can be passed around
// - they can capture variables from the surrounding environment
//
// That "capture" ability is what makes closures special compared to
// plain function pointers.
//
// In simple terms:
//
// Function:
//     stands alone
//
// Closure:
//     can remember values from where it was created


fn closure_examples() {
    // This closure takes two i32 inputs and returns their multiplication.
    //
    // Syntax:
    //
    //     |a: i32, b: i32| a * b
    //
    // Left side of `| |` = parameters
    // Right side = body
    //
    // This is similar to:
    //
    //     fn multiply(a: i32, b: i32) -> i32 {
    //         a * b
    //     }
    //
    // But closures are often shorter and inline.
    let multiply = |a: i32, b: i32| a * b;

    println!("Closure multiply: {}", multiply(3, 4));

    // Another closure: one input, square it.
    let square = |x: i32| x * x;

    println!("Square: {}", square(5));

    // --------------------------------------------------------
    // CAPTURING ENVIRONMENT
    // --------------------------------------------------------
    //
    // This is where closures become more powerful than normal functions.
    //
    // A closure can "capture" variables from outside itself.

    let extra = 10;

    let add_extra = |x: i32| x + extra;

    println!("Closure capturing environment: {}", add_extra(5));

    // `add_extra` can use `extra` even though `extra` is not passed
    // as a normal parameter.
    //
    // That is a key feature of closures.
    //
    // A normal function like this would NOT be allowed:
    //
    // fn bad_example(x: i32) -> i32 {
    //     x + extra   // error: function cannot just capture local outside vars
    // }
    //
    // Closures can do this; normal functions cannot.


    // --------------------------------------------------------
    // WHY CLOSURES MATTER
    // --------------------------------------------------------
    //
    // Closures are used heavily with iterators and collections.
    //
    // Example:
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();

    println!("Doubled with closure: {:?}", doubled);

    // `map(|n| n * 2)` means:
    // "For each item, apply this little rule."
    //
    // The closure here is:
    //     |n| n * 2
    //
    // So closures are very common in real Rust code.
}


// ============================================================
// GENERICS (COMPILE-TIME POLYMORPHISM)
// ============================================================
//
// Generics let us write code that works for many types.
//
// Instead of writing:
//
//     fn add_i32(a: i32, b: i32) -> i32
//     fn add_f64(a: f64, b: f64) -> f64
//
// we can write one generic function.
//
// `T` is a placeholder type.
// It means "some type to be decided later".
//
// But Rust needs rules.
// Not every type can be added with `+`.
// So we add a trait bound:
//
//     T: std::ops::Add<Output = T>
//
// Meaning:
// T must support the `+` operator,
// and the result of adding two T values must also be T.
//
// This is a form of polymorphism.
// Specifically, compile-time polymorphism (static dispatch).
fn generic_add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
    // Same final-expression rule.
}


// ============================================================
// STRUCTS (OOP-STYLE DATA)
// ============================================================
//
// A `struct` is a custom data type.
// Similar idea to a class's data members in OOP.
//
// Rust separates:
// - data (`struct`)
// - behavior (`impl`)
//
// Unlike some OOP languages, methods are not written inside the struct body.
// They are written in an `impl` block.

struct Person {
    name: String,
    age: u32,
    // `String` is an owned, growable UTF-8 string type.
    //
    // `u32` means unsigned 32-bit integer.
    // Since age cannot be negative, u32 makes sense.
}


// `impl` means "implementation block".
// This is where methods for `Person` are defined.
impl Person {
    // Associated function (similar to a static method in OOP).
    //
    // It is called like:
    //     Person::new(...)
    //
    // It does not take `self`, so it belongs to the type,
    // not to one specific object instance.
    fn new(name: &str, age: u32) -> Self {
        // `&str` is a string slice: a borrowed view into string data.
        //
        // Why take `&str` instead of `String`?
        // Because it is more flexible.
        //
        // Both string literals and String references can often be passed conveniently.
        //
        // `Self` here means `Person`.
        // So this function returns a `Person`.
        Self {
            name: name.to_string(),
            age,
        }
        // `name.to_string()` converts borrowed string data into an owned String.
        //
        // `age,` is shorthand for:
        //     age: age
    }

    // Method borrowing self immutably.
    //
    // `&self` means:
    // - this method can read the object
    // - but cannot modify it
    //
    // In OOP terms, this is like a method operating on the current object.
    fn greet(&self) {
        println!("Hi, I'm {} and I'm {} years old", self.name, self.age);
    }

    // Method borrowing self mutably.
    //
    // `&mut self` means:
    // - this method can modify the object
    //
    // Rust makes mutation explicit.
    fn have_birthday(&mut self) {
        self.age += 1;
    }
}


// ============================================================
// TRAITS (RUST'S MAIN TOOL FOR SHARED BEHAVIOR)
// ============================================================
//
// A trait defines shared behavior.
// It is similar to an interface in Java/C#.
//
// Instead of saying:
// "Dog inherits from Animal class"
//
// Rust usually says:
// "Dog implements the Animal trait"
//
// This is one of the biggest shifts from traditional OOP.
// Rust prefers traits over inheritance.

trait Animal {
    fn speak(&self);
    // Any type implementing Animal must provide a `speak` method.
}


// These structs have no fields.
// They are still valid types.
//
// `struct Dog;` is called a unit-like struct.
struct Dog;
struct Cat;


// Here Dog implements the Animal trait.
impl Animal for Dog {
    fn speak(&self) {
        println!("Dog says: Woof!");
    }
}


// Here Cat implements the Animal trait.
impl Animal for Cat {
    fn speak(&self) {
        println!("Cat says: Meow!");
    }
}


// ============================================================
// POLYMORPHISM IN RUST
// ============================================================
//
// Polymorphism means:
// "one interface, many forms"
//
// In simple words:
// We can write code that works with many different concrete types
// as long as they follow a shared contract.
//
// In Rust, that shared contract is usually a trait.
//
// There are two major styles shown below:
//
// 1. Static dispatch  -> generics
// 2. Dynamic dispatch -> trait objects


// ------------------------------------------------------------
// STATIC DISPATCH
// ------------------------------------------------------------
//
// `T: Animal` means:
// T can be any type, as long as it implements Animal.
//
// This is resolved at compile time.
// The compiler generates specialized code for each used type.
//
// Benefits:
// - very fast
// - no runtime lookup overhead
//
// Downsides:
// - separate generated code per type
fn make_sound<T: Animal>(animal: T) {
    animal.speak();
}


// ------------------------------------------------------------
// DYNAMIC DISPATCH
// ------------------------------------------------------------
//
// `&dyn Animal` means:
// a reference to "some value of some type that implements Animal"
//
// The exact concrete type can be decided at runtime.
//
// This uses a vtable-like mechanism behind the scenes.
// That is why this is called dynamic dispatch.
//
// This feels more like classic OOP polymorphism.
fn make_sound_dyn(animal: &dyn Animal) {
    animal.speak();
}


// ============================================================
// MAIN FUNCTION
// ============================================================
//
// Every executable Rust program starts from `main`.
fn main() {
    println!("--- BASIC FUNCTIONS ---");
    greet();

    println!("Add: {}", add(5, 3));
    println!("Subtract: {}", subtract(10, 4));

    println!("\n--- HIGHER ORDER FUNCTION ---");

    let result = apply_operation(add, 2, 3);
    println!("Apply operation using add: {}", result);

    let result2 = apply_operation(subtract, 10, 7);
    println!("Apply operation using subtract: {}", result2);

    println!("\n--- CLOSURES ---");
    closure_examples();

    println!("\n--- GENERICS ---");
    println!("Generic add (i32): {}", generic_add(2, 3));
    println!("Generic add (f64): {}", generic_add(2.5, 3.5));

    println!("\n--- STRUCTS & METHODS ---");

    // `mut` means this binding is mutable.
    // Without `mut`, we would not be allowed to call
    // a method needing `&mut self`.
    let mut person = Person::new("JS", 25);

    person.greet();
    person.have_birthday();
    person.greet();

    println!("\n--- POLYMORPHISM WITH TRAITS ---");

    let dog = Dog;
    let cat = Cat;

    // STATIC POLYMORPHISM
    //
    // At compile time, Rust knows the concrete types:
    // one call uses Dog
    // one call uses Cat
    make_sound(dog);
    make_sound(cat);

    // DYNAMIC POLYMORPHISM
    //
    // Here we pass trait objects by reference.
    // This is useful when we want to handle different concrete
    // types through one common interface at runtime.
    let dog = Dog;
    let cat = Cat;

    make_sound_dyn(&dog);
    make_sound_dyn(&cat);

    // A more realistic dynamic dispatch example:
    //
    // We can store different animal types together
    // using trait object references.
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in animals {
        animal.speak();
    }
}
