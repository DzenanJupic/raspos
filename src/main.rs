#![feature(
naked_functions, asm, core_intrinsics, panic_info_message, global_asm, asm_sym,
custom_test_frameworks,
)]

#![no_std]
#![no_main]

#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod arch;
#[macro_use]
pub mod print;
pub mod kernel;
pub mod sync;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("\n\n\nKernel {}", info);
    arch::wait_forever()
}

#[cfg(test)]
mod test_runner {
    pub trait Test {
        fn run(&self);
    }

    impl<F: Fn()> Test for F {
        fn run(&self) {
            print!("{} ...", core::any::type_name::<F>());
            self();
            println!(" ok");
        }
    }

    pub fn test_runner(tests: &[&dyn Test]) {
        println!("Running {} tests", tests.len());
        for test in tests { test.run(); }
    }
}
