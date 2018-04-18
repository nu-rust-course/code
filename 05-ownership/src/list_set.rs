#![allow(dead_code)]
//! Sets, represented as sorted, singly-linked lists.

use std::cmp::Ordering::{self, Less, Equal, Greater};
use std::default::Default;
use std::iter::{Extend, FromIterator};
use std::mem;

/// A set of elements of type `T`.
///
/// # Example
///
/// ```
/// use ownership::list_set::Set;
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
// Invariant: the elements must be sorted according to <T as Ord>.

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
    /// # use ownership::list_set::Set;
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
    /// # use ownership::list_set::Set;
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
    /// # use ownership::list_set::Set;
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

    /// Returns a borrowing iterator over the elements of the set.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set = Set::from_iter(vec![1, 3, 5]);
    /// let mut result = Vec::new();
    ///
    /// for elt in set.iter() {
    ///     result.push(elt);
    /// }
    ///
    /// assert_eq!( result, &[&1, &3, &5] );
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
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
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set = Set::from_iter(vec![3, 5, 4]);
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
    /// # use ownership::list_set::Set;
    /// let mut set = Set::new();
    /// set.insert(3);
    /// set.insert(5);
    /// set.insert(4);
    ///
    /// assert!(!set.contains(&2));
    /// assert!( set.contains(&3));
    /// assert!( set.contains(&4));
    /// assert!( set.contains(&5));
    /// assert!(!set.contains(&6));
    /// ```
    pub fn insert(&mut self, element: T) -> bool {
        let mut cur = CursorMut::new(self);

        while !cur.is_empty() {
            match element.cmp(cur.data()) {
                Less => break,
                Equal => return false,
                Greater => cur.advance(),
            }
        }

        cur.insert(element);

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
    /// # use ownership::list_set::Set;
    /// let mut set = Set::new();
    ///
    /// assert_eq!(None, set.replace(5));
    /// assert_eq!(Some(5), set.replace(5));
    /// ```
    pub fn replace(&mut self, element: T) -> Option<T> {
        let mut cur = CursorMut::new(self);

        while !cur.is_empty() {
            match element.cmp(cur.data()) {
                Less => break,
                Equal => {
                    let old_data = mem::replace(cur.data(), element);
                    return Some(old_data);
                }
                Greater => cur.advance(),
            }
        }

        cur.insert(element);

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
    /// # use ownership::list_set::Set;
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
        let mut cur = CursorMut::new(self);

        while !cur.is_empty() {
            match element.cmp(cur.data()) {
                Less => break,
                Equal => return Some(cur.remove()),
                Greater => cur.advance(),
            }
        }

        None
    }
}

struct CursorMut<'a, T: 'a> {
    link: Option<&'a mut Link<T>>,
    len: &'a mut usize,
}

impl<'a, T: 'a> CursorMut<'a, T> {
    fn new(set: &'a mut Set<T>) -> Self {
        CursorMut {
            link: Some(&mut set.head),
            len: &mut set.len,
        }
    }

    fn is_empty(&self) -> bool {
        if let Some(&mut Some(_)) = self.link {false} else {true}
    }

    fn data(&mut self) -> &mut T {
        if let Some(&mut Some(ref mut node_ptr)) = self.link {
            &mut node_ptr.data
        } else {
            panic!("CursorMut::data: empty cursor");
        }
    }

    fn advance(&mut self) {
        let link = self.link.take().expect("CursorMut::advance: empty cursor");
//      match link {
//          &mut Some(ref mut node_ptr) => self.link = Some(&mut node_ptr.link),
//          _ => panic!("CursorMut::advance: no next link"),
//      }
        self.link = Some(&mut link.as_mut().expect("CursorMut::advance: no next link").link);
    }

    fn remove(&mut self) -> T {
        if let Some(ref mut link) = self.link {
            if let Some(node_ptr) = link.take() {
                let node = *node_ptr;
                **link = node.link;
                *self.len -= 1;
                node.data
            } else {
                panic!("CursorMut::remove: no node to remove");
            }
        } else {
            panic!("CursorMut::remove: empty cursor");
        }
    }

    fn insert(&mut self, data: T) {
        if let Some(ref mut link_ptr) = self.link {
            **link_ptr = Some(Box::new(Node {
                data,
                link: link_ptr.take(),
            }));
            *self.len += 1;
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
/// # use ownership::list_set::Set;
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
/// # use ownership::list_set::Set;
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
        let mut cur = CursorMut::new(&mut self.0);
        if cur.is_empty() {
            None
        } else {
            Some(cur.remove())
        }
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

impl<T: Ord> Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<T: Ord> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut result = Set::new();
        result.extend(iter);
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

        result
    }
}

