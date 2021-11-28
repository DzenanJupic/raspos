pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
