#![feature(naked_functions, asm, core_intrinsics, panic_info_message, global_asm)]
#![cfg(target_arch = "aarch64")]

#![no_std]
#![no_main]

#[cfg_attr(target_arch = "aarch64", path = "_arch/aarch64/boot.rs")]
mod boot;
#[cfg_attr(target_arch = "aarch64", path = "_arch/aarch64/lib.rs")]
pub mod lib;

#[macro_use]
pub mod print;
pub mod kernel;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    print!("\n\n\nKernel panicked");

    if let Some(location) = info.location() {
        print!(" at {}", location)
    }

    if let Some(msg) = info.message() {
        print!(":\n{}", msg);
    }

    println!();

    lib::wait_forever();
}
