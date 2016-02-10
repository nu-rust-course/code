use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    lock: Mutex<usize>,
    cv:   Condvar,
}

impl Semaphore {
    pub fn new(value: usize) -> Self {
        Semaphore {
            lock: Mutex::new(value),
            cv:   Condvar::new(),
        }
    }

    pub fn raise(&self) {
        let mut guard = self.lock.lock().unwrap();
        *guard += 1;
        self.cv.notify_one();
    }

    pub fn lower(&self) {
        let mut guard = self.lock.lock().unwrap();

        while *guard == 0 {
            guard = self.cv.wait(guard).unwrap();
        }

        *guard -= 1;
    }

    pub fn try_lower(&self) -> Option<()> {
        let mut guard = self.lock.lock().unwrap();

        if *guard == 0 {
            None
        } else {
            *guard -= 1;
            Some(())
        }
    }
}
