pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {}
}

pub const SUCCESS_EXIT_CODE: usize = 0x10;
pub const FAILURE_EXIT_CODE: usize = 0x11;

pub fn shut_down(exit_code: super::ExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
