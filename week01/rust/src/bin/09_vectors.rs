//! 09_vectors.rs - Vec, &[T], push, iteration
//!
//! Vec<T> is Rust's growable array type.
//! &[T] is a slice - a borrowed view into contiguous data.

fn main() {
    println!("=== Creating Vectors ===");

    // Empty vector with type annotation
    let v1: Vec<i32> = Vec::new();

    // Using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];

    // With capacity (optimization)
    let v3: Vec<i32> = Vec::with_capacity(10);

    println!("v1 (empty): {:?}, len={}, capacity={}", v1, v1.len(), v1.capacity());
    println!("v2: {:?}", v2);
    println!("v3 (with capacity): len={}, capacity={}", v3.len(), v3.capacity());

    println!("\n=== Modifying Vectors ===");
    let mut v = Vec::new();

    // push adds to end
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push: {:?}", v);

    // pop removes from end, returns Option
    let popped = v.pop();
    println!("Popped: {:?}, remaining: {:?}", popped, v);

    // insert at index
    v.insert(0, 0);
    println!("After insert(0, 0): {:?}", v);

    // remove at index
    let removed = v.remove(0);
    println!("Removed index 0: {}, remaining: {:?}", removed, v);

    println!("\n=== Accessing Elements ===");
    let v = vec![10, 20, 30, 40, 50];

    // Indexing (panics if out of bounds)
    println!("v[0] = {}", v[0]);
    println!("v[2] = {}", v[2]);

    // get() returns Option (safe)
    println!("v.get(2) = {:?}", v.get(2));
    println!("v.get(99) = {:?}", v.get(99));

    // First and last
    println!("first() = {:?}", v.first());
    println!("last() = {:?}", v.last());

    println!("\n=== Slices (&[T]) ===");
    let v = vec![1, 2, 3, 4, 5];

    // Borrow entire vector as slice
    let slice: &[i32] = &v;
    println!("Full slice: {:?}", slice);

    // Partial slices
    let first_two = &v[0..2];
    let last_two = &v[3..];
    let middle = &v[1..4];

    println!("First two: {:?}", first_two);
    println!("Last two: {:?}", last_two);
    println!("Middle: {:?}", middle);

    println!("\n=== Iteration ===");
    let v = vec![1, 2, 3];

    // Immutable iteration
    print!("for &x in &v: ");
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // With index
    print!("enumerate: ");
    for (i, x) in v.iter().enumerate() {
        print!("({},{}) ", i, x);
    }
    println!();

    // Mutable iteration
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x *= 2;
    }
    println!("After doubling: {:?}", v);

    println!("\n=== Useful Methods ===");
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("len(): {}", v.len());
    println!("is_empty(): {}", v.is_empty());
    println!("contains(&5): {}", v.contains(&5));

    let mut v = v.clone();
    v.sort();
    println!("After sort(): {:?}", v);

    v.reverse();
    println!("After reverse(): {:?}", v);

    v.dedup();  // Remove consecutive duplicates (needs sorted first)
    v.sort();
    v.dedup();
    println!("After sort+dedup(): {:?}", v);

    println!("\n=== Functions with Slices ===");
    fn sum(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    fn print_slice(slice: &[i32]) {
        println!("Slice: {:?}", slice);
    }

    let v = vec![1, 2, 3, 4, 5];

    // Vec coerces to slice
    println!("Sum of vec: {}", sum(&v));
    print_slice(&v);

    // Works with arrays too
    let arr = [10, 20, 30];
    println!("Sum of array: {}", sum(&arr));

    println!("\n=== Vec of Strings ===");
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push("Bob".to_string());
    names.push(format!("Charlie"));

    println!("Names: {:?}", names);

    // Iterate with references
    for name in &names {
        println!("Hello, {}!", name);
    }

    println!("\n=== Converting ===");
    // Vec to slice (borrowing)
    let v = vec![1, 2, 3];
    let _s: &[i32] = &v;

    // Slice to Vec (cloning)
    let slice = &[4, 5, 6];
    let v: Vec<i32> = slice.to_vec();
    println!("slice.to_vec(): {:?}", v);

    // Array to Vec
    let arr = [7, 8, 9];
    let v: Vec<i32> = arr.to_vec();
    println!("array.to_vec(): {:?}", v);
}
