use std::env;
use std::process;

unsafe extern "C" {
    fn pack_bytes_s(b3: u32, b2: u32, b1: u32, b0: u32) -> i32;
}

fn pack_bytes(b3: u32, b2: u32, b1: u32, b0: u32) -> i32 {
    let mut val: u32 = b3;
    val = (val << 8) | b2;
    val = (val << 8) | b1;
    val = (val << 8) | b0;
    val as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("usage: pack_bytes <b3> <b2> <b1> <b0>");
        process::exit(-1);
    }

    let b3: u32 = args[1].parse().unwrap_or(0);
    let b2: u32 = args[2].parse().unwrap_or(0);
    let b1: u32 = args[3].parse().unwrap_or(0);
    let b0: u32 = args[4].parse().unwrap_or(0);

    let rust_result = pack_bytes(b3, b2, b1, b0);
    println!("Rust: {}", rust_result);

    let s_result = unsafe { pack_bytes_s(b3, b2, b1, b0) };
    println!("Asm: {}", s_result);
}
