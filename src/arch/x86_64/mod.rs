pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {}
}
