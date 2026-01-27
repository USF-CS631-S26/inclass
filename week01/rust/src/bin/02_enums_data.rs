//! 02_enums_data.rs - Enum variants holding values
//!
//! Rust enums can hold data, making them powerful "tagged unions"
//! or "sum types". Each variant can hold different types of data.

#[derive(Debug)]
enum Message {
    // Unit variant (no data)
    Quit,

    // Named fields (like a struct)
    Move { x: i32, y: i32 },

    // Single value
    Write(String),

    // Multiple values (tuple-like)
    ChangeColor(u8, u8, u8),
}

/// Token with associated data
#[derive(Debug)]
enum Token {
    Eof,
    Plus,
    Minus,
    Number(i64),           // Holds the numeric value
    Identifier(String),    // Holds the identifier name
    StringLit(String),     // Holds the string content
}

/// Result-like enum for demonstration
#[derive(Debug)]
enum ParseResult {
    Success(i32),
    Error(String),
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => {
            println!("Received Quit message");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

fn describe_token(token: &Token) -> String {
    match token {
        Token::Eof => "end of file".to_string(),
        Token::Plus => "plus operator".to_string(),
        Token::Minus => "minus operator".to_string(),
        Token::Number(n) => format!("number: {}", n),
        Token::Identifier(name) => format!("identifier: {}", name),
        Token::StringLit(s) => format!("string: \"{}\"", s),
    }
}

fn parse_number(s: &str) -> ParseResult {
    match s.parse::<i32>() {
        Ok(n) => ParseResult::Success(n),
        Err(_) => ParseResult::Error(format!("'{}' is not a valid number", s)),
    }
}

fn main() {
    println!("=== Enums with Data ===");

    // Create enum variants with data
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello!".to_string()),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in &messages {
        process_message(msg);
    }

    println!("\n=== Token Enum ===");
    let tokens = vec![
        Token::Number(42),
        Token::Plus,
        Token::Identifier("foo".to_string()),
        Token::Eof,
    ];

    for token in &tokens {
        println!("{:?} -> {}", token, describe_token(token));
    }

    println!("\n=== Extracting Data with Match ===");
    let token = Token::Number(123);

    // Extract the value using match
    match &token {
        Token::Number(value) => println!("Got number: {}", value),
        _ => println!("Not a number"),
    }

    // Using if let for single variant
    if let Token::Number(n) = &token {
        println!("The number is: {}", n);
    }

    println!("\n=== Result-like Pattern ===");
    let inputs = ["42", "hello", "-17", "3.14"];

    for input in inputs {
        match parse_number(input) {
            ParseResult::Success(n) => println!("Parsed '{}' -> {}", input, n),
            ParseResult::Error(e) => println!("Error: {}", e),
        }
    }

    println!("\n=== Nested Destructuring ===");
    let msg = Message::Move { x: 5, y: 10 };

    // Destructure and bind with conditions
    match msg {
        Message::Move { x, y } if x == y => println!("Diagonal move"),
        Message::Move { x: 0, y } => println!("Vertical move to y={}", y),
        Message::Move { x, y: 0 } => println!("Horizontal move to x={}", x),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        _ => {}
    }
}
