extern "C" {
    fn and_s(a: u32, b: u32) -> u32;
    fn or_s(a: u32, b: u32) -> u32;
    fn xor_s(a: u32, b: u32) -> u32;
    fn not_s(a: u32) -> u32;
    fn sll_w(a: u32, n: u32) -> u32;
    fn srl_w(a: u32, n: u32) -> u32;
    fn sra_w(a: i32, n: u32) -> i32;
}

fn prbin(v: u32, bits: u32) {
    for i in (0..bits).rev() {
        print!("{}", (v >> i) & 1);
    }
    println!();
}

fn main() {
    let a: u8 = 0b11001100;
    let b: u8 = 0b10101010;

    println!("a = 0x{:02x}", a);
    prbin(a as u32, 8);
    println!("b = 0x{:02x}", b);
    prbin(b as u32, 8);
    println!();

    // AND
    let r = a & b;
    print!("Rust: a & b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    let r = unsafe { and_s(a as u32, b as u32) } as u8;
    print!("Asm:  a & b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    println!();

    // OR
    let r = a | b;
    print!("Rust: a | b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    let r = unsafe { or_s(a as u32, b as u32) } as u8;
    print!("Asm:  a | b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    println!();

    // XOR
    let r = a ^ b;
    print!("Rust: a ^ b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    let r = unsafe { xor_s(a as u32, b as u32) } as u8;
    print!("Asm:  a ^ b = 0x{:02x} ", r);
    prbin(r as u32, 8);
    println!();

    // NOT
    let r = !a;
    print!("Rust: !a = 0x{:02x} ", r);
    prbin(r as u32, 8);
    let r = unsafe { not_s(a as u32) } as u8;
    print!("Asm:  !a = 0x{:02x} ", r);
    prbin(r as u32, 8);
    println!();

    // Shift operations on u32
    let x: u32 = 0x0000000F;
    println!("x = 0x{:08x}", x);
    prbin(x, 32);
    println!();

    // SLL
    let r = x << 4;
    print!("Rust: x << 4 = 0x{:08x} ", r);
    prbin(r, 32);
    let r = unsafe { sll_w(x, 4) };
    print!("Asm:  x << 4 = 0x{:08x} ", r);
    prbin(r, 32);
    println!();

    // SRL
    let y: u32 = 0xF0000000;
    println!("y = 0x{:08x}", y);
    prbin(y, 32);

    let r = y >> 4;
    print!("Rust: y >> 4 = 0x{:08x} ", r);
    prbin(r, 32);
    let r = unsafe { srl_w(y, 4) };
    print!("Asm:  y >> 4 = 0x{:08x} ", r);
    prbin(r, 32);
    println!();

    // SRA
    let sy: i32 = 0xF0000000u32 as i32;
    println!("sy = 0x{:08x} (signed)", sy);
    prbin(sy as u32, 32);

    let r = sy >> 4;
    print!("Rust: sy >> 4 = 0x{:08x} ", r);
    prbin(r as u32, 32);
    let r = unsafe { sra_w(sy, 4) };
    print!("Asm:  sy >> 4 = 0x{:08x} ", r);
    prbin(r as u32, 32);
}
