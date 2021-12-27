use core::fmt;

use uart_16550::SerialPort;

pub struct SerialConsole(SerialPort);

impl SerialConsole {
    /// SAFETY:
    ///     This function may only be called once.
    pub unsafe fn new() -> Self {
        let mut serial_port = SerialPort::new(0x3f8);
        serial_port.init();
        Self(serial_port)
    }
}

impl fmt::Write for SerialConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args)
    }
}
