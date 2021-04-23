//
// Examples first!
//

#[cfg(test)]
mod tests {
    use super::{cons, List};

    #[test]
    fn build_and_access() {
        let mut xs = List::new();
        xs.cons(5).cons(6).cons(7);

        assert_eq!(xs.len(), 3);
        assert_eq!(xs.first_copied(), Some(7));
        assert_eq!(xs.nth_copied(1), Some(6));
        assert_eq!(xs.nth_copied(2), Some(5));

        assert!(!xs.nth_rest(2).unwrap().empty());
        assert!(xs.nth_rest(3).is_some());
        assert!(xs.nth_rest(3).unwrap().empty());
        assert!(xs.nth_rest(4).is_none());
    }

    #[test]
    fn sharing() {
        let mut xs = List::new();
        xs.cons(5).cons(6).cons(7);
        let ys = cons(4, xs.clone());
        assert_eq!(xs.len(), 3);
        assert_eq!(ys.len(), 4);

        xs.cons(4).cons(3);
        assert_eq!(xs.len(), 5);
        assert_eq!(ys.len(), 4);
    }

    #[test]
    fn mutation() {
        let mut xs = List::new();
        xs.cons(5).cons(6).cons(7);

        assert_eq!(xs.nth_copied(0), Some(7));
        assert_eq!(xs.nth_copied(1), Some(6));
        assert_eq!(xs.nth_copied(2), Some(5));
        assert!(xs.nth_copied(3).is_none());

        xs.map_nth_mut(1, |o| o.map(|r| *r = 12));
        assert_eq!(xs.nth_copied(0), Some(7));
        assert_eq!(xs.nth_copied(1), Some(12));
        assert_eq!(xs.nth_copied(2), Some(5));
        assert!(xs.nth_copied(3).is_none());

        xs.rest().unwrap().rest_mut().unwrap().cons(55);
        assert_eq!(xs.nth_copied(0), Some(7));
        assert_eq!(xs.nth_copied(1), Some(12));
        assert_eq!(xs.nth_copied(2), Some(55));
        assert_eq!(xs.nth_copied(3), Some(5));
        assert!(xs.nth_copied(4).is_none());
    }
}

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

//
// Data definitions
//

/// A list is an optional, shared pointer to a mutable node:
#[derive(Debug)]
pub struct List<T>(Option<Rc<RefCell<Node<T>>>>);

/// A node has an element and another list:
#[derive(Debug)]
struct Node<T> {
    first: T,
    rest: List<T>,
}

//
// Free function
//

/// Creates a longer list from an element and an additional list.
pub fn cons<T>(first: T, rest: List<T>) -> List<T> {
    let node = Node { first, rest };
    List(Some(Rc::new(RefCell::new(node))))
}

//
// Inherent method imps
//

impl<T> List<T> {
    /// Constructs an empty list.
    pub fn new() -> Self {
        List(None)
    }

    /// Modifies a list by adding a new element to the front of it.
    pub fn cons(&mut self, first: T) -> &mut Self {
        *self = cons(first, self.clone());
        self
    }

    /// Checks whether a list is empty.
    pub fn empty(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the length of a list.
    pub fn len(&self) -> usize {
        let mut count = 0;

        let mut current = self.clone();
        while let Some(next) = current.rest() {
            current = next;
            count += 1;
        }

        count
    }

    /// Returns a guarded reference to the first element of a list.
    pub fn first(&self) -> Option<Ref<T>> {
        self.map_ref(|node| &node.first)
    }

    /// Returns the rest of a list.
    pub fn rest(&self) -> Option<List<T>> {
        self.0.as_ref().map(|rc| rc.borrow().rest.clone())
    }

    /// Returns a guarded *mutable* reference to the first element of a
    /// list.
    pub fn first_mut(&self) -> Option<RefMut<T>> {
        self.map_ref_mut(|node| &mut node.first)
    }

    /// Returns a guarded *mutable* reference to the rest of a list.
    pub fn rest_mut(&self) -> Option<RefMut<List<T>>> {
        self.map_ref_mut(|node| &mut node.rest)
    }

    /// Returns a guarded reference to the rest of a list. Usually
    /// [`List::rest`] is easier to use, but this method avoids a
    /// reference count bump.
    pub fn rest_ref(&self) -> Option<Ref<List<T>>> {
        self.map_ref(|node| &node.rest)
    }

    /// Applies the given function to an option containing a reference
    /// to the first element, returning the function’s result.
    pub fn map_first<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R,
    {
        if let Some(rc) = self.0.as_ref() {
            f(Some(&rc.borrow().first))
        } else {
            f(None)
        }
    }

    /// Applies the given function to an option containing a *mutable*
    /// reference to the first element, returning the function’s result.
    pub fn map_first_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&mut T>) -> R,
    {
        if let Some(rc) = self.0.as_ref() {
            f(Some(&mut rc.borrow_mut().first))
        } else {
            f(None)
        }
    }

    /// Applies the given function to an option containing a reference
    /// to the `n`th element (if it exists), returning the function’s result.
    pub fn map_nth<F, R>(&self, n: usize, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R,
    {
        if let Some(lst) = self.nth_rest(n) {
            lst.map_first(f)
        } else {
            f(None)
        }
    }

    /// Returns the `n`th `rest` of a list (if it exists), like calling
    /// [`List::rest`] `n` times.
    pub fn nth_rest(&self, n: usize) -> Option<List<T>> {
        let mut current = self.clone();
        for _ in 0..n {
            current = current.rest()?;
        }
        Some(current)
    }

    /// Applies the given function to an option containing a *mutable*
    /// reference to the `n`th element (if it exists), returning the
    /// function’s result.
    pub fn map_nth_mut<F, R>(&self, n: usize, f: F) -> R
    where
        F: FnOnce(Option<&mut T>) -> R,
    {
        if let Some(lst) = self.nth_rest(n) {
            lst.map_first_mut(f)
        } else {
            f(None)
        }
    }

    // Helper
    fn map_ref<U>(&self, f: impl FnOnce(&Node<T>) -> &U) -> Option<Ref<U>> {
        self.0.as_ref().map(|rc| Ref::map(rc.borrow(), f))
    }

    // Helper
    fn map_ref_mut<U>(&self, f: impl FnOnce(&mut Node<T>) -> &mut U) -> Option<RefMut<U>> {
        self.0.as_ref().map(|rc| RefMut::map(rc.borrow_mut(), f))
    }
}

// Methods for `Copy` elements.
impl<T: Copy> List<T> {
    /// Returns a copy of the first element, if it is copyable.
    pub fn first_copied(&self) -> Option<T> {
        self.first().map(|r| *r)
    }

    /// Returns a copy of the `n`th element if it exists.
    pub fn nth_copied(&self, n: usize) -> Option<T> {
        self.nth_rest(n)?.first_copied()
    }
}

// Methods for `Clone` elements.
impl<T: Clone> List<T> {
    /// Returns a clone of the first element, if it is cloneable.
    pub fn first_cloned(&self) -> Option<T> {
        self.first().map(|r| T::clone(&r))
    }

    /// Returns a clone of the `n`th element if it exists.
    pub fn nth_cloned(&self, n: usize) -> Option<T> {
        self.nth_rest(n)?.first_cloned()
    }
}

//
// Trait impls
//

impl<T> Clone for List<T> {
    /// A list is cloned by cloning the shared pointer inside, which
    /// means that the original and the result share their structure.
    fn clone(&self) -> Self {
        List(self.0.clone())
    }
}

impl<T> std::default::Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}
