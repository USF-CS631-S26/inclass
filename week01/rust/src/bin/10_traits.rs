//! 10_traits.rs - Display, Debug, Clone, and custom traits
//!
//! Traits define shared behavior. They're similar to interfaces
//! in other languages. #[derive] auto-implements common traits.

use std::fmt;

/// A point with derived traits
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// A token that implements Display manually
#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Identifier(String),
}

// Implement Display trait for Token
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "NUMBER({})", n),
            Token::Plus => write!(f, "PLUS"),
            Token::Minus => write!(f, "MINUS"),
            Token::Identifier(s) => write!(f, "IDENT({})", s),
        }
    }
}

// Implement Display for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A custom trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default implementation
    fn describe(&self) -> String {
        format!("Area: {:.2}, Perimeter: {:.2}", self.area(), self.perimeter())
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    // Override default implementation
    fn describe(&self) -> String {
        format!("Circle with radius {:.2}: {}", self.radius,
                format!("Area: {:.2}, Perimeter: {:.2}", self.area(), self.perimeter()))
    }
}

// Function that takes any type implementing Shape
fn print_shape_info(shape: &dyn Shape) {
    println!("  {}", shape.describe());
}

// Generic function with trait bound
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("=== Debug Trait ===");
    let p = Point { x: 3.0, y: 4.0 };

    // {:?} uses Debug
    println!("Debug: {:?}", p);

    // {:#?} uses Debug with pretty-printing
    println!("Pretty Debug: {:#?}", p);

    println!("\n=== Display Trait ===");
    // {} uses Display
    println!("Display: {}", p);

    let token = Token::Number(42);
    println!("Debug: {:?}", token);
    println!("Display: {}", token);

    println!("\n=== Clone Trait ===");
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();  // Explicit clone
    let p3 = p1;          // Copy (because Point derives Copy)

    println!("p1 = {:?}", p1);  // Still valid because Point is Copy
    println!("p2 = {:?}", p2);
    println!("p3 = {:?}", p3);

    // Token doesn't derive Copy, so clone moves
    let t1 = Token::Plus;
    let t2 = t1.clone();
    println!("t1 = {:?}, t2 = {:?}", t1, t2);

    println!("\n=== PartialEq Trait ===");
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = Point { x: 3.0, y: 4.0 };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    println!("p1 != p3: {}", p1 != p3);

    println!("\n=== Custom Trait ===");
    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };

    println!("Rectangle: {:?}", rect);
    println!("  Area: {}", rect.area());
    println!("  Perimeter: {}", rect.perimeter());

    println!("Circle: {:?}", circle);
    println!("  Area: {:.2}", circle.area());
    println!("  Perimeter: {:.2}", circle.perimeter());

    println!("\n=== Trait Objects (dyn) ===");
    let shapes: Vec<&dyn Shape> = vec![&rect, &circle];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}:", i + 1);
        print_shape_info(*shape);
    }

    println!("\n=== Generic with Trait Bound ===");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    println!("\n=== Common Derivable Traits ===");
    println!("Debug   - Enable {{:?}} formatting");
    println!("Clone   - Enable .clone() method");
    println!("Copy    - Enable implicit copies (for small types)");
    println!("PartialEq - Enable == and != comparison");
    println!("Eq      - Marker trait for full equality");
    println!("PartialOrd - Enable <, >, <=, >= comparison");
    println!("Ord     - Enable total ordering");
    println!("Hash    - Enable use as HashMap key");
    println!("Default - Enable Default::default()");
}
