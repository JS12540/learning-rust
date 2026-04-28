// ============================================================================
// RUST LAB: Vec + HashMap
// ============================================================================
//
// This is a full beginner-friendly lab for:
//
// 1. Vec<T>
// 2. HashMap<K, V>
// 3. ownership ideas related to Vec and HashMap
// 4. borrowing with references
// 5. mutable vs immutable access
// 6. indexing vs get()
// 7. iteration
// 8. push / pop / insert / remove
// 9. contains / len / is_empty / clear
// 10. sorting Vec
// 11. common HashMap methods
// 12. entry API
// 13. counting frequency with HashMap
// 14. storing structs in Vec
// 15. storing enums in Vec
// 16. nested collections
//
// This file is intentionally LONG and HEAVILY COMMENTED.
// The goal is teaching, not being short.
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
// SECTION 1: use
// ============================================================================
//
// "use" is a Rust keyword that brings a path into scope.
//
// Without this:
//
//     std::collections::HashMap
//
// With this:
//
//     use std::collections::HashMap;
//
// That means we can just write HashMap instead of the full path every time.
//
// HashMap lives in the standard library module:
//     std::collections
//
// Vec is special:
// Vec is so common that it is already available in the prelude,
// so we do NOT need to import it manually.
//
// ============================================================================

use std::collections::HashMap;



// ============================================================================
// SECTION 2: #[derive(...)] AGAIN
// ============================================================================
//
// Rust uses TRAITS to define shared behavior.
//
// Common traits:
// - Debug      -> print with {:?}
// - Clone      -> make an explicit duplicate
// - Copy       -> simple copy for small stack-only data
// - PartialEq  -> compare with ==
// - Eq         -> stronger equality trait
// - Hash       -> allows a type to be used as a key in HashMap in many cases
//
// Writing traits manually is possible, but repetitive.
//
// So Rust provides:
//
//     #[derive(Debug, Clone, PartialEq)]
//
// That asks the compiler to automatically generate implementations.
//
// WHY WE USE derive HERE:
// - Debug: print values easily
// - Clone: duplicate when useful
// - PartialEq: compare values
//
// IMPORTANT:
// Copy is NOT used for types containing String,
// because String owns heap data and is not Copy.
//
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    age: u32,
    active: bool,
}



// ============================================================================
// SECTION 3: AN ENUM WE WILL STORE INSIDE A Vec
// ============================================================================
//
// enum means:
// "one value that can be one of several variants"
//
// We include this to show that Vec can store ANY single type,
// including your own enums.
//
// Here each TaskStatus value is exactly one variant at a time.
//
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}



// ============================================================================
// SECTION 4: WHAT IS Vec<T> ?
// ============================================================================
//
// Vec is short for "vector".
//
// Full type name style:
//     Vec<T>
//
// T is a generic type parameter.
// That means Vec can store many different types.
//
// Examples:
// - Vec<i32>      -> vector of integers
// - Vec<String>   -> vector of strings
// - Vec<Student>  -> vector of Student structs
//
// WHAT Vec IS USED FOR:
// A Vec is a growable, ordered collection of values.
//
// KEY CHARACTERISTICS:
// - growable
// - stored in order
// - same type for all elements
// - stored contiguously in memory
//
// WHY PEOPLE USE Vec ALL THE TIME:
// - it is the main dynamic list type in Rust
// - great when order matters
// - great when you want to add/remove items
//
// THINK OF Vec AS:
// "A resizable array"
//
// ============================================================================



// ============================================================================
// SECTION 5: WHAT IS HashMap<K, V> ?
// ============================================================================
//
// HashMap is a key-value collection.
//
// Full type style:
//     HashMap<K, V>
//
// K = key type
// V = value type
//
// Examples:
// - HashMap<String, i32>
// - HashMap<&str, u32>
// - HashMap<String, Student>
//
// WHAT HashMap IS USED FOR:
// - fast lookup by key
// - storing relationships
// - counting things
// - caching
// - configuration maps
//
// THINK OF HashMap AS:
// "Dictionary / map / object-like lookup table"
//
// Example:
// key   -> value
// "apple" -> 3
// "banana" -> 5
//
// IMPORTANT:
// HashMap is NOT about order.
// If you print it, insertion order is not guaranteed.
//
// ============================================================================



