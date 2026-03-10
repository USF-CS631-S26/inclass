fn main() {
    let vals: Vec<i8> = vec![0, 1, -1, 127, -128, 42, -42];

    for v in vals {
        println!("{:4} = 0x{:02x}", v, v as u8);
    }
}
