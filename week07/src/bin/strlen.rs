use std::ffi::CString;

extern "C" {
    fn strlen_s(s: *const u8) -> usize;
}

fn strlen_c(s: &str) -> usize {
    s.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let s = if args.len() > 1 { &args[1] } else { "hello" };

    let r = strlen_c(s);
    println!("Rust: {}", r);

    let cs = CString::new(s).unwrap();
    let r = unsafe { strlen_s(cs.as_ptr() as *const u8) };
    println!("Asm: {}", r);
}
