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
    print!("\n\n\nKernel panicked");

    if let Some(location) = info.location() {
        print!(" at {}", location)
    }

    if let Some(msg) = info.message() {
        print!(":\n{}", msg);
    }

    println!();

    arch::wait_forever()
}