// ============================================================================
// SECTION 6: VEC CREATION
// ============================================================================
//
// There are multiple ways to create a Vec.
//
// 1. Vec::new()
// 2. vec![...]
//
// "vec!" is a MACRO, not a normal function.
//
// MACRO:
// - ends with !
// - expands into code
//
// Example:
//     vec![1, 2, 3]
//
// creates a Vec containing 1, 2, 3.
//
// ============================================================================

fn vec_creation_examples() {
    println!("\n====================================================");
    println!("vec_creation_examples");
    println!("====================================================");

    // Empty vector with explicit type annotation.
    // Type annotation is needed here because Rust cannot infer the element type
    // from an empty Vec::new() alone.
    let mut numbers: Vec<i32> = Vec::new();

    // push adds values to the end.
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("numbers created with Vec::new() = {:?}", numbers);

    // Created directly with vec! macro.
    let fruits = vec!["apple", "banana", "orange"];

    println!("fruits created with vec! macro = {:?}", fruits);

    // Another example with Strings.
    let words = vec![
        String::from("rust"),
        String::from("is"),
        String::from("fun"),
    ];

    println!("words = {:?}", words);
}



// ============================================================================
// SECTION 7: BASIC VEC METHODS
// ============================================================================
//
// Common Vec methods:
//
// push(value)      -> add to end
// pop()            -> remove from end, returns Option<T>
// len()            -> number of elements
// is_empty()       -> true if no elements
// clear()          -> remove all elements
//
// IMPORTANT CONCEPT:
// pop() returns Option<T>
//
// Why?
// Because the Vec may be empty.
// Rust forces us to handle that possibility safely.
//
// ============================================================================

fn vec_basic_methods() {
    println!("\n====================================================");
    println!("vec_basic_methods");
    println!("====================================================");

    let mut values = vec![1, 2, 3];

    println!("start values = {:?}", values);
    println!("len = {}", values.len());
    println!("is_empty = {}", values.is_empty());

    values.push(4);
    println!("after push(4) = {:?}", values);

    let popped = values.pop();
    println!("after pop() values = {:?}", values);
    println!("pop() returned = {:?}", popped);

    values.clear();
    println!("after clear() values = {:?}", values);
    println!("is_empty now = {}", values.is_empty());
}



// ============================================================================
// SECTION 8: INDEXING VS get()
// ============================================================================
//
// Vec supports indexing:
//
//     values[0]
//
// But indexing can PANIC if index is out of bounds.
//
// PANIC means:
// the program stops with a runtime error.
//
// Safer alternative:
//
//     values.get(index)
//
// get() returns Option<&T>
//
// Why Option?
// - Some(&value) if index exists
// - None if index does not exist
//
// This is a classic Rust design:
// risky operations often have safe alternatives.
//
// ============================================================================

fn vec_indexing_vs_get() {
    println!("\n====================================================");
    println!("vec_indexing_vs_get");
    println!("====================================================");

    let values = vec![100, 200, 300];

    // Indexing when we KNOW index exists.
    let first = values[0];
    println!("values[0] = {}", first);

    // Safe access with get().
    let maybe_second = values.get(1);
    println!("values.get(1) = {:?}", maybe_second);

    let maybe_tenth = values.get(10);
    println!("values.get(10) = {:?}", maybe_tenth);

    match values.get(2) {
        Some(value) => println!("Found index 2, value = {}", value),
        None => println!("Index 2 not found"),
    }

    match values.get(99) {
        Some(value) => println!("Found index 99, value = {}", value),
        None => println!("Index 99 not found"),
    }
}



// ============================================================================
// SECTION 9: OWNERSHIP WITH Vec
// ============================================================================
//
// Rust ownership matters a lot with Vec.
//
// Example:
//
//     let v1 = vec![1, 2, 3];
//     let v2 = v1;
//
// For Vec, this MOVES ownership.
// v1 can no longer be used afterward.
//
// Why?
// Because Vec owns heap data.
//
// If you want a second independent Vec, use clone():
//
//     let v2 = v1.clone();
//
// That creates a real duplicate.
//
// ============================================================================

