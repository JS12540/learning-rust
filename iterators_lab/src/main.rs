// ============================================================================
// RUST LAB: ITERATORS + CLOSURES
// ============================================================================
//
// This file is a deep beginner-friendly lab for:
//
// 1. What an iterator is
// 2. Why Rust uses iterators so much
// 3. The Iterator trait
// 4. iter(), iter_mut(), into_iter()
// 5. Ownership and borrowing with iteration
// 6. map()
// 7. filter()
// 8. collect()
// 9. for_each()
// 10. find()
// 11. any() / all()
// 12. fold()
// 13. reduce()
// 14. enumerate()
// 15. zip()
// 16. take() / skip()
// 17. chaining iterator adapters
// 18. lazy evaluation
// 19. What a closure is
// 20. Closure syntax
// 21. Type inference in closures
// 22. Capturing environment
// 23. Fn / FnMut / FnOnce basics
// 24. move closures
// 25. Sorting with closures
// 26. Returning iterators from pipelines
// 27. Common beginner mistakes
//
// This file is intentionally LONG and HEAVILY COMMENTED.
// The goal is not short code.
// The goal is understanding.
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
// SECTION 1: IMPORTS
// ============================================================================
//
// We only need standard library items.
//
// Ordering is imported for custom sort examples.
//
// std::cmp::Ordering is an enum usually used in comparisons:
//
// - Ordering::Less
// - Ordering::Equal
// - Ordering::Greater
//
// ============================================================================

use std::cmp::Ordering;



// ============================================================================
// SECTION 2: #[derive(...)] AGAIN
// ============================================================================
//
// Rust types often derive common traits.
//
// Common derive traits used in labs:
// - Debug      -> print with {:?}
// - Clone      -> make explicit copies
// - PartialEq  -> compare with ==
// - Eq         -> stronger equality relation
//
// We use Debug because we want easy printing.
// We use Clone when we want a copy of owned data like String.
//
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    age: u32,
    score: u32,
}



// ============================================================================
// SECTION 3: WHAT IS AN ITERATOR?
// ============================================================================
//
// An iterator is a value that produces items one by one.
//
// Core idea:
// Instead of manually indexing through data,
// Rust often uses iterators to process sequences.
//
// Very roughly, an iterator answers:
// "What is the next item?"
//
// The core trait behind this is:
//
//     Iterator
//
// The central method is:
//
//     next()
//
// Conceptually:
//
// trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// IMPORTANT:
// - next() returns Some(item) while items remain
// - next() returns None when iteration is finished
//
// This should remind you of Option.
// Rust uses Option a lot for safe control flow.
//
// ============================================================================



// ============================================================================
// SECTION 4: WHAT IS A CLOSURE?
// ============================================================================
//
// A closure is an anonymous function.
//
// "Anonymous" means:
// it does not need a normal function name like fn add(...) { ... }
//
// Example:
//
//     let add_one = |x| x + 1;
//
// Here:
// - |x| is the parameter list
// - x + 1 is the body
//
// Closures are used constantly with iterators.
//
// Why?
// Because iterators often need a little piece of behavior:
//
// - map each item into something
// - filter items by a condition
// - sort using custom logic
//
// Closures let us pass behavior directly inline.
//
// ============================================================================



// ============================================================================
// SECTION 5: iter(), iter_mut(), into_iter()
// ============================================================================
//
// These are some of the MOST IMPORTANT iterator-related methods in Rust.
//
// Given a collection like Vec<T>:
//
// 1. iter()
//    - borrows each element immutably
//    - yields &T
//
// 2. iter_mut()
//    - borrows each element mutably
//    - yields &mut T
//
// 3. into_iter()
//    - consumes the collection
//    - yields T for owned collections like Vec<T>
//
// This is deeply connected to ownership.
//
// Example with Vec<i32>:
//
// let v = vec![1, 2, 3];
//
// v.iter()       -> iterator over &i32
// v.iter_mut()   -> iterator over &mut i32
// v.into_iter()  -> iterator over i32, consuming v
//
// ============================================================================

fn iter_iter_mut_into_iter_examples() {
    println!("\n====================================================");
    println!("iter_iter_mut_into_iter_examples");
    println!("====================================================");

    let values = vec![10, 20, 30];

    println!("Using iter() -> immutable borrows:");
    for value in values.iter() {
        // value has type &i32
        println!("value from iter() = {}", value);
    }

    // values is still usable because iter() only borrowed.
    println!("values after iter() = {:?}", values);

    let mut values2 = vec![1, 2, 3];

    println!("\nUsing iter_mut() -> mutable borrows:");
    for value in values2.iter_mut() {
        // value has type &mut i32
        *value *= 10;
    }
    println!("values2 after iter_mut() = {:?}", values2);

    let values3 = vec![7, 8, 9];

    println!("\nUsing into_iter() -> ownership moves out:");
    for value in values3.into_iter() {
        // value has type i32 here
        println!("owned value from into_iter() = {}", value);
    }

    // values3 cannot be used here anymore because it was consumed.
}



