#![feature(naked_functions, asm, core_intrinsics, panic_info_message, global_asm, asm_sym)]

#![no_std]
#![no_main]

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
