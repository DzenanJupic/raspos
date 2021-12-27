use core::fmt::Write;

use log::{Metadata, Record};

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        cfg!(feature = "qemu")
    }

    fn log(&self, record: &Record) {
        let res = write!(
            crate::print::serial_console().lock(),
            "{: <5} [{}:{}]: {}\n",
            record.level(), record.target(), record.line().unwrap_or_default(), record.args(),
        );

        res.expect("failed to write log message to console");
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    if cfg!(feature = "qemu") {
        log::set_logger(&LOGGER)
            .expect("Logger was initialized multiple times");
        log::set_max_level(log::LevelFilter::Trace);
    }
}