// ============================================================================
// SECTION 6: MANUAL next() EXAMPLE
// ============================================================================
//
// This is important because it reveals what iteration really is.
//
// next() is the core method.
//
// It takes &mut self because calling next() changes iterator state.
// After each call, the iterator advances.
//
// ============================================================================

fn manual_next_example() {
    println!("\n====================================================");
    println!("manual_next_example");
    println!("====================================================");

    let values = vec![100, 200, 300];

    let mut iter = values.iter();

    println!("first  next() = {:?}", iter.next());
    println!("second next() = {:?}", iter.next());
    println!("third  next() = {:?}", iter.next());
    println!("fourth next() = {:?}", iter.next());

    // After items are exhausted, next() keeps returning None.
    println!("fifth  next() = {:?}", iter.next());
}



// ============================================================================
// SECTION 7: FOR LOOP IS BUILT ON ITERATION
// ============================================================================
//
// A for loop in Rust works with IntoIterator.
//
// This means:
// if something can be turned into an iterator,
// for can loop over it.
//
// Very roughly:
//
// for item in collection {
//     ...
// }
//
// is sugar over iteration machinery.
//
// This is why Vec, arrays, ranges, HashMap, etc. can all work in for loops.
//
// ============================================================================

fn for_loop_with_range_example() {
    println!("\n====================================================");
    println!("for_loop_with_range_example");
    println!("====================================================");

    // 1..=5 is an inclusive range.
    // It produces 1, 2, 3, 4, 5.
    for n in 1..=5 {
        println!("range item = {}", n);
    }
}



// ============================================================================
// SECTION 8: ITERATOR ADAPTERS VS CONSUMING ADAPTERS
// ============================================================================
//
// This is a big concept.
//
// Iterator methods are often grouped into:
//
// 1. Iterator adapters
//    - transform an iterator into another iterator
//    - examples: map, filter, take, skip, enumerate, zip
//
// 2. Consuming adapters
//    - consume the iterator to produce a final result
//    - examples: collect, sum, count, for_each, find, fold, reduce
//
// IMPORTANT:
// Many iterator chains do nothing until a consuming operation happens.
// This is because iterators are LAZY.
//
// Lazy means:
// work is postponed until needed.
//
// ============================================================================



// ============================================================================
// SECTION 9: LAZY EVALUATION
// ============================================================================
//
// Iterator pipelines are usually lazy.
//
// Example:
//
// let iter = values.iter().map(|x| x * 2);
//
// At this point, map has been defined,
// but actual work is not completed yet.
//
// Once we do something like collect() or for_each(),
// the iterator is consumed and the computation happens.
//
// ============================================================================

fn lazy_evaluation_example() {
    println!("\n====================================================");
    println!("lazy_evaluation_example");
    println!("====================================================");

    let values = vec![1, 2, 3, 4];

    let doubled_iter = values.iter().map(|x| {
        println!("mapping {}", x);
        x * 2
    });

    println!("Iterator created. No collection yet.");

    // Now the computation actually runs because collect consumes the iterator.
    let doubled: Vec<i32> = doubled_iter.collect();

    println!("doubled = {:?}", doubled);
}



// ============================================================================
// SECTION 10: map()
// ============================================================================
//
// map() transforms each item into another item.
//
// Signature conceptually:
//
// iterator.map(|item| transformed_item)
//
// It returns a NEW iterator.
// It does not modify the original collection directly.
//
// Example:
// [1, 2, 3] -> map(|x| x * 2) -> [2, 4, 6]
//
// ============================================================================

fn map_examples() {
    println!("\n====================================================");
    println!("map_examples");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("original numbers = {:?}", numbers);
    println!("doubled numbers  = {:?}", doubled);

    let words = vec!["rust", "is", "great"];

    let uppercase: Vec<String> = words
        .iter()
        .map(|word| word.to_uppercase())
        .collect();

    println!("words      = {:?}", words);
    println!("uppercase  = {:?}", uppercase);
}



// ============================================================================
// SECTION 11: filter()
// ============================================================================
//
// filter() keeps only items that satisfy a condition.
//
// Example:
// [1, 2, 3, 4, 5, 6]
// filter even numbers
// => [2, 4, 6]
//
// Important detail:
// filter closure returns bool.
//
// If true  -> keep item
// If false -> discard item
//
// ============================================================================

