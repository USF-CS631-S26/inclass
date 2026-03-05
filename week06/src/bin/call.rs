extern "C" {
    fn foo_s(a: i32) -> i32;
}

fn bar(a: i32) -> i32 {
    a + 1
}

fn foo(a: i32) -> i32 {
    bar(a) + 1
}

fn main() {
    let r = foo(3);
    println!("Rust: {}", r);

    let r = unsafe { foo_s(3) };
    println!("Asm: {}", r);
}
