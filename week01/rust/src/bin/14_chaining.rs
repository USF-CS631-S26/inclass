//! 14_chaining.rs - Chaining iterator methods
//!
//! Rust iterators are lazy and composable. You can chain multiple
//! transformation methods together for expressive data processing.

fn main() {
    println!("=== Basic Iterator Chaining ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Chain: filter even -> double -> sum
    let result: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)  // Keep even numbers
        .map(|&x| x * 2)           // Double each
        .sum();                    // Sum them

    println!("Sum of doubled evens: {}", result);

    println!("\n=== Common Iterator Methods ===");

    let nums = vec![1, 2, 3, 4, 5];

    // map: transform each element
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map (double): {:?}", doubled);

    // filter: keep elements matching predicate
    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (evens): {:?}", evens);

    // take: first n elements
    let first_three: Vec<&i32> = nums.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // skip: skip first n elements
    let after_two: Vec<&i32> = nums.iter().skip(2).collect();
    println!("skip(2): {:?}", after_two);

    // enumerate: add indices
    let indexed: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    println!("enumerate: {:?}", indexed);

    println!("\n=== Chaining Multiple Operations ===");

    let words = vec!["hello", "world", "rust", "is", "awesome"];

    // Get lengths of words starting with vowels
    let vowel_word_lengths: Vec<usize> = words.iter()
        .filter(|w| {
            let first = w.chars().next().unwrap_or(' ');
            matches!(first, 'a' | 'e' | 'i' | 'o' | 'u')
        })
        .map(|w| w.len())
        .collect();

    println!("Lengths of vowel-starting words: {:?}", vowel_word_lengths);

    println!("\n=== filter_map: Filter and Transform ===");

    let strings = vec!["1", "two", "3", "four", "5"];

    // Parse numbers, ignoring failures
    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("Parsed numbers: {:?}", numbers);

    println!("\n=== flat_map: Flatten Nested Iterators ===");

    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];

    let flat: Vec<i32> = nested.iter()
        .flat_map(|v| v.iter().copied())
        .collect();

    println!("Flattened: {:?}", flat);

    println!("\n=== Reduction Operations ===");

    let nums = vec![1, 2, 3, 4, 5];

    // sum
    let total: i32 = nums.iter().sum();
    println!("sum: {}", total);

    // product
    let product: i32 = nums.iter().product();
    println!("product: {}", product);

    // fold: custom reduction
    let sum_of_squares = nums.iter()
        .fold(0, |acc, &x| acc + x * x);
    println!("sum of squares (fold): {}", sum_of_squares);

    // reduce: similar to fold but uses first element
    let max = nums.iter()
        .copied()
        .reduce(|a, b| if a > b { a } else { b });
    println!("max (reduce): {:?}", max);

    println!("\n=== Finding Elements ===");

    let nums = vec![1, 2, 3, 4, 5, 6];

    // find: first matching element
    let first_even = nums.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    // position: index of first match
    let pos = nums.iter().position(|&x| x > 3);
    println!("Position of first > 3: {:?}", pos);

    // any: check if any matches
    let has_five = nums.iter().any(|&x| x == 5);
    println!("Has 5: {}", has_five);

    // all: check if all match
    let all_positive = nums.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive);

    println!("\n=== Chaining with Options ===");

    fn process_input(input: Option<&str>) -> Option<i32> {
        input
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<i32>().ok())
            .map(|n| n * 2)
    }

    println!("process_input(Some(\"  42  \")): {:?}", process_input(Some("  42  ")));
    println!("process_input(Some(\"\")): {:?}", process_input(Some("")));
    println!("process_input(Some(\"abc\")): {:?}", process_input(Some("abc")));
    println!("process_input(None): {:?}", process_input(None));

    println!("\n=== zip: Combine Iterators ===");

    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];

    let results: Vec<(&str, i32)> = names.iter()
        .copied()
        .zip(scores.iter().copied())
        .collect();

    println!("Zipped: {:?}", results);

    // Process zipped data
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{}: {}", name, score);
    }

    println!("\n=== Practical Example: Scanner Tokens ===");

    #[derive(Debug)]
    enum Token {
        Number(i64),
        Plus,
        Minus,
        Whitespace,
        Unknown(char),
    }

    let input = "12 + 34 - 5";

    // Tokenize by character patterns
    let tokens: Vec<Token> = input.chars()
        .map(|c| match c {
            '0'..='9' => Token::Number((c as i64) - ('0' as i64)),
            '+' => Token::Plus,
            '-' => Token::Minus,
            ' ' => Token::Whitespace,
            _ => Token::Unknown(c),
        })
        .filter(|t| !matches!(t, Token::Whitespace))
        .collect();

    println!("Tokens: {:?}", tokens);

    println!("\n=== Chaining is Lazy ===");

    // Nothing happens until .collect() or similar
    let lazy_iter = (1..100)
        .filter(|x| {
            println!("  Filtering {}", x);  // Won't print yet
            x % 2 == 0
        })
        .take(3);

    println!("Created lazy iterator (nothing printed yet)");

    // Now it executes
    println!("Collecting:");
    let result: Vec<i32> = lazy_iter.collect();
    println!("Result: {:?}", result);
}
