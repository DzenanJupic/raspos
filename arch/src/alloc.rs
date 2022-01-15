use core::alloc::{GlobalAlloc, Layout};

use libcore::sync::Mutex;

use crate::raw::Allocator;

pub(crate) static ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new());

pub struct AllocatorHandle {
    _private: (),
}

impl AllocatorHandle {
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

unsafe impl GlobalAlloc for AllocatorHandle {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATOR.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATOR.lock().dealloc(ptr, layout)
    }
}
