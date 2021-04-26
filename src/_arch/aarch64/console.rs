use core::fmt;

#[inline]
pub fn console() -> &'static crate::lib::Mutex<impl fmt::Write> {
    static CONSOLE: crate::lib::Mutex<QemuOutput> = crate::lib::Mutex::new(QemuOutput);
    &CONSOLE
}

struct QemuOutput;

impl fmt::Write for QemuOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }

        Ok(())
    }
}
