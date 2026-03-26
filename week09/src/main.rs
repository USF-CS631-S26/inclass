mod bits;
mod rv_emu;

use rv_emu::{rv_emulate, rv_init, RvState};

unsafe extern "C" {
    fn add2_s(a0: i32, a1: i32) -> i32;
}

fn decode() {
    let r = unsafe { add2_s(3, 4) };
    println!("add2_s(3, 4) = {}", r);

    let pc = add2_s as *const u32;

    let iw = unsafe { *pc };
    println!("[pc = {:p}] iw = {:X}", pc, iw);

    // Decode R-type fields
    let opcode = iw & 0b1111111;
    let funct3 = (iw >> 12) & 0b111;
    let funct7 = (iw >> 25) & 0b1111111;
    let rd     = (iw >> 7)  & 0b11111;
    let rs1    = (iw >> 15) & 0b11111;
    let rs2    = (iw >> 20) & 0b11111;

    println!("opcode  = {}", opcode);
    println!("funct3  = {}", funct3);
    println!("funct7  = {}", funct7);
    println!("rd      = {}", rd);
    println!("rs1     = {}", rs1);
    println!("rs2     = {}", rs2);

    // Advance to next instruction
    let pc = unsafe { pc.add(1) };
    let iw = unsafe { *pc };
    println!("[pc = {:p}] iw = {:X}", pc, iw);
}

fn signext() {
    let immu: u64 = 0b111111111101;
    let start: u32 = 11;
    let distance: u32 = 64 - start;

    let imm: i64 = ((immu as i64) << distance) >> distance;

    println!("imm = {}", imm);
}

fn emu_add2() {
    let r = unsafe { add2_s(3, 4) };
    println!("Asm: add2_s(3, 4) = {}", r);

    let mut state = RvState::new();
    rv_init(&mut state, add2_s as *const u32, 3, 4, 0, 0);
    let r = rv_emulate(&mut state);
    println!("Emu: add2_s(3, 4) = {}", r as i32);
}

fn main() {
    println!("== decode ==");
    decode();

    println!();

    println!("== signext ==");
    signext();

    println!();

    println!("== emu_add2 ==");
    emu_add2();
}
