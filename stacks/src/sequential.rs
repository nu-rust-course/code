//! Sequential stacks.

/// A sequential stack.
///
/// Represented as a linked list, so all operations are O(1).
///
/// # Example
///
/// ```
/// use stacks::sequential::Stack;
///
/// let mut stack = Stack::new();
///
/// stack.push(3);
/// stack.push(4);
/// stack.push(5);
/// assert_eq!(Some(5), stack.try_pop());
/// assert_eq!(Some(4), stack.try_pop());
/// assert_eq!(Some(3), stack.try_pop());
/// assert_eq!(None, stack.try_pop());
/// ```
#[derive(Clone, Debug)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    len:  usize,
}

#[derive(Clone, Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    /// Returns a new, empty stack.
    pub fn new() -> Self {
        Stack {
            head: None,
            len:  0,
        }
    }

    /// Checks whether the stack is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::sequential::Stack;
    /// let mut stack = Stack::new();
    ///
    /// assert!(stack.is_empty());
    /// stack.push(5);
    /// assert!(! stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the number of elements in the stack.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::sequential::Stack;
    /// let mut stack = Stack::new();
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
        self.len
    }

    /// Pushes an element on top of the stack.
    pub fn push(&mut self, data: T) {
        let old_head = self.head.take();
        let new_head = Some(Box::new(Node {
            data: data,
            next: old_head,
        }));
        self.len += 1;
        self.head = new_head;
    }

    /// Removes and returns the top element of the stack, or `None` if
    /// empty.
    pub fn try_pop(&mut self) -> Option<T> {
        self.head.take().map(|node_ptr| {
            let node = *node_ptr;
            self.len -= 1;
            self.head = node.next;
            node.data
        })
    }
}

