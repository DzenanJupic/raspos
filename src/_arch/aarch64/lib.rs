pub use console::console;

mod console;

#[inline(always)]
pub fn hold() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}
