#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

struct SharedVars {
    x: AtomicUsize,
    y: AtomicUsize,
}

impl SharedVars {
    fn new() -> Self {
        SharedVars {
            x: AtomicUsize::new(0),
            y: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn left(&self, order: Ordering) -> usize {
        self.x.store(1, order);
        self.y.load(order)
    }

    #[inline]
    fn right(&self, order: Ordering) -> usize {
        self.y.store(1, order);
        self.x.load(order)
    }
}

/// Runs one experiment, returning the values produced by left and right
/// threads.
#[inline]
fn run(order: Ordering) -> (usize, usize) {
    let shared_l = Arc::new(SharedVars::new());
    let shared_r = shared_l.clone();

    let handle_l = thread::spawn(move|| shared_l.left(order));
    let handle_r = thread::spawn(move|| shared_r.right(order));

    (handle_l.join().unwrap(), handle_r.join().unwrap())
}

fn is_valid(l: usize, r: usize) -> bool {
    (l == 0 && r == 1) || (l == 1 && r == 0) || (l == 1 && r == 1)
}

#[inline]
fn search(n: usize, order: Ordering) -> Option<(usize, usize)> {
    for _ in 0 .. n {
        let (l, r) = run(order);

        if !is_valid(l, r) {
            return Some((l, r))
        }
    }

    return None;
}

// This test should always succeed.
#[test]
fn seq_cst_does_not_find_anything() {
    assert_eq!(None, search(100_000, Ordering::SeqCst));
}

// This test can fail, but it can also succeed.
#[test]
fn relaxed_usually_finds_something() {
    assert_eq!(Some((0, 0)), search(1_000_000, Ordering::Relaxed));
}
