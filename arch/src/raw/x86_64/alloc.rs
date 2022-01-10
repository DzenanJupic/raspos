use core::alloc::{GlobalAlloc, Layout};

use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::VirtAddr;

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024; // 100 KB

static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

pub struct Allocator {
    _private: (),
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            _private: (),
        }
    }

    /// Initializes the kernel heap
    ///
    /// ### SAFETY:
    /// - This function may only be called once
    pub(crate) unsafe fn init(
        mapper: &mut impl Mapper<Size4KiB>,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>
    ) -> Result<(), MapToError<Size4KiB>> {
        let page_range = {
            let heap_start = VirtAddr::new(HEAP_START as u64);
            let heap_end = heap_start + HEAP_SIZE - 1u64;
            let heap_start_page = Page::containing_address(heap_start);
            let heap_end_page = Page::containing_address(heap_end);

            Page::range_inclusive(heap_start_page, heap_end_page)
        };

        for page in page_range {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

            mapper
                .map_to(page, frame, flags, frame_allocator)?
                .flush();
        }

        ALLOCATOR
            .lock()
            .init(HEAP_START, HEAP_SIZE);

        Ok(())
    }
}


unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATOR.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATOR.dealloc(ptr, layout)
    }
}
