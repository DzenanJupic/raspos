pub use console::console;

mod console;
mod mutex;

pub type Mutex<T> = lock_api::Mutex<mutex::RawMutex, T>;

#[inline(always)]
pub fn hold() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}
