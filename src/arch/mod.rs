#[cfg(not(any(
target_arch = "aarch64",
target_arch = "x86_64",
)))]
compile_error!("unsupported target arch");

#[cfg_attr(target_arch = "aarch64", path = "aarch64/boot.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64/boot.rs")]
mod boot;

#[cfg_attr(target_arch = "aarch64", path = "aarch64/mod.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64/mod.rs")]
mod imp;

#[inline(never)]
pub fn wait_forever() -> ! {
    imp::wait_forever()
}

#[inline]
pub fn console() -> &'static crate::sync::Mutex<impl core::fmt::Write> {
    static CONSOLE: crate::sync::Mutex<imp::Console> = crate::sync::Mutex::new(imp::Console::new());
    &CONSOLE
}
