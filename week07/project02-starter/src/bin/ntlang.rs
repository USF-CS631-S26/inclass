use std::process::{self, Command};

fn find_gcc() -> String {
    // Prefer cross-compiler if available (Docker), fall back to native gcc
    if Command::new("riscv64-linux-gnu-gcc").arg("--version")
        .stdout(process::Stdio::null()).stderr(process::Stdio::null())
        .status().is_ok()
    {
        "riscv64-linux-gnu-gcc".to_string()
    } else {
        "gcc".to_string()
    }
}

fn main() {
    println!("ntlang");
}
