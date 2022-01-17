use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

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
    let mut port = Port::new(0x60);

    unsafe {
        let scancode: u8 = port.read();
        crate::add_keyboard_scan_code(scancode);
        PICS.lock().notify_end_of_interrupt(Interrupt::KeyBoard as u8);
    }
}
