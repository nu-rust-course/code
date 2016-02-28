extern crate rand;

use std::cmp::Ordering;
use std::cmp::Ordering::*;

#[derive(Debug)]
pub struct SkipList<T> {
    len:   usize,
    ratio: usize,
    head:  Links<T>,
}

#[derive(Debug)]
struct Links<T> {
    spine: Option<Box<Node<T>>>,
    skips: Vec<*mut Node<T>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data:  T,
    links: Links<T>,
}

impl<T> SkipList<T> {
    /// Constructs a new, empty `SkipList<T>`. Equivalent to
    /// `SkipList::with_ratio(2)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    /// let mut list: SkipList<usize> = SkipList::new();
    /// ```
    pub fn new() -> Self {
        Self::with_ratio(2)
    }

    /// Constructs a new, empty `SkipList<T>` with height ratio equivalent
    /// to tree of degree `ratio`.
    ///
    /// TODO: Rename this.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    /// let mut list: SkipList<usize> = SkipList::with_ratio(3);
    /// ```
    pub fn with_ratio(ratio: usize) -> Self {
        SkipList {
            len:   0,
            ratio: ratio,
            head:  Links {
                spine: None,
                skips: vec![],
            },
        }
    }

    /// Returns true if the skip list contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut list = SkipList::<usize>::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the skip list.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut list = SkipList::<usize>::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            len:   self.len,
            links: &self.head,
        }
    }
}

impl<T: Ord> SkipList<T> {
    // pub fn lookup(&self, key: &T) -> Option<&T> {
    //     let mut links = &self.head;

    //     loop {
    //         if links.is_empty() {
    //             break;
    //         }

    //         for next in links.iter().rev() {
    //             if let &Some(ref next) = next {
    //                 match key.cmp(&next.data) {
    //                     Less    => {
    //                         // move down a level
    //                     }
    //                     Equal   => {
    //                         return Some(&next.data)
    //                     }
    //                     Greater => {
    //                         links = &next.links;
    //                         break;
    //                     }
    //                 }
    //             } else {break}
    //         }
    //     }

    //     None
    // }

    pub fn insert(&mut self, key: T) -> Option<T> {
        // Make a new node of the given height with empty links
        let height = choose_height(self.ratio);
        let mut new = Box::new(Node {
            data:  key,
            links: Links {
                spine: None,
                skips: vec![0 as *mut _; height],
            },
        });

        // Loop variable points to a Links<T>
        let mut links = &mut self.head;

        // Make sure the head is tall enough by adding an necessary
        // levels pointed to the new node
        while links.skips.len() < height {
            links.skips.push(&mut *new)
        }

        // Outer loop moves down a level, inner loop moves to the
        // right following a skip link.
        'outer: for level in (0 .. links.skips.len()).rev() {
            'inner: loop {
                // If the current skip link is the last of its height
                // then we need to move down, but first, link the
                // current skip link to the new node if the new node is
                // tall enough.
                if links.skips[level].is_null() {
                    if level < height {
                        links.skips[level] = &mut *new;
                    }
                    continue 'outer;
                }

                // If the next node is the new node already inserted
                // on this level, move down
                if links.skips[level] == &mut *new {
                    continue 'outer;
                }

                // Get access to the next node. This should be safe
                // because we maintain the invariant that all raw
                // pointers in the skip list point to nodes owned by
                // the skip list.
                let next = unsafe { &mut *links.skips[level] };

                // Compare the data to insert to the data of the next
                // (skipped to) node:
                match new.data.cmp(&next.data) {
                    // If the new data should come before, then we
                    // insert the new data into the list at this level
                    // and drop down a level.
                    Less     => {
                        new.links.skips[level] = next;
                        links.skips[level]     = &mut *new;
                        continue 'outer;
                    }

                    // If the new data should replace the old data,
                    // then we replace the old data with the new on the
                    // list at this level and drop down a level.
                    Equal   => {
                        new.links.skips[level] = next.links.skips[level];
                        links.skips[level]     = &mut *new;
                        continue 'outer;
                    }

                    // If the new data should come later, we move to
                    // the right following the skip link.
                    Greater => {
                        links = &mut next.links;
                        continue 'inner;
                    }
                }
            }
        }

        // Now we've zeroed in on where it should go, and we have to
        // do a final search at the spine level.
        while let Some(ref mut next) = links.spine {
            match new.data.cmp(&next.data) {
                Less    => unimplemented!(),
                Equal   => unimplemented!(),
                Greater => unimplemented!(),
            }
        }

        None
    }

    // pub fn remove(&mut self, key: &T) -> Option<T> {
    //     None
    // }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T: 'a> {
    len:   usize,
    links: &'a Links<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref node) = self.links.spine {
            self.len  -= 1;
            self.links = &node.links;
            Some(&node.data)
        } else {None}
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> { }

impl<T: PartialEq> PartialEq for Links<T> {
    fn eq(&self, other: &Self) -> bool {
        self.spine.eq(&other.spine)
    }
}

impl<T: Eq> Eq for Links<T> { }

impl<T: PartialOrd> PartialOrd for Links<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.spine.partial_cmp(&other.spine)
    }
}

impl<T: Ord> Ord for Links<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.spine.cmp(&other.spine)
    }
}

impl<T: PartialEq> PartialEq for SkipList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head.eq(&other.head)
    }
}

impl<T: Eq> Eq for SkipList<T> { }

impl<T: PartialOrd> PartialOrd for SkipList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.head.partial_cmp(&other.head)
    }
}

impl<T: Ord> Ord for SkipList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.head.cmp(&other.head)
    }
}

impl<'a, T> IntoIterator for &'a SkipList<T> {
    type Item     = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    len:   usize,
    links: Links<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.len -= 1;
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> { }

impl<T> IntoIterator for SkipList<T> {
    type Item     = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            links: self.head,
            len:   self.len,
        }
    }
}

/*
impl<T: PartialEq> PartialEq for SkipList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter() == other.iter()
    }
}
*/

pub fn choose_height(ratio: usize) -> usize {
    let mut height = 0;

    while rand::random::<usize>() % ratio == 0 {
        height += 1;
    }

    height
}
