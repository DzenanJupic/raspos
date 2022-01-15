use core::alloc::Layout;
use core::ptr::NonNull;

use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::VirtAddr;

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024; // 100 KB

pub struct Allocator {
    head: Block,
}

unsafe impl Send for Allocator {}

#[repr(C, align(16))]
#[derive(Debug)]
struct Block {
    size: usize,
    prev: Option<NonNull<Block>>,
    next: Option<NonNull<Block>>,
}

impl Allocator {
    pub(crate) const fn new() -> Self {
        Self {
            head: Block {
                size: 0,
                prev: None,
                next: None
            },
        }
    }

    pub(crate) unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let mut first = match self.head.next {
            Some(first) => first,
            None => return core::ptr::null_mut(),
        };

        let allocation_size = {
            let blocks = layout.size() / core::mem::size_of::<Block>();
            (blocks + 1) * core::mem::size_of::<Block>()
        };

        let mut block = first.as_mut();

        // find the first block that's big enough for this allocation
        loop {
            match block.next {
                _ if block.size >= allocation_size => break,
                Some(mut ptr) => block = ptr.as_mut(),
                None => return core::ptr::null_mut(),
            }
        }

        let allocation_ptr = block as *mut _ as *mut u8;

        // split the block if it's bigger then the requested allocation
        if block.size > allocation_size {
            assert!(
                block.size - allocation_size >= core::mem::size_of::<Block>(),
                "The allocation size must be a multiple of the Block size\n\
                allocation_size: {}, block_size: {}, size_of_block: {}",
                allocation_size, block.size, core::mem::size_of::<Block>(),
            );

            // create a new block in the remaining heap space of the current `block`
            let remaining_start = allocation_ptr.add(allocation_size) as *mut Block;
            remaining_start.write(Block {
                size: block.size - allocation_size,
                prev: Some(NonNull::new_unchecked(block as *mut _)),
                next: block.next,
            });
            let remaining_ptr = NonNull::new_unchecked(remaining_start);

            // insert the new `remaining` block after the current `block`
            if let Some(mut next) = block.next {
                next.as_mut().prev = Some(remaining_ptr);
            }
            block.next = Some(remaining_ptr);
        }

        // remove the current `block` from the list
        if let Some(mut prev) = block.prev {
            prev.as_mut().next = block.next;
        }
        if let Some(mut next) = block.next {
            next.as_mut().prev = block.prev;
        }

        allocation_ptr
    }

    pub(crate) unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        assert!(
            HEAP_START <= (ptr as usize) && (ptr as usize) < HEAP_START + HEAP_SIZE,
            "invalid deallocation ptr {:p}", ptr,
        );

        let allocation_size = {
            let blocks = layout.size() / core::mem::size_of::<Block>();
            (blocks + 1) * core::mem::size_of::<Block>()
        };

        // create a new block from the returned memory
        let block_ptr = NonNull::new_unchecked(ptr as *mut Block);
        block_ptr.as_ptr().write(Block {
            size: allocation_size,
            prev: Some(NonNull::new_unchecked(&mut self.head)),
            next: self.head.next,
        });

        // insert the new block after the head element
        if let Some(mut first) = self.head.next {
            first.as_mut().prev = Some(block_ptr);
        }
        self.head.next = Some(block_ptr);

        self.defragmentate();
    }

    unsafe fn defragmentate(&mut self) {
        let mut next_block = self.head.next;
        while let Some(mut block_ptr) = next_block {
            let next_ptr = {
                let block_size = block_ptr.as_ref().size;
                let block_ptr = block_ptr.as_ptr() as *mut u8;
                let next_ptr = block_ptr.add(block_size);
                next_ptr as *mut Block
            };

            let mut next_child = self.head.next;
            while let Some(mut child_ptr) = next_child {
                let child = child_ptr.as_mut();

                if child_ptr.as_ptr() == next_ptr {
                    if let Some(mut prev) = child.prev {
                        prev.as_mut().next = child.next;
                    }
                    if let Some(mut next) = child.next {
                        next.as_mut().prev = child.prev;
                    }
                    block_ptr.as_mut().size += child.size;
                    break
                }

                next_child = child_ptr.as_ref().next;
            }

            next_block = block_ptr.as_ref().next;
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
        assert!(
            HEAP_SIZE > core::mem::size_of::<Block>(),
            "heap size is to small to initialize the allocator",
        );

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

        crate::alloc::ALLOCATOR
            .lock()
            .init_self();

        Ok(())
    }

    unsafe fn init_self(&mut self) {
        assert!(self.head.next.is_none(), "Tried to initialized the allocator twice");

        let ptr = NonNull::new(HEAP_START as *mut Block).unwrap();

        ptr.as_ptr().write(Block {
            size: HEAP_SIZE,
            prev: Some(NonNull::new_unchecked(&mut self.head)),
            next: None,
        });

        self.head.next = Some(ptr);
    }
}
