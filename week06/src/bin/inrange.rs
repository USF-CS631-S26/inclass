extern "C" {
    fn inrange_s(x: i32, a: i32, b: i32) -> i32;
}

fn inrange(x: i32, a: i32, b: i32) -> i32 {
    if x >= a && x <= b {
        1
    } else {
        0
    }
}

fn main() {
    let r = inrange(3, 1, 7);
    println!("Rust: inrange(3,1,7) = {}", r);

    let r = unsafe { inrange_s(3, 1, 7) };
    println!("Asm: inrange(3,1,7) = {}", r);

    let r = inrange(10, 1, 7);
    println!("Rust: inrange(10,1,7) = {}", r);

    let r = unsafe { inrange_s(10, 1, 7) };
    println!("Asm: inrange(10,1,7) = {}", r);
}
