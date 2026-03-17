use std::env;
use std::process;

unsafe extern "C" {
    fn unpack_bytes_s(val: i32, bytes: *mut u32);
}

fn unpack_bytes(val: i32, bytes: &mut [u32; 4]) {
    let mut v = val as u32;
    for i in 0..4 {
        bytes[i] = v & 0xFF;
        v >>= 8;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: unpack_bytes <int>");
        process::exit(-1);
    }

    let val: i32 = args[1].parse().unwrap_or(0);

    let mut bytes = [0u32; 4];
    unpack_bytes(val, &mut bytes);
    println!("Rust: {} {} {} {}", bytes[3], bytes[2], bytes[1], bytes[0]);

    let mut bytes = [0u32; 4];
    unsafe { unpack_bytes_s(val, bytes.as_mut_ptr()) };
    println!("Asm: {} {} {} {}", bytes[3], bytes[2], bytes[1], bytes[0]);
}
