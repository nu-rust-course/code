use std::mem;
use super::iter8or::*;

/// First, a Vector iterator. We're make a read-only, by-reference
/// iterator, which is the default (and the only one we can do without
/// special knowledge of `Vec`'s implementation. So we store a reference
/// to a vector and the position of the next element to return.
pub struct VecIter<'a, T: 'a> {
    base: &'a Vec<T>,
    pos:  usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pos < self.base.len() {
            let result = &self.base[self.pos];
            self.pos += 1;
            Some(result)
        } else {None}
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Uses `ExactSizeIterator` from below.
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> ExactSizeIterator for VecIter<'a, T> {
    fn len(&self) -> usize {
        self.base.len() - self.pos
    }
}

/// What if we want to implement `DoubleEndedIterator` for `VecIter`?
/// Well, we would have to add another field. But wait a minute.
/// Remember how a reference to a vector isn't usually a useful type,
/// and we'd usually use a slice instead? Well, a slice already supports
/// double-ended iteration! Watch:
pub struct SliceIter<'a, T: 'a>(&'a [T]);

impl<'a, T> SliceIter<'a, T> {
    pub fn of_slice(slice: &'a [T]) -> Self {
        SliceIter(slice)
    }

    /// Of course, we don't need `VecIter` at all, because `SliceIter` is
    /// strictly more general.
    pub fn of_vec(vec: &'a Vec<T>) -> Self {
        SliceIter(vec.as_slice())
    }
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.0.split_first().map(|(result, rest)| {
            self.0 = rest;
            result
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> ExactSizeIterator for SliceIter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> DoubleEndedIterator for SliceIter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.0.split_last().map(|(result, rest)| {
            self.0 = rest;
            result
        })
    }
}

pub struct SliceIterMut<'a, T: 'a>(&'a mut [T]);

impl<'a, T> SliceIterMut<'a, T> {
    pub fn of_slice(slice: &'a mut [T]) -> Self {
        SliceIterMut(slice)
    }

    pub fn of_vec(vec: &'a mut Vec<T>) -> Self {
        SliceIterMut::of_slice(vec.as_mut_slice())
    }
}

impl<'a, T> Iterator for SliceIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let tmp = mem::replace(&mut self.0, &mut []);
        tmp.split_first_mut().map(|(first, rest)| {
            self.0 = rest;
            first
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> ExactSizeIterator for SliceIterMut<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> DoubleEndedIterator for SliceIterMut<'a, T> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        let tmp = mem::replace(&mut self.0, &mut []);
        tmp.split_last_mut().map(|(last, rest)| {
            self.0 = rest;
            last
        })
    }
}
