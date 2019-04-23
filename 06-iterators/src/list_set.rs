//! Sets, represented as sorted, singly-linked lists.

//use super::{Iter8or, IntoIter8or, FromIter8or, ExactSizeIter8or, Xtend};
use std::cmp::Ordering::*;

use std::default::Default;
use std::mem;

/// A set of elements of type `T`.
///
/// # Example
///
/// ```
/// use iterators::list_set::Set;
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
    /// # use iterators::list_set::Set;
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
    /// # use iterators::list_set::Set;
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
    /// # use iterators::list_set::Set;
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

//    /// Returns a borrowing iterator over the elements of the set.
//    ///
//    /// # Example
//    ///
//    /// ```
//    /// # use iterators::list_set::Set;
//    /// use iterators::{Iter8or, FromIter8or};
//    ///
//    /// let set = Set::from_iter(vec![1, 3, 5]);
//    /// let mut result = Vec::new();
//    ///
//    /// let mut iter = set.iter();
//    ///
//    /// while let Some(elt) = iter.next() {
//    ///     result.push(elt);
//    /// }
//    ///
//    /// assert_eq!( result, &[&1, &3, &5] );
//    /// ```
//    pub fn iter(&self) -> Iter<T> {
//        self.into_iter8or()
//    }

//    /// Returns an iterator that removes and returns elements satisfying a predicate, leaving the
//    /// rest in the set.
//    pub fn drain_filter<P: FnMut(&T) -> bool>(&mut self, pred: P) -> DrainFilter<T, P> {
//        let len = self.len;
//        DrainFilter {
//            cursor: CursorMut::new(self),
//            pred,
//            len,
//        }
//    }
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
    /// # use iterators::list_set::Set;
    /// use iterators::FromIter8or;
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
    /// # use iterators::list_set::Set;
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
    /// # use iterators::list_set::Set;
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
    /// # use iterators::list_set::Set;
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

#[derive(Debug)]
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

//impl<T: Ord> Ord for Set<T> {
//    fn cmp(&self, other: &Set<T>) -> Ordering {
//        let mut i = self.into_iter8or();
//        let mut j = other.into_iter8or();
//
//        loop {
//            match (i.next(), j.next()) {
//                (None, None) => return Equal,
//                (None, Some(_)) => return Less,
//                (Some(_), None) => return Greater,
//                (Some(a), Some(b)) => match a.cmp(b) {
//                    Less => return Less,
//                    Greater => return Greater,
//                    Equal => continue,
//                }
//            }
//        }
//    }
//}

//impl<T: Ord> PartialOrd for Set<T> {
//    fn partial_cmp(&self, other: &Set<T>) -> Option<Ordering> {
//        Some(self.cmp(other))
//    }
//}
//
//impl<T: Ord> PartialEq for Set<T> {
//    fn eq(&self, other: &Set<T>) -> bool {
//        self.cmp(other) == Equal
//    }
//}
//
//impl<T: Ord> Eq for Set<T> {}

//impl<T: Clone> Clone for Set<T> {
//    fn clone(&self) -> Self {
//        let mut result = Set::new();
//
//        {
//            let mut cur  = CursorMut::new(&mut result);
//            let mut iter = self.into_iter8or();
//
//            while let Some(each) = iter.next() {
//                cur.insert(each.clone());
//                cur.advance();
//            }
//        }
//
//        result
//    }
//}

//#[test]
//fn test_clone() {
//    let set1: Set<usize> = vec![3, 5, 4].into_iter8or().collect();
//    let set2 = set1.clone();
//    assert_eq!(set2, set1);
//}