fn vec_ownership_example() {
    println!("\n====================================================");
    println!("vec_ownership_example");
    println!("====================================================");

    let original = vec![1, 2, 3];

    // clone makes a deep duplicate of the Vec data.
    let copied = original.clone();

    println!("original = {:?}", original);
    println!("copied   = {:?}", copied);

    // If we had written:
    //
    // let moved = original;
    //
    // then original would be moved and unusable afterward.
}



// ============================================================================
// SECTION 10: BORROWING A Vec
// ============================================================================
//
// Instead of moving ownership, we can borrow.
//
// &Vec<T>  -> immutable borrow
// &mut Vec<T> -> mutable borrow
//
// More idiomatic in many function signatures is:
// &[T] and &mut [T] for slices
//
// A slice is a view into contiguous elements.
// We will show a simple borrowed Vec example first.
//
// ============================================================================

fn print_vec(values: &Vec<i32>) {
    println!("borrowed vec = {:?}", values);
}

fn push_into_vec(values: &mut Vec<i32>, value: i32) {
    values.push(value);
}

fn vec_borrowing_example() {
    println!("\n====================================================");
    println!("vec_borrowing_example");
    println!("====================================================");

    let mut data = vec![10, 20, 30];

    print_vec(&data);

    push_into_vec(&mut data, 40);
    println!("after mutable borrow and push = {:?}", data);
}



// ============================================================================
// SECTION 11: ITERATION OVER Vec
// ============================================================================
//
// Common ways to iterate:
//
// 1. for value in &vec
//    - borrow each element immutably
//
// 2. for value in &mut vec
//    - borrow each element mutably
//
// 3. for value in vec
//    - move each element out
//
// IMPORTANT:
// Which one you choose affects ownership.
//
// ============================================================================

fn vec_iteration_examples() {
    println!("\n====================================================");
    println!("vec_iteration_examples");
    println!("====================================================");

    let values = vec![5, 10, 15];

    println!("Iterating immutably:");
    for value in &values {
        println!("value = {}", value);
    }

    println!("values still usable after immutable iteration = {:?}", values);

    let mut values2 = vec![1, 2, 3];

    println!("Iterating mutably and changing each value:");
    for value in &mut values2 {
        *value *= 10;
    }
    println!("values2 after mutation = {:?}", values2);

    let values3 = vec![7, 8, 9];

    println!("Iterating by value (moves elements):");
    for value in values3 {
        println!("moved value = {}", value);
    }

    // values3 cannot be used here because it was moved into the loop.
}



// ============================================================================
// SECTION 12: insert / remove
// ============================================================================
//
// Vec supports inserting/removing by index.
//
// insert(index, value)
// remove(index)
//
// IMPORTANT:
// These may shift later elements.
//
// That means these operations can be more expensive than push/pop at the end.
//
// ============================================================================

fn vec_insert_remove_examples() {
    println!("\n====================================================");
    println!("vec_insert_remove_examples");
    println!("====================================================");

    let mut values = vec![10, 20, 40];

    println!("start = {:?}", values);

    values.insert(2, 30);
    println!("after insert(2, 30) = {:?}", values);

    let removed = values.remove(1);
    println!("removed element = {}", removed);
    println!("after remove(1) = {:?}", values);
}



// ============================================================================
// SECTION 13: contains
// ============================================================================
//
// contains(&value) checks whether a Vec has a matching value.
//
// It uses PartialEq to compare.
//
// That is why custom types often derive PartialEq.
//
// ============================================================================

fn vec_contains_example() {
    println!("\n====================================================");
    println!("vec_contains_example");
    println!("====================================================");

    let numbers = vec![10, 20, 30, 40];

    println!("numbers = {:?}", numbers);
    println!("contains 20? {}", numbers.contains(&20));
    println!("contains 99? {}", numbers.contains(&99));

    let students = vec![
        Student {
            name: String::from("Jay"),
            age: 25,
            active: true,
        },
        Student {
            name: String::from("Sara"),
            age: 22,
            active: false,
        },
    ];

    let target = Student {
        name: String::from("Jay"),
        age: 25,
        active: true,
    };

    println!("students contains target? {}", students.contains(&target));
}



