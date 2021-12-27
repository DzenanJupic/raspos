use x86_64::structures::idt::InterruptDescriptorTable;

use libcore::lazy::Lazy;

mod exceptions;

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(exceptions::double_fault_handler)
            .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
});
