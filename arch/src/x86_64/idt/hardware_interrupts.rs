use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

use libcore::lazy::Lazy;
use libcore::sync::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    Timer = PIC_1_OFFSET,
    KeyBoard,
}

pub extern "x86-interrupt" fn timer_handler(_: InterruptStackFrame) {
    // log::info!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(Interrupt::Timer as u8); }
}

pub extern "x86-interrupt" fn keyboard_handler(_: InterruptStackFrame) {
    use pc_keyboard::{Keyboard, ScancodeSet1, layouts::Uk105Key, HandleControl, DecodedKey};

    static KEYBOARD: Lazy<Mutex<Keyboard<Uk105Key, ScancodeSet1>>> = Lazy::new(|| {
        Mutex::new(Keyboard::new(
            Uk105Key,
            ScancodeSet1,
            HandleControl::Ignore,
        ))
    });

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut keyboard = KEYBOARD.lock();


    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(c) => log::info!("{}", c),
                DecodedKey::RawKey(k) => log::info!("{:?}", k),
            }
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(Interrupt::KeyBoard as u8); }
}
