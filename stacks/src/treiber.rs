//! Lock-free stacks.
//!
//! This code is based on [an article by Aaron
//! Turon](https://aturon.github.io/blog/2015/08/27/epoch/).

extern crate crossbeam;

use std::ptr;
use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};

use self::crossbeam::mem::epoch::{self, Atomic, Owned};

/// A lock-free stack.
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
}

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> TreiberStack<T> {
    /// Returns a new, empty stack.
    pub fn new() -> TreiberStack<T> {
        TreiberStack {
            head: Atomic::null()
        }
    }

    /// Checks whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.head.load(Acquire, &epoch::pin()).is_none()
    }

    /// Pushes an element on top of the stack.
    pub fn push(&self, data: T) {
        let mut new_node = Owned::new(Node {
            data: data,
            next: Atomic::null(),
        });

        let guard = epoch::pin();

        loop {
            let head = self.head.load(Acquire, &guard);
            new_node.next.store_shared(head, Relaxed);

            match self.head.cas(head, Some(new_node), Release) {
                Ok(_) => return,
                Err(owned) => new_node = owned.unwrap(),
            }
        }
    }

    /// Removes and returns the top element of the stack, or `None` if
    /// empty.
    pub fn pop(&self) -> Option<T> {
        let guard = epoch::pin();

        loop {
            if let Some(head) = self.head.load(Acquire, &guard) {
                let next = head.next.load(Relaxed, &guard);

                if self.head.cas_shared(Some(head), next, Release) {
                    return Some(unsafe {
                        guard.unlinked(head);
                        ptr::read(&head.data)
                    });
                }
            } else {
                return None;
            }
        }
    }
}