fn filter_examples() {
    println!("\n====================================================");
    println!("filter_examples");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let evens: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .collect();

    println!("numbers = {:?}", numbers);
    println!("evens   = {:?}", evens);

    let words = vec!["cat", "elephant", "dog", "giraffe"];

    let long_words: Vec<&str> = words
        .iter()
        .copied()
        .filter(|word| word.len() > 3)
        .collect();

    println!("words      = {:?}", words);
    println!("long_words = {:?}", long_words);
}



// ============================================================================
// SECTION 12: WHY copied() OR cloned() SOMETIMES APPEARS
// ============================================================================
//
// This is an important ownership-related concept.
//
// If you do:
//
// numbers.iter()
//
// then the iterator yields references, like &i32.
//
// Sometimes you want owned values instead of references.
//
// For Copy types like i32, char, bool:
// - copied() is convenient
//
// For Clone types like String:
// - cloned() is convenient
//
// Example:
//
// numbers.iter().copied()
//
// transforms iterator items from &i32 into i32.
//
// ============================================================================

fn copied_and_cloned_examples() {
    println!("\n====================================================");
    println!("copied_and_cloned_examples");
    println!("====================================================");

    let numbers = vec![10, 20, 30];

    let owned_numbers: Vec<i32> = numbers.iter().copied().collect();
    println!("owned_numbers from copied() = {:?}", owned_numbers);

    let names = vec![String::from("Jay"), String::from("Mira")];

    let cloned_names: Vec<String> = names.iter().cloned().collect();
    println!("original names = {:?}", names);
    println!("cloned_names   = {:?}", cloned_names);
}



// ============================================================================
// SECTION 13: collect()
// ============================================================================
//
// collect() consumes an iterator and builds a collection.
//
// Very common forms:
// - collect::<Vec<_>>()
// - collect::<HashMap<_, _>>() in other contexts
//
// Because collect is generic, Rust sometimes needs type hints.
//
// Example:
// let v: Vec<i32> = iterator.collect();
//
// ============================================================================

fn collect_examples() {
    println!("\n====================================================");
    println!("collect_examples");
    println!("====================================================");

    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("squares = {:?}", squares);

    let chars: Vec<char> = "rust".chars().collect();
    println!("chars from \"rust\" = {:?}", chars);
}



// ============================================================================
// SECTION 14: for_each()
// ============================================================================
//
// for_each() is a consuming adapter.
//
// It runs a closure for each item.
//
// Often used for side effects like printing.
//
// Example:
// iterator.for_each(|item| println!("{}", item));
//
// Note:
// a normal for loop is often clearer.
// But for_each is still useful and common.
//
// ============================================================================

fn for_each_example() {
    println!("\n====================================================");
    println!("for_each_example");
    println!("====================================================");

    let values = vec![2, 4, 6];

    values.iter().for_each(|value| {
        println!("for_each saw {}", value);
    });
}



// ============================================================================
// SECTION 15: find()
// ============================================================================
//
// find() searches for the first item matching a condition.
//
// It returns Option<Item>.
//
// Why Option?
// Because a matching item may or may not exist.
//
// ============================================================================

fn find_example() {
    println!("\n====================================================");
    println!("find_example");
    println!("====================================================");

    let numbers = vec![3, 7, 10, 15, 20];

    let first_even = numbers.iter().find(|x| **x % 2 == 0);
    println!("first_even from iter() = {:?}", first_even);

    let first_gt_100 = numbers.iter().find(|x| **x > 100);
    println!("first_gt_100 = {:?}", first_gt_100);
}



// ============================================================================
// SECTION 16: any() AND all()
// ============================================================================
//
// any(predicate)
// - true if at least one item matches
//
// all(predicate)
// - true if every item matches
//
// Both are consuming adapters.
//
// ============================================================================

fn any_all_examples() {
    println!("\n====================================================");
    println!("any_all_examples");
    println!("====================================================");

    let numbers = vec![2, 4, 6, 8];

    let has_odd = numbers.iter().any(|x| x % 2 != 0);
    let all_even = numbers.iter().all(|x| x % 2 == 0);

    println!("numbers  = {:?}", numbers);
    println!("has_odd  = {}", has_odd);
    println!("all_even = {}", all_even);
}



// ============================================================================
// SECTION 17: count() AND sum()
// ============================================================================
//
// count() counts how many items remain in an iterator.
// sum() adds items together.
//
// These are consuming adapters.
//
// ============================================================================

fn count_sum_examples() {
    println!("\n====================================================");
    println!("count_sum_examples");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4, 5];

    let count = numbers.iter().count();
    let sum: i32 = numbers.iter().copied().sum();

    println!("numbers = {:?}", numbers);
    println!("count   = {}", count);
    println!("sum     = {}", sum);
}



