//! Modified from `spin::mutex`

use crate::process::yield_now;
use core::cell::UnsafeCell;
use core::default::Default;
use core::fmt;
use core::marker::Sync;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};

/// This type provides MUTual EXclusion based on spinning.
pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new spinlock wrapping the supplied data.
    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let Mutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    fn obtain_lock(&self) {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false {
            // Wait until the lock looks unlocked before retrying
            while self.lock.load(Ordering::Relaxed) {
                yield_now();
                // spin_loop_hint();
            }
        }
    }

    /// Locks the spinlock and returns a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    pub fn lock(&self) -> MutexGuard<T> {
        self.obtain_lock();
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}
