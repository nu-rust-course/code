use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::mem;

/// Sets, represented as sorted lists.
///
/// # Example
///
/// ```
/// use list_sets::sequential::Set;
///
/// let mut set = Set::new();
///
/// assert!(set.is_empty());
/// assert_eq!(set.len(), 0);
///
/// set.insert("this");
/// ```
#[derive(Debug)]
pub struct Set<T> {
    len:  usize,
    head: Link<T>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Link<T>(Option<Box<Node<T>>>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data: T,
    link: Link<T>,
}

impl<T> Set<T> {
    /// Creates a new, empty list-set.
    ///
    /// # Example
    ///
    /// ```
    /// use list_sets::sequential::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert("hello");
    /// ```
    pub fn new() -> Self {
        Set {
            len:  0,
            head: Link::new(),
        }
    }

    /// Returns whether a set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use list_sets::sequential::Set;
    ///
    /// let set: Set<String> = Set::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the set.
    ///
    /// # Example
    ///
    /// ```
    /// use list_sets::sequential::Set;
    ///
    /// let set: Set<String> = Set::new();
    /// assert_eq!(set.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Ord> Set<T> {
    /// Adds an element to the set. If an equal (according to `cmp`)
    /// element `old` is already in the set, returns `Some(old)`:.
    ///
    /// # Example
    ///
    /// ```
    /// use list_sets::sequential::Set;
    ///
    /// let mut set = Set::new();
    ///
    /// set.insert(5);
    /// assert_eq!(set.len(), 1);
    /// set.insert(6);
    /// assert_eq!(set.len(), 2);
    /// set.insert(7);
    /// assert_eq!(set.len(), 3);
    ///
    /// set.insert(6);
    /// assert_eq!(set.len(), 3);
    ///
    /// set.insert(8);
    /// assert_eq!(set.len(), 4);
    /// ```
    pub fn insert(&mut self, key: T) -> Option<T> {
        let result = self.head.insert(key);

        if result.is_none() {
            self.len += 1;
        }

        result
    }

    // pub fn lookup(&self, key, &T) -> Option<&T> {
    //     &self.
    // }
}

impl<T> Link<T> {
    fn new() -> Self {
        Link(None)
    }
}

impl<T: Ord> Link<T> {
    fn insert(&mut self, key: T) -> Option<T> {

        match self.0 {
            None => {
                self.0 = Some(Box::new(Node {
                    data: key,
                    link: Link::new(),
                }));

                None
            },
            Some(ref mut box_node) => box_node.insert(key)
        }
    }
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, mut key: T) -> Option<T> {
        match key.cmp(&self.data) {
            Less    => {
                None
            }
            Equal   => {
                mem::swap(&mut key, &mut self.data);
                Some(key)
            }
            Greater => self.link.insert(key),
        }
    }
}

impl<T: PartialEq> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head.eq(&other.head)
    }
}

impl<T: Eq> Eq for Set<T> {}

impl<T: PartialOrd> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.head.partial_cmp(&other.head)
    }
}

impl<T: Ord> Ord for Set<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.head.cmp(&other.head)
    }
}

#[cfg(test)]
mod insert_test {
    use super::*;

    #[test]
    fn insert_descending_actually_inserts() {
        let mut set = Set::new();

        set.insert(7);
        assert_eq!(set.len(), 1);
        set.insert(6);
        assert_eq!(set.len(), 2);
        set.insert(5);
        assert_eq!(set.len(), 3);
    }
}
