//! Coarsely-synchronized stacks.

use super::sequential::Stack;

use std::sync::{Mutex, MutexGuard};

/// A coarsely-synchronized stack.
///
/// This can be shared between threads by wrapping it in an `Arc`.
#[derive(Debug)]
pub struct CoarseStack<T>(Mutex<Stack<T>>);

impl<T> CoarseStack<T> {
    /// Returns a new, empty stack.
    pub fn new() -> Self {
        Self::from_seq(Stack::new())
    }

    /// Converts a [sequential stack](../sequential/struct.Stack.html)
    /// into a concurrent `CoarseStack`.
    pub fn from_seq(seq: Stack<T>) -> Self {
        CoarseStack(Mutex::new(seq))
    }

    /// Converts a concurrent `CoarseStack` into a [sequential
    /// stack](../sequential/struct.Stack.html).
    pub fn into_seq(self) -> Stack<T> {
        self.0.into_inner().expect("CoarseStack mutex poisoned")
    }

    fn lock(&self) -> MutexGuard<Stack<T>> {
        self.0.lock().expect("CoarseStack mutex poisoned")
    }

    /// Checks whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }

    /// Returns the number of elements in the stack.
    pub fn len(&self) -> usize {
        self.lock().len()
    }

    /// Pushes an element on top of the stack.
    pub fn push(&self, data: T) {
        self.lock().push(data)
    }

    /// Removes and returns the top element of the stack, or `None` if
    /// empty.
    pub fn pop(&self) -> Option<T> {
        self.lock().pop()
    }
}

impl<T: Clone> CoarseStack<T> {
    /// Gets a clone of the top element of the stack, if there is one.
    pub fn peek(&self) -> Option<T> {
        self.lock().peek().cloned()
    }
}

impl<T: Clone> Clone for CoarseStack<T> {
    fn clone(&self) -> Self {
        CoarseStack::from_seq(self.lock().clone())
    }
}

#[test]
fn two_threads_cooperate() {
    use std::{sync, thread};

    let stack  = sync::Arc::new(CoarseStack::new());
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
