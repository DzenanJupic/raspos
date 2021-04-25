#![feature(naked_functions, asm, core_intrinsics)]
#![cfg(target_arch = "aarch64")]

#![no_std]
#![no_main]

#[path = "_arch/aarch64/boot.rs"]
mod boot;
#[path = "_arch/aarch64/lib.rs"]
pub mod lib;

pub mod kernel;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    lib::hold();
}