// ============================================================================
// SECTION 14: SORTING Vec
// ============================================================================
//
// Vec provides sorting methods.
//
// Common methods:
// - sort()
// - sort_by(...)
// - sort_by_key(...)
//
// sort() works when items implement Ord.
//
// For custom logic, use sort_by or sort_by_key.
//
// ============================================================================

fn vec_sorting_examples() {
    println!("\n====================================================");
    println!("vec_sorting_examples");
    println!("====================================================");

    let mut numbers = vec![5, 1, 4, 2, 3];
    numbers.sort();
    println!("sorted numbers = {:?}", numbers);

    let mut words = vec![
        String::from("banana"),
        String::from("apple"),
        String::from("orange"),
    ];
    words.sort();
    println!("sorted words = {:?}", words);

    let mut students = vec![
        Student {
            name: String::from("Charlie"),
            age: 19,
            active: true,
        },
        Student {
            name: String::from("Alice"),
            age: 25,
            active: false,
        },
        Student {
            name: String::from("Bob"),
            age: 21,
            active: true,
        },
    ];

    students.sort_by_key(|student| student.age);
    println!("students sorted by age = {:?}", students);

    students.sort_by_key(|student| student.name.clone());
    println!("students sorted by name = {:?}", students);
}



// ============================================================================
// SECTION 15: STORING STRUCTS IN Vec
// ============================================================================
//
// Vec can store your custom structs.
//
// This is extremely common in Rust.
//
// Example use cases:
// - list of users
// - list of products
// - list of tasks
//
// ============================================================================

fn vec_of_structs_example() {
    println!("\n====================================================");
    println!("vec_of_structs_example");
    println!("====================================================");

    let students = vec![
        Student {
            name: String::from("Jay"),
            age: 28,
            active: true,
        },
        Student {
            name: String::from("Mira"),
            age: 24,
            active: false,
        },
    ];

    for student in &students {
        println!(
            "Student -> name: {}, age: {}, active: {}",
            student.name, student.age, student.active
        );
    }
}



// ============================================================================
// SECTION 16: STORING ENUMS IN Vec
// ============================================================================
//
// A Vec can also store enum values,
// as long as every element is of the same enum type.
//
// This is useful for lists of states, commands, messages, etc.
//
// ============================================================================

fn vec_of_enums_example() {
    println!("\n====================================================");
    println!("vec_of_enums_example");
    println!("====================================================");

    let tasks = vec![
        TaskStatus::Todo,
        TaskStatus::InProgress,
        TaskStatus::Done,
        TaskStatus::Todo,
    ];

    for task in &tasks {
        match task {
            TaskStatus::Todo => println!("Task is Todo"),
            TaskStatus::InProgress => println!("Task is InProgress"),
            TaskStatus::Done => println!("Task is Done"),
        }
    }
}



// ============================================================================
// SECTION 17: WHAT IS A SLICE?
// ============================================================================
//
// A slice is a borrowed view into contiguous data.
//
// For Vec, the most common slice type is:
//
//     &[T]
//
// Mutable slice:
//
//     &mut [T]
//
// Why slices matter:
// - more flexible than &Vec<T>
// - can borrow part or all of a Vec
// - many APIs prefer slices
//
// ============================================================================

fn print_slice(values: &[i32]) {
    println!("slice = {:?}", values);
}

fn slice_examples() {
    println!("\n====================================================");
    println!("slice_examples");
    println!("====================================================");

    let data = vec![10, 20, 30, 40, 50];

    print_slice(&data);
    print_slice(&data[1..4]);

    let first_three: &[i32] = &data[..3];
    println!("first_three = {:?}", first_three);
}



// ============================================================================
// SECTION 18: WHAT IS HashMap<K, V> IN PRACTICE?
// ============================================================================
//
// HashMap stores key-value pairs.
//
// Example:
//
// "apple" -> 3
//
// Methods we will use:
// - HashMap::new()
// - insert()
// - get()
// - remove()
// - contains_key()
// - len()
// - is_empty()
// - clear()
//
// ============================================================================

