#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    crate::kernel_main();
}
