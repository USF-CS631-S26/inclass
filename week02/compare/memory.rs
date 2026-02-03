/*
 * memory.rs - Demonstrating how Rust prevents C's memory pitfalls
 *
 * Key concepts:
 * - Ownership: Each value has exactly one owner
 * - Borrowing: References let you access data without taking ownership
 * - Lifetimes: The compiler tracks how long references are valid
 * - Drop: Automatic deallocation when owner goes out of scope
 */

/* ============================================================
 * SECTION 1: Stack Allocation (similar to C)
 * ============================================================
 * Stack allocation works similarly, but Rust tracks ownership.
 */

fn stack_example() {
    // Stack-allocated array - fixed size
    let greeting: [u8; 32] = *b"Hello, World!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

    // More commonly, we use &str for string literals (stored in binary)
    let message: &str = "Hello, World!";

    println!("Stack/static string: {}", message);
    println!("  Length: {} bytes", message.len());

    // greeting goes out of scope here - automatically cleaned up
    // No manual free needed, no way to forget!
    let _ = greeting; // silence unused warning
}

/* ============================================================
 * SECTION 2: Rust PREVENTS Dangling Pointers
 * ============================================================
 * This code would NOT COMPILE in Rust - the compiler catches the bug!
 */

// UNCOMMENT TO SEE COMPILER ERROR:
// fn dangerous_stack_return() -> &str {
//     let local_string = String::from("I live on the stack");
//
//     // ERROR: cannot return reference to local variable
//     // The compiler knows local_string will be dropped!
//     &local_string
// }

// The SAFE way: return an owned value (moves ownership to caller)
fn safe_return() -> String {
    let local_string = String::from("I live on the heap");

    // Ownership is MOVED to the caller
    // local_string is not dropped here - caller now owns it
    local_string
}

fn demonstrate_no_dangling_pointer() {
    println!("=== No Dangling Pointers in Rust ===");

    // We receive ownership of the String
    let my_string = safe_return();

    println!("Received owned string: {}", my_string);
    println!("This is always safe - no undefined behavior possible!\n");

    // my_string is automatically dropped (freed) here
}

/* ============================================================
 * SECTION 3: Heap Allocation with String
 * ============================================================
 * String is Rust's heap-allocated, growable string type.
 * Ownership is explicit and enforced by the compiler.
 */

fn heap_example() {
    println!("=== Heap Allocation Example ===");

    // String allocates on the heap
    let mut my_string = String::from("Hello from the heap!");

    println!("Heap string: {}", my_string);
    println!("  Length:   {} bytes", my_string.len());
    println!("  Capacity: {} bytes\n", my_string.capacity());

    // We can grow it (like realloc, but automatic)
    my_string.push_str(" Plus more text!");
    println!("After push: {}", my_string);
    println!("  New capacity: {} bytes\n", my_string.capacity());

    // NO MANUAL FREE NEEDED!
    // my_string is automatically freed when it goes out of scope
    // This is called "Drop" in Rust
}

/* ============================================================
 * SECTION 4: Rust PREVENTS Common Memory Bugs
 * ============================================================
 */

fn no_memory_leaks() {
    println!("=== No Memory Leaks (normally) ===");

    let data = String::from("I will be automatically freed!");

    println!("Allocated: {}", data);

    // When data goes out of scope, it's automatically dropped
    // No way to "forget" to free it!
    // (Note: Rust does have mem::forget for special cases, but you have to explicitly use it)

    println!("(data will be freed automatically at end of scope)\n");
}

fn no_double_free() {
    println!("=== No Double Free ===");

    let data = String::from("Free me once");

    // This MOVES ownership - data can no longer be used
    let _moved = data;

    // UNCOMMENT TO SEE COMPILER ERROR:
    // let _double = data;  // ERROR: use of moved value

    println!("Ownership moved - original variable is now invalid");
    println!("Compiler prevents any possibility of double free!\n");

    // Only _moved will be dropped (once!)
}

fn no_use_after_free() {
    println!("=== No Use After Free ===");

    let data = String::from("Valid data");
    println!("Before drop: {}", data);

    // Explicitly drop (free) the data
    drop(data);

    // UNCOMMENT TO SEE COMPILER ERROR:
    // println!("After drop: {}", data);  // ERROR: use of moved value

    println!("(accessing data after drop is a compile-time error!)\n");
}

