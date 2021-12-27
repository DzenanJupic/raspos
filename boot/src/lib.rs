#![no_std]

#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod boot;

extern "C" {
    fn kernel_main() -> !;
}
