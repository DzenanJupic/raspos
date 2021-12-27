#![feature(
naked_functions, asm, core_intrinsics, panic_info_message, global_asm, asm_sym,
custom_test_frameworks,
)]

#![no_std]
#![no_main]
#![test_runner(kernel::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate boot;
#[macro_use]
extern crate kernel;

test_runtime!(crate::test_main);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    main();
    arch::shut_down(arch::ExitCode::Success);
}

pub fn main() {
    kernel::init();

    println!("Hello From Rust!");
    serial_println!("Hello Qemu!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("\n\n\nKernel {}", info);
    serial_println!("\n\n\nKernel {}", info);
    arch::wait_forever()
}
