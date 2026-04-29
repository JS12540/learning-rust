// ============================================================================
// RUST LAB: MODULES + TRAITS
// ============================================================================
//
// This is a deep, one-file Rust lab for:
//
// 1. modules
// 2. paths
// 3. pub
// 4. use
// 5. self / super / crate
// 6. organizing code with modules
// 7. traits
// 8. implementing traits
// 9. trait methods
// 10. default trait methods
// 11. traits as function parameters
// 12. trait bounds
// 13. multiple trait bounds
// 14. generic functions with traits
// 15. impl Trait
// 16. associated types of thought (intro level explanation)
// 17. derived traits vs manual traits
// 18. common standard traits like Debug, Clone, PartialEq
//
// This file is intentionally LONG and HEAVILY COMMENTED.
// The goal is not to be short.
// The goal is to explain the concepts in detail.
//
// ----------------------------------------------------------------------------
// HOW TO RUN
// ----------------------------------------------------------------------------
//
// If using Cargo:
//     cargo run
//
// If using rustc directly:
//     rustc main.rs
//     ./main
//
// ============================================================================



// ============================================================================
// SECTION 1: WHAT IS A MODULE?
// ============================================================================
//
// A module is a way to organize code into named sections.
//
// Keyword:
//     mod
//
// Basic syntax:
//
//     mod my_module {
//         // items here
//     }
//
// WHY MODULES EXIST:
// - organize code
// - group related items together
// - avoid name conflicts
// - control visibility
// - make large programs easier to understand
//
// In real Rust projects, modules are often split across multiple files.
// But in this lab, everything is in ONE FILE so you can study it easily.
//
// IMPORTANT IDEA:
// A module is a namespace.
//
// Namespace means:
// it gives names a "place" so different things can have the same short name
// in different modules without colliding.
//
// Example:
//
// mod a {
//     fn hello() {}
// }
//
// mod b {
//     fn hello() {}
// }
//
// These are different because their full paths are:
// - a::hello
// - b::hello
//
// ============================================================================



// ============================================================================
// SECTION 2: WHAT IS A PATH?
// ============================================================================
//
// A path is how Rust finds an item.
//
// Examples:
// - crate::animals::dog::bark
// - self::helper
// - super::parent_function
// - std::fmt::Debug
//
// Think of a path like a filesystem path, but for code.
//
// Common path keywords:
// - crate -> start from current crate root
// - self  -> current module
// - super -> parent module
//
// ============================================================================



// ============================================================================
// SECTION 3: WHAT DOES pub MEAN?
// ============================================================================
//
// By default, Rust items inside modules are private.
//
// That means:
// - a function inside a module cannot automatically be called from outside
// - a struct's fields are also private by default
//
// Keyword:
//     pub
//
// pub means:
// "make this visible from outside the current module"
//
// Common uses:
// - pub fn
// - pub struct
// - pub enum
// - pub mod
// - pub field_name: Type
//
// IMPORTANT:
// Making a struct public does NOT automatically make its fields public.
//
// Example:
//
// pub struct User {
//     pub name: String,
//     age: u32,
// }
//
// Here User is public, name is public, but age is still private.
//
// ============================================================================



// ============================================================================
// SECTION 4: WHAT IS A TRAIT?
// ============================================================================
//
// A trait defines shared behavior.
//
// Keyword:
//     trait
//
// Basic syntax:
//
//     trait Speak {
//         fn speak(&self);
//     }
//
// Think of a trait like:
// "Any type that implements this trait promises to provide this behavior"
//
// Traits are one of the MOST IMPORTANT Rust concepts.
//
// They are used for:
// - shared behavior
// - polymorphism
// - generic constraints
// - operator overloading
// - formatting
// - cloning
// - comparing
// - iterators
// - and much more
//
// Many things in Rust are built on traits.
//
// Example standard traits:
// - Debug
// - Clone
// - Copy
// - PartialEq
// - Eq
// - PartialOrd
// - Ord
// - Default
// - Iterator
//
// ============================================================================



// ============================================================================
// SECTION 5: #[derive(...)] AGAIN
// ============================================================================
//
// Rust has many standard traits that can often be auto-generated.
//
// Syntax:
//     #[derive(Debug, Clone, PartialEq)]
//
// This asks the compiler to generate implementations for those traits.
//
// WHY USE derive?
// - less boilerplate
// - common behavior
// - easier debugging
//
// Common derives:
//
// Debug
// - allows printing with {:?}
//
// Clone
// - allows explicit duplication with .clone()
//
// Copy
// - allows simple copy instead of move for small simple types
//
// PartialEq
// - allows == and !=
//
// Eq
// - stronger equality guarantee
//
// IMPORTANT:
// Not every trait can be derived.
// Some traits must be implemented manually.
//
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    age: u32,
}