/* ============================================================
 * SECTION 5: Borrowing - Safe References
 * ============================================================
 * Borrowing lets you access data without taking ownership.
 * The compiler ensures references are always valid.
 */

// Borrows the string (immutable reference) - cannot modify
fn print_string(s: &str) {
    println!("Borrowed string: {}", s);
    // We don't own s - it will NOT be dropped when this function returns
}

// Takes OWNERSHIP - caller loses access
fn take_ownership(s: String) {
    println!("Taking ownership of: {}", s);
    // s is dropped at end of function - memory freed
}

// Borrows mutably - can modify, but doesn't own
fn modify_string(s: &mut String) {
    s.push_str(" [modified]");
    // We borrowed mutably, but don't own - not dropped here
}

fn clear_ownership_demo() {
    println!("=== Clear Ownership Demo ===");

    let mut heap_str = String::from("Heap string");

    // Borrowing is explicit with &
    print_string(&heap_str);

    // We still own heap_str, can use it again
    println!("Still valid: {}", heap_str);

    // Mutable borrow
    modify_string(&mut heap_str);
    println!("After modification: {}", heap_str);

    // This MOVES ownership - heap_str is now invalid
    take_ownership(heap_str);

    // UNCOMMENT TO SEE COMPILER ERROR:
    // println!("{}", heap_str);  // ERROR: use of moved value

    println!("(heap_str is now invalid - compiler enforces this!)\n");
}

/* ============================================================
 * SECTION 6: Lifetimes - Compiler Tracks Reference Validity
 * ============================================================
 */

// The lifetime 'a says: the returned reference lives as long as the input
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn lifetime_demo() {
    println!("=== Lifetime Demo ===");

    let string1 = String::from("hello");

    {
        let string2 = String::from("world!");
        let result = longest(&string1, &string2);
        println!("Longest: {}", result);
        // result is valid here because both string1 and string2 are valid
    }

    // UNCOMMENT TO SEE COMPILER ERROR:
    // This would be an error because string2 is dropped:
    // let string2 = String::from("world!");
    // let result;
    // {
    //     let string2 = String::from("world!");
    //     result = longest(&string1, &string2);
    // }  // string2 dropped here
    // println!("{}", result);  // ERROR: string2 does not live long enough

    println!("Compiler ensures references never outlive their data!\n");
}

/* ============================================================
 * SECTION 7: Comparison with C
 * ============================================================
 */

fn comparison_with_c() {
    println!("=== Rust vs C Memory Safety ===\n");

    println!("| C Problem              | Rust Solution                    |");
    println!("|------------------------|----------------------------------|");
    println!("| Dangling pointers      | Lifetime checking at compile time|");
    println!("| Memory leaks           | Automatic Drop when out of scope |");
    println!("| Double free            | Ownership - only one owner       |");
    println!("| Use after free         | Move semantics invalidate source |");
    println!("| Ownership confusion    | Explicit ownership in type system|");
    println!("| Buffer overflow        | Bounds checking (can be bypassed)|");
    println!();

    println!("Key insight: Rust's ownership rules are enforced at COMPILE TIME.");
    println!("There is NO runtime overhead for memory safety checks.");
    println!("You get C-like performance with memory safety guarantees.\n");
}

/* ============================================================
 * MAIN
 * ============================================================
 */

fn main() {
    println!("========================================");
    println!("Rust Memory Safety Demonstration");
    println!("========================================\n");

    println!("=== Stack Allocation ===");
    stack_example();
    println!();

    demonstrate_no_dangling_pointer();

    heap_example();

    no_memory_leaks();

    no_double_free();

    no_use_after_free();

    clear_ownership_demo();

    lifetime_demo();

    comparison_with_c();

    println!("========================================");
    println!("Rust Memory Safety Summary:");
    println!("========================================");
    println!("1. Ownership: Each value has exactly one owner");
    println!("2. Borrowing: & for read-only, &mut for read-write");
    println!("3. Lifetimes: Compiler tracks reference validity");
    println!("4. Drop: Automatic cleanup when owner goes out of scope");
    println!("5. No null: Option<T> makes absence explicit");
    println!();
    println!("All enforced at compile time = zero runtime cost!");
}
