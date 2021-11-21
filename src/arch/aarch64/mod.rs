pub use console::Console;

mod console;

pub fn wait_forever() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}
