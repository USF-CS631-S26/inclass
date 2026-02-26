use std::env;
use std::process;

extern "C" {
    fn add1_s(a: i32) -> i32;
}

fn add1_c(a: i32) -> i32 {
    a + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: add1 val");
        process::exit(-1);
    }

    let val: i32 = args[1].parse().unwrap_or(0);

    let r = add1_c(val);
    println!("add1_c({}) = {}", val, r);

    let r = unsafe { add1_s(val) };
    println!("add1_s({}) = {}", val, r);
}
