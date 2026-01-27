//! 01_enums.rs - Simple enum without data
//!
//! Enums define a type with a fixed set of possible values.
//! Each variant is a distinct value of the enum type.

/// A simple enum representing directions
#[derive(Debug)]  // Allows printing with {:?}
enum Direction {
    North,
    East,
    South,
    West,
}

/// Enum representing token types in a scanner
#[derive(Debug, PartialEq)]  // PartialEq allows == comparison
enum TokenType {
    Eof,
    Plus,
    Minus,
    Star,
    Slash,
    Number,
    Identifier,
}

/// Function that takes an enum and returns something
fn describe_direction(dir: &Direction) -> &'static str {
    match dir {
        Direction::North => "heading north",
        Direction::East => "heading east",
        Direction::South => "heading south",
        Direction::West => "heading west",
    }
}

/// Turn right from current direction
fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    println!("=== Basic Enum Usage ===");

    // Create enum values
    let heading = Direction::North;
    let token = TokenType::Plus;

    // Debug printing with {:?}
    println!("heading = {:?}", heading);
    println!("token = {:?}", token);

    // Using match to handle enum
    println!("\n=== Match on Enum ===");
    println!("{}", describe_direction(&heading));

    // Enum comparison (requires PartialEq)
    println!("\n=== Enum Comparison ===");
    let tok1 = TokenType::Number;
    let tok2 = TokenType::Number;
    let tok3 = TokenType::Plus;

    println!("tok1 == tok2: {}", tok1 == tok2);
    println!("tok1 == tok3: {}", tok1 == tok3);

    // Using enum in control flow
    println!("\n=== Enum in Control Flow ===");
    let current = Direction::East;

    // Using if let for single variant check
    if let Direction::East = current {
        println!("We're heading east!");
    }

    // Chain of turns
    println!("\n=== Chain of Turns ===");
    let mut dir = Direction::North;
    for i in 0..5 {
        println!("Step {}: {:?}", i, dir);
        dir = turn_right(dir);
    }

    // Match with multiple patterns
    println!("\n=== Match Multiple Patterns ===");
    let movement = Direction::South;
    match movement {
        Direction::North | Direction::South => println!("Moving vertically"),
        Direction::East | Direction::West => println!("Moving horizontally"),
    }
}
