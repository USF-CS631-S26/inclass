//! 04_methods.rs - &self, &mut self, self
//!
//! Methods in Rust have different ways to receive self:
//! - &self      : borrows immutably (read-only access)
//! - &mut self  : borrows mutably (can modify)
//! - self       : takes ownership (consumes the value)

#[derive(Debug, Clone)]
struct Counter {
    value: i32,
    name: String,
}

impl Counter {
    /// Associated function (no self) - constructor
    fn new(name: &str) -> Counter {
        Counter {
            value: 0,
            name: name.to_string(),
        }
    }

    /// Takes &self - borrows immutably
    /// Can read fields but not modify them
    /// The original value remains usable after the call
    fn get_value(&self) -> i32 {
        self.value
    }

    /// Takes &self - another read-only method
    fn display(&self) {
        println!("{}: {}", self.name, self.value);
    }

    /// Takes &mut self - borrows mutably
    /// Can modify fields
    /// Requires the variable to be declared `mut`
    fn increment(&mut self) {
        self.value += 1;
    }

    /// Takes &mut self - can modify multiple times
    fn add(&mut self, amount: i32) {
        self.value += amount;
    }

    /// Takes &mut self - resets the counter
    fn reset(&mut self) {
        self.value = 0;
    }

    /// Takes self (by value) - consumes the counter
    /// After calling this, the original cannot be used
    fn into_value(self) -> i32 {
        println!("Consuming counter '{}'", self.name);
        self.value
    }

    /// Takes self - transforms into a new type
    fn into_string(self) -> String {
        format!("Counter '{}' had value {}", self.name, self.value)
    }
}

/// Demonstrates builder pattern using self
#[derive(Debug)]
struct RequestBuilder {
    url: String,
    method: String,
    timeout: u32,
}

impl RequestBuilder {
    fn new(url: &str) -> RequestBuilder {
        RequestBuilder {
            url: url.to_string(),
            method: "GET".to_string(),
            timeout: 30,
        }
    }

    /// Takes self, returns Self - enables chaining
    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn timeout(mut self, seconds: u32) -> Self {
        self.timeout = seconds;
        self
    }

    fn build(self) -> String {
        format!("{} {} (timeout: {}s)", self.method, self.url, self.timeout)
    }
}

fn main() {
    println!("=== &self Methods (Immutable Borrow) ===");
    let counter = Counter::new("visits");

    // Can call &self methods multiple times
    counter.display();
    println!("Value: {}", counter.get_value());
    counter.display();  // Still works, we only borrowed

    println!("\n=== &mut self Methods (Mutable Borrow) ===");
    let mut counter = Counter::new("clicks");  // Must be mut

    counter.display();
    counter.increment();
    counter.display();
    counter.add(5);
    counter.display();
    counter.reset();
    counter.display();

    println!("\n=== self Methods (Takes Ownership) ===");
    let counter = Counter::new("requests");
    let mut counter_clone = counter.clone();  // Clone before consuming

    counter_clone.add(42);

    // This consumes counter_clone - it can't be used after
    let final_value = counter_clone.into_value();
    println!("Final value: {}", final_value);

    // This would be an error:
    // counter_clone.display();  // Error: value moved

    // Original is still usable (we cloned it)
    println!("Original counter value: {}", counter.get_value());

    // Consume with into_string
    let summary = counter.into_string();
    println!("{}", summary);

    println!("\n=== Builder Pattern with self ===");
    // Chain methods that take and return self
    let request = RequestBuilder::new("https://api.example.com")
        .method("POST")
        .timeout(60)
        .build();

    println!("Request: {}", request);

    println!("\n=== Summary of Self Types ===");
    println!("&self     - Borrow immutably, can read");
    println!("&mut self - Borrow mutably, can modify");
    println!("self      - Take ownership, consume value");
}
