use bootloader::BootInfo;
pub use x86_64::instructions::interrupts::{
    are_enabled as interrupts_are_enabled,
    disable as disable_interrupts,
    enable as enable_interrupts,
};
use x86_64::instructions::segmentation::Segment;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

pub use self::{
    alloc::Allocator,
    console::Console,
};

mod alloc;
mod boot;
mod console;
mod gdt;
mod idt;
mod memory;

fn init(boot_info: &'static BootInfo) {
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

    // initialize memory
    let mut page_table = unsafe {
        let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let level_4_table = memory::active_level_4_table(physical_mem_offset);
        OffsetPageTable::new(level_4_table, physical_mem_offset)
    };
    let mut frame_allocator = unsafe {
        memory::PhysicalMemoryAllocator::new(&boot_info.memory_map)
    };
    unsafe {
        alloc::Allocator::init(&mut page_table, &mut frame_allocator)
            .expect("failed to initialize the heap")
    };

    x86_64::instructions::interrupts::enable();
}

pub fn wait_forever() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn shut_down(_: crate::ExitCode) {
    super::wait_forever();
}
