#![feature(
naked_functions, asm, core_intrinsics, panic_info_message, global_asm, asm_sym,
custom_test_frameworks,
)]

#![no_std]
#![no_main]

#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod arch;
#[macro_use]
pub mod print;
pub mod kernel;
pub mod sync;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("\n\n\nKernel {}", info);
    arch::wait_forever()
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests { test(); }
}