fn hashmap_creation_examples() {
    println!("\n====================================================");
    println!("hashmap_creation_examples");
    println!("====================================================");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("math"), 95);
    scores.insert(String::from("science"), 88);

    println!("scores = {:?}", scores);

    let mut capitals = HashMap::new();
    capitals.insert("India", "New Delhi");
    capitals.insert("Japan", "Tokyo");
    capitals.insert("France", "Paris");

    println!("capitals = {:?}", capitals);
}



// ============================================================================
// SECTION 19: INSERTING INTO HashMap
// ============================================================================
//
// insert(key, value)
//
// If the key is new:
// - pair is added
//
// If the key already exists:
// - old value is replaced
//
// Return value of insert():
// - None if key was not present
// - Some(old_value) if key existed and got replaced
//
// ============================================================================

fn hashmap_insert_replace_example() {
    println!("\n====================================================");
    println!("hashmap_insert_replace_example");
    println!("====================================================");

    let mut prices = HashMap::new();

    let old = prices.insert(String::from("apple"), 100);
    println!("first insert returned = {:?}", old);

    let old = prices.insert(String::from("apple"), 120);
    println!("second insert returned old value = {:?}", old);

    println!("prices = {:?}", prices);
}



// ============================================================================
// SECTION 20: ACCESSING VALUES IN HashMap
// ============================================================================
//
// get(&key) returns Option<&V>
//
// Why Option?
// Because the key may not exist.
//
// IMPORTANT:
// For maps with String keys, get usually takes a borrowed form like &str
// or &String depending on context.
//
// ============================================================================

fn hashmap_get_examples() {
    println!("\n====================================================");
    println!("hashmap_get_examples");
    println!("====================================================");

    let mut stock = HashMap::new();
    stock.insert(String::from("pen"), 10);
    stock.insert(String::from("notebook"), 5);

    println!("stock = {:?}", stock);

    let pen_stock = stock.get("pen");
    println!("stock.get(\"pen\") = {:?}", pen_stock);

    let bag_stock = stock.get("bag");
    println!("stock.get(\"bag\") = {:?}", bag_stock);

    match stock.get("notebook") {
        Some(count) => println!("notebook count = {}", count),
        None => println!("notebook key not found"),
    }
}



// ============================================================================
// SECTION 21: contains_key / remove / len / is_empty / clear
// ============================================================================
//
// Common utility methods for HashMap.
//
// contains_key(&key) -> bool
// remove(&key)       -> Option<V>
// len()              -> number of entries
// is_empty()         -> bool
// clear()            -> remove all entries
//
// ============================================================================

fn hashmap_basic_methods() {
    println!("\n====================================================");
    println!("hashmap_basic_methods");
    println!("====================================================");

    let mut map = HashMap::new();

    map.insert("a", 1);
    map.insert("b", 2);

    println!("map = {:?}", map);
    println!("len = {}", map.len());
    println!("is_empty = {}", map.is_empty());
    println!("contains_key(\"a\") = {}", map.contains_key("a"));
    println!("contains_key(\"z\") = {}", map.contains_key("z"));

    let removed = map.remove("a");
    println!("remove(\"a\") returned = {:?}", removed);
    println!("map after remove = {:?}", map);

    map.clear();
    println!("map after clear = {:?}", map);
    println!("is_empty now = {}", map.is_empty());
}



// ============================================================================
// SECTION 22: ITERATING OVER HashMap
// ============================================================================
//
// Common forms:
//
// for (key, value) in &map
// for (key, value) in &mut map
// for (key, value) in map
//
// Similar ownership rules apply as with Vec.
//
// HashMap order is not guaranteed.
//
// ============================================================================

fn hashmap_iteration_examples() {
    println!("\n====================================================");
    println!("hashmap_iteration_examples");
    println!("====================================================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Jay"), 90);
    scores.insert(String::from("Sara"), 95);
    scores.insert(String::from("Mira"), 88);

    println!("Immutable iteration:");
    for (name, score) in &scores {
        println!("{} -> {}", name, score);
    }

    println!("Mutable iteration (add 5):");
    for (_name, score) in &mut scores {
        *score += 5;
    }

    for (name, score) in &scores {
        println!("{} -> {}", name, score);
    }
}