// ============================================================================
// SECTION 6: MODULE EXAMPLES START HERE
// ============================================================================
//
// In this file we define several modules.
//
// Each module shows different module concepts.
// We'll also use traits inside and outside modules.
//
// ============================================================================



// ============================================================================
// SECTION 7: A SIMPLE MODULE
// ============================================================================
//
// This module contains helper functions about math.
//
// Since the module itself is private by default to the file scope,
// it is accessible in this file because we're in the same crate root.
// But functions inside are also private unless marked pub.
//
// We make chosen functions public so they can be used outside the module.
//
// ============================================================================

mod math_tools {
    // Public function: callable from outside this module.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // Private function: only available inside math_tools.
    fn secret_formula(x: i32) -> i32 {
        x * 1000
    }

    // Public function that internally uses a private function.
    pub fn demo_internal_use(x: i32) -> i32 {
        // This works because secret_formula is in the same module.
        secret_formula(x) + 1
    }
}



// ============================================================================
// SECTION 8: NESTED MODULES
// ============================================================================
//
// Modules can contain submodules.
//
// Example:
// company
//   ├─ hr
//   └─ engineering
//
// Paths become:
// - company::hr::...
// - company::engineering::...
//
// ============================================================================

mod company {
    pub mod hr {
        pub fn hire_employee(name: &str) {
            println!("HR hired employee: {}", name);
        }
    }

    pub mod engineering {
        pub fn build_feature(feature: &str) {
            println!("Engineering built feature: {}", feature);
        }
    }
}



// ============================================================================
// SECTION 9: self IN MODULES
// ============================================================================
//
// self means:
// "current module"
//
// Example:
// self::helper()
//
// This is useful when you want to be explicit that you're referencing
// something from the same module.
//
// ============================================================================

mod greetings {
    pub fn say_hello() {
        println!("Hello from greetings::say_hello");
        self::helper();
    }

    fn helper() {
        println!("Hello from greetings::helper");
    }
}



// ============================================================================
// SECTION 10: super IN MODULES
// ============================================================================
//
// super means:
// "parent module"
//
// Useful in nested modules.
//
// Example:
// super::something
//
// means:
// go one module level up, then find something.
//
// ============================================================================

mod outer {
    pub fn outer_function() {
        println!("This is outer::outer_function");
    }

    pub mod inner {
        pub fn call_parent() {
            println!("inner is about to call parent using super::");
            super::outer_function();
        }
    }
}



// ============================================================================
// SECTION 11: crate IN PATHS
// ============================================================================
//
// crate means:
// "start from the root of the current crate"
//
// It is an absolute path inside your own project.
//
// Example:
// crate::math_tools::add(1, 2)
//
// This is very useful in larger projects because it is explicit and clear.
//
// ============================================================================

mod path_demo {
    pub fn show() {
        let result = crate::math_tools::add(10, 20);
        println!("crate path demo result = {}", result);
    }
}



// ============================================================================
// SECTION 12: STRUCTS INSIDE MODULES
// ============================================================================
//
// A public struct can be defined inside a module.
//
// Again:
// making the struct public does NOT make fields public automatically.
//
// ============================================================================

mod models {
    #[derive(Debug, Clone)]
    pub struct User {
        pub username: String,
        age: u32,
    }

    impl User {
        // Public constructor-like associated function.
        pub fn new(username: &str, age: u32) -> Self {
            Self {
                username: username.to_string(),
                age,
            }
        }

        // Public getter method.
        pub fn age(&self) -> u32 {
            self.age
        }
    }
}



// ============================================================================
// SECTION 13: enum INSIDE MODULES
// ============================================================================
//
// Enums can also live inside modules.
//
// ============================================================================

mod status {
    #[derive(Debug, Clone, Copy)]
    pub enum TaskStatus {
        Todo,
        InProgress,
        Done,
    }

    pub fn print_status(status: TaskStatus) {
        match status {
            TaskStatus::Todo => println!("Status: Todo"),
            TaskStatus::InProgress => println!("Status: InProgress"),
            TaskStatus::Done => println!("Status: Done"),
        }
    }
}