// ============================================================================
// SECTION 18: fold()
// ============================================================================
//
// fold() is one of the most important iterator methods.
//
// It accumulates a result by repeatedly combining items.
//
// Shape:
//
// iterator.fold(initial_value, |accumulator, item| new_accumulator)
//
// Example: sum numbers
// fold(0, |acc, x| acc + x)
//
// Why "fold"?
// Because it folds many items into one final result.
//
// ============================================================================

fn fold_examples() {
    println!("\n====================================================");
    println!("fold_examples");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4];

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum via fold = {}", sum);

    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("product via fold = {}", product);

    let joined = ["Rust", " ", "Lab"]
        .iter()
        .fold(String::new(), |mut acc, part| {
            acc.push_str(part);
            acc
        });

    println!("joined string via fold = {}", joined);
}



// ============================================================================
// SECTION 19: reduce()
// ============================================================================
//
// reduce() is similar to fold() but without an explicit initial value.
//
// It uses the first item as the starting accumulator.
//
// Because the iterator may be empty, reduce returns Option<Item>.
//
// ============================================================================

fn reduce_examples() {
    println!("\n====================================================");
    println!("reduce_examples");
    println!("====================================================");

    let numbers = vec![5, 10, 15, 20];

    let sum = numbers.iter().copied().reduce(|acc, x| acc + x);
    println!("sum via reduce = {:?}", sum);

    let max = numbers.iter().copied().reduce(|acc, x| acc.max(x));
    println!("max via reduce = {:?}", max);

    let empty: Vec<i32> = vec![];
    let empty_reduce = empty.into_iter().reduce(|acc, x| acc + x);
    println!("reduce on empty vec = {:?}", empty_reduce);
}



// ============================================================================
// SECTION 20: enumerate()
// ============================================================================
//
// enumerate() pairs each item with its index.
//
// It produces items like:
// (0, first_item), (1, second_item), ...
//
// Very useful when you need both index and value.
//
// ============================================================================

fn enumerate_example() {
    println!("\n====================================================");
    println!("enumerate_example");
    println!("====================================================");

    let names = vec!["Jay", "Mira", "Sara"];

    for (index, name) in names.iter().enumerate() {
        println!("index = {}, name = {}", index, name);
    }
}



// ============================================================================
// SECTION 21: zip()
// ============================================================================
//
// zip() combines two iterators into one iterator of pairs.
//
// Example:
// [1, 2, 3] zipped with ["a", "b", "c"]
// -> [(1, "a"), (2, "b"), (3, "c")]
//
// Stops when the shorter iterator ends.
//
// ============================================================================

fn zip_example() {
    println!("\n====================================================");
    println!("zip_example");
    println!("====================================================");

    let ids = vec![1, 2, 3];
    let names = vec!["Jay", "Mira", "Sara"];

    let pairs: Vec<(i32, &str)> = ids.into_iter().zip(names.into_iter()).collect();
    println!("zipped pairs = {:?}", pairs);
}



// ============================================================================
// SECTION 22: take() AND skip()
// ============================================================================
//
// take(n) -> keep only first n items
// skip(n) -> ignore first n items
//
// These are iterator adapters.
//
// ============================================================================

fn take_skip_examples() {
    println!("\n====================================================");
    println!("take_skip_examples");
    println!("====================================================");

    let values = vec![10, 20, 30, 40, 50];

    let first_two: Vec<i32> = values.iter().copied().take(2).collect();
    let after_two: Vec<i32> = values.iter().copied().skip(2).collect();

    println!("values    = {:?}", values);
    println!("first_two = {:?}", first_two);
    println!("after_two = {:?}", after_two);
}



// ============================================================================
// SECTION 23: CHAINING ITERATOR OPERATIONS
// ============================================================================
//
// One of the most common Rust styles is chaining.
//
// Example pipeline:
// - iterate
// - filter
// - map
// - collect
//
// Each step is small and focused.
//
// ============================================================================

fn chaining_example() {
    println!("\n====================================================");
    println!("chaining_example");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();

    println!("numbers = {:?}", numbers);
    println!("even squares = {:?}", result);
}



// ============================================================================
// SECTION 24: ITERATING OVER CUSTOM STRUCTS
// ============================================================================
//
// Here we use iterators with Vec<Student>.
//
// We will:
// - map names
// - filter top performers
// - collect results
//
// ============================================================================

