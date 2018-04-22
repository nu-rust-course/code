use std::ops::IndexMut;

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

/// We might try to implement an iterator that allows mutating
/// the underlying slice, but it turns out no matter what we do,
/// there's a borrowing problem.
pub struct SliceIterMut<'a, T: 'a> {
    slice: &'a mut [T],
    start: usize,
    limit: usize
}

impl<'a, T> SliceIterMut<'a, T> {
    pub fn of_slice(slice: &'a mut [T]) -> Self {
        let limit = slice.len();
        SliceIterMut { slice, start: 0, limit, }
    }

    pub fn of_vec(vec: &'a mut Vec<T>) -> Self {
        SliceIterMut::of_slice(vec.as_mut_slice())
    }
}

impl<'a, T> Iterator for SliceIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.start < self.limit {
            let result = self.slice.index_mut(self.start);
            self.start += 1;
            Some(result);
            None // nooooooo
        } else {None}
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> ExactSizeIterator for SliceIterMut<'a, T> {
    fn len(&self) -> usize {
        self.limit - self.start
    }
}


