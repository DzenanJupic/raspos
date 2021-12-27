pub use console::Console;

mod console;
mod interrupts;

pub fn wait_forever() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn init_idt() {
    use x86_64::structures::idt::InterruptDescriptorTable;
    use libcore::lazy::Lazy;

    static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(interrupts::breakpoint_handler);
        idt.double_fault.set_handler_fn(interrupts::double_fault_handler);

        idt
    });

    IDT.load();
}

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
