use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    log::info!("reached breakpoint: {:#?}", sf);
}

pub extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _: u64) -> ! {
    panic!("double fault: {:#?}", sf);
}


pub extern "x86-interrupt" fn page_fault_handler(sf: InterruptStackFrame, err: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    log::error!(
        "PAGE FAULT\nAccessed Address: {:?}\nError Code: {:?}\n{:#?}",
        Cr2::read(), err, sf,
    );
    crate::wait_forever();
}
