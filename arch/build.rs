use std::env::var;
use std::path::Path;

fn main() {
    let linker_file = format!("src/raw/{}/link.ld", var("CARGO_CFG_TARGET_ARCH").unwrap());

    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    println!("cargo:rerun-if-changed=build.rs");

    if Path::new(&linker_file).is_file() {
        println!("cargo:rerun-if-changed={}", linker_file);
    }
}