// ============================================================================
// SECTION 14: use
// ============================================================================
//
// use brings paths into scope.
//
// Without use:
//     crate::status::TaskStatus::Todo
//
// With use:
//     use crate::status::TaskStatus;
//
// Then:
//     TaskStatus::Todo
//
// Or you can import specific items:
//
//     use crate::math_tools::add;
//
// Then just call:
//     add(1, 2)
//
// We will do some use statements below.
//
// ============================================================================

use crate::math_tools::add;
use crate::models::User;
use crate::status::TaskStatus;



// ============================================================================
// SECTION 15: MODULE ORGANIZATION MENTAL MODEL
// ============================================================================
//
// In real projects, a common structure may look like:
//
// crate
// ├─ main.rs
// ├─ models
// ├─ services
// ├─ utils
// └─ traits
//
// Each module groups related logic.
//
// Good module design usually means:
// - one clear purpose per module
// - related items grouped together
// - visibility kept as narrow as possible
//
// "As narrow as possible" means:
// don't make everything pub by default.
// Expose only what outside code truly needs.
//
// ============================================================================



// ============================================================================
// SECTION 16: FIRST TRAIT EXAMPLE
// ============================================================================
//
// Let's define our own trait.
//
// This trait says:
// any type implementing Describable must provide a describe() method.
//
// ============================================================================

trait Describable {
    fn describe(&self) -> String;
}



// ============================================================================
// SECTION 17: IMPLEMENTING A TRAIT
// ============================================================================
//
// Syntax:
//
// impl TraitName for TypeName {
//     ...
// }
//
// This means:
// "TypeName implements TraitName"
//
// After a type implements the trait, it can be used anywhere
// that expects that trait.
//
// ============================================================================

impl Describable for Student {
    fn describe(&self) -> String {
        format!("Student(name={}, age={})", self.name, self.age)
    }
}



// ============================================================================
// SECTION 18: ANOTHER TYPE IMPLEMENTING THE SAME TRAIT
// ============================================================================
//
// Traits are useful because MULTIPLE types can implement the same trait.
//
// This is shared behavior.
//
// ============================================================================

#[derive(Debug, Clone)]
struct Book {
    title: String,
    pages: u32,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book(title={}, pages={})", self.title, self.pages)
    }
}



// ============================================================================
// SECTION 19: TRAIT METHODS CAN HAVE DEFAULT IMPLEMENTATIONS
// ============================================================================
//
// A trait method does not always need to be required.
//
// You can provide a default implementation.
//
// That means:
// if a type implements the trait but does not override that method,
// it gets the default behavior.
//
// ============================================================================

trait Summary {
    fn summary(&self) -> String {
        String::from("Default summary")
    }
}

