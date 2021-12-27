use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

const INCOMPLETE: usize = 0x0;
const RUNNING: usize = 0x1;
const COMPLETE: usize = 0x2;

pub struct Once<T> {
    state: AtomicUsize,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Sync for Once<T> where T: Sync {}

impl<T> Once<T> {
    pub const INIT: Self = Self {
        state: AtomicUsize::new(INCOMPLETE),
        value: UnsafeCell::new(MaybeUninit::uninit()),
    };

    pub const fn new() -> Self {
        Self::INIT
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked_mut() })
        } else {
            None
        }
    }

    #[inline]
    pub fn set_once<F: FnOnce() -> T>(&self, f: F) -> &T {
        let res = self.state.compare_exchange(
            INCOMPLETE,
            RUNNING,
            Ordering::Acquire,
            Ordering::Relaxed,
        );

        match res {
            Err(COMPLETE) => unsafe {
                self.get_unchecked()
            }
            Ok(_) => unsafe {
                self.set_unchecked(f());
                self.get_unchecked()
            }
            Err(RUNNING) => {
                while !self.is_initialized() { core::hint::spin_loop(); }
                unsafe { self.get_unchecked() }
            }
            _ => unsafe { core::hint::unreachable_unchecked() }
        }
    }

    /// SAFETY:
    /// The state has to be COMPLETE
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &T {
        debug_assert!(self.is_initialized());
        (&*self.value.get()).assume_init_ref()
    }

    /// SAFETY:
    /// The state has to be COMPLETE
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        debug_assert!(self.is_initialized());
        (&mut *self.value.get()).assume_init_mut()
    }

    /// SAFETY:
    /// The state has to be set from INCOMPLETE to RUNNING by the the caller
    #[inline]
    pub unsafe fn set_unchecked(&self, value: T) {
        debug_assert_eq!(self.state.load(Ordering::Acquire), RUNNING);
        (&mut *self.value.get()).write(value);
        self.state.store(COMPLETE, Ordering::Release);
    }
}
