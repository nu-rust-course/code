//! Classic semaphores.

#![allow(unused)]
#![allow(clippy::mutex_atomic)]

//use std::default::Default;
use std::sync::{Mutex, Condvar};

/// A semaphore is a synchronization primitive that keeps a count, which
/// cannot go below zero. Raising the count always succeeds, but
/// lowering the count will block rather than go below zero.
#[derive(Debug)]
pub struct Semaphore {
    counter: Mutex<usize>,
    condvar: Condvar,
}

// These tests don't test much. I'm not sure how to test concurrency
// primitives.
#[cfg(all(test, ignore_please))]
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
