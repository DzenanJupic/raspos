use core::cell::Cell;

use super::Once;

pub struct Lazy<T, F = fn() -> T> {
    once: Once<T>,
    init: Cell<Option<F>>,
}

unsafe impl<T, F> Sync for Lazy<T, F> where T: Sync, F: Sync {}

impl<T, F> Lazy<T, F> {
    pub const fn new(f: F) -> Self {
        Self {
            once: Once::new(),
            init: Cell::new(Some(f)),
        }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn force(this: &Self) -> &T {
        this.once.set_once(|| {
            let init = this.init
                .take()
                .expect("Lazy was left in an poisoned state");
            init()
        })
    }
}

impl<T, F: FnOnce() -> T> core::ops::Deref for Lazy<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Self::force(self)
    }
}

impl<T: Default> Default for Lazy<T> {
    fn default() -> Self {
        Self::new(T::default)
    }
}
