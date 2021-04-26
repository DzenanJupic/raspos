use core::fmt::{self, Write};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    crate::lib::console()
        .lock()
        .write_fmt(args)
        .expect("Failed to write to the hardware console");
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => ( $crate::print::_print(format_args!($($arg)*)); );
}

#[macro_export]
macro_rules! println {
    () => ( $crate::print!("\n"); );
    ($($arg:tt)*) => ( $crate::print!("{}\n", format_args!($($arg)*)); );
}
