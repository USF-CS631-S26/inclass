fn main() {
    cc::Build::new()
        .file("asm/first_s.s")
        .file("asm/add1_s.s")
        .file("asm/add3_s.s")
        .file("asm/add3arr_s.s")
        .file("asm/ifelse_s.s")
        .file("asm/loop_s.s")
        .compile("asm_functions");

    println!("cargo:rerun-if-changed=asm/first_s.s");
    println!("cargo:rerun-if-changed=asm/add1_s.s");
    println!("cargo:rerun-if-changed=asm/add3_s.s");
    println!("cargo:rerun-if-changed=asm/add3arr_s.s");
    println!("cargo:rerun-if-changed=asm/ifelse_s.s");
    println!("cargo:rerun-if-changed=asm/loop_s.s");
}
