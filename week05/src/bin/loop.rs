use std::env;
use std::process;

extern "C" {
    fn loop_s(n: i32) -> i32;
}

fn loop_fn(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: loop n");
        process::exit(-1);
    }

    let n: i32 = args[1].parse().unwrap_or(0);

    let r = loop_fn(n);
    println!("Rust: {}", r);

    let r = unsafe { loop_s(n) };
    println!("Asm: {}", r);
}
