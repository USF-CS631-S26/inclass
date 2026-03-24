fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("riscv") {
        cc::Build::new()
            .flag("-march=rv64g")
            .file("asm/add2_s.s")
            .compile("asm_functions");

        println!("cargo:rustc-link-arg-bins=-lasm_functions");
    }

    println!("cargo:rerun-if-changed=asm/add2_s.s");
}
