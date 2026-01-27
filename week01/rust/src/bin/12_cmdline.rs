//! 12_cmdline.rs - env::args(), collect()
//!
//! Command line arguments are accessed through std::env::args().
//! It returns an iterator that yields String values.

use std::env;

fn print_usage(program: &str) {
    println!("Usage: {} [options] <args...>", program);
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --verbose  Enable verbose output");
    println!("  -n <number>    Specify a number");
}

fn main() {
    println!("=== Basic Argument Access ===");

    // Collect args into a Vec<String>
    let args: Vec<String> = env::args().collect();

    println!("Number of arguments: {}", args.len());
    println!("\nAll arguments:");
    for (i, arg) in args.iter().enumerate() {
        println!("  args[{}] = \"{}\"", i, arg);
    }

    // First argument is always the program name
    let program_name = &args[0];
    println!("\nProgram name: {}", program_name);

    println!("\n=== Argument Processing ===");

    let mut verbose = false;
    let mut number: Option<i32> = None;
    let mut positional_args: Vec<&str> = Vec::new();

    // Skip program name (index 0)
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage(program_name);
                return;
            }
            "-v" | "--verbose" => {
                verbose = true;
                println!("Verbose mode enabled");
            }
            "-n" => {
                // Next argument should be the number
                i += 1;
                if i < args.len() {
                    match args[i].parse::<i32>() {
                        Ok(n) => {
                            number = Some(n);
                            println!("Number set to: {}", n);
                        }
                        Err(_) => {
                            eprintln!("Error: '{}' is not a valid number", args[i]);
                        }
                    }
                } else {
                    eprintln!("Error: -n requires an argument");
                }
            }
            arg if arg.starts_with('-') => {
                eprintln!("Unknown option: {}", arg);
            }
            arg => {
                positional_args.push(arg);
                println!("Positional argument: \"{}\"", arg);
            }
        }
        i += 1;
    }

    println!("\n=== Summary ===");
    println!("Verbose: {}", verbose);
    println!("Number: {:?}", number);
    println!("Positional arguments: {:?}", positional_args);

    // Using iterator methods
    println!("\n=== Iterator Methods ===");

    // Skip first, collect rest
    let args_without_program: Vec<&String> = args.iter().skip(1).collect();
    println!("Args without program: {:?}", args_without_program);

    // Check if any arg matches
    let has_help = args.iter().any(|arg| arg == "-h" || arg == "--help");
    println!("Has help flag: {}", has_help);

    // Find specific argument
    if let Some(n_arg) = args.iter().position(|arg| arg == "-n") {
        if n_arg + 1 < args.len() {
            println!("Value after -n: {}", args[n_arg + 1]);
        }
    }

    // Filter arguments
    let flags: Vec<&String> = args.iter()
        .filter(|arg| arg.starts_with('-'))
        .collect();
    println!("All flags: {:?}", flags);

    println!("\n=== Example Runs ===");
    if args.len() < 2 {
        println!("Try running with arguments:");
        println!("  cargo run --bin 12_cmdline hello world");
        println!("  cargo run --bin 12_cmdline -v -n 42 file.txt");
        println!("  cargo run --bin 12_cmdline --help");
    }

    println!("\n=== Environment Variables ===");
    // Also possible to read env vars
    if let Ok(path) = env::var("PATH") {
        let first_path = path.split(':').next().unwrap_or("(empty)");
        println!("First PATH entry: {}", first_path);
    }

    // Current directory
    if let Ok(cwd) = env::current_dir() {
        println!("Current directory: {}", cwd.display());
    }
}
