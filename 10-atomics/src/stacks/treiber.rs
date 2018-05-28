//! Lock-free stacks.
//!
//! This code is based on [an article by Aaron
//! Turon](https://aturon.github.io/blog/2015/08/27/epoch/).

use std::ptr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Release, AcqRel, Relaxed};

use epoch::{self, Atomic, Owned};

/// A lock-free stack.
///
/// # Example
///
/// ```
/// use stacks::treiber::TreiberStack;
///
/// let stack = TreiberStack::new();
///
/// stack.push(3);
/// stack.push(4);
/// stack.push(5);
/// assert_eq!(Some(5), stack.pop());
/// assert_eq!(Some(4), stack.pop());
/// assert_eq!(Some(3), stack.pop());
/// assert_eq!(None, stack.pop());
/// ```
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
    len:  AtomicUsize,
}

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> TreiberStack<T> {
    /// Returns a new, empty stack.
    pub fn new() -> TreiberStack<T> {
        TreiberStack {
            head: Atomic::null(),
            len:  AtomicUsize::new(0),
        }
    }

    /// Checks whether the stack is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::treiber::TreiberStack;
    /// let stack = TreiberStack::new();
    ///
    /// assert!(stack.is_empty());
    /// stack.push(5);
    /// assert!(! stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.load(Acquire, &epoch::pin()).is_null()
    }

    /// Returns a snapshop of the number of elements in the stack.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::treiber::TreiberStack;
    /// let stack = TreiberStack::new();
    ///
    /// assert_eq!(0, stack.len());
    /// stack.push(1);
    /// assert_eq!(1, stack.len());
    /// stack.push(2);
    /// assert_eq!(2, stack.len());
    /// stack.push(3);
    /// assert_eq!(3, stack.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len.load(Acquire)
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
            new_node.next.store(head, Relaxed);

            match self.head.compare_and_set(head, new_node, Release, &guard) {
                Ok(_) => {
                    self.len.fetch_add(1, AcqRel);
                    return;
                }
                Err(owned) => new_node = owned.new,
            }
        }
    }

    /// Removes and returns the top element of the stack, or `None` if
    /// empty.
    pub fn pop(&self) -> Option<T> {
        let guard = epoch::pin();

        loop {
            let shared_head = self.head.load(Acquire, &guard);
            if let Some(head) = unsafe { shared_head.as_ref() } {
                let next = head.next.load(Relaxed, &guard);

                if self.head.compare_and_set(shared_head, next, Release, &guard).is_ok() {
                    self.len.fetch_sub(1, AcqRel);
                    return Some(unsafe {
                        guard.defer(move || shared_head.into_owned());
                        ptr::read(&head.data)
                    });
                }
            } else {
                return None;
            }
        }
    }
}

impl<T: Clone> TreiberStack<T> {
    /// Gets a clone of the top element of the stack, if there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::treiber::TreiberStack;
    /// let stack = TreiberStack::new();
    ///
    /// assert_eq!(None, stack.peek());
    /// stack.push(3);
    /// assert_eq!(Some(3), stack.peek());
    /// stack.push(4);
    /// assert_eq!(Some(4), stack.peek());
    /// ```
    pub fn peek(&self) -> Option<T> {
        let guard = epoch::pin();
        let shared_head = self.head.load(Acquire, &guard);
        unsafe { shared_head.as_ref() }.map(|head| head.data.clone())
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[test]
fn two_threads_cooperate() {
    use std::{sync, thread};

    let stack  = sync::Arc::new(TreiberStack::new());
    let stack1 = stack.clone();
    let stack2 = stack.clone();

    let handle1 = thread::spawn(move || {
        for i in 0 .. 5 {
            stack1.push(i);
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 5 .. 10 {
            stack2.push(i);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let mut actual = Vec::new();
    while let Some(element) = stack.pop() {
        actual.push(element);
    }
    actual.sort();

    let expected: Vec<usize> = (0 .. 10).collect();

    assert_eq!(expected, actual);
}
