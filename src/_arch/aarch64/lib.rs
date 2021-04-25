#[inline(always)]
pub fn hold() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}
