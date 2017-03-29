use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    counter: Mutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(count: usize) -> Self
    {
        Semaphore {
            counter: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    pub fn raise(&self) {
        *self.counter.lock().unwrap() += 1;
        self.condvar.notify_one();
    }

    pub fn lower(&self) {
        let mut guard = self.counter.lock().unwrap();

        while *guard == 0 {
            guard = self.condvar.wait(guard).unwrap();
        }

        *guard -= 1;
    }

    pub fn try_lower(&self) -> bool {
        let mut guard = self.counter.lock().unwrap();

        if *guard == 0 {
            false
        } else {
            *guard -= 1;
            true
        }
    }
}