#[test]
fn test_clone() {
    let set1: Set<usize> = vec![3, 5, 4].into_iter().collect();
    let set2 = set1.clone();
    assert_eq!(set2, set1);
}

impl<T: Ord> Set<T> {
    /// Returns whether two sets are disjoint.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![1, 2]);
    /// let set2 = Set::from_iter(vec![3, 4]);
    /// let set3 = Set::from_iter(vec![1, 3]);
    ///
    /// assert!(!set1.is_disjoint(&set1));
    /// assert!( set1.is_disjoint(&set2));
    /// assert!(!set1.is_disjoint(&set3));
    /// assert!( set2.is_disjoint(&set1));
    /// assert!(!set2.is_disjoint(&set2));
    /// assert!(!set2.is_disjoint(&set3));
    /// assert!(!set3.is_disjoint(&set1));
    /// assert!(!set3.is_disjoint(&set2));
    /// assert!(!set3.is_disjoint(&set3));
    /// ```
    pub fn is_disjoint(&self, other: &Set<T>) -> bool {
        let mut i = &self.head;
        let mut j = &other.head;

        while let (&Some(ref ilink), &Some(ref jlink)) = (i, j) {
            match ilink.data.cmp(&jlink.data) {
                Less    => i = &ilink.link,
                Greater => j = &jlink.link,
                Equal   => return false,
            }
        }

        true
    }

    /// Returns whether `self` is a subset of `other`.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![2]);
    /// let set2 = Set::from_iter(vec![1, 2, 3]);
    /// let set3 = Set::from_iter(vec![1, 2, 3, 4]);
    ///
    /// assert!( set1.is_subset(&set1));
    /// assert!( set1.is_subset(&set2));
    /// assert!( set1.is_subset(&set3));
    /// assert!(!set2.is_subset(&set1));
    /// assert!( set2.is_subset(&set2));
    /// assert!( set2.is_subset(&set3));
    /// assert!(!set3.is_subset(&set1));
    /// assert!(!set3.is_subset(&set2));
    /// assert!( set3.is_subset(&set3));
    /// ```
    pub fn is_subset(&self, other: &Set<T>) -> bool {
        let mut i = &self.head;
        let mut j = &other.head;

        while let (&Some(ref ilink), &Some(ref jlink)) = (i, j) {
            match ilink.data.cmp(&jlink.data) {
                Less    => return false,
                Greater => j = &jlink.link,
                Equal   => {
                    i = &ilink.link;
                    j = &jlink.link;
                }
            }
        }

        i.is_none() || j.is_some()
    }

    /// Returns whether `self` is a superset of `other`.
    pub fn is_superset(&self, other: &Set<T>) -> bool {
        other.is_subset(self)
    }
}

impl<T: Ord + Clone> Set<T> {
    /// Returns the intersection of two sets.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
    ///
    /// let set3 = Set::from_iter(vec![1, 3]);
    ///
    /// assert_eq!(set3, set1.intersection(&set2));
    /// assert_eq!(set3, set2.intersection(&set1));
    /// ```
    pub fn intersection(&self, other: &Set<T>) -> Self {
        let mut result = Set::new();

        {
            let mut cur = CursorMut::new(&mut result);

            let mut i = self.into_iter().peekable();
            let mut j = other.into_iter().peekable();

            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
                match a.cmp(b) {
                    Less => {
                        i.next();
                    }
                    Greater => {
                        j.next();
                    }
                    Equal => {
                        cur.insert(a.clone());
                        cur.advance();
                        i.next();
                        j.next();
                    }
                }
            }
        }

