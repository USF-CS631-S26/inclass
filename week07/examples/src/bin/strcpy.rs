use std::ffi::{CStr, CString};

extern "C" {
    fn strcpy_s(dest: *mut u8, src: *const u8) -> *mut u8;
}

fn strcpy_c(dest: &mut [u8], src: &str) {
    let bytes = src.as_bytes();
    dest[..bytes.len()].copy_from_slice(bytes);
    dest[bytes.len()] = 0;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let s = if args.len() > 1 { &args[1] } else { "hello" };

    // Rust version
    let mut buf = vec![0u8; 256];
    strcpy_c(&mut buf, s);
    let result = CStr::from_bytes_until_nul(&buf).unwrap();
    println!("Rust: {}", result.to_str().unwrap());

    // Asm version
    let mut buf = vec![0u8; 256];
    let cs = CString::new(s).unwrap();
    unsafe { strcpy_s(buf.as_mut_ptr(), cs.as_ptr() as *const u8) };
    let result = CStr::from_bytes_until_nul(&buf).unwrap();
    println!("Asm: {}", result.to_str().unwrap());
}
