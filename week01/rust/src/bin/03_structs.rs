//! 03_structs.rs - Struct definition with impl block
//!
//! Structs group related data together. Impl blocks add
//! methods and associated functions to structs.

/// A simple 2D point
#[derive(Debug, Clone, Copy)]  // Derive useful traits
struct Point {
    x: f64,
    y: f64,
}

/// Implementation block for Point
impl Point {
    /// Associated function (like a static method) - no self
    /// Called with Point::new(x, y)
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// Another associated function - creates origin point
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// Method taking &self (borrows, read-only)
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Method taking &mut self (borrows, can modify)
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

/// A rectangle defined by two corners
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Rectangle {
        Rectangle {
            top_left: Point::new(x1, y1),
            bottom_right: Point::new(x2, y2),
        }
    }

    fn width(&self) -> f64 {
        (self.bottom_right.x - self.top_left.x).abs()
    }

    fn height(&self) -> f64 {
        (self.bottom_right.y - self.top_left.y).abs()
    }

    fn area(&self) -> f64 {
        self.width() * self.height()
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }
}

/// Struct with String field (owned data)
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,  // Field init shorthand when variable name matches
        }
    }

    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }

    fn have_birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old!", self.name, self.age);
    }
}

fn main() {
    println!("=== Creating Structs ===");

    // Using associated function (constructor)
    let p1 = Point::new(3.0, 4.0);
    let origin = Point::origin();

    // Direct initialization
    let p2 = Point { x: 1.0, y: 2.0 };

    println!("p1 = {:?}", p1);
    println!("origin = {:?}", origin);
    println!("p2 = {:?}", p2);

    println!("\n=== Calling Methods ===");
    println!("p1 distance from origin: {}", p1.distance_from_origin());

    // Mutable method requires mutable variable
    let mut p3 = Point::new(0.0, 0.0);
    println!("p3 before translate: {:?}", p3);
    p3.translate(5.0, 3.0);
    println!("p3 after translate(5, 3): {:?}", p3);

    println!("\n=== Nested Structs ===");
    let rect = Rectangle::new(0.0, 0.0, 10.0, 5.0);
    println!("Rectangle: {:?}", rect);
    println!("Width: {}, Height: {}", rect.width(), rect.height());
    println!("Area: {}", rect.area());

    let test_point = Point::new(5.0, 2.5);
    let outside_point = Point::new(15.0, 2.5);
    println!("{:?} inside rect: {}", test_point, rect.contains(&test_point));
    println!("{:?} inside rect: {}", outside_point, rect.contains(&outside_point));

    println!("\n=== Struct with String Field ===");
    let mut alice = Person::new("Alice", 30);
    println!("{:?}", alice);
    println!("{}", alice.greet());
    alice.have_birthday();

    println!("\n=== Struct Update Syntax ===");
    let p4 = Point { x: 100.0, ..p1 };  // Copy y from p1
    println!("p4 (x=100, y from p1): {:?}", p4);
}
