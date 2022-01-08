pub use x86_64::instructions::interrupts::{
    are_enabled as interrupts_are_enabled,
    disable as disable_interrupts,
    enable as enable_interrupts,
};
use x86_64::instructions::segmentation::Segment;

pub use console::Console;

mod boot;
mod console;
mod gdt;
mod idt;

fn init() {
    // initialize the global descriptor table
    gdt::GDT.0.load();
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(gdt::GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(gdt::GDT.1.tss_selector);
    }

    // initialize the interrupt descriptor table
    idt::IDT.load();

    // initialize hardware interrupts (intel PIC8259)
    unsafe { idt::PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();
}

pub fn wait_forever() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn shut_down(_: super::ExitCode) {
    super::wait_forever();
}
