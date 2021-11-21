use core::fmt;

pub struct Console;

impl Console {
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, b);
            }
        }

        Ok(())
    }
}
