#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate boot;

pub use logger::init_logger;

#[macro_use]
pub mod print;

mod logger;
pub mod sync;


pub mod tests {
    #[cfg(all(test, not(feature = "qemu")))]
    compile_error!("tests can only be executed in qemu");

    #[macro_export]
    macro_rules! test_runtime {
        ($test_main:path) => {
            #[cfg(test)]
            #[no_mangle]
            pub extern "C" fn kernel_main() -> ! {
                $test_main();
                ::arch::shut_down(arch::ExitCode::Success);
            }

            #[cfg(test)]
            #[panic_handler]
            fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
                $crate::serial_println!(" failed\n\n{}", info);
                ::arch::shut_down(arch::ExitCode::Failed);
            }
        };
    }

    test_runtime!(crate::test_main);

    pub trait Test {
        fn run(&self);
    }

    impl<F: Fn()> Test for F {
        fn run(&self) {
            serial_print!("{} ...", core::any::type_name::<F>());
            self();
            serial_println!(" ok");
        }
    }

    pub fn test_runner(tests: &[&dyn Test]) {
        serial_println!("Running {} tests", tests.len());
        for test in tests { test.run(); }
        serial_println!("all {} tests passed successfully", tests.len());
    }
}
