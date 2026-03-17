use std::env;
use std::ffi::{CStr, CString};
use std::process;

unsafe extern "C" {
    fn rstr_rec_s(dst: *mut u8, src: *const u8);
}

fn rstr_rec_func(dst: &mut Vec<u8>, dst_idx: usize, src: &[u8], src_idx: usize) {
    if src_idx == usize::MAX {
        return;
    }

    dst[dst_idx] = src[src_idx];

    if src_idx == 0 {
        return;
    }

    rstr_rec_func(dst, dst_idx + 1, src, src_idx - 1);
}

fn rstr_rec(src: &str) -> String {
    let src_bytes = src.as_bytes();
    let src_len = src_bytes.len();
    let mut dst = vec![0u8; src_len];

    if src_len > 0 {
        rstr_rec_func(&mut dst, 0, src_bytes, src_len - 1);
    }

    String::from_utf8(dst).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: rstr_rec <string>");
        process::exit(-1);
    }

    let input = &args[1];

    let rust_result = rstr_rec(input);
    println!("Rust: {}", rust_result);

    let c_src = CString::new(input.as_str()).unwrap();

    let mut dst_buf = [0u8; 1024];
    unsafe { rstr_rec_s(dst_buf.as_mut_ptr(), c_src.as_ptr() as *const u8) };
    let s_result = unsafe { CStr::from_ptr(dst_buf.as_ptr() as *const std::ffi::c_char) };
    println!("Asm: {}", s_result.to_str().unwrap());
}
