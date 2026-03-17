fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("riscv") {
        cc::Build::new()
            .file("asm/get_bitseq_s.s")
            .file("asm/get_bitseq_signed_s.s")
            .file("asm/pack_bytes_s.s")
            .file("asm/unpack_bytes_s.s")
            .file("asm/rstr_s.s")
            .file("asm/rstr_rec_s.s")
            .compile("asm_functions");

        // Explicitly pass link flags to binary targets, since the [lib] crate
        // doesn't reference these symbols and Cargo won't propagate them.
        println!("cargo:rustc-link-arg-bins=-lasm_functions");
    }

    println!("cargo:rerun-if-changed=asm/get_bitseq_s.s");
    println!("cargo:rerun-if-changed=asm/get_bitseq_signed_s.s");
    println!("cargo:rerun-if-changed=asm/pack_bytes_s.s");
    println!("cargo:rerun-if-changed=asm/unpack_bytes_s.s");
    println!("cargo:rerun-if-changed=asm/rstr_s.s");
    println!("cargo:rerun-if-changed=asm/rstr_rec_s.s");
}
