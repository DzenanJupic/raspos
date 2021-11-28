pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {}
}

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