impl Summary for Student {
    fn summary(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl Summary for Book {
    // This type does NOT override summary().
    // So Book gets the default implementation.
}



// ============================================================================
// SECTION 20: TRAIT WITH REQUIRED + DEFAULT METHODS
// ============================================================================
//
// A trait can mix:
// - required methods
// - default methods
//
// ============================================================================

trait AnimalSound {
    // Required method: implementer MUST define this.
    fn sound(&self) -> &str;

    // Default method: implementer gets this behavior automatically
    // unless it chooses to override it.
    fn speak(&self) {
        println!("Animal says: {}", self.sound());
    }
}

struct Dog;
struct Cat;

impl AnimalSound for Dog {
    fn sound(&self) -> &str {
        "Woof"
    }
}

impl AnimalSound for Cat {
    fn sound(&self) -> &str {
        "Meow"
    }

    // We override the default method here.
    fn speak(&self) {
        println!("Cat specifically says: {}", self.sound());
    }
}



// ============================================================================
// SECTION 21: TRAITS AS FUNCTION PARAMETERS
// ============================================================================
//
// If a function needs any type that implements a trait,
// we can write:
//
// fn print_description(item: &impl Describable)
//
// This means:
// item can be any type, as long as it implements Describable.
//
// This is called "impl Trait" syntax in parameter position.
//
// ============================================================================

fn print_description(item: &impl Describable) {
    println!("Description: {}", item.describe());
}



// ============================================================================
// SECTION 22: TRAIT BOUNDS WITH GENERICS
// ============================================================================
//
// Another way:
//
// fn print_description_generic<T: Describable>(item: &T)
//
// This is similar, but uses an explicit generic type parameter.
//
// Both styles are common.
//
// ============================================================================

fn print_description_generic<T: Describable>(item: &T) {
    println!("Generic description: {}", item.describe());
}



// ============================================================================
// SECTION 23: MULTIPLE TRAIT BOUNDS
// ============================================================================
//
// Sometimes a type must implement more than one trait.
//
// Syntax:
//
// T: TraitA + TraitB
//
// Example below:
// we require Debug AND Summary.
//
// ============================================================================

fn show_debug_and_summary<T: std::fmt::Debug + Summary>(item: &T) {
    println!("Debug   => {:?}", item);
    println!("Summary => {}", item.summary());
}



// ============================================================================
// SECTION 24: WHERE CLAUSE
// ============================================================================
//
// Long trait bounds can make function signatures hard to read.
//
// Rust provides where clauses.
//
// Example:
//
// fn my_func<T, U>(x: &T, y: &U)
// where
//     T: TraitA + TraitB,
//     U: TraitC,
// {
//     ...
// }
//
// This is often cleaner.
//
// ============================================================================

fn compare_descriptions<T, U>(left: &T, right: &U)
where
    T: Describable,
    U: Describable,
{
    println!("Left  => {}", left.describe());
    println!("Right => {}", right.describe());
}



// ============================================================================
// SECTION 25: TRAITS ENABLE POLYMORPHISM
// ============================================================================
//
// Polymorphism means:
// different concrete types can be used through the same shared interface.
//
// In Rust, traits are one of the main tools for this.
//
// Student and Book are very different types,
// but both implement Describable.
//
// So both can be passed to functions expecting Describable.
//
// ============================================================================



// ============================================================================
// SECTION 26: STANDARD TRAITS ARE EVERYWHERE
// ============================================================================
//
// You have already been using traits even if you didn't realize it.
//
// Examples:
//
// Debug
// - used by println!("{:?}", value)
//
// Clone
// - used by value.clone()
//
// PartialEq
// - used by ==
//
// Default
// - often used to create default values
//
// We will show a simple custom Default example.
//
// ============================================================================

#[derive(Debug, Default)]
struct Config {
    retries: u32,
    verbose: bool,
}



// ============================================================================
// SECTION 27: MANUAL TRAIT IMPLEMENTATION
// ============================================================================
//
// Not all traits need derive.
// You can implement traits manually.
//
// Here we implement Summary manually for Config.
//
// ============================================================================

impl Summary for Config {
    fn summary(&self) -> String {
        format!("Config(retries={}, verbose={})", self.retries, self.verbose)
    }
}



// ============================================================================
// SECTION 28: TRAIT METHODS CAN USE self, &self, &mut self
// ============================================================================
//
// Just like normal impl methods, trait methods can use:
//
// self
// - take ownership
//
// &self
// - immutable borrow
//
// &mut self
// - mutable borrow
//
// Example below shows a mutable trait method.
//
// ============================================================================

trait Resettable {
    fn reset(&mut self);
}

impl Resettable for Config {
    fn reset(&mut self) {
        self.retries = 0;
        self.verbose = false;
    }
}



// ============================================================================
// SECTION 29: ASSOCIATED FUNCTIONS IN TRAITS
// ============================================================================
//
// Traits can also define associated functions,
// not just methods taking self.
//
// Example:
//
// trait Create {
//     fn create() -> Self;
// }
//
// Self here means "the implementing type".
//
// ============================================================================

trait Create {
    fn create() -> Self;
}

#[derive(Debug)]
struct EmptyMessage;

impl Create for EmptyMessage {
    fn create() -> Self {
        EmptyMessage
    }
}



// ============================================================================
// SECTION 30: TRAITS CAN BE IMPLEMENTED FOR MULTIPLE TYPES
// ============================================================================
//
// Let's create a simple trait called Area.
//
// Then implement it for different shapes.
//
// ============================================================================

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}



// ============================================================================
// SECTION 31: USING TRAITS IN GENERIC ALGORITHMS
// ============================================================================
//
// If multiple types implement the same trait,
// generic functions can work with all of them.
//
// ============================================================================

fn print_area<T: Area>(shape: &T) {
    println!("Area = {}", shape.area());
}



// ============================================================================
// SECTION 32: impl Trait AS RETURN TYPE
// ============================================================================
//
// Rust also allows impl Trait in return position.
//
// Example:
// fn make_speaker() -> impl Summary
//
// This means:
// "This function returns some concrete type implementing Summary"
//
// IMPORTANT BEGINNER NOTE:
// A function returning impl Trait must still return ONE concrete type
// consistently for all branches.
//
// You cannot return Student in one branch and Book in another
// unless you use more advanced techniques.
//
// ============================================================================

fn make_default_book() -> impl Summary {
    Book {
        title: String::from("Rust Notes"),
        pages: 120,
    }
}



// ============================================================================
// SECTION 33: TRAITS AND METHOD CALLS
// ============================================================================
//
// Once a trait is in scope and implemented for a type,
// you can call its methods like normal methods.
//
// Example:
// student.summary()
//
// This feels like OOP-style method syntax,
// but under the hood it is trait-based.
//
// ============================================================================



// ============================================================================
// SECTION 34: DERIVE VS MANUAL IMPLEMENTATION
// ============================================================================
//
// Use derive when:
// - standard automatic behavior is enough
//
// Implement manually when:
// - custom behavior is needed
// - derive is not available
//
// Example:
// - Debug can often be derived
// - Summary is our own trait, so we manually implement it
//
// ============================================================================



// ============================================================================
// SECTION 35: MODULE WITH ITS OWN TRAIT
// ============================================================================
//
// Traits can live inside modules too.
//
// Then you reference them by path.
//
// ============================================================================

mod devices {
    pub trait Switchable {
        fn turn_on(&self);
        fn turn_off(&self);
    }

