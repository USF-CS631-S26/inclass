use std::env;
use std::process;

extern "C" {
    fn add3_s(a: i32, b: i32, c: i32) -> i32;
}

fn add3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("usage: add3 a b c");
        process::exit(-1);
    }

    let a: i32 = args[1].parse().unwrap_or(0);
    let b: i32 = args[2].parse().unwrap_or(0);
    let c: i32 = args[3].parse().unwrap_or(0);

    let r = add3(a, b, c);
    println!("Rust: add3({}, {}, {}) = {}", a, b, c, r);

    let r = unsafe { add3_s(a, b, c) };
    println!("add3_s({}, {}, {}) = {}", a, b, c, r);
}