fn iterator_with_structs_example() {
    println!("\n====================================================");
    println!("iterator_with_structs_example");
    println!("====================================================");

    let students = vec![
        Student {
            name: "Jay".to_string(),
            age: 28,
            score: 90,
        },
        Student {
            name: "Mira".to_string(),
            age: 24,
            score: 75,
        },
        Student {
            name: "Sara".to_string(),
            age: 30,
            score: 95,
        },
    ];

    let names: Vec<String> = students.iter().map(|student| student.name.clone()).collect();
    println!("student names = {:?}", names);

    let top_students: Vec<&Student> = students.iter().filter(|student| student.score >= 90).collect();
    println!("top_students = {:?}", top_students);

    let average_score =
        students.iter().map(|student| student.score).sum::<u32>() as f64 / students.len() as f64;

    println!("average_score = {:.2}", average_score);
}



// ============================================================================
// SECTION 25: WHAT IS A CLOSURE SYNTAX?
// ============================================================================
//
// General closure syntax:
//
// |parameters| expression
//
// or
//
// |parameters| {
//     multiple statements
// }
//
// Examples:
//
// |x| x + 1
// |x, y| x + y
// || 42
//
// Closures can often infer parameter and return types.
//
// Equivalent normal function:
//
// fn add_one(x: i32) -> i32 {
//     x + 1
// }
//
// Equivalent closure:
//
// let add_one = |x: i32| -> i32 { x + 1 };
//
// Usually Rust can infer the types so you write:
//
// let add_one = |x| x + 1;
//
// ============================================================================

fn closure_syntax_examples() {
    println!("\n====================================================");
    println!("closure_syntax_examples");
    println!("====================================================");

    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(10) = {}", add_one(10));

    let multiply = |a, b| a * b;
    println!("multiply(3, 4) = {}", multiply(3, 4));

    let say_hello = || println!("hello from closure");
    say_hello();
}



// ============================================================================
// SECTION 26: CLOSURE TYPE INFERENCE
// ============================================================================
//
// Closures are often more flexible than functions in local usage.
//
// Rust can infer parameter and return types from how the closure is used.
//
// But once inferred for a closure value, that closure type becomes fixed
// for that usage context.
//
// ============================================================================

fn closure_type_inference_example() {
    println!("\n====================================================");
    println!("closure_type_inference_example");
    println!("====================================================");

    let square = |x| x * x;

    println!("square(5) = {}", square(5));

    // Once used with i32 here, the closure's inferred type becomes tied to that.
    // This is why closures are not always "fully generic" the way named generic
    // functions can be.
}



// ============================================================================
// SECTION 27: CLOSURES CAN CAPTURE ENVIRONMENT
// ============================================================================
//
// This is one of the biggest differences from plain functions.
//
// A closure can use variables from the surrounding scope.
//
// Example:
//
// let factor = 10;
// let multiply = |x| x * factor;
//
// The closure captures factor from its environment.
//
// This is why closures are extremely powerful.
//
// ============================================================================

fn closure_capture_example() {
    println!("\n====================================================");
    println!("closure_capture_example");
    println!("====================================================");

    let factor = 10;

    let multiply_by_factor = |x| x * factor;

    println!("multiply_by_factor(3) = {}", multiply_by_factor(3));
    println!("multiply_by_factor(7) = {}", multiply_by_factor(7));
}



// ============================================================================
// SECTION 28: HOW CLOSURES CAPTURE
// ============================================================================
//
// Closures can capture variables in different ways:
//
// 1. By immutable borrow
// 2. By mutable borrow
// 3. By move (taking ownership)
//
// Rust decides the least powerful capture needed,
// unless you force move.
//
// ============================================================================

fn closure_capture_mutably_example() {
    println!("\n====================================================");
    println!("closure_capture_mutably_example");
    println!("====================================================");

    let mut count = 0;

    // This closure mutates count, so it captures count mutably.
    let mut increment = || {
        count += 1;
        println!("count inside closure = {}", count);
    };

    increment();
    increment();
    increment();

    println!("count after closure calls = {}", count);
}



// ============================================================================
// SECTION 29: Fn, FnMut, FnOnce
// ============================================================================
//
// These are closure traits.
//
// Very important concept.
//
// 1. Fn
//    - closure can be called with shared reference behavior
//    - does not need to mutate captured environment
//    - does not consume captured values
//
// 2. FnMut
//    - closure may mutate captured environment
//
// 3. FnOnce
//    - closure may consume captured values
//    - can always be called at least once
//
// Relationship idea:
// Fn is the most restrictive in a good way
// FnMut is more powerful
// FnOnce is the most general / ownership-taking
//
// Many APIs specify one of these bounds.
//
// ============================================================================

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn call_fnmut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

