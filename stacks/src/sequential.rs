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
/// assert_eq!(Some(5), stack.pop());
/// assert_eq!(Some(4), stack.pop());
/// assert_eq!(Some(3), stack.pop());
/// assert_eq!(None, stack.pop());
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
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node_ptr| {
            let node = *node_ptr;
            self.len -= 1;
            self.head = node.next;
            node.data
        })
    }

    /// Allows viewing the top element of the stack, if there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::sequential::Stack;
    /// let mut stack = Stack::new();
    ///
    /// assert_eq!(None, stack.peek());
    /// stack.push(3);
    /// assert_eq!(Some(&3), stack.peek());
    /// stack.push(4);
    /// assert_eq!(Some(&4), stack.peek());
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node_ptr| &node_ptr.data)
    }
}

impl<T: Clone> Clone for Stack<T> {
    /// Clones a stack by cloning its contents.
    ///
    /// # Example
    ///
    /// ```
    /// # use stacks::sequential::Stack;
    /// let mut stack = Stack::new();
    ///
    /// stack.push(3);
    /// stack.push(4);
    /// stack.push(5);
    ///
    /// let mut stack2 = stack.clone();
    ///
    /// assert_eq!(Some(5), stack.pop());
    /// assert_eq!(Some(4), stack.pop());
    /// assert_eq!(Some(3), stack.pop());
    /// assert_eq!(None, stack.pop());
    ///
    /// assert_eq!(Some(5), stack2.pop());
    /// assert_eq!(Some(4), stack2.pop());
    /// assert_eq!(Some(3), stack2.pop());
    /// assert_eq!(None, stack2.pop());
    /// ```
    fn clone(&self) -> Self {
        let mut result = Stack {
            head: None,
            len:  self.len,
        };

        {
            let mut src = &self.head;
            let mut dst = Some(&mut result.head);

            while let &Some(ref src_node) = src {
                let dst_ref = dst.take().unwrap();

                *dst_ref = Some(Box::new(Node {
                    data: src_node.data.clone(),
                    next: None,
                }));

                if let Some(ref mut dst_node) = *dst_ref {
                    dst = Some(&mut dst_node.next);
                }

                src = &src_node.next;
            }
        }

        result
    }
}
