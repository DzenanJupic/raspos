#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kernel::main();
    crate::arch::wait_forever();
}