    pub struct Light;
    pub struct Fan;

    impl Switchable for Light {
        fn turn_on(&self) {
            println!("Light turned on");
        }

        fn turn_off(&self) {
            println!("Light turned off");
        }
    }

    impl Switchable for Fan {
        fn turn_on(&self) {
            println!("Fan turned on");
        }

        fn turn_off(&self) {
            println!("Fan turned off");
        }
    }
}



// ============================================================================
// SECTION 36: USING A TRAIT FROM A MODULE
// ============================================================================
//
// To use trait methods cleanly, it is often useful to bring the trait into scope.
//
// We use the full path or use statement.
//
// ============================================================================

use crate::devices::Switchable;



// ============================================================================
// SECTION 37: TRAITS AND API DESIGN
// ============================================================================
//
// Traits are powerful for API design.
//
// Instead of requiring one exact concrete type,
// a function can accept any type with the needed behavior.
//
// This makes code:
// - flexible
// - reusable
// - testable
//
// Good rule:
// depend on behavior, not just concrete types.
//
// ============================================================================



// ============================================================================
// SECTION 38: COMMON STANDARD LIBRARY TRAITS EXPLAINED
// ============================================================================
//
// Debug
// - developer-friendly printing
//
// Clone
// - explicit duplication
//
// Copy
// - cheap implicit copying, only for simple types
//
// PartialEq
// - == and !=
//
// Default
// - a default value
//
// These are traits too.
//
// ============================================================================



// ============================================================================
// SECTION 39: EXAMPLE OF PartialEq
// ============================================================================
//
// Because Student derived PartialEq,
// we can compare students using ==
//
// ============================================================================

fn partial_eq_example() {
    println!("\n====================================================");
    println!("partial_eq_example");
    println!("====================================================");

    let s1 = Student {
        name: String::from("Jay"),
        age: 28,
    };

    let s2 = Student {
        name: String::from("Jay"),
        age: 28,
    };

    let s3 = Student {
        name: String::from("Mira"),
        age: 24,
    };

    println!("s1 == s2 ? {}", s1 == s2);
    println!("s1 == s3 ? {}", s1 == s3);
}



// ============================================================================
// SECTION 40: EXAMPLE OF Clone
// ============================================================================
//
// Because Student derived Clone,
// we can make a duplicate.
//
// clone() is explicit.
// Rust makes cloning explicit to keep ownership clear.
//
// ============================================================================

fn clone_example() {
    println!("\n====================================================");
    println!("clone_example");
    println!("====================================================");

    let original = Student {
        name: String::from("Jay"),
        age: 28,
    };

    let copied = original.clone();

    println!("original = {:?}", original);
    println!("copied   = {:?}", copied);
}



// ============================================================================
// SECTION 41: USING MODULE ITEMS IN MULTIPLE WAYS
// ============================================================================
//
// We can access module items:
//
// 1. full path
//    crate::math_tools::add(1, 2)
//
// 2. imported name through use
//    add(1, 2)
//
// Both are valid.
//
// ============================================================================

fn module_path_examples() {
    println!("\n====================================================");
    println!("module_path_examples");
    println!("====================================================");

    let a = crate::math_tools::add(2, 3);
    let b = add(4, 5);

    println!("crate::math_tools::add(2, 3) = {}", a);
    println!("add(4, 5) via use import     = {}", b);

    let product = crate::math_tools::multiply(6, 7);
    println!("multiply(6, 7) = {}", product);

    let hidden_demo = crate::math_tools::demo_internal_use(3);
    println!("demo_internal_use(3) = {}", hidden_demo);
}



// ============================================================================
// SECTION 42: USING PUBLIC STRUCTS FROM MODULES
// ============================================================================
//
// User is public.
// username field is public.
// age field is private.
// so outside the module we must use public APIs like methods for age.
//
// ============================================================================

fn module_struct_example() {
    println!("\n====================================================");
    println!("module_struct_example");
    println!("====================================================");

    let user = User::new("jay.shah", 28);

    println!("public username field = {}", user.username);
    println!("private age accessed through method = {}", user.age());
}



// ============================================================================
// SECTION 43: USING ENUMS FROM MODULES
// ============================================================================

fn module_enum_example() {
    println!("\n====================================================");
    println!("module_enum_example");
    println!("====================================================");

    let status1 = TaskStatus::Todo;
    let status2 = TaskStatus::InProgress;
    let status3 = TaskStatus::Done;

    crate::status::print_status(status1);
    crate::status::print_status(status2);
    crate::status::print_status(status3);
}



// ============================================================================
// SECTION 44: USING self / super / crate DEMOS
// ============================================================================

fn module_keyword_examples() {
    println!("\n====================================================");
    println!("module_keyword_examples");
    println!("====================================================");

    crate::greetings::say_hello();
    crate::outer::inner::call_parent();
    crate::path_demo::show();
    crate::company::hr::hire_employee("Asha");
    crate::company::engineering::build_feature("Payments Dashboard");
}



// ============================================================================
// SECTION 45: TRAIT USAGE EXAMPLES
// ============================================================================

fn trait_basic_examples() {
    println!("\n====================================================");
    println!("trait_basic_examples");
    println!("====================================================");

    let student = Student {
        name: String::from("Jay"),
        age: 28,
    };

    let book = Book {
        title: String::from("Rust Book"),
        pages: 550,
    };

    println!("student.describe() = {}", student.describe());
    println!("book.describe()    = {}", book.describe());

    print_description(&student);
    print_description(&book);

    print_description_generic(&student);
    print_description_generic(&book);

    compare_descriptions(&student, &book);
}



// ============================================================================
// SECTION 46: DEFAULT TRAIT METHOD EXAMPLES
// ============================================================================

fn default_trait_method_examples() {
    println!("\n====================================================");
    println!("default_trait_method_examples");
    println!("====================================================");

    let student = Student {
        name: String::from("Mira"),
        age: 24,
    };

    let book = Book {
        title: String::from("Systems Design"),
        pages: 300,
    };

    println!("Student summary = {}", student.summary());
    println!("Book summary    = {}", book.summary());
}



// ============================================================================
// SECTION 47: ANIMAL TRAIT EXAMPLES
// ============================================================================

fn animal_trait_examples() {
    println!("\n====================================================");
    println!("animal_trait_examples");
    println!("====================================================");

    let dog = Dog;
    let cat = Cat;

    dog.speak();
    cat.speak();
}



// ============================================================================
// SECTION 48: STANDARD TRAIT EXAMPLES
// ============================================================================

fn standard_trait_examples() {
    println!("\n====================================================");
    println!("standard_trait_examples");
    println!("====================================================");

    let config = Config::default();
    println!("Config::default() => {:?}", config);
    println!("config.summary()  => {}", config.summary());
}



// ============================================================================
// SECTION 49: MUTABLE TRAIT METHOD EXAMPLE
// ============================================================================

fn resettable_example() {
    println!("\n====================================================");
    println!("resettable_example");
    println!("====================================================");

    let mut config = Config {
        retries: 5,
        verbose: true,
    };

    println!("Before reset => {:?}", config);
    config.reset();
    println!("After reset  => {:?}", config);
}



// ============================================================================
// SECTION 50: ASSOCIATED FUNCTION TRAIT EXAMPLE
// ============================================================================

fn create_trait_example() {
    println!("\n====================================================");
    println!("create_trait_example");
    println!("====================================================");

    let msg = EmptyMessage::create();
    println!("Created via trait associated function => {:?}", msg);
}



// ============================================================================
// SECTION 51: AREA TRAIT EXAMPLE
// ============================================================================

fn area_trait_example() {
    println!("\n====================================================");
    println!("area_trait_example");
    println!("====================================================");

    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle {
        radius: 3.0,
    };

    print_area(&rect);
    print_area(&circle);
}



// ============================================================================
// SECTION 52: impl Trait RETURN EXAMPLE
// ============================================================================

fn impl_trait_return_example() {
    println!("\n====================================================");
    println!("impl_trait_return_example");
    println!("====================================================");

    let item = make_default_book();
    println!("Returned summary => {}", item.summary());
}



// ============================================================================
// SECTION 53: TRAIT FROM MODULE EXAMPLE
// ============================================================================

fn module_trait_example() {
    println!("\n====================================================");
    println!("module_trait_example");
    println!("====================================================");

    let light = crate::devices::Light;
    let fan = crate::devices::Fan;

    light.turn_on();
    light.turn_off();

    fan.turn_on();
    fan.turn_off();
}



// ============================================================================
// SECTION 54: WHEN TO USE MODULES
// ============================================================================
//
// Use modules when:
// - code is growing
// - you want clean organization
// - you want to separate domains
// - you want visibility control
//
// Example module ideas:
// - models
// - services
// - utils
// - api
// - config
// - traits
//
// ============================================================================

fn module_summary_notes() {
    println!("\n====================================================");
    println!("module_summary_notes");
    println!("====================================================");

    println!("Use modules to:");
    println!("- group related code");
    println!("- create namespaces");
    println!("- control visibility with pub");
    println!("- keep large projects manageable");
}



// ============================================================================
// SECTION 55: WHEN TO USE TRAITS
// ============================================================================
//
// Use traits when:
// - multiple types share behavior
// - functions should accept behavior, not one exact type
// - you want reusable generic APIs
// - you want default shared behavior
//
// ============================================================================

fn trait_summary_notes() {
    println!("\n====================================================");
    println!("trait_summary_notes");
    println!("====================================================");

    println!("Use traits to:");
    println!("- define shared behavior");
    println!("- write generic reusable functions");
    println!("- enable polymorphism");
    println!("- provide default behavior");
}



// ============================================================================
// SECTION 56: MAIN
// ============================================================================
//
// Program starts here.
//
// We call all examples one by one.
//
// ============================================================================

fn main() {
    println!("====================================================");
    println!("RUST LAB: MODULES + TRAITS");
    println!("====================================================");

    module_path_examples();
    module_struct_example();
    module_enum_example();
    module_keyword_examples();

    trait_basic_examples();
    default_trait_method_examples();
    animal_trait_examples();
    standard_trait_examples();
    resettable_example();
    create_trait_example();
    area_trait_example();
    impl_trait_return_example();
    module_trait_example();

    partial_eq_example();
    clone_example();

    module_summary_notes();
    trait_summary_notes();

    println!("\n====================================================");
    println!("LAB COMPLETE");
    println!("====================================================");
}



// ============================================================================
// FINAL SUMMARY
// ============================================================================
//
// MODULES
// -------
// Keyword:
// - mod
//
// Purpose:
// - organize code
// - create namespaces
// - group related items
// - control visibility
//
// Important keywords:
// - pub   -> make visible outside module
// - use   -> bring paths into scope
// - self  -> current module
// - super -> parent module
// - crate -> crate root
//
// Important notes:
// - items are private by default
// - pub struct does not make fields public automatically
//
// TRAITS
// ------
// Keyword:
// - trait
//
// Purpose:
// - define shared behavior
// - enable polymorphism
// - constrain generics
//
// Important syntax:
// - trait MyTrait { ... }
// - impl MyTrait for MyType { ... }
//
// Common features:
// - required methods
// - default methods
// - trait bounds
// - impl Trait
//
// COMMON TRAITS
// -------------
// - Debug
// - Clone
// - PartialEq
// - Default
//
// RULE OF THUMB
// -------------
// Modules organize code.
// Traits organize behavior.
//
// In a big Rust program:
// - modules help structure your codebase
// - traits help structure shared capabilities
//
// ============================================================================
