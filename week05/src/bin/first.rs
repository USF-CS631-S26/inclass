extern "C" {
    fn first_s() -> i32;
}

fn first_c() -> i32 {
    let x = 3;
    let y = 99;
    x + y
}

fn main() {
    let r = first_c();
    println!("first_c() = {}", r);

    let r = unsafe { first_s() };
    println!("first_s() = {}", r);
}