// ============================================================================
// SECTION 23: OWNERSHIP WITH HashMap
// ============================================================================
//
// HashMap owns its keys and values unless they are reference types.
//
// If you insert a String into a HashMap<String, i32>,
// ownership of that String moves into the map.
//
// After move, original variable cannot be used.
//
// ============================================================================

fn hashmap_ownership_example() {
    println!("\n====================================================");
    println!("hashmap_ownership_example");
    println!("====================================================");

    let team_name = String::from("engineering");
    let score = 99;

    let mut map = HashMap::new();

    map.insert(team_name, score);

    println!("map = {:?}", map);

    // team_name cannot be used here anymore because it was moved into the map.
    // score is Copy, so it was copied, not moved.
}



// ============================================================================
// SECTION 24: HashMap WITH STRUCT VALUES
// ============================================================================
//
// HashMap can store your custom types as values.
//
// Example:
// key: String
// value: Student
//
// ============================================================================

fn hashmap_with_struct_values() {
    println!("\n====================================================");
    println!("hashmap_with_struct_values");
    println!("====================================================");

    let mut students_by_id: HashMap<String, Student> = HashMap::new();

    students_by_id.insert(
        String::from("s1"),
        Student {
            name: String::from("Jay"),
            age: 28,
            active: true,
        },
    );

    students_by_id.insert(
        String::from("s2"),
        Student {
            name: String::from("Mira"),
            age: 24,
            active: false,
        },
    );

    println!("students_by_id = {:?}", students_by_id);

    match students_by_id.get("s1") {
        Some(student) => println!("Found s1 -> {:?}", student),
        None => println!("s1 not found"),
    }
}



// ============================================================================
// SECTION 25: HashMap WITH ENUM VALUES
// ============================================================================
//
// We can store enums as values too.
//
// ============================================================================

fn hashmap_with_enum_values() {
    println!("\n====================================================");
    println!("hashmap_with_enum_values");
    println!("====================================================");

    let mut task_map: HashMap<String, TaskStatus> = HashMap::new();

    task_map.insert(String::from("task_1"), TaskStatus::Todo);
    task_map.insert(String::from("task_2"), TaskStatus::InProgress);
    task_map.insert(String::from("task_3"), TaskStatus::Done);

    for (task_name, status) in &task_map {
        print!("{} -> ", task_name);

        match status {
            TaskStatus::Todo => println!("Todo"),
            TaskStatus::InProgress => println!("InProgress"),
            TaskStatus::Done => println!("Done"),
        }
    }
}



// ============================================================================
// SECTION 26: THE entry API
// ============================================================================
//
// HashMap has a very important API called entry().
//
// Why it exists:
// Sometimes you want to:
// - insert if missing
// - modify if present
//
// Common pattern:
//
//     map.entry(key).or_insert(default_value);
//
// Meaning:
// - if key exists, return mutable reference to its current value
// - if key does not exist, insert default_value and return mutable reference
//
// Very useful for counters and accumulation.
//
// ============================================================================

fn hashmap_entry_or_insert_example() {
    println!("\n====================================================");
    println!("hashmap_entry_or_insert_example");
    println!("====================================================");

    let mut scores = HashMap::new();

    scores.insert(String::from("Jay"), 50);

    let jay_score = scores.entry(String::from("Jay")).or_insert(0);
    *jay_score += 10;

    let sara_score = scores.entry(String::from("Sara")).or_insert(0);
    *sara_score += 20;

    println!("scores after entry/or_insert = {:?}", scores);
}



// ============================================================================
// SECTION 27: COUNTING WORD FREQUENCY WITH HashMap
// ============================================================================
//
// This is one of the classic HashMap examples.
//
// Idea:
// - split text into words
// - for each word
// - increment its count in the map
//
// This is a perfect use case for entry().
//
// ============================================================================

fn word_frequency_example() {
    println!("\n====================================================");
    println!("word_frequency_example");
    println!("====================================================");

    let text = "rust is fast and rust is safe and rust is fun";

    let mut counts: HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("word counts = {:?}", counts);

    match counts.get("rust") {
        Some(count) => println!("'rust' appears {} times", count),
        None => println!("'rust' not found"),
    }
}



