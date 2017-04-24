//! Classic semaphores.

use std::default::Default;
use std::sync::{Mutex, Condvar};

/// A semaphore is a synchronization primitive that keeps a count, which
/// cannot go below zero. Raising the count always succeeds, but
/// lowering the count will block rather than go below zero.
#[derive(Debug)]
pub struct Semaphore {
    counter: Mutex<usize>,
    condvar: Condvar,
}

/// An RIAA guard that automatically raises a semaphore when dropped.
/// Created by [`Semaphore::access`](fn.access.html).
#[derive(Debug)]
pub struct SemaphoreGuard<'a>(&'a Semaphore);

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        self.0.raise()
    }
}

impl Semaphore {
    /// Creates a new semaphore with the given initial count.
    pub fn new(count: usize) -> Self
    {
        Semaphore {
            counter: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    /// Returns the current count of the semaphore. Note that the
    /// semaphore count may change between a call to `count` and any
    /// subsequent attempts to raise or lower.
    pub fn count(&self) -> usize {
        *self.counter.lock().unwrap()
    }

    /// Raises the semaphore. This operation always succeeds.
    pub fn raise(&self) {
        *self.counter.lock().unwrap() += 1;
        self.condvar.notify_one();
    }

    /// Lowers the semaphore; if the count is 0, blocks until another
    /// thread raises.
    pub fn lower(&self) {
        let mut guard = self.counter.lock().unwrap();

        while *guard == 0 {
            guard = self.condvar.wait(guard).unwrap();
        }

        *guard -= 1;
    }

    /// Tries to lower the semaphore without blocking; returns `true`
    /// for success or `false` if the count was 0.
    pub fn try_lower(&self) -> bool {
        let mut guard = self.counter.lock().unwrap();

        if *guard == 0 {
            false
        } else {
            *guard -= 1;
            true
        }
    }

    /// Lowers the semaphore and returns an RAII guard that will
    /// automatically raise it when dropped.
    pub fn access(&self) -> SemaphoreGuard {
        self.lower();
        SemaphoreGuard(self)
    }
}

impl Default for Semaphore {
    /// The default semaphore starts with a count of 0.
    fn default() -> Self {
        Semaphore::new(0)
    }
}

// These tests don't test much. I'm not sure how to test concurrency
// primitives.
#[cfg(test)]
mod test {
    use super::Semaphore;

    #[test]
    fn raise_lower() {
        let sem = Semaphore::new(1);
        sem.lower();
        sem.raise();
        sem.lower();
    }

    #[test]
    fn access() {
        let sem = Semaphore::new(1);
        let _guard = sem.access();
    }

    #[test]
    fn try_lower_true() {
        let sem = Semaphore::new(1);
        assert!(sem.try_lower());
    }

    #[test]
    fn try_lower_false() {
        let sem = Semaphore::new(0);
        assert!(!sem.try_lower());
    }

    #[test]
    fn two_threads() {
        use std::sync::Arc;
        use std::thread;

        let sem  = Arc::new(Semaphore::new(0));
        let sem2 = sem.clone();
        let join = thread::spawn(move || sem2.lower());

        sem.raise();
        join.join().unwrap();
    }
}
