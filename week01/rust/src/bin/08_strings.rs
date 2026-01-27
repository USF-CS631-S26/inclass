//! 08_strings.rs - String vs &str, String methods
//!
//! Rust has two main string types:
//! - String: owned, growable, heap-allocated
//! - &str: borrowed string slice, view into a string

fn main() {
    println!("=== String vs &str ===");

    // &str - string slice, usually from literals
    let slice: &str = "hello";  // Stored in binary, immutable

    // String - owned, heap-allocated, mutable
    let mut owned: String = String::from("hello");

    println!("slice: {}", slice);
    println!("owned: {}", owned);

    // String can be modified
    owned.push_str(" world");
    println!("owned after push_str: {}", owned);

    // &str cannot be modified
    // slice.push_str("!"); // Error!

    println!("\n=== Creating Strings ===");
    let s1 = String::new();              // Empty string
    let s2 = String::from("hello");      // From &str
    let s3 = "world".to_string();        // Using to_string()
    let s4 = format!("{} {}", s2, s3);   // Using format!

    println!("s1 (empty): '{}'", s1);
    println!("s2: '{}'", s2);
    println!("s3: '{}'", s3);
    println!("s4: '{}'", s4);

    println!("\n=== String Methods ===");
    let mut s = String::from("Hello");

    // Appending
    s.push(' ');           // Push single char
    s.push_str("World");   // Push string slice
    println!("After push: {}", s);

    // Length
    println!("Length (bytes): {}", s.len());
    println!("Is empty: {}", s.is_empty());

    // Capacity (allocated memory)
    println!("Capacity: {}", s.capacity());

    println!("\n=== String Slicing ===");
    let hello = String::from("Hello, World!");

    // Get a slice (must be valid UTF-8 boundaries)
    let world = &hello[7..12];  // "World"
    println!("Slice [7..12]: {}", world);

    // Be careful: this panics on invalid boundaries
    // let invalid = &hello[0..1]; // OK for ASCII

    println!("\n=== Iterating Characters ===");
    let s = "Hello";

    // By characters
    print!("Chars: ");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    // By bytes
    print!("Bytes: ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    // With indices
    print!("Char indices: ");
    for (i, c) in s.char_indices() {
        print!("({}:{}) ", i, c);
    }
    println!();

    println!("\n=== Useful String Methods ===");
    let s = "  Hello World  ";

    println!("Original: '{}'", s);
    println!("trim(): '{}'", s.trim());
    println!("to_uppercase(): '{}'", s.to_uppercase());
    println!("to_lowercase(): '{}'", s.to_lowercase());

    let s = "hello world";
    println!("\ncontains('world'): {}", s.contains("world"));
    println!("starts_with('hello'): {}", s.starts_with("hello"));
    println!("ends_with('world'): {}", s.ends_with("world"));
    println!("replace('world', 'rust'): {}", s.replace("world", "rust"));

    println!("\n=== Splitting Strings ===");
    let s = "one,two,three";

    print!("split(','): ");
    for part in s.split(',') {
        print!("'{}' ", part);
    }
    println!();

    let words = "hello world rust";
    print!("split_whitespace(): ");
    for word in words.split_whitespace() {
        print!("'{}' ", word);
    }
    println!();

    println!("\n=== Collecting into String ===");
    let words = vec!["hello", "world"];
    let joined = words.join(" ");
    println!("join: {}", joined);

    // Collect chars into String
    let chars: String = "hello".chars().filter(|c| *c != 'l').collect();
    println!("Filtered 'l': {}", chars);

    println!("\n=== String Conversion ===");
    // Parse string to number
    let num: i32 = "42".parse().unwrap();
    println!("Parsed '42' to i32: {}", num);

    // Number to string
    let s = 42.to_string();
    println!("42 to string: {}", s);

    println!("\n=== Ownership and Borrowing ===");
    fn takes_ownership(s: String) {
        println!("  Took ownership of: {}", s);
    }

    fn borrows(s: &str) {
        println!("  Borrowed: {}", s);
    }

    let owned = String::from("hello");

    // Borrowing doesn't move
    borrows(&owned);
    borrows(&owned);  // Can borrow multiple times

    // This moves the String
    takes_ownership(owned);
    // println!("{}", owned);  // Error: owned was moved
}
