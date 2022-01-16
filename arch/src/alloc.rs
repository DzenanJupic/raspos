use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use libcore::sync::Mutex;

#[global_allocator]
pub(crate) static ALLOCATOR: LockedAllocator = LockedAllocator::new();

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation failed: {:?}", layout);
}

pub(crate) struct LockedAllocator {
    inner: Mutex<Allocator>,
}

impl LockedAllocator {
    pub(crate) const fn new() -> Self {
        Self { inner: Mutex::new(Allocator::new()) }
    }

    pub(crate) unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        self.inner
            .lock()
            .init(heap_start, heap_size)
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner
            .lock()
            .alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner
            .lock()
            .dealloc(ptr, layout)
    }
}

struct Allocator {
    slabs: [Option<&'static mut Block>; Allocator::BLOCK_SIZES.len()],
    fallback: linked_list_allocator::Heap,
}

impl Allocator {
    /// ### SAFETY:
    /// - Each block size has to be a power of 2
    const BLOCK_SIZES: &'static [usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

    const fn new() -> Self {
        Self {
            slabs: [Block::EMPTY; Self::BLOCK_SIZES.len()],
            fallback: linked_list_allocator::Heap::empty(),
        }
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        match Self::get_block_index(layout) {
            Some(index) => self.alloc_block(layout, index),
            None => self.alloc_fallback(layout),
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        match Self::get_block_index(layout) {
            Some(index) => self.dealloc_block(ptr, index),
            None => self.dealloc_fallback(ptr, layout),
        }
    }

    /// ### SAFETY:
    /// - The memory region must be valid for reads and writes
    /// - The allocator must get exclusive access to the memory region
    /// - This function may only be called once
    unsafe fn init(&mut self, heap_bottom: *mut u8, heap_size: usize) {
        assert!(
            heap_size > core::mem::size_of::<Block>(),
            "heap size is to small to initialize the allocator",
        );

        self.fallback.init(heap_bottom as _, heap_size);
    }

    #[inline(always)]
    fn get_block_index(layout: Layout) -> Option<usize> {
        let allocation_size = core::cmp::max(
            layout.size(),
            layout.align(),
        );

        Self::BLOCK_SIZES
            .iter()
            .position(|&size| size >= allocation_size)
    }

    #[inline(always)]
    unsafe fn alloc_block(&mut self, layout: Layout, index: usize) -> *mut u8 {
        match self.slabs[index].take() {
            Some(block) => {
                self.slabs[index] = block.next.take();
                block as *mut _ as *mut u8
            }
            None => self.alloc_fallback(layout),
        }
    }

    #[inline(always)]
    unsafe fn dealloc_block(&mut self, ptr: *mut u8, index: usize) {
        let ptr = ptr as *mut Block;
        let next = self.slabs[index].take();
        ptr.write(Block { next });
        self.slabs[index] = Some(&mut *ptr);
    }

    #[cold]
    unsafe fn alloc_fallback(&mut self, layout: Layout) -> *mut u8 {
        let mut res = self.fallback
            .allocate_first_fit(layout);

        if core::intrinsics::unlikely(res.is_err()) {
            self.defragment();
            res = self.fallback.allocate_first_fit(layout);
        }

        res
            .map(NonNull::as_ptr)
            .unwrap_or(core::ptr::null_mut())
    }

    #[cold]
    unsafe fn dealloc_fallback(&mut self, ptr: *mut u8, layout: Layout) {
        let ptr = NonNull::new(ptr).unwrap();
        self.fallback.deallocate(ptr, layout);
    }

    #[cold]
    unsafe fn defragment(&mut self) {
        for index in 0..Self::BLOCK_SIZES.len() {
            let layout = Layout::from_size_align(
                Self::BLOCK_SIZES[index],
                Self::BLOCK_SIZES[index],
            ).unwrap();

            while let Some(block) = self.slabs[index].take() {
                self.slabs[index] = block.next.take();
                let ptr = NonNull::new_unchecked(block as *mut _ as *mut u8);
                self.fallback.deallocate(ptr, layout);
            }
        }
    }
}

#[derive(Debug)]
struct Block {
    next: Option<&'static mut Self>,
}

impl Block {
    const EMPTY: Option<&'static mut Self> = None;
}
