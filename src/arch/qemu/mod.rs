pub use console::SerialConsole as Console;

mod console;

pub fn shut_down(exit_code: super::ExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
