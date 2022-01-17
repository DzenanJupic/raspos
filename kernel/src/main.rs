#![feature(
naked_functions, core_intrinsics, panic_info_message, asm_sym,
custom_test_frameworks,
)]

#![no_std]
#![no_main]
#![test_runner(kernel::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
#[macro_use]
extern crate kernel;

mod compile_time_checks;

test_runtime!(crate::test_main);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    main();
    arch::shut_down(arch::ExitCode::Success);
}

pub fn main() {
    kernel::init_logger();

    println!("Hello From Rust!");
    serial_println!("Hello Qemu!");

    kernel::Executor::new()
        .spawn(kernel::event_handlers::print_key_presses())
        .run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("\n\n\nKernel {}", info);
    serial_println!("\n\n\nKernel {}", info);
    arch::wait_forever()
}
