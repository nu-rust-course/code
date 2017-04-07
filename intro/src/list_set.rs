#![allow(dead_code)]
//! Sets, represented as sorted, singly-linked lists.

use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::default::Default;
use std::iter::FromIterator;
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
#[derive(Debug)]
pub struct Set<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
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

    /// Returns a draining iterator, which consumes the elements of
    /// the set as it iterates.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// set.insert(2);
    /// set.insert(3);
    /// set.insert(4);
    ///
    /// {
    ///     let mut drain = set.drain();
    ///     assert_eq!(Some(2), drain.next());
    ///     assert_eq!(Some(3), drain.next());
    ///     assert_eq!(Some(4), drain.next());
    ///     assert_eq!(None, drain.next());
    /// }
    ///
    /// assert!(set.is_empty());
    /// ```
    pub fn drain(&mut self) -> Drain<T> {
        Drain(self)
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Set::new()
    }
}

impl<T: Ord> Set<T> {
    /// Checks whether the given set contains the given element.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let set: Set<usize> = vec![3, 5, 4].into_iter().collect();
    ///
    /// assert!(!set.contains(&2));
    /// assert!( set.contains(&3));
    /// assert!( set.contains(&4));
    /// assert!( set.contains(&5));
    /// assert!(!set.contains(&6));
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        let mut current = &self.head;

        while let Some(ref node) = *current {
            match element.cmp(&node.data) {
                Less => return false,
                Equal => return true,
                Greater => current = &node.link,
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

            while !cur.is_empty() {
                match element.cmp(cur.data()) {
                    Less => break,
                    Equal => return false,
                    Greater => cur.advance(),
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

            while !cur.is_empty() {
                match element.cmp(cur.data()) {
                    Less => break,
                    Equal => {
                        return Some(mem::replace(cur.data(), element));
                    }
                    Greater => cur.advance(),
                }
            }

            cur.insert(element);
        }

        self.len += 1;
        None
    }

    /// Removes the given element from the set.
    ///
    /// Returns `Some(data)` where `data` was the element, if removed,
    /// or `None` if the element didn’t exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// assert_eq!(false,   set.contains(&5));
    /// assert_eq!(true,    set.insert(5));
    /// assert_eq!(true,    set.contains(&5));
    /// assert_eq!(false,   set.insert(5));
    /// assert_eq!(Some(5), set.remove(&5));
    /// assert_eq!(false,   set.contains(&5));
    /// ```
    pub fn remove(&mut self, element: &T) -> Option<T> {
        let mut result = None;

        {
            let mut cur = CursorMut::new(self);

            while !cur.is_empty() {
                match element.cmp(cur.data()) {
                    Less => break,
                    Equal => {
                        result = Some(cur.remove());
                        break;
                    }
                    Greater => cur.advance(),
                }
            }
        }

        if result.is_some() {
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

    fn is_empty(&self) -> bool {
        if let Some(&mut Some(_)) = self.0 {false} else {true}
    }

    fn data(&mut self) -> &mut T {
        if let Some(&mut Some(ref mut node)) = self.0 {
            &mut node.data
        } else {
            panic!("CursorMut::data: empty cursor");
        }
    }

    fn advance(&mut self) {
        if let Some(link) = self.0.take() {
            self.0 = link.as_mut().map(|node| &mut node.link);
        } else {
            panic!("CursorMut::advance: no next link");
        }
    }

    fn remove(&mut self) -> T {
        if let Some(ref mut link) = self.0 {
            if let Some(node_ptr) = mem::replace(*link, None) {
                let node = *node_ptr;
                mem::replace(*link, node.link);
                node.data
            } else {
                panic!("CursorMut::remove: no node to remove");
            }
        } else {
            panic!("CursorMut::remove: empty cursor");
        }
    }

    fn insert(&mut self, data: T) {
        if let Some(ref mut link) = self.0 {
            let old_link = mem::replace(*link, None);
            let new_link = Some(Box::new(Node {
                data: data,
                link: old_link,
            }));
            mem::replace(*link, new_link);
        } else {
            panic!("CursorMut::insert: empty cursor");
        }
    }
}

/// An immutable iterator over the elements of a `Set`.
///
/// # Example
///
/// ```
/// # use intro::list_set::Set;
/// let mut set = Set::new();
///
/// set.insert(2);
/// set.insert(4);
/// set.insert(3);
///
/// let mut iter = (&set).into_iter();
///
/// assert_eq!(Some(&2), iter.next());
/// assert_eq!(Some(&3), iter.next());
/// assert_eq!(Some(&4), iter.next());
/// assert_eq!(None, iter.next());
/// ```
#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    link: &'a Link<T>,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match *self.link {
            Some(ref node_ptr) => {
                self.link = &node_ptr.link;
                self.len -= 1;
                Some(&node_ptr.data)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> IntoIterator for &'a Set<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        Iter {
            link: &self.head,
            len: self.len,
        }
    }
}

/// An iterator that consumes a `Set` as it iterates.
///
/// # Example
///
/// ```
/// # use intro::list_set::Set;
/// let mut set = Set::new();
///
/// set.insert(2);
/// set.insert(4);
/// set.insert(3);
///
/// let mut iter = set.into_iter();
///
/// assert_eq!(Some(2), iter.next());
/// assert_eq!(Some(3), iter.next());
/// assert_eq!(Some(4), iter.next());
/// assert_eq!(None, iter.next());
/// ```
#[derive(Debug)]
pub struct IntoIter<T>(Set<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;

        {
            let mut cur = CursorMut::new(&mut self.0);
            if !cur.is_empty() {
                result = Some(cur.remove())
            }
        }

        if result.is_some() {
            self.0.len -= 1;
        }

        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

impl<T> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/// A draining iterator.
#[derive(Debug)]
pub struct Drain<'a, T: 'a>(&'a mut Set<T>);

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = None;

        {
            let mut cur = CursorMut::new(self.0);
            if !cur.is_empty() {
                result = Some(cur.remove());
            }
        }

        if result.is_some() {
            self.0.len -= 1;
        }

        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<'a, T> ExactSizeIterator for Drain<'a, T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<T: Ord> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut result = Set::new();

        for elem in iter {
            result.insert(elem);
        }

        result
    }
}

impl<T: Ord> Ord for Set<T> {
    fn cmp(&self, other: &Set<T>) -> Ordering {
        let mut i = self.into_iter();
        let mut j = other.into_iter();

        loop {
            match (i.next(), j.next()) {
                (None, None) => return Equal,
                (None, Some(_)) => return Less,
                (Some(_), None) => return Greater,
                (Some(a), Some(b)) => match a.cmp(b) {
                    Less => return Less,
                    Greater => return Greater,
                    Equal => continue,
                }
            }
        }
    }
}

impl<T: Ord> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Set<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialEq for Set<T> {
    fn eq(&self, other: &Set<T>) -> bool {
        self.cmp(other) == Equal
    }
}

impl<T: Ord> Eq for Set<T> {}

impl<T: Clone> Clone for Set<T> {
    fn clone(&self) -> Self {
        let mut result = Set::new();

        {
            let mut cur = CursorMut::new(&mut result);

            for each in self {
                cur.insert(each.clone());
                cur.advance();
            }
        }

        result.len = self.len;

        result
    }
}

#[test]
fn test_clone() {
    let set1: Set<usize> = vec![3, 5, 4].into_iter().collect();
    let set2 = set1.clone();
    assert_eq!(set2, set1);
}
