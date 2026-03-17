extern "C" {
    fn intdr_s(p: *mut i32);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let val: i32 = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        5
    };

    // Rust version
    let mut x = val;
    x += 1;
    println!("Rust: {}", x);

    // Asm version
    let mut x = val;
    unsafe { intdr_s(&mut x as *mut i32) };
    println!("Asm: {}", x);
}
