pub use console::console;

mod console;
mod mutex;

pub type Mutex<T> = lock_api::Mutex<mutex::RawMutex, T>;

#[inline(never)]
pub fn wait_forever() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}
