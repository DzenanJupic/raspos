pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
