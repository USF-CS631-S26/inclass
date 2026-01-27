//! 11_borrowing.rs - References, & operator, ownership
//!
//! Rust's ownership system ensures memory safety without garbage collection.
//! Key rules:
//! 1. Each value has exactly one owner
//! 2. When owner goes out of scope, value is dropped
//! 3. You can have either one &mut reference OR any number of & references

fn main() {
    println!("=== Ownership Basics ===");

    // s1 owns the String
    let s1 = String::from("hello");

    // Ownership moves to s2; s1 is no longer valid
    let s2 = s1;

    // println!("{}", s1);  // Error: s1 was moved
    println!("s2 = {}", s2);

    // Clone creates a deep copy (both own their data)
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    println!("\n=== Immutable References (&) ===");
    let s = String::from("hello");

    // Create references (borrows) - doesn't move ownership
    let r1 = &s;
    let r2 = &s;

    // Can have multiple immutable references
    println!("r1 = {}, r2 = {}", r1, r2);
    println!("s still valid: {}", s);

    println!("\n=== Mutable References (&mut) ===");
    let mut s = String::from("hello");

    // Only one mutable reference at a time
    let r = &mut s;
    r.push_str(" world");
    println!("Through mutable ref: {}", r);

    // r is no longer used, so we can use s again
    println!("s = {}", s);

    println!("\n=== References in Functions ===");

    // Function that borrows (doesn't take ownership)
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s = String::from("hello");
    let len = calculate_length(&s);  // Pass reference
    println!("Length of '{}' is {}", s, len);  // s still valid

    // Function that mutably borrows
    fn append_exclamation(s: &mut String) {
        s.push('!');
    }

    let mut s = String::from("hello");
    append_exclamation(&mut s);
    println!("After append: {}", s);

    println!("\n=== References Must Be Valid ===");
    // References cannot outlive the data they reference

    // This would be an error:
    // let r;
    // {
    //     let s = String::from("hello");
    //     r = &s;  // Error: s doesn't live long enough
    // }
    // println!("{}", r);

    // Correct version:
    let s = String::from("hello");
    let r = &s;
    println!("Reference is valid: {}", r);

    println!("\n=== Slices Are References ===");
    let s = String::from("hello world");

    // String slice is a reference to part of a String
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("hello = '{}', world = '{}'", hello, world);

    // String literal is a slice
    let literal: &str = "I'm a string literal";
    println!("{}", literal);

    println!("\n=== Dereferencing ===");
    let x = 5;
    let r = &x;

    // Use * to dereference
    println!("r = {}", r);    // Auto-deref for printing
    println!("*r = {}", *r);  // Explicit dereference

    // Comparison auto-derefs
    assert!(*r == 5);
    assert!(r == &5);  // Also works

    let mut y = 10;
    let r = &mut y;
    *r += 1;  // Modify through reference
    println!("y after *r += 1: {}", y);

    println!("\n=== Method Auto-Borrowing ===");
    let s = String::from("hello");

    // These are equivalent:
    println!("len = {}", s.len());       // Auto-borrows
    println!("len = {}", (&s).len());    // Explicit borrow
    println!("len = {}", String::len(&s)); // Function syntax

    println!("\n=== Rules Summary ===");
    println!("1. At any time, you can have EITHER:");
    println!("   - One mutable reference (&mut T)");
    println!("   - Any number of immutable references (&T)");
    println!("2. References must always be valid");
    println!("3. The borrow checker enforces these at compile time");

    println!("\n=== Common Pattern: Return Reference ===");
    fn first_word(s: &str) -> &str {
        for (i, c) in s.char_indices() {
            if c == ' ' {
                return &s[0..i];
            }
        }
        s
    }

    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: '{}'", word);
}
