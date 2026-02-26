use std::env;
use std::process;

extern "C" {
    fn ifelse_s(val: i32) -> i32;
}

fn ifelse(val: i32) -> i32 {
    if val > 0 {
        1
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: ifelse val");
        process::exit(-1);
    }

    let val: i32 = args[1].parse().unwrap_or(0);

    let r = ifelse(val);
    println!("Rust: {}", r);

    let r = unsafe { ifelse_s(val) };
    println!("Asm: {}", r);
}