fn call_fnonce<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn fn_fnmut_fnonce_examples() {
    println!("\n====================================================");
    println!("fn_fnmut_fnonce_examples");
    println!("====================================================");

    let say_hi = || println!("Hi from Fn closure");
    call_fn(say_hi);

    let mut counter = 0;
    let increase = || {
        counter += 1;
        println!("counter in FnMut closure = {}", counter);
    };
    call_fnmut(increase);

    let text = String::from("owned text");
    let consume_text = move || {
        println!("FnOnce closure consumed text: {}", text);
    };
    call_fnonce(consume_text);
}



// ============================================================================
// SECTION 30: move CLOSURES
// ============================================================================
//
// "move" forces the closure to take ownership of captured variables.
//
// Syntax:
//
// move |x| ...
//
// Why use it?
// - when closure must own captured data
// - often needed for threads
// - useful when you want the closure independent of outer scope
//
// Note:
// move does not necessarily mean data is immediately destroyed.
// It means ownership is moved into the closure.
//
// ============================================================================

fn move_closure_example() {
    println!("\n====================================================");
    println!("move_closure_example");
    println!("====================================================");

    let message = String::from("hello from moved closure");

    let printer = move || {
        println!("{}", message);
    };

    printer();

    // message cannot be used here because ownership moved into the closure.
}



// ============================================================================
// SECTION 31: CLOSURES WITH sort_by_key() AND sort_by()
// ============================================================================
//
// Closures are commonly used for sorting.
//
// sort_by_key(|item| key)
// - extracts a key used for ordering
//
// sort_by(|a, b| ...)
// - custom comparison using Ordering
//
// ============================================================================

fn sorting_with_closures_example() {
    println!("\n====================================================");
    println!("sorting_with_closures_example");
    println!("====================================================");

    let mut students = vec![
        Student {
            name: "Mira".to_string(),
            age: 24,
            score: 88,
        },
        Student {
            name: "Jay".to_string(),
            age: 28,
            score: 95,
        },
        Student {
            name: "Sara".to_string(),
            age: 30,
            score: 91,
        },
    ];

    students.sort_by_key(|student| student.age);
    println!("students sorted by age = {:?}", students);

    students.sort_by(|a, b| b.score.cmp(&a.score));
    println!("students sorted by descending score = {:?}", students);

    students.sort_by(|a, b| {
        let score_order = a.score.cmp(&b.score);
        if score_order == Ordering::Equal {
            a.name.cmp(&b.name)
        } else {
            score_order
        }
    });

    println!("students sorted by score then name = {:?}", students);
}



// ============================================================================
// SECTION 32: filter_map()
// ============================================================================
//
// filter_map() combines filter + map.
//
// The closure returns Option<U>.
//
// - Some(value) -> keep transformed value
// - None        -> discard item
//
// This is very elegant when parsing or conditionally transforming.
//
// ============================================================================

fn filter_map_example() {
    println!("\n====================================================");
    println!("filter_map_example");
    println!("====================================================");

    let items = vec!["10", "abc", "25", "xyz", "40"];

    let parsed_numbers: Vec<i32> = items
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("items = {:?}", items);
    println!("parsed_numbers = {:?}", parsed_numbers);
}



// ============================================================================
// SECTION 33: flat_map()
// ============================================================================
//
// flat_map() maps each item to an iterable,
// then flattens the results into one iterator.
//
// Beginner mental model:
// - map into groups
// - flatten into one stream
//
// ============================================================================

fn flat_map_example() {
    println!("\n====================================================");
    println!("flat_map_example");
    println!("====================================================");

    let words = vec!["hi", "rust"];

    let chars: Vec<char> = words.iter().flat_map(|word| word.chars()).collect();

    println!("words = {:?}", words);
    println!("all chars flattened = {:?}", chars);
}



// ============================================================================
// SECTION 34: inspect()
// ============================================================================
//
// inspect() lets you look at items in the middle of an iterator chain.
//
// It is useful for debugging pipelines.
//
// It does not change items.
// It just observes them.
//
// ============================================================================

fn inspect_example() {
    println!("\n====================================================");
    println!("inspect_example");
    println!("====================================================");

    let result: Vec<i32> = (1..=5)
        .inspect(|x| println!("before map: {}", x))
        .map(|x| x * 10)
        .inspect(|x| println!("after map: {}", x))
        .collect();

    println!("final result = {:?}", result);
}



// ============================================================================
// SECTION 35: POSITIONAL THINKING - REFERENCES VS OWNED VALUES
// ============================================================================
//
// This is one of the biggest beginner pain points.
//
// Example:
//
// values.iter().map(|x| x * 2)
//
// Here x is usually &i32 if iter() is used.
// Rust can dereference automatically in many simple arithmetic cases.
//
// But with Strings or custom structs, ownership matters more.
//
// Common patterns:
//
// iter()       -> borrowed items
// iter_mut()   -> mutable borrowed items
// into_iter()  -> owned items
//
// copied()     -> for Copy types from references to values
// cloned()     -> for Clone types from references to owned clones
//
// ============================================================================

