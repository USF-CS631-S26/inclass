extern "C" {
    fn add4f_s(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn add4f_callee_s(a: i32, b: i32, c: i32, d: i32) -> i32;
}

fn add4f(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a + b + c + d
}

fn main() {
    let r = add4f(1, 2, 3, 4);
    println!("Rust: {}", r);

    let r = unsafe { add4f_s(1, 2, 3, 4) };
    println!("Asm (caller-saved): {}", r);

    let r = unsafe { add4f_callee_s(1, 2, 3, 4) };
    println!("Asm (callee-saved): {}", r);
}
