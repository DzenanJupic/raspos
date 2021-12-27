use core::fmt::{self, Write};

use libcore::lazy::Lazy;
use libcore::sync::Mutex;

#[doc(hidden)]
pub fn console() -> &'static Mutex<impl Write> {
    static CONSOLE: Lazy<Mutex<arch::Console>> = Lazy::new(|| {
        // SAFETY: this will only be called once
        let console = unsafe { arch::Console::new() };
        Mutex::new(console)
    });

    &CONSOLE
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    arch::without_interrupts(|| {
        console()
            .lock()
            .write_fmt(args)
            .expect("Failed to write to the hardware console");
    })
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => ( $crate::print::_print(::core::format_args!($($arg)*)) );
}

#[macro_export]
macro_rules! println {
    () => ( $crate::print!("\n") );
    ($($arg:tt)*) => ( $crate::print!("{}\n", ::core::format_args!($($arg)*)) );
}

#[doc(hidden)]
#[cfg(feature = "qemu")]
pub fn serial_console() -> &'static Mutex<impl Write> {
    static CONSOLE: Lazy<Mutex<arch::qemu::Console>> = Lazy::new(|| {
        // SAFETY: this will only be called once
        let console = unsafe { arch::qemu::Console::new() };
        Mutex::new(console)
    });

    &CONSOLE
}

#[doc(hidden)]
#[cfg(feature = "qemu")]
pub fn _serial_print(args: fmt::Arguments) {
    arch::without_interrupts(|| {
        serial_console()
            .lock()
            .write_fmt(args)
            .expect("Failed to write to serial port console")
    })
}

#[macro_export]
#[cfg(feature = "qemu")]
macro_rules! serial_print {
    () => {};
    ($($arg:tt)*) => ( $crate::print::_serial_print(::core::format_args!($($arg)*)) );
}

#[macro_export]
#[cfg(feature = "qemu")]
macro_rules! serial_println {
    () => ( $crate::print!("\n") );
    ($($arg:tt)*) => ( $crate::serial_print!("{}\n", ::core::format_args!($($arg)*)) );
}