// ============================================================================
// SECTION 28: GROUPING DATA WITH HashMap<String, Vec<T>>
// ============================================================================
//
// Collections can be nested.
//
// Example:
// HashMap<String, Vec<String>>
//
// This means:
// each key maps to a vector of values.
//
// Very useful for:
// - grouping
// - indexes
// - categories
//
// ============================================================================

fn hashmap_of_vectors_example() {
    println!("\n====================================================");
    println!("hashmap_of_vectors_example");
    println!("====================================================");

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    groups
        .entry(String::from("backend"))
        .or_insert(Vec::new())
        .push(String::from("Jay"));

    groups
        .entry(String::from("backend"))
        .or_insert(Vec::new())
        .push(String::from("Sara"));

    groups
        .entry(String::from("frontend"))
        .or_insert(Vec::new())
        .push(String::from("Mira"));

    println!("groups = {:?}", groups);

    for (team, members) in &groups {
        println!("team = {}", team);
        for member in members {
            println!("  member = {}", member);
        }
    }
}



// ============================================================================
// SECTION 29: Vec<HashMap<...>> EXAMPLE
// ============================================================================
//
// We can also put HashMaps inside a Vec.
//
// Example meaning:
// a list of records
//
// ============================================================================

fn vec_of_hashmaps_example() {
    println!("\n====================================================");
    println!("vec_of_hashmaps_example");
    println!("====================================================");

    let mut record1 = HashMap::new();
    record1.insert(String::from("name"), String::from("Jay"));
    record1.insert(String::from("role"), String::from("Engineer"));

    let mut record2 = HashMap::new();
    record2.insert(String::from("name"), String::from("Mira"));
    record2.insert(String::from("role"), String::from("Designer"));

    let records = vec![record1, record2];

    for (index, record) in records.iter().enumerate() {
        println!("record {} = {:?}", index, record);
    }
}



// ============================================================================
// SECTION 30: COLLECTING INTO Vec OR HashMap
// ============================================================================
//
// Rust iterators can often be collected into collections using collect().
//
// collect() is generic, so type information is often needed.
//
// Example:
// let v: Vec<_> = iterator.collect();
//
// collect is powerful, but it may look advanced at first.
// This section shows simple examples.
//
// ============================================================================

fn collect_examples() {
    println!("\n====================================================");
    println!("collect_examples");
    println!("====================================================");

    let numbers: Vec<i32> = (1..=5).collect();
    println!("collected Vec from range = {:?}", numbers);

    let pairs = vec![
        (String::from("apple"), 3),
        (String::from("banana"), 5),
        (String::from("orange"), 2),
    ];

    let fruit_map: HashMap<String, i32> = pairs.into_iter().collect();
    println!("collected HashMap from pairs = {:?}", fruit_map);
}



// ============================================================================
// SECTION 31: RESERVING CAPACITY
// ============================================================================
//
// Both Vec and HashMap can reserve capacity.
//
// Capacity = how much space is currently allocated before needing growth.
//
// This is an optimization concept.
//
// Common methods:
// - Vec::with_capacity(n)
// - HashMap::with_capacity(n)
//
// Not required for correctness.
// Useful for performance when you know approximate size in advance.
//
// ============================================================================

fn capacity_examples() {
    println!("\n====================================================");
    println!("capacity_examples");
    println!("====================================================");

    let mut values: Vec<i32> = Vec::with_capacity(10);
    println!("initial vec len = {}, capacity = {}", values.len(), values.capacity());

    values.push(1);
    values.push(2);
    values.push(3);

    println!("after pushes vec len = {}, capacity = {}", values.len(), values.capacity());

    let map: HashMap<String, i32> = HashMap::with_capacity(20);
    println!("created HashMap with_capacity(20), len = {}", map.len());
}



// ============================================================================
// SECTION 32: COMMON BORROWING RULE WITH HashMap
// ============================================================================
//
// A very important Rust idea:
//
// You cannot have mutable and immutable borrows that conflict.
//
// Example with HashMap:
// if you hold an immutable reference from get(),
// you generally should not try to mutably change the map at the same time.
//
// This function demonstrates the safe pattern:
// use the immutable borrow, then let it end, then mutate.
//
// ============================================================================