fn references_vs_owned_example() {
    println!("\n====================================================");
    println!("references_vs_owned_example");
    println!("====================================================");

    let names = vec![
        String::from("jay"),
        String::from("mira"),
        String::from("sara"),
    ];

    let uppercased: Vec<String> = names.iter().map(|name| name.to_uppercase()).collect();

    println!("original names = {:?}", names);
    println!("uppercased     = {:?}", uppercased);

    let consumed_lengths: Vec<usize> = names.into_iter().map(|name| name.len()).collect();

    println!("consumed lengths = {:?}", consumed_lengths);

    // names cannot be used here anymore because into_iter consumed it.
}



// ============================================================================
// SECTION 36: AVERAGE WITH ITERATORS
// ============================================================================
//
// This shows a practical small computation.
//
// ============================================================================

fn average_example() {
    println!("\n====================================================");
    println!("average_example");
    println!("====================================================");

    let scores = vec![80_u32, 90, 100, 70];

    let total: u32 = scores.iter().copied().sum();
    let average = total as f64 / scores.len() as f64;

    println!("scores  = {:?}", scores);
    println!("total   = {}", total);
    println!("average = {:.2}", average);
}



// ============================================================================
// SECTION 37: BUILDING A SMALL PIPELINE
// ============================================================================
//
// This combines several concepts:
//
// - iter()
// - filter()
// - map()
// - collect()
//
// ============================================================================

fn practical_pipeline_example() {
    println!("\n====================================================");
    println!("practical_pipeline_example");
    println!("====================================================");

    let students = vec![
        Student {
            name: "Jay".to_string(),
            age: 28,
            score: 90,
        },
        Student {
            name: "Mira".to_string(),
            age: 24,
            score: 65,
        },
        Student {
            name: "Sara".to_string(),
            age: 30,
            score: 95,
        },
        Student {
            name: "Omar".to_string(),
            age: 22,
            score: 70,
        },
    ];

    let passed_names: Vec<String> = students
        .iter()
        .filter(|student| student.score >= 70)
        .map(|student| student.name.clone())
        .collect();

    println!("passed_names = {:?}", passed_names);
}



// ============================================================================
// SECTION 38: CLOSURES AS FUNCTION PARAMETERS
// ============================================================================
//
// Functions can accept closures.
//
// This is extremely common in Rust.
//
// Example generic bound:
//
// F: Fn(i32) -> i32
//
// Meaning:
// F is some type that can be called like a function,
// taking i32 and returning i32.
//
// ============================================================================

fn apply_to_10<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(10)
}

fn closure_parameter_example() {
    println!("\n====================================================");
    println!("closure_parameter_example");
    println!("====================================================");

    let result1 = apply_to_10(|x| x + 5);
    let result2 = apply_to_10(|x| x * 3);

    println!("apply_to_10(|x| x + 5) = {}", result1);
    println!("apply_to_10(|x| x * 3) = {}", result2);
}



// ============================================================================
// SECTION 39: RETURNING VALUES FROM CLOSURES
// ============================================================================
//
// Closures can return values like functions.
//
// If the body is a single expression,
// that expression becomes the return value.
//
// Example:
// |x| x + 1
//
// If using braces, the last expression without semicolon is returned.
//
// Example:
// |x| {
//     let y = x + 1;
//     y
// }
//
// ============================================================================

fn closure_return_example() {
    println!("\n====================================================");
    println!("closure_return_example");
    println!("====================================================");

    let add_one = |x| x + 1;

    let add_two = |x| {
        let y = x + 2;
        y
    };

    println!("add_one(5) = {}", add_one(5));
    println!("add_two(5) = {}", add_two(5));
}



// ============================================================================
// SECTION 40: COMMON BEGINNER MISTAKES
// ============================================================================
//
// 1. Forgetting iterators are lazy
//    - map() alone does not execute fully
//
// 2. Confusing iter() vs into_iter()
//    - iter() borrows
//    - into_iter() consumes
//
// 3. Forgetting collect()
//    - if you want a Vec result, you often need collect()
//
// 4. Not understanding reference item types
//    - iter() often yields &T, not T
//
// 5. Trying to use a moved value after into_iter()
//
// 6. Forgetting closures can capture outer variables
//
// 7. Using complex iterator chains too early
//    - sometimes a for loop is clearer
//
// ============================================================================

