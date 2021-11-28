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

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitCode {
    Success = imp::SUCCESS_EXIT_CODE,
    Failed = imp::FAILURE_EXIT_CODE,
}

#[inline(never)]
pub fn shut_down(exit_code: ExitCode) -> ! {
    imp::shut_down(exit_code);
    panic!("failed to shut down device");
}

#[inline]
pub fn console() -> &'static crate::sync::Mutex<impl core::fmt::Write> {
    use crate::sync::Mutex;
    use lazy_static::lazy::Lazy;

    static CONSOLE: Lazy<Mutex<imp::Console>> = Lazy::INIT;
    CONSOLE.get(|| Mutex::new(imp::Console::new()))
}
