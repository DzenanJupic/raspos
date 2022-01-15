#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(feature = "qemu", allow(dead_code))]

pub use raw::{
    disable_interrupts,
    enable_interrupts,
    interrupts_are_enabled,
};

extern "C" {
    /// The entry function into the arch-independent kernel.
    fn kernel_main() -> !;
}

mod compile_time_checks;

mod raw;
#[cfg(feature = "qemu")]
pub mod qemu;

pub mod alloc;


#[inline(never)]
pub fn wait_forever() -> ! {
    raw::wait_forever()
}

pub fn without_interrupts<T, F: FnOnce() -> T>(f: F) -> T {
    let enabled = interrupts_are_enabled();
    if enabled { disable_interrupts() }
    let ret = f();
    if enabled { enable_interrupts() }
    ret
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[inline(never)]
pub fn shut_down(exit_code: ExitCode) -> ! {
    #[cfg(feature = "qemu")]
        qemu::shut_down(exit_code);
    #[cfg(not(feature = "qemu"))]
        raw::shut_down(exit_code);
    panic!("failed to shut down device");
}

pub struct Console(raw::Console);

impl Console {
    /// SAFETY:
    ///     This function may only be called once. Creating multiple consoles might lead to
    ///     undefined behaviour.
    pub unsafe fn new() -> Self {
        Self(raw::Console::new())
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        core::fmt::Write::write_str(&mut self.0, s)
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        core::fmt::Write::write_char(&mut self.0, c)
    }

    fn write_fmt(self: &mut Self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        core::fmt::Write::write_fmt(&mut self.0, args)
    }
}
