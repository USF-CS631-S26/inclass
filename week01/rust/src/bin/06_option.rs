//! 06_option.rs - Some, None, handling absence
//!
//! Option<T> is Rust's way of handling nullable values.
//! Instead of null, we use Some(value) or None.
//!
//! enum Option<T> {
//!     Some(T),
//!     None,
//! }

fn find_first_word(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.split_whitespace().next().unwrap())
    }
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn find_index(haystack: &[i32], needle: i32) -> Option<usize> {
    for (i, &value) in haystack.iter().enumerate() {
        if value == needle {
            return Some(i);
        }
    }
    None
}

fn main() {
    println!("=== Creating Options ===");
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("no_number = {:?}", no_number);

    println!("\n=== Matching on Option ===");
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got nothing"),
    }

    println!("\n=== Using if let ===");
    if let Some(n) = some_number {
        println!("The number is: {}", n);
    }

    if let None = no_number {
        println!("There's no number");
    }

    println!("\n=== Option Methods ===");

    // is_some() and is_none()
    println!("some_number.is_some() = {}", some_number.is_some());
    println!("no_number.is_none() = {}", no_number.is_none());

    // unwrap() - panics if None (use carefully!)
    let value = some_number.unwrap();
    println!("Unwrapped value: {}", value);

    // unwrap_or() - provide default for None
    let value = no_number.unwrap_or(0);
    println!("Unwrap or default: {}", value);

    // unwrap_or_else() - compute default lazily
    let value = no_number.unwrap_or_else(|| {
        println!("Computing default...");
        -1
    });
    println!("Unwrap or else: {}", value);

    println!("\n=== Option in Functions ===");

    // find_first_word
    for s in ["hello world", "  ", "rust", ""] {
        match find_first_word(s) {
            Some(word) => println!("First word of '{}': '{}'", s, word),
            None => println!("No words in '{}'", s),
        }
    }

    // divide
    println!("\n10 / 2 = {:?}", divide(10.0, 2.0));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));

    // find_index
    let numbers = [10, 20, 30, 40, 50];
    println!("\nIndex of 30: {:?}", find_index(&numbers, 30));
    println!("Index of 99: {:?}", find_index(&numbers, 99));

    println!("\n=== map() - Transform the Value ===");
    let maybe_string = Some("hello");
    let maybe_len = maybe_string.map(|s| s.len());
    println!("Length of {:?} = {:?}", maybe_string, maybe_len);

    let nothing: Option<&str> = None;
    let nothing_len = nothing.map(|s| s.len());
    println!("Length of {:?} = {:?}", nothing, nothing_len);

    println!("\n=== and_then() - Chaining Options ===");
    fn parse_and_double(s: &str) -> Option<i32> {
        s.parse::<i32>().ok().map(|n| n * 2)
    }

    let input = Some("21");
    let result = input.and_then(parse_and_double);
    println!("parse_and_double({:?}) = {:?}", input, result);

    let bad_input = Some("not a number");
    let result = bad_input.and_then(parse_and_double);
    println!("parse_and_double({:?}) = {:?}", bad_input, result);

    println!("\n=== or() and or_else() ===");
    let first: Option<i32> = None;
    let second = Some(42);

    println!("{:?}.or({:?}) = {:?}", first, second, first.or(second));
    println!("{:?}.or({:?}) = {:?}", second, first, second.or(first));

    println!("\n=== Option with ? Operator ===");
    fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        Some(a? + b?)  // Returns None if either is None
    }

    println!("add(Some(1), Some(2)) = {:?}", add_options(Some(1), Some(2)));
    println!("add(Some(1), None) = {:?}", add_options(Some(1), None));
}
