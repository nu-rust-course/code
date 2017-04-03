#![allow(dead_code)]
//! Sets, represented as sorted, singly-linked lists.

use std::cmp::Ordering;
use std::mem;

/// A set of elements of type `T`.
///
/// # Example
///
/// ```
/// use intro::list_set::Set;
///
/// let mut set = Set::new();
///
/// set.insert("a");
/// set.insert("b");
///
/// if set.contains(&"a") {
///     set.insert("c");
/// }
/// ```
pub struct Set<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

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
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    /// set.insert("hello");
    /// ```
    pub fn new() -> Self {
        Set {
            len:  0,
            head: None,
        }
    }

    /// Returns whether a set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    /// assert!(set.is_empty());
    ///
    /// set.insert(5);
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    /// assert_eq!(0, set.len());
    ///
    /// set.insert(5);
    /// assert_eq!(1, set.len());
    ///
    /// set.insert(6);
    /// assert_eq!(2, set.len());
    ///
    /// set.insert(5);
    /// assert_eq!(2, set.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Ord> Set<T> {
    /// Checks whether the given set contains the given element.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// assert!(!set.contains(&"a"));
    ///
    /// set.insert("a");
    /// assert!(set.contains(&"a"));
    /// assert!(!set.contains(&"b"));
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        let mut current = &self.head;

        while let Some(ref node) = *current {
            match element.cmp(&node.data) {
                Ordering::Less => return false,
                Ordering::Equal => return true,
                Ordering::Greater => current = &node.link,
            }
        }

        false
    }

    /// Adds the element to the set.
    ///
    /// Returns `true` if the set did not previously contain the
    /// element, and `false` if it did.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    /// set.insert(3);
    /// set.insert(5);
    /// set.insert(4);
    ///
    /// assert!(!set.contains(&2));
    /// assert!(set.contains(&3));
    /// assert!(set.contains(&4));
    /// assert!(set.contains(&5));
    /// assert!(!set.contains(&6));
    /// ```
    pub fn insert(&mut self, element: T) -> bool {
        {
            let mut cur = CursorMut::new(self);

            while !cur.is_nil() {
                match element.cmp(cur.element()) {
                    Ordering::Less => break,
                    Ordering::Equal => return false,
                    Ordering::Greater => cur.next(),
                }
            }

            cur.insert(element);
        }

        self.len += 1;
        true
    }

    /// Adds the element to the set if absent, or replaces it if
    /// present.
    ///
    /// Returns `Some` of the old element if it was present.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// assert_eq!(None, set.replace(5));
    /// assert_eq!(Some(5), set.replace(5));
    /// ```
    pub fn replace(&mut self, element: T) -> Option<T> {
        {
            let mut cur = CursorMut::new(self);

            while !cur.is_nil() {
                match element.cmp(cur.element()) {
                    Ordering::Less => break,
                    Ordering::Equal => return Some(cur.replace(element)),
                    Ordering::Greater => cur.next(),
                }
            }

            cur.insert(element);
        }

        self.len += 1;
        None
    }

    /// Removes the given element from the set.
    ///
    /// Returns `true` if the element was removed and `false` if it
    /// didn't exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// assert_eq!(false, set.contains(&5));
    /// assert_eq!(true,  set.insert(5));
    /// assert_eq!(true,  set.contains(&5));
    /// assert_eq!(false, set.insert(5));
    /// assert_eq!(true,  set.remove(&5));
    /// assert_eq!(false, set.contains(&5));
    /// ```
    pub fn remove(&mut self, element: &T) -> bool {
        let mut result = false;

        {
            let mut cur = CursorMut::new(self);

            while !cur.is_nil() {
                if element == cur.element() {
                    cur.remove();
                    result = true;
                    break;
                }

                cur.next();
            }
        }

        if result {
            self.len -= 1;
        }

        result
    }
}

struct CursorMut<'a, T: 'a>(Option<&'a mut Link<T>>);

impl<'a, T: 'a> CursorMut<'a, T> {
    fn new(set: &'a mut Set<T>) -> Self {
        CursorMut(Some(&mut set.head))
    }

    fn element(&self) -> &T {
        if let Some(&mut Some(ref node)) = self.0 {
            &node.data
        } else {
            panic!("CursorMut::element: no element");
        }
    }

    fn is_nil(&self) -> bool {
        if let Some(&mut Some(_)) = self.0 {false} else {true}
    }

    fn next(&mut self) {
        if let Some(link) = self.0.take() {
            self.0 = link.as_mut().map(|node| &mut node.link);
        } else {
            panic!("CursorMut::next: no next link");
        }
    }

    fn replace(&mut self, element: T) -> T {
        if let Some(&mut Some(ref mut node)) = self.0 {
            mem::replace(&mut node.data, element)
        } else {
            panic!("CursorMut::replace: no element");
        }
    }

    fn map_link<F>(&mut self, mapper: F)
        where F: FnOnce(Link<T>) -> Link<T>
    {

        if let Some(ref mut link) = self.0 {
            let old_link = mem::replace(*link, None);
            mem::replace(*link, mapper(old_link));
        } else {
            panic!("CursorMut::map_link: empty cursor");
        }
    }

    fn remove(&mut self) {
        self.map_link(|link| match link {
            Some(node) => node.link,
            None => None
        });
    }

    fn insert(&mut self, element: T) {
        self.map_link(|link| Some(Box::new(Node {
            data: element,
            link: link,
        })));
    }
}

