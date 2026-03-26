use crate::bits::{get_bits, sign_extend};

const RV_ZERO: usize = 0;
const RV_RA: usize = 1;
const RV_SP: usize = 2;
pub const RV_A0: usize = 10;
pub const RV_A1: usize = 11;
pub const RV_A2: usize = 12;
pub const RV_A3: usize = 13;
const RV_NUM_REGS: usize = 32;

const STACK_SIZE: usize = 8192;

const FMT_R: u32 = 0b0110011;
const FMT_I_ARITH: u32 = 0b0010011;
const FMT_I_JALR: u32 = 0b1100111;

pub struct RvState {
    pub regs: [u64; RV_NUM_REGS],
    pub pc: *const u8,
    pub stack: [u8; STACK_SIZE],
}

impl RvState {
    pub fn new() -> Box<Self> {
        Box::new(RvState {
            regs: [0; RV_NUM_REGS],
            pc: std::ptr::null(),
            stack: [0; STACK_SIZE],
        })
    }
}

fn unsupported(s: &str, n: u32) -> ! {
    eprintln!("unsupported {} 0x{:x}", s, n);
    std::process::exit(-1);
}

fn get_rd(iw: u32) -> usize {
    get_bits(iw as u64, 7, 5) as usize
}

fn get_funct3(iw: u32) -> u32 {
    get_bits(iw as u64, 12, 3)
}

fn get_funct7(iw: u32) -> u32 {
    get_bits(iw as u64, 25, 7)
}

fn get_rs1(iw: u32) -> usize {
    get_bits(iw as u64, 15, 5) as usize
}

fn get_rs2(iw: u32) -> usize {
    get_bits(iw as u64, 20, 5) as usize
}

fn run_r_format(s: &mut RvState, iw: u32) {
    let rd = get_rd(iw);
    let funct3 = get_funct3(iw);
    let funct7 = get_funct7(iw);
    let rs1 = get_rs1(iw);
    let rs2 = get_rs2(iw);

    match (funct3, funct7) {
        (0b000, 0b0000000) => {
            // add
            s.regs[rd] = s.regs[rs1].wrapping_add(s.regs[rs2]);
        }
        _ => unsupported("R-type funct3", funct3),
    }

    s.pc = unsafe { s.pc.add(4) };
}

fn run_i_arith(s: &mut RvState, iw: u32) {
    let rd = get_rd(iw);
    let rs1 = get_rs1(iw);
    let funct3 = get_funct3(iw);
    let imm = sign_extend(get_bits(iw as u64, 20, 12) as u64, 12);

    match funct3 {
        0b000 => {
            // addi
            s.regs[rd] = (s.regs[rs1] as i64).wrapping_add(imm) as u64;
        }
        _ => unsupported("I-arith funct3", funct3),
    }

    s.pc = unsafe { s.pc.add(4) };
}

fn run_i_jalr(s: &mut RvState, iw: u32) {
    let rd = get_rd(iw);
    let rs1 = get_rs1(iw);
    let imm = sign_extend(get_bits(iw as u64, 20, 12) as u64, 12);
    let target = (s.regs[rs1] as i64).wrapping_add(imm) as u64;

    if rd != 0 {
        s.regs[rd] = (s.pc as u64).wrapping_add(4);
    }

    s.pc = target as *const u8;
}

fn rv_one(state: &mut RvState) {
    let iw = unsafe { *(state.pc as *const u32) };
    let opcode = get_bits(iw as u64, 0, 7);

    match opcode {
        FMT_R => run_r_format(state, iw),
        FMT_I_ARITH => run_i_arith(state, iw),
        FMT_I_JALR => run_i_jalr(state, iw),
        _ => unsupported("format", opcode),
    }
}

pub fn rv_init(
    state: &mut RvState,
    target: *const u32,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
) {
    state.pc = target as *const u8;
    state.regs[RV_A0] = a0;
    state.regs[RV_A1] = a1;
    state.regs[RV_A2] = a2;
    state.regs[RV_A3] = a3;

    state.regs[RV_ZERO] = 0;
    state.regs[RV_RA] = 0;
    state.regs[RV_SP] = unsafe { state.stack.as_ptr().add(STACK_SIZE) as u64 };
}

pub fn rv_emulate(state: &mut RvState) -> u64 {
    while !state.pc.is_null() {
        rv_one(state);
        // Ensure x0 stays 0
        state.regs[RV_ZERO] = 0;
    }
    state.regs[RV_A0]
}
