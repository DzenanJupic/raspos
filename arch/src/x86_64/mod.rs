use x86_64::instructions::segmentation::Segment;

pub use console::Console;

mod console;
mod gdt;
mod idt;

pub fn wait_forever() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn init() {
    // initialize the global descriptor table
    gdt::GDT.0.load();
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(gdt::GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(gdt::GDT.1.tss_selector);
    }

    // initialize the interrupt descriptor table
    idt::IDT.load();
}

pub fn shut_down(_: super::ExitCode) {
    wait_forever();
}
