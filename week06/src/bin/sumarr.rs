use std::env;
use std::process;

extern "C" {
    fn sumarr_idx_s(arr: *const i32, len: i32) -> i32;
    fn sumarr_ptr_s(arr: *const i32, len: i32) -> i32;
}

fn sumarr(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &v in arr {
        sum += v;
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: sumarr v1 v2 ...");
        process::exit(-1);
    }

    let arr: Vec<i32> = args[1..].iter().map(|s| s.parse().unwrap_or(0)).collect();
    let len = arr.len() as i32;

    let r = sumarr(&arr);
    println!("Rust: {}", r);

    let r = unsafe { sumarr_idx_s(arr.as_ptr(), len) };
    println!("Asm (idx): {}", r);

    let r = unsafe { sumarr_ptr_s(arr.as_ptr(), len) };
    println!("Asm (ptr): {}", r);
}
