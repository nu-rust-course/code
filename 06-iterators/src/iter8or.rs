pub trait Iter8or {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

pub trait IntoIter8or {
    type Item;
    type IntoIter: Iter8or<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait ExactSizeIter8or : Iter8or {
    fn len(&self) -> usize {
        self.size_hint().0
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Iter8or> IntoIter8or for T {
    type Item = T::Item;
    type IntoIter = T;

    fn into_iter(self) -> <Self as IntoIter8or>::IntoIter {
        self
    }
}

