use x86_64::structures::idt::InterruptDescriptorTable;

pub use hardware_interrupts::PICS;
use libcore::lazy::Lazy;

use crate::imp::idt::hardware_interrupts::Interrupt;

mod exceptions;
mod hardware_interrupts;

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    // CPU exceptions
    idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(exceptions::double_fault_handler)
            .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.page_fault.set_handler_fn(exceptions::page_fault_handler);

    // hardware interrupts
    idt[Interrupt::Timer as usize].set_handler_fn(hardware_interrupts::timer_handler);
    idt[Interrupt::KeyBoard as usize].set_handler_fn(hardware_interrupts::keyboard_handler);

    idt
});
