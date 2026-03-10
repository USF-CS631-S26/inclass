fn main() {
    let x: i32 = 1;
    let ptr = &x as *const i32 as *const u8;
    let first_byte = unsafe { *ptr };

    if first_byte == 1 {
        println!("Little-endian");
    } else {
        println!("Big-endian");
    }
}
