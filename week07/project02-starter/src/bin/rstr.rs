use std::env;
use std::ffi::{CStr, CString};
use std::process;

unsafe extern "C" {
    fn rstr_s(dst: *mut u8, src: *const u8);
}

fn rstr(src: &str) -> String {
    let src_bytes = src.as_bytes();
    let src_len = src_bytes.len();
    let mut dst = vec![0u8; src_len];

    let mut j = src_len;
    for i in 0..src_len {
        j -= 1;
        dst[i] = src_bytes[j];
    }

    String::from_utf8(dst).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: rstr <string>");
        process::exit(-1);
    }

    let input = &args[1];

    let rust_result = rstr(input);
    println!("Rust: {}", rust_result);

    let c_src = CString::new(input.as_str()).unwrap();

    let mut dst_buf = [0u8; 1024];
    unsafe { rstr_s(dst_buf.as_mut_ptr(), c_src.as_ptr() as *const u8) };
    let s_result = unsafe { CStr::from_ptr(dst_buf.as_ptr() as *const std::ffi::c_char) };
    println!("Asm: {}", s_result.to_str().unwrap());
}
