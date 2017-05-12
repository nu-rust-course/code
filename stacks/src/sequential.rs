/// A sequential stack.
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
#[derive(Debug)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    len:  usize,
}

#[derive(Debug)]
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

