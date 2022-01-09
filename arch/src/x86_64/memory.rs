use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, structures::paging::PageTable, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};

/// Returns a mutable reference to the active level 4 table.
///
/// ### SAFETY:
/// - The complete physical memory must be mapped to virtual memory at the passed `physical_memory_offset`
/// - This function must be only called once to avoid aliasing `&mut` references
pub unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (table_frame, _) = Cr3::read();

    let phys_addr = table_frame.start_address();
    let virt_addr = physical_mem_offset + phys_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();

    &mut *page_table_ptr
}

pub struct PhysicalMemoryAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl PhysicalMemoryAllocator {
    /// Create a PhysicalMemoryAllocator from the passed memory map.
    ///
    /// ### SAFETY:This function is unsafe because the caller must guarantee that
    /// - The passed memory map must be valid
    /// - All frames that are marked as `USABLE` must actually be unused
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    /// An iterator over the usable physical frames.
    /// This also contains frames that might already be used by the OS. 'usable' in this case
    /// only refers to the initial state of the frame after booting.
    fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
        self.memory_map
            .iter()
            .filter(|mr| mr.region_type == MemoryRegionType::Usable)
            .map(|mr| mr.range.start_addr()..mr.range.end_addr())
            .flat_map(|range| range.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysicalMemoryAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        // TODO:
        //   This implementation is far from perfect, since we create the usable_frames
        //   iterator on each call.

        let frame = self
            .usable_frames()
            .nth(self.next);
        self.next += 1;
        frame
    }
}