impl<T: Ord> Set<T> {
    /// Returns whether two sets are disjoint.
    ///
    /// # Example
    ///
    /// ```
    /// # use iterators::list_set::Set;
    /// use iterators::FromIter8or;
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
    /// # use iterators::list_set::Set;
    /// use iterators::FromIter8or;
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

//impl<T: Ord + Clone> Set<T> {
//    /// Returns the intersection of two sets.
//    ///
//    /// # Example
//    ///
//    /// ```
//    /// # use iterators::list_set::Set;
//    /// use iterators::FromIter8or;
//    ///
//    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
//    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
//    ///
//    /// let set3 = Set::from_iter(vec![1, 3]);
//    ///
//    /// assert_eq!(set3, set1.intersection(&set2));
//    /// assert_eq!(set3, set2.intersection(&set1));
//    /// ```
//    pub fn intersection(&self, other: &Set<T>) -> Self {
//        let mut result = Set::new();
//
//        {
//            let mut cur = CursorMut::new(&mut result);
//
//            let mut i = self.into_iter8or().peekable();
//            let mut j = other.into_iter8or().peekable();
//
//            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
//                match a.cmp(b) {
//                    Less => {
//                        i.next();
//                    }
//                    Greater => {
//                        j.next();
//                    }
//                    Equal => {
//                        cur.insert(a.clone());
//                        cur.advance();
//                        i.next();
//                        j.next();
//                    }
//                }
//            }
//        }
//
//        result
//    }
//
//    /// Returns the union of two sets.
//    ///
//    /// # Example
//    ///
//    /// ```
//    /// # use iterators::list_set::Set;
//    /// use iterators::FromIter8or;
//    ///
//    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
//    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
//    ///
//    /// let set3 = Set::from_iter(vec![1, 2, 3, 4, 5, 7]);
//    ///
//    /// assert_eq!(set3, set1.union(&set2));
//    /// assert_eq!(set3, set2.union(&set1));
//    /// ```
//    pub fn union(&self, other: &Set<T>) -> Self {
//        let mut result = Set::new();
//
//        {
//            let mut cur = CursorMut::new(&mut result);
//
//            let mut i = self.into_iter8or().peekable();
//            let mut j = other.into_iter8or().peekable();
//
//            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
//                match a.cmp(b) {
//                    Less => {
//                        cur.insert(a.clone());
//                        cur.advance();
//                        i.next();
//                    }
//                    Greater => {
//                        cur.insert(b.clone());
//                        cur.advance();
//                        j.next();
//                    }
//                    Equal => {
//                        cur.insert(a.clone());
//                        cur.advance();
//                        i.next();
//                        j.next();
//                    }
//                }
//            }
//
//            while let Some(a) = i.next() {
//                cur.insert(a.clone());
//                cur.advance();
//            }
//
//            while let Some(b) = j.next() {
//                cur.insert(b.clone());
//                cur.advance();
//            }
//        }
//
//        result
//    }
//
//    /// Returns the difference of two sets.
//    ///
//    /// # Example
//    ///
//    /// ```
//    /// # use iterators::list_set::Set;
//    /// use iterators::FromIter8or;
//    ///
//    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
//    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
//    ///
//    /// let set3 = Set::from_iter(vec![5, 7]);
//    /// let set4 = Set::from_iter(vec![2, 4]);
//    ///
//    /// assert_eq!(set3, set1.difference(&set2));
//    /// assert_eq!(set4, set2.difference(&set1));
//    /// ```
//    pub fn difference(&self, other: &Set<T>) -> Self {
//        let mut result = Set::new();
//
//        {
//            let mut cur = CursorMut::new(&mut result);
//
//            let mut i = self.into_iter8or().peekable();
//            let mut j = other.into_iter8or().peekable();
//
//            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
//                match a.cmp(b) {
//                    Less => {
//                        cur.insert(a.clone());
//                        cur.advance();
//                        i.next();
//                    }
//                    Greater => {
//                        j.next();
//                    }
//                    Equal => {
//                        i.next();
//                        j.next();
//                    }
//                }
//            }
//
//            while let Some(a) = i.next() {
//                cur.insert(a.clone());
//                cur.advance();
//            }
//        }
//
//        result
//    }
//
//    /// Returns the symmetric difference of two sets.
//    ///
//    /// # Example
//    ///
//    /// ```
//    /// # use iterators::list_set::Set;
//    /// use iterators::FromIter8or;
//    ///
//    /// let set1 = Set::from_iter(vec![1, 3, 5, 7]);
//    /// let set2 = Set::from_iter(vec![1, 2, 3, 4]);
//    ///
//    /// let set3 = Set::from_iter(vec![2, 4, 5, 7]);
//    ///
//    /// assert_eq!(set3, set1.symmetric_difference(&set2));
//    /// assert_eq!(set3, set2.symmetric_difference(&set1));
//    /// ```
//    pub fn symmetric_difference(&self, other: &Set<T>) -> Self {
//        let mut result = Set::new();
//
//        {
//            let mut cur = CursorMut::new(&mut result);
//
//            let mut i = self.into_iter8or().peekable();
//            let mut j = other.into_iter8or().peekable();
//
//            while let (Some(&a), Some(&b)) = (i.peek(), j.peek()) {
//                match a.cmp(b) {
//                    Less => {
//                        cur.insert(a.clone());
//                        cur.advance();
//                        i.next();
//                    }
//                    Greater => {
//                        cur.insert(b.clone());
//                        cur.advance();
//                        j.next();
//                    }
//                    Equal => {
//                        i.next();
//                        j.next();
//                    }
//                }
//            }
//
//            while let Some(a) = i.next() {
//                cur.insert(a.clone());
//                cur.advance();
//            }
//
//            while let Some(b) = j.next() {
//                cur.insert(b.clone());
//                cur.advance();
//            }
//        }
//
//        result
//    }
//}

//#[cfg(test)]
//mod random_tests {
//    use super::Set;
//    use crate::iter8or::{IntoIter8or, FromIter8or, Iter8or};
//
//    quickcheck! {
//        fn prop_member(vec: Vec<usize>, elems: Vec<usize>) -> bool {
//            let set = v2s(&vec);
//
//            for elem in elems {
//                let in_v = (&vec).into_iter8or().any(|x| *x == elem);
//                if set.contains(&elem) != in_v {
//                    return false;
//                }
//            }
//
//            true
//        }
//
//        fn prop_intersection(v1: Vec<usize>, v2: Vec<usize>) -> bool {
//            let s1 = v2s(&v1);
//            let s2 = v2s(&v2);
//            let s3 = s1.intersection(&s2);
//
//            for &elem in &v1 {
//                if s3.contains(&elem) != s2.contains(&elem) {
//                    return false;
//                }
//            }
//
//            for &elem in &v2 {
//                if s3.contains(&elem) != s1.contains(&elem) {
//                    return false;
//                }
//            }
//
//            let mut s3i = s3.iter();
//            while let Some(&elem) = s3i.next() {
//                if !s1.contains(&elem) || !s2.contains(&elem) {
//                    return false;
//                }
//            }
//
//            true
//        }
//
//        fn prop_union(v1: Vec<usize>, v2: Vec<usize>) -> bool {
//            let s1 = v2s(&v1);
//            let s2 = v2s(&v2);
//            let s3 = s1.union(&s2);
//
//            for &elem in &v1 {
//                if !s3.contains(&elem) {
//                    return false;
//                }
//            }
//
//            for &elem in &v2 {
//                if !s3.contains(&elem) {
//                    return false;
//                }
//            }
//
//            let mut s3i = s3.iter();
//            while let Some(&elem) = s3i.next() {
//                if !s1.contains(&elem) && !s2.contains(&elem) {
//                    return false;
//                }
//            }
//
//            true
//        }
//    }
//
//    fn v2s<T: Clone + Ord>(vec: &Vec<T>) -> Set<T> {
//        FromIter8or::from_iter(vec.into_iter8or().map(Clone::clone))
//    }
//}
