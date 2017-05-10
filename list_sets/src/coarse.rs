use super::sequential as seq;

use std::borrow::Borrow;
use std::default::Default;
use std::sync::{Mutex, MutexGuard, atomic};

static LOCK_ORDER: atomic::AtomicUsize = atomic::ATOMIC_USIZE_INIT;

pub struct Set<T> {
    mutex: Mutex<seq::Set<T>>,
    order: usize,
}

impl<T> Set<T> {
    fn lock(&self) -> MutexGuard<seq::Set<T>> {
        self.mutex.lock().expect("Set mutex poisoned")
    }

    fn lock2<'a, 'b, U>(&'a self, other: &'b Set<U>)
            -> (MutexGuard<'a, seq::Set<T>>, MutexGuard<'b, seq::Set<U>>)
    {
        let set1;
        let set2;

        if self.order < other.order {
            set1 = self.lock();
            set2 = other.lock();
        } else {
            set2 = other.lock();
            set1 = self.lock();
        }

        (set1, set2)
    }

    pub fn from_seq(set: seq::Set<T>) -> Self {
        let order = LOCK_ORDER.fetch_add(1, atomic::Ordering::SeqCst);
        Set {
            mutex: Mutex::new(set),
            order: order,
        }
    }

    pub fn into_seq(self) -> seq::Set<T> {
        self.mutex.into_inner().expect("Set mutex poisoned")
    }

    pub fn new() -> Self {
        Self::from_seq(seq::Set::new())
    }

    pub fn is_poisoned(&self) -> bool {
        self.mutex.is_poisoned()
    }

    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }

    pub fn len(&self) -> usize {
        self.lock().len()
    }

    pub fn remove_min(&self) -> Option<T> {
        self.lock().remove_min()
    }

    pub fn drain(&self) -> Drain<T> {
        Drain(self.lock())
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Set::new()
    }
}

impl<T: Ord> Set<T> {
    pub fn contains<Q: ?Sized>(&self, element: &Q) -> bool
        where T: Borrow<Q>, Q: Ord
    {
        self.lock().contains(element)
    }

    pub fn insert(&self, element: T) -> bool {
        self.lock().insert(element)
    }

    pub fn replace(&self, element: T) -> Option<T> {
        self.lock().replace(element)
    }

    pub fn remove<Q: ?Sized>(&self, element: &Q) -> Option<T>
        where T: Borrow<Q>, Q: Ord
    {
        self.lock().remove(element)
    }

    pub fn is_disjoint(&self, other: &Set<T>) -> bool {
        let (set1, set2) = self.lock2(other);
        set1.is_disjoint(&*set2)
    }

    pub fn is_subset(&self, other: &Set<T>) -> bool {
        let (set1, set2) = self.lock2(other);
        set1.is_subset(&*set2)
    }

    pub fn is_superset(&self, other: &Set<T>) -> bool {
        let (set1, set2) = self.lock2(other);
        set1.is_superset(&*set2)
    }
}

impl<T: Ord + Clone> Set<T> {
    pub fn intersection(&self, other: &Set<T>) -> Self {
        let (set1, set2) = self.lock2(other);
        Self::from_seq(set1.intersection(&*set2))
    }

    pub fn union(&self, other: &Set<T>) -> Self {
        let (set1, set2) = self.lock2(other);
        Self::from_seq(set1.union(&*set2))
    }

    pub fn difference(&self, other: &Set<T>) -> Self {
        let (set1, set2) = self.lock2(other);
        Self::from_seq(set1.difference(&*set2))
    }

    pub fn symmetric_difference(&self, other: &Set<T>) -> Self {
        let (set1, set2) = self.lock2(other);
        Self::from_seq(set1.symmetric_difference(&*set2))
    }
}

pub type IntoIter<T> = seq::IntoIter<T>;

impl<'a, T> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.into_seq().into_iter()
    }
}

pub struct Drain<'a, T: 'a>(MutexGuard<'a, seq::Set<T>>);

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.remove_min()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}

impl<'a, T> ExactSizeIterator for Drain<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
