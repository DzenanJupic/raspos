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

#[cfg(feature = "qemu")]
mod qemu;

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
    use core::fmt;

    struct Console {
        arch: imp::Console,
        #[cfg(feature = "qemu")]
        qemu: qemu::Console,
    }

    impl Console {
        fn new() -> Self {
            Self {
                arch: imp::Console::new(),
                #[cfg(feature = "qemu")]
                qemu: qemu::Console::new(),
            }
        }
    }

    impl fmt::Write for Console {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            #[cfg(feature = "qemu")] {
                fmt::Write::write_str(&mut self.qemu, s)
                    .expect("failed to write to qemu console");
            }
            fmt::Write::write_str(&mut self.arch, s)
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            #[cfg(feature = "qemu")] {
                fmt::Write::write_char(&mut self.qemu, c)
                    .expect("failed to write to qemu console");
            }
            fmt::Write::write_char(&mut self.arch, c)
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            #[cfg(feature = "qemu")] {
                fmt::Write::write_fmt(&mut self.qemu, args)
                    .expect("failed to write to qemu console");
            }
            fmt::Write::write_fmt(&mut self.arch, args)
        }
    }

    static CONSOLE: Lazy<Mutex<Console>> = Lazy::INIT;
    CONSOLE.get(|| Mutex::new(Console::new()))
}
