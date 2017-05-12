//! Read-write–synchronized stacks.

use super::sequential::Stack;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A read-write–synchronized stack.
///
/// This can be shared between threads by wrapping it in an `Arc`.
#[derive(Debug)]
pub struct RwStack<T>(RwLock<Stack<T>>);

impl<T> RwStack<T> {
    /// Returns a new, empty stack.
    pub fn new() -> Self {
        Self::from_seq(Stack::new())
    }

    /// Converts a [sequential stack](../sequential/struct.Stack.html)
    /// into a concurrent `RwStack`.
    pub fn from_seq(seq: Stack<T>) -> Self {
        RwStack(RwLock::new(seq))
    }

    /// Converts a concurrent `RwStack` into a [sequential
    /// stack](../sequential/struct.Stack.html).
    pub fn into_seq(self) -> Stack<T> {
        self.0.into_inner().expect("RwStack lock poisoned")
    }

    fn lock_read(&self) -> RwLockReadGuard<Stack<T>> {
        self.0.read().expect("RwStack lock poisoned")
    }

    fn lock_write(&self) -> RwLockWriteGuard<Stack<T>> {
        self.0.write().expect("RwStack lock poisoned")
    }

    /// Checks whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.lock_read().is_empty()
    }

    /// Returns the number of elements in the stack.
    pub fn len(&self) -> usize {
        self.lock_read().len()
    }

    /// Pushes an element on top of the stack.
    pub fn push(&self, data: T) {
        self.lock_write().push(data)
    }

    /// Removes and returns the top element of the stack, or `None` if
    /// empty.
    pub fn pop(&self) -> Option<T> {
        self.lock_write().pop()
    }
}

impl<T: Clone> RwStack<T> {
    /// Gets a clone of the top element of the stack, if there is one.
    pub fn peek(&self) -> Option<T> {
        self.lock_read().peek().map(|data| data.clone())
    }
}

impl<T: Clone> Clone for RwStack<T> {
    fn clone(&self) -> Self {
        RwStack::from_seq(self.lock_read().clone())
    }
}

#[test]
fn two_threads_cooperate() {
    use std::{sync, thread};

    let stack  = sync::Arc::new(RwStack::new());
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
