use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    log::info!("reached breakpoint: {:#?}", sf);
}
