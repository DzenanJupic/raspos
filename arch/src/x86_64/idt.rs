use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use libcore::lazy::Lazy;

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
});

pub extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    log::info!("reached breakpoint: {:#?}", sf);
}

pub extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _: u64) -> ! {
    panic!("double fault: {:#?}", sf);
}
