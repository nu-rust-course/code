use crossbeam::mem::epoch::{self, Atomic, Owned};

use std::borrow::Borrow;
use std::cmp::Ordering::*;
use std::ptr;
use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};

#[derive(Debug)]
pub struct Set<T> {
    head: Atomic<Node<T>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Set {
            head: Atomic::null()
        }
    }
}

impl<T: Ord> Set<T> {
    pub fn is_empty(&self) -> bool {
        self.head.load(Acquire, &epoch::pin()).is_none()
    }

    pub fn contains<Q>(&self, element: &Q) -> bool
        where T: Borrow<Q>, Q: ?Sized + Ord
    {
        let guard = epoch::pin();

        let mut ptr = &self.head;

        while let Some(node) = ptr.load(Relaxed, &guard) {
            match element.cmp(node.data.borrow()) {
                Less => {
                    ptr = &node.next;
                }

                Equal => {
                    return true;
                }

                Greater => {
                    return false;
                }
            }
        }

        false
    }

    pub fn insert(&self, element: T) -> bool {
        let mut new_node = Owned::new(Node {
            data: element,
            next: Atomic::null()
        });

        let guard = epoch::pin();

        'retry: loop {
            let mut ptr = &self.head;

            while let Some(node) = ptr.load(Acquire, &guard) {
                match new_node.data.cmp(&node.data) {
                    Less => {
                        ptr = &node.next;
                    }

                    Equal => {
                        return false;
                    }

                    Greater => {
                        new_node.next.store_shared(Some(node), Relaxed);
                        match ptr.cas(Some(node), Some(new_node), Release) {
                            Ok(_) => return true,
                            Err(owned) => {
                                new_node = owned.unwrap();
                                continue 'retry;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn remove_min(&self) -> Option<T> {
        let guard = epoch::pin();

        loop {
            match self.head.load(Acquire, &guard) {
                Some(head) => {
                    let next = head.next.load(Relaxed, &guard);

                    if self.head.cas_shared(Some(head), next, Release) {
                        unsafe {
                            guard.unlinked(head);
                            return Some(ptr::read(&head.data));
                        }
                    }
                }

                None => return None,
            }
        }
    }

    pub fn remove<Q>(&self, element: &Q) -> Option<T>
        where T: Borrow<Q>, Q: ?Sized + Ord
    {
        let guard = epoch::pin();

        'retry: loop {
            let mut ptr = &self.head;

            while let Some(node) = ptr.load(Acquire, &guard) {
                match element.cmp(&node.data.borrow()) {
                    Less => {
                        ptr = &node.next;
                    }

                    Equal => {
                        let next = node.next.load(Relaxed, &guard);
                        if ptr.cas_shared(Some(node), next, Release) {
                            unsafe {
                                guard.unlinked(node);
                                return Some(ptr::read(&node.data));
                            }
                        } else {
                            continue 'retry;
                        }
                    }

                    Greater => {
                        return None;
                    }
                }
            }
        }
    }
}