fn beginner_mistake_notes() {
    println!("\n====================================================");
    println!("beginner_mistake_notes");
    println!("====================================================");

    println!("Remember:");
    println!("- iter() borrows");
    println!("- iter_mut() mutably borrows");
    println!("- into_iter() consumes ownership");
    println!("- map/filter are lazy");
    println!("- collect()/sum()/count()/for_each() consume the iterator");
    println!("- closures can capture variables from outer scope");
}



// ============================================================================
// SECTION 41: WHEN TO PREFER A FOR LOOP
// ============================================================================
//
// Iterators are powerful, but not every situation should be a long chain.
//
// Prefer a for loop when:
// - logic is more imperative
// - readability is better
// - side effects dominate
//
// Prefer iterator chains when:
// - transformation pipeline is clear
// - data flow is concise and readable
//
// Rust style values clarity more than cleverness.
//
// ============================================================================

fn for_loop_vs_iterator_style_example() {
    println!("\n====================================================");
    println!("for_loop_vs_iterator_style_example");
    println!("====================================================");

    let numbers = vec![1, 2, 3, 4, 5, 6];

    let mut even_squares_loop = Vec::new();
    for n in &numbers {
        if n % 2 == 0 {
            even_squares_loop.push(n * n);
        }
    }

    let even_squares_iter: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();

    println!("result from loop     = {:?}", even_squares_loop);
    println!("result from iterator = {:?}", even_squares_iter);
}



// ============================================================================
// SECTION 42: MAIN
// ============================================================================
//
// Program entry point.
//
// We run each example one by one.
//
// ============================================================================

fn main() {
    println!("====================================================");
    println!("RUST LAB: ITERATORS + CLOSURES");
    println!("====================================================");

    iter_iter_mut_into_iter_examples();
    manual_next_example();
    for_loop_with_range_example();
    lazy_evaluation_example();
    map_examples();
    filter_examples();
    copied_and_cloned_examples();
    collect_examples();
    for_each_example();
    find_example();
    any_all_examples();
    count_sum_examples();
    fold_examples();
    reduce_examples();
    enumerate_example();
    zip_example();
    take_skip_examples();
    chaining_example();
    iterator_with_structs_example();
    closure_syntax_examples();
    closure_type_inference_example();
    closure_capture_example();
    closure_capture_mutably_example();
    fn_fnmut_fnonce_examples();
    move_closure_example();
    sorting_with_closures_example();
    filter_map_example();
    flat_map_example();
    inspect_example();
    references_vs_owned_example();
    average_example();
    practical_pipeline_example();
    closure_parameter_example();
    closure_return_example();
    beginner_mistake_notes();
    for_loop_vs_iterator_style_example();

    println!("\n====================================================");
    println!("LAB COMPLETE");
    println!("====================================================");
}



// ============================================================================
// FINAL SUMMARY
// ============================================================================
//
// ITERATORS
// ---------
// An iterator produces items one by one.
//
// Core trait:
// - Iterator
//
// Core method:
// - next() -> Option<Item>
//
// COMMON METHODS
// --------------
// iter()       -> immutable references
// iter_mut()   -> mutable references
// into_iter()  -> owned items, consuming collection
//
// ADAPTERS
// --------
// map()        -> transform items
// filter()     -> keep matching items
// enumerate()  -> add index
// zip()        -> pair items from two iterators
// take()       -> first n items
// skip()       -> ignore first n items
// inspect()    -> observe items in pipeline
// filter_map() -> combine filtering + mapping
// flat_map()   -> flatten nested iterables
//
// CONSUMING ADAPTERS
// ------------------
// collect() -> build collection
// sum()     -> add items
// count()   -> count items
// for_each()-> perform side effect
// find()    -> first matching item
// any()     -> does any item match?
// all()     -> do all items match?
// fold()    -> accumulate with initial value
// reduce()  -> accumulate without explicit initial value
//
// IMPORTANT IDEAS
// ---------------
// - iterators are lazy
// - many chains do nothing until consumed
// - ownership matters: iter vs into_iter
// - references vs owned values is a major concept
//
// CLOSURES
// --------
// A closure is an anonymous function.
//
// Syntax:
// |x| x + 1
//
// Important features:
// - can infer types
// - can capture surrounding variables
// - often used with iterators
//
// CLOSURE TRAITS
// --------------
// Fn     -> shared access capture style
// FnMut  -> mutable capture style
// FnOnce -> may consume captured values
//
// move
// ----
// move forces captured values into the closure by ownership.
//
// RULE OF THUMB
// -------------
// Use iterator chains when they make the data flow clear.
// Use for loops when they are easier to read.
//
// ============================================================================
