//! 13_loop_matches.rs - loop construct, matches! macro
//!
//! The `loop` keyword creates an infinite loop that can return values.
//! The `matches!` macro is a convenient way to check patterns.

#[derive(Debug, PartialEq)]
enum State {
    Ready,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Eof,
}

fn main() {
    println!("=== Basic loop ===");

    let mut count = 0;
    loop {
        count += 1;
        println!("count = {}", count);

        if count >= 3 {
            break;  // Exit the loop
        }
    }

    println!("\n=== loop with return value ===");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };
    println!("Loop returned: {}", result);

    println!("\n=== loop with continue ===");

    let mut n = 0;
    loop {
        n += 1;

        if n > 5 {
            break;
        }

        if n % 2 == 0 {
            continue;  // Skip even numbers
        }

        println!("Odd: {}", n);
    }

    println!("\n=== Labeled loops ===");

    // Labels allow breaking from outer loops
    let mut found = false;
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i * j == 6 {
                println!("Found: {} * {} = 6", i, j);
                found = true;
                break 'outer;  // Break from outer loop
            }
        }
    }
    println!("Search complete, found = {}", found);

    println!("\n=== loop vs while true ===");

    // Both are infinite loops, but `loop` is preferred:
    // 1. Clearer intent
    // 2. Compiler knows it's infinite (better optimization)
    // 3. Can return values

    // while true { ... }  // Works but less idiomatic
    // loop { ... }        // Preferred

    println!("\n=== matches! macro basics ===");

    let state = State::Running;

    // Without matches! - verbose
    let is_running = match state {
        State::Running => true,
        _ => false,
    };
    println!("is_running (match): {}", is_running);

    // With matches! - concise
    let is_running = matches!(state, State::Running);
    println!("is_running (matches!): {}", is_running);

    println!("\n=== matches! with multiple patterns ===");

    let states = [State::Ready, State::Running, State::Paused, State::Stopped];

    for state in &states {
        // Check if state is "active" (Ready or Running)
        let is_active = matches!(state, State::Ready | State::Running);
        println!("{:?} is active: {}", state, is_active);
    }

    println!("\n=== matches! with guards ===");

    let numbers = [0, 5, 10, 15, 20];

    for n in numbers {
        // Check if number is between 5 and 15 (inclusive)
        let in_range = matches!(n, x if x >= 5 && x <= 15);
        println!("{} in range [5,15]: {}", n, in_range);
    }

    println!("\n=== matches! with enum data ===");

    let tokens = vec![
        Token::Number(42),
        Token::Plus,
        Token::Number(-5),
        Token::Eof,
    ];

    for token in &tokens {
        // Check if token is a positive number
        let is_positive = matches!(token, Token::Number(n) if *n > 0);
        println!("{:?} is positive number: {}", token, is_positive);
    }

    println!("\n=== Using matches! in filter ===");

    let numbers: Vec<Option<i32>> = vec![Some(1), None, Some(2), None, Some(3)];

    // Filter to keep only Some values
    let some_count = numbers.iter()
        .filter(|x| matches!(x, Some(_)))
        .count();
    println!("Count of Some values: {}", some_count);

    println!("\n=== loop as state machine ===");

    let mut state = State::Ready;
    let mut iterations = 0;

    loop {
        iterations += 1;
        println!("State: {:?}", state);

        state = match state {
            State::Ready => State::Running,
            State::Running if iterations >= 3 => State::Paused,
            State::Running => State::Running,
            State::Paused => State::Stopped,
            State::Stopped => break,
        };
    }
    println!("State machine completed after {} iterations", iterations);

    println!("\n=== Combining loop and matches! ===");

    let input = vec![Token::Number(1), Token::Plus, Token::Number(2), Token::Eof];
    let mut iter = input.iter();

    loop {
        match iter.next() {
            Some(token) if matches!(token, Token::Eof) => {
                println!("Reached end of input");
                break;
            }
            Some(token) => {
                println!("Processing: {:?}", token);
            }
            None => {
                println!("Iterator exhausted unexpectedly");
                break;
            }
        }
    }
}
