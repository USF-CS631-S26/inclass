//! 05_match.rs - Match expressions
//!
//! Match is Rust's powerful pattern matching construct.
//! It must be exhaustive - all cases must be handled.

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i32, y: i32 },
    Paste(String),
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn describe_event(event: &WebEvent) -> String {
    match event {
        WebEvent::PageLoad => "Page loaded".to_string(),
        WebEvent::KeyPress(c) => format!("Key pressed: '{}'", c),
        WebEvent::Click { x, y } => format!("Clicked at ({}, {})", x, y),
        WebEvent::Paste(text) => format!("Pasted: {}", text),
    }
}

fn main() {
    println!("=== Basic Match ===");
    let coins = [Coin::Penny, Coin::Dime, Coin::Quarter, Coin::Nickel];
    for coin in &coins {
        println!("{:?} = {} cents", coin, coin_value(coin));
    }

    println!("\n=== Match with Destructuring ===");
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('a'),
        WebEvent::Click { x: 100, y: 200 },
        WebEvent::Paste("Hello".to_string()),
    ];

    for event in &events {
        println!("{:?} -> {}", event, describe_event(event));
    }

    println!("\n=== Match on Integers ===");
    for n in 1..=6 {
        let description = match n {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "many",  // _ is catch-all pattern
        };
        println!("{} is {}", n, description);
    }

    println!("\n=== Match with Ranges ===");
    for age in [5, 13, 18, 25, 65, 80] {
        let category = match age {
            0..=12 => "child",
            13..=19 => "teenager",
            20..=64 => "adult",
            _ => "senior",  // 65 and above (or negative - catch all)
        };
        println!("Age {} is a {}", age, category);
    }

    println!("\n=== Match with Guards ===");
    for n in [-5, 0, 3, 7, 12] {
        let description = match n {
            x if x < 0 => "negative",
            0 => "zero",
            x if x % 2 == 0 => "positive even",
            _ => "positive odd",
        };
        println!("{} is {}", n, description);
    }

    println!("\n=== Match with Multiple Patterns ===");
    for c in ['a', 'e', 'x', '5', '@'] {
        let category = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
            'a'..='z' => "consonant",
            '0'..='9' => "digit",
            _ => "other",
        };
        println!("'{}' is a {}", c, category);
    }

    println!("\n=== Match with Binding ===");
    let point = (3, 7);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at x={}", x),
        (0, y) => println!("On y-axis at y={}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    println!("\n=== Match as Expression ===");
    let number = 42;
    let is_answer = match number {
        42 => true,
        _ => false,
    };
    println!("{} is the answer: {}", number, is_answer);

    println!("\n=== Match on References ===");
    let values = vec![1, 2, 3];
    for value in &values {
        // value is &i32, match on the reference
        match value {
            &1 => println!("Got one"),
            &2 => println!("Got two"),
            n => println!("Got {}", n),  // n is &i32
        }
    }

    println!("\n=== Nested Match ===");
    let opt_num: Option<i32> = Some(42);

    match opt_num {
        Some(n) => match n {
            0 => println!("Zero"),
            1..=10 => println!("Small: {}", n),
            _ => println!("Large: {}", n),
        },
        None => println!("No value"),
    }
}
