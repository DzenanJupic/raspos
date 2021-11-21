use core::sync::atomic::{AtomicBool, Ordering};

use lock_api::GuardSend;

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct RawMutex(AtomicBool);

unsafe impl lock_api::RawMutex for RawMutex {
    const INIT: Self = Self(AtomicBool::new(false));

    type GuardMarker = GuardSend;

    #[inline]
    fn lock(&self) {
        loop {
            if self.try_lock() { return; }

            for _ in 0..10 {
                core::hint::spin_loop();
            }
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        self.0.compare_exchange_weak(
            UNLOCKED,
            LOCKED,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_ok()
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.0.store(UNLOCKED, Ordering::Release);
    }

    #[inline]
    fn is_locked(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