        result
    }

    /// Returns the union of two sets.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
    ///
    /// let set3 = Set::from_iter(vec![1, 2, 3, 4, 5, 7]);
    ///
    /// assert_eq!(set3, set1.union(&set2));
    /// assert_eq!(set3, set2.union(&set1));
    /// ```
    pub fn union(&self, other: &Set<T>) -> Self {
        let mut result = Set::new();

        {
            let mut cur = CursorMut::new(&mut result);

            let mut i = self.into_iter().peekable();
            let mut j = other.into_iter().peekable();

            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
                match a.cmp(b) {
                    Less => {
                        cur.insert(a.clone());
                        cur.advance();
                        i.next();
                    }
                    Greater => {
                        cur.insert(b.clone());
                        cur.advance();
                        j.next();
                    }
                    Equal => {
                        cur.insert(a.clone());
                        cur.advance();
                        i.next();
                        j.next();
                    }
                }
            }

            for a in i {
                cur.insert(a.clone());
                cur.advance();
            }

            for b in j {
                cur.insert(b.clone());
                cur.advance();
            }
        }

        result
    }

    /// Returns the difference of two sets.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
    ///
    /// let set3 = Set::from_iter(vec![5, 7]);
    /// let set4 = Set::from_iter(vec![2, 4]);
    ///
    /// assert_eq!(set3, set1.difference(&set2));
    /// assert_eq!(set4, set2.difference(&set1));
    /// ```
    pub fn difference(&self, other: &Set<T>) -> Self {
        let mut result = Set::new();

        {
            let mut cur = CursorMut::new(&mut result);

            let mut i = self.into_iter().peekable();
            let mut j = other.into_iter().peekable();

            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
                match a.cmp(b) {
                    Less => {
                        cur.insert(a.clone());
                        cur.advance();
                        i.next();
                    }
                    Greater => {
                        j.next();
                    }
                    Equal => {
                        i.next();
                        j.next();
                    }
                }
            }

            for a in i {
                cur.insert(a.clone());
                cur.advance();
            }
        }

        result
    }

    /// Returns the symmetric difference of two sets.
    ///
    /// # Example
    ///
    /// ```
    /// # use ownership::list_set::Set;
    /// use std::iter::FromIterator;
    ///
    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
    ///
    /// let set3 = Set::from_iter(vec![2, 4, 5, 7]);
    ///
    /// assert_eq!(set3, set1.symmetric_difference(&set2));
    /// assert_eq!(set3, set2.symmetric_difference(&set1));
    /// ```
    pub fn symmetric_difference(&self, other: &Set<T>) -> Self {
        let mut result = Set::new();

        {
            let mut cur = CursorMut::new(&mut result);

            let mut i = self.into_iter().peekable();
            let mut j = other.into_iter().peekable();

            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
                match a.cmp(b) {
                    Less => {
                        cur.insert(a.clone());
                        cur.advance();
                        i.next();
                    }
                    Greater => {
                        cur.insert(b.clone());
                        cur.advance();
                        j.next();
                    }
                    Equal => {
                        i.next();
                        j.next();
                    }
                }
            }

            for a in i {
                cur.insert(a.clone());
                cur.advance();
            }

            for b in j {
                cur.insert(b.clone());
                cur.advance();
            }
        }

        result
    }
}

#[cfg(test)]
mod random_tests {
    use super::Set;

    quickcheck! {
        fn prop_member(vec: Vec<usize>, elems: Vec<usize>) -> bool {
            let set = v2s(&vec);

            for elem in elems {
                let in_v = (&vec).into_iter().any(|x| *x == elem);
                if set.contains(&elem) != in_v {
                    return false;
                }
            }

            true
        }

        fn prop_intersection(v1: Vec<usize>, v2: Vec<usize>) -> bool {
            let s1 = v2s(&v1);
            let s2 = v2s(&v2);
            let s3 = s1.intersection(&s2);

            for &elem in &v1 {
                if s3.contains(&elem) != s2.contains(&elem) {
                    return false;
                }
            }

            for &elem in &v2 {
                if s3.contains(&elem) != s1.contains(&elem) {
                    return false;
                }
            }

            for &elem in &s3 {
                if !s1.contains(&elem) || !s2.contains(&elem) {
                    return false;
                }
            }

            true
        }

        fn prop_union(v1: Vec<usize>, v2: Vec<usize>) -> bool {
            let s1 = v2s(&v1);
            let s2 = v2s(&v2);
            let s3 = s1.union(&s2);

            for &elem in &v1 {
                if !s3.contains(&elem) {
                    return false;
                }
            }

            for &elem in &v2 {
                if !s3.contains(&elem) {
                    return false;
                }
            }

            for &elem in &s3 {
                if !s1.contains(&elem) && !s2.contains(&elem) {
                    return false;
                }
            }

            true
        }
    }

    fn v2s<T: Clone + Ord>(vec: &Vec<T>) -> Set<T> {
        ::std::iter::FromIterator::from_iter(vec.clone())
    }
}
