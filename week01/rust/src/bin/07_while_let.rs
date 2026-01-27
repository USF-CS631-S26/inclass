//! 07_while_let.rs - Pattern matching in loops
//!
//! `while let` continues looping as long as the pattern matches.
//! It's commonly used with Option and iterators.

fn main() {
    println!("=== Basic while let with Option ===");
    let mut stack = vec![1, 2, 3, 4, 5];

    // pop() returns Option<T>
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    println!("Stack is empty: {:?}", stack);

    println!("\n=== while let with Iterator ===");
    let names = vec!["Alice", "Bob", "Charlie"];
    let mut iter = names.iter();

    // next() returns Option<&T>
    while let Some(name) = iter.next() {
        println!("Hello, {}!", name);
    }

    println!("\n=== Simulating a Scanner with while let ===");
    struct CharStream {
        chars: Vec<char>,
        pos: usize,
    }

    impl CharStream {
        fn new(s: &str) -> Self {
            CharStream {
                chars: s.chars().collect(),
                pos: 0,
            }
        }

        fn next(&mut self) -> Option<char> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Some(c)
            } else {
                None
            }
        }

        fn peek(&self) -> Option<char> {
            if self.pos < self.chars.len() {
                Some(self.chars[self.pos])
            } else {
                None
            }
        }
    }

    let mut stream = CharStream::new("hello");
    print!("Characters: ");
    while let Some(c) = stream.next() {
        print!("{} ", c);
    }
    println!();

    println!("\n=== while let vs loop + match ===");
    // These are equivalent:

    // Using while let (preferred)
    let mut nums = vec![3, 2, 1];
    println!("Using while let:");
    while let Some(n) = nums.pop() {
        println!("  {}", n);
    }

    // Using loop + match (more verbose)
    nums = vec![3, 2, 1];
    println!("Using loop + match:");
    loop {
        match nums.pop() {
            Some(n) => println!("  {}", n),
            None => break,
        }
    }

    println!("\n=== while let with Multiple Patterns ===");
    // Note: while let currently doesn't support | patterns directly,
    // but you can use nested conditions

    let mut values: Vec<Result<i32, &str>> = vec![
        Ok(1),
        Err("skip"),
        Ok(2),
        Ok(3),
    ];

    println!("Processing values:");
    while let Some(result) = values.pop() {
        match result {
            Ok(n) => println!("  Got number: {}", n),
            Err(e) => println!("  Skipping: {}", e),
        }
    }

    println!("\n=== while let for Processing Queue ===");
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    queue.push_back("task1");
    queue.push_back("task2");
    queue.push_back("task3");

    println!("Processing queue:");
    while let Some(task) = queue.pop_front() {
        println!("  Processing: {}", task);
    }

    println!("\n=== Nested while let ===");
    let mut outer = vec![vec![1, 2], vec![3, 4, 5], vec![6]];

    println!("Flattening nested vectors:");
    while let Some(mut inner) = outer.pop() {
        while let Some(value) = inner.pop() {
            print!("{} ", value);
        }
    }
    println!();

    println!("\n=== while let with peek() ===");
    let mut stream = CharStream::new("a1b2");

    println!("Scanning alphanumeric:");
    while let Some(c) = stream.peek() {
        if c.is_alphanumeric() {
            stream.next();  // Consume it
            println!("  Found: {}", c);
        } else {
            break;
        }
    }
}
