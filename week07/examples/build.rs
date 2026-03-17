fn main() {
    cc::Build::new()
        .file("asm/bits_s.s")
        .file("asm/strlen_s.s")
        .file("asm/strcpy_s.s")
        .file("asm/get_bitseq_s.s")
        .file("asm/intdr_s.s")
        .compile("asm_functions");

    println!("cargo:rerun-if-changed=asm/bits_s.s");
    println!("cargo:rerun-if-changed=asm/strlen_s.s");
    println!("cargo:rerun-if-changed=asm/strcpy_s.s");
    println!("cargo:rerun-if-changed=asm/get_bitseq_s.s");
    println!("cargo:rerun-if-changed=asm/intdr_s.s");
}
