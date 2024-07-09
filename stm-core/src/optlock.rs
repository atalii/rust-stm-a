use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

/// An OptLock (optimistic lock) provides block-free locking. When a lock is
/// contended, locking simply fails.
///
/// This fits better into the model of transactional memory than a traditional
/// RwLock or Mutex, as contention can be detected earlier and the transaction
/// can fail earlier.
pub struct OptLock<T> {
    lock: RawOptLock,
    value: RefCell<T>,
}

pub struct OptLockGuard<'a, T> {
    inner: RefMut<'a, T>,
    lock: &'a RawOptLock,
}

struct RawOptLock {
    locked: AtomicBool,
}

unsafe impl<T> Sync for OptLock<T> {}

impl<T> OptLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            lock: RawOptLock::new(),
            value: RefCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> Option<OptLockGuard<'a, T>> {
        match self.lock.lock() {
            true => Some(OptLockGuard {
                inner: self.value.borrow_mut(),
                lock: &self.lock,
            }),
            false => None,
        }
    }
}

impl<'a, T> Drop for OptLockGuard<'a, T> {
    fn drop(&mut self) {
        // SAFETY: Lock guards are unique by construction, so we can call unlock.
        unsafe { self.lock.unlock() }
    }
}

impl<'a, T> Deref for OptLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for OptLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl RawOptLock {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> bool {
        // XXX: am i getting orderings right?
        let was_locked = self.locked.swap(true, Ordering::Acquire);
        !was_locked
    }

    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
