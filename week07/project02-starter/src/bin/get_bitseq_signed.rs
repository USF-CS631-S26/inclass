use std::env;
use std::process;

unsafe extern "C" {
    fn get_bitseq_signed_s(num: u32, start: i32, end: i32) -> i32;
}

fn get_bitseq(num: u32, start: i32, end: i32) -> u32 {
    let len = (end - start) + 1;
    let val = num >> start;
    let mask = if len == 32 {
        0xFFFFFFFF
    } else {
        (1u32 << len) - 1
    };
    val & mask
}

fn get_bitseq_signed(num: u32, start: i32, end: i32) -> i32 {
    let val = get_bitseq(num, start, end);
    let len = (end - start) + 1;
    let shift_amt = 32 - len;
    let val = val << shift_amt;
    ((val as i32) >> shift_amt) as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("usage: get_bitseq_signed number start_bit end_bit");
        process::exit(-1);
    }

    let num: u32 = args[1].parse().unwrap_or(0);
    let start: i32 = args[2].parse().unwrap_or(0);
    let end: i32 = args[3].parse().unwrap_or(0);

    let rust_result = get_bitseq_signed(num, start, end);
    println!("Rust: {}", rust_result);

    let s_result = unsafe { get_bitseq_signed_s(num, start, end) };
    println!("Asm: {}", s_result);
}
