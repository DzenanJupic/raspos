use std::env::var;

fn main() {
    let linker_file = format!("src/_arch/{}/link.ld", var("CARGO_CFG_TARGET_ARCH").unwrap());

    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
}
