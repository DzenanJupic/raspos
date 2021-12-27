#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

kernel::test_runtime!(test_main);

/// Execution should continue normally after a breakpoint interrupt
#[test_case]
fn breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
