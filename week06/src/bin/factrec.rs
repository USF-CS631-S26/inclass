use std::env;
use std::process;

extern "C" {
    fn factrec_s(n: i32) -> i32;
}

fn factrec(n: i32) -> i32 {
    if n <= 0 {
        1
    } else {
        n * factrec(n - 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: factrec n");
        process::exit(-1);
    }

    let n: i32 = args[1].parse().unwrap_or(0);

    let r = factrec(n);
    println!("Rust: {}", r);

    let r = unsafe { factrec_s(n) };
    println!("Asm: {}", r);
}
