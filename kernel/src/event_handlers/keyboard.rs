use crossbeam_queue::ArrayQueue;

use libcore::lazy::Once;

static SCANCODES: Once<ArrayQueue<u8>> = Once::new();

#[no_mangle]
pub extern "C" fn add_keyboard_scan_code(scancode: u8) {
    let _ = SCANCODES
        .get()
        .expect("Scancode queue uninitialized")
        .push(scancode)
        .map_err(|_| log::warn!("scancode queue full, dropping keyboard input"));
}
