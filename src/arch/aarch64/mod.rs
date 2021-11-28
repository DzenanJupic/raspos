pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}

pub const SUCCESS_EXIT_CODE: usize = 0;
pub const FAILURE_EXIT_CODE: usize = 1;

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
