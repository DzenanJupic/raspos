use core::fmt;

pub struct Console;

impl Console {
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        todo!()
    }
}
