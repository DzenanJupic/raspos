#[no_mangle]
pub unsafe extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    super::init(boot_info);
    crate::kernel_main();
}
