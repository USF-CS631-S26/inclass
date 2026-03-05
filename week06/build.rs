fn main() {
    cc::Build::new()
        .file("asm/call_s.s")
        .file("asm/add4f_s.s")
        .file("asm/add4f_callee_s.s")
        .file("asm/arr_s.s")
        .file("asm/inrange_s.s")
        .file("asm/factrec_s.s")
        .file("asm/sumarr_idx_s.s")
        .file("asm/sumarr_ptr_s.s")
        .compile("asm_functions");

    println!("cargo:rerun-if-changed=asm/call_s.s");
    println!("cargo:rerun-if-changed=asm/add4f_s.s");
    println!("cargo:rerun-if-changed=asm/add4f_callee_s.s");
    println!("cargo:rerun-if-changed=asm/arr_s.s");
    println!("cargo:rerun-if-changed=asm/inrange_s.s");
    println!("cargo:rerun-if-changed=asm/factrec_s.s");
    println!("cargo:rerun-if-changed=asm/sumarr_idx_s.s");
    println!("cargo:rerun-if-changed=asm/sumarr_ptr_s.s");
}