fn hashmap_borrowing_example() {
    println!("\n====================================================");
    println!("hashmap_borrowing_example");
    println!("====================================================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Jay"), 90);

    {
        let score_ref = scores.get("Jay");
        println!("borrowed score_ref = {:?}", score_ref);
    } // immutable borrow ends here

    scores.insert(String::from("Mira"), 95);
    println!("scores after mutation = {:?}", scores);
}



// ============================================================================
// SECTION 33: WHEN TO USE Vec VS HashMap
// ============================================================================
//
// Use Vec when:
// - order matters
// - you want a list
// - you access by position/index
// - you append items often
//
// Use HashMap when:
// - you need lookup by key
// - you want key-value relationships
// - order does not matter
// - you are counting or grouping
//
// Sometimes you use both together.
//
// ============================================================================

fn vec_vs_hashmap_summary() {
    println!("\n====================================================");
    println!("vec_vs_hashmap_summary");
    println!("====================================================");

    println!("Use Vec when:");
    println!("- you want an ordered list");
    println!("- you access elements by index");
    println!("- you push/pop items");

    println!("\nUse HashMap when:");
    println!("- you want key -> value mapping");
    println!("- you need fast lookup by key");
    println!("- order is not the main concern");
}



// ============================================================================
// SECTION 34: MAIN
// ============================================================================
//
// Program starts here.
//
// We run all examples one by one.
//
// ============================================================================

fn main() {
    println!("====================================================");
    println!("RUST LAB: Vec + HashMap");
    println!("====================================================");

    vec_creation_examples();
    vec_basic_methods();
    vec_indexing_vs_get();
    vec_ownership_example();
    vec_borrowing_example();
    vec_iteration_examples();
    vec_insert_remove_examples();
    vec_contains_example();
    vec_sorting_examples();
    vec_of_structs_example();
    vec_of_enums_example();
    slice_examples();

    hashmap_creation_examples();
    hashmap_insert_replace_example();
    hashmap_get_examples();
    hashmap_basic_methods();
    hashmap_iteration_examples();
    hashmap_ownership_example();
    hashmap_with_struct_values();
    hashmap_with_enum_values();
    hashmap_entry_or_insert_example();
    word_frequency_example();
    hashmap_of_vectors_example();
    vec_of_hashmaps_example();

    collect_examples();
    capacity_examples();
    hashmap_borrowing_example();
    vec_vs_hashmap_summary();

    println!("\n====================================================");
    println!("LAB COMPLETE");
    println!("====================================================");
}



// ============================================================================
// FINAL SUMMARY
// ============================================================================
//
// VEC
// ----
// Vec<T> is a growable ordered collection.
//
// Common methods:
// - Vec::new()
// - vec![...]
// - push()
// - pop()
// - insert()
// - remove()
// - get()
// - len()
// - is_empty()
// - clear()
// - contains()
// - sort()
//
// Important ideas:
// - stores one type T
// - ordered
// - growable
// - indexing can panic
// - get() is safer and returns Option<&T>
//
// HASHMAP
// -------
// HashMap<K, V> stores key-value pairs.
//
// Common methods:
// - HashMap::new()
// - insert()
// - get()
// - remove()
// - contains_key()
// - len()
// - is_empty()
// - clear()
// - entry()
// - or_insert()
//
// Important ideas:
// - lookup by key
// - no guaranteed order
// - keys and values are owned unless references are used
// - get() returns Option<&V>
//
// OWNERSHIP
// ---------
// Vec and HashMap own their contents.
// Assigning them usually moves ownership.
// clone() creates a real duplicate.
//
// BORROWING
// ---------
// - &Vec<T> / &[T] for reading
// - &mut Vec<T> for changing
// - &HashMap<K, V> for reading
// - &mut HashMap<K, V> for changing
//
// ENTRY API
// ---------
// Very useful for:
// - insert if missing
// - update if existing
// - counting frequencies
//
// RULE OF THUMB
// -------------
// Vec     -> ordered list
// HashMap -> key-value lookup
//
// ============================================================================
