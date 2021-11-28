use crate::arch;

/// This is the entrypoint of the kernel. It's the responsibility of the arch-dependent _start
/// function to call into the kernel.
#[inline(never)]
#[cfg_attr(test, allow(unreachable_code))]
pub fn main() -> ! {
    init_logger();

    #[cfg(test)] {
        crate::test_main();
        arch::shut_down(arch::ExitCode::Success);
    }

    println!("Hello From Rust!");
    arch::shut_down(arch::ExitCode::Success);
}


fn init_logger() {
    use log::{Metadata, Record};
    use core::fmt::Write;
    use crate::arch::console;

    static LOGGER: Logger = Logger;

    struct Logger;

    impl log::Log for Logger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            let res = write!(
                console().lock(),
                "{: <5} [{}:{}]: {}\n",
                record.level(), record.target(), record.line().unwrap_or_default(), record.args(),
            );

            res.expect("failed to write to console");
        }

        fn flush(&self) {}
    }

    log::set_logger(&LOGGER)
        .expect("Logger was initialized multiple times");
    log::set_max_level(log::LevelFilter::Trace);
}
