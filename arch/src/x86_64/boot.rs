#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    super::init();
    crate::kernel_main();
}
