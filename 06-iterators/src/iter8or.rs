pub trait Iter8or: Sized {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(mut self) -> usize {
        let mut result = 0;

        while let Some(_) = self.next() {
            result += 1;
        }

        result
    }

    fn last(mut self) -> Option<Self::Item> {
        let mut result = None;

        while let Some(item) = self.next() {
            result = Some(item);
        }

        result
    }

    fn nth(mut self, mut n: usize) -> Option<Self::Item> {
        while n > 0 {
            if self.next().is_none() { return None; }
            n -= 1;
        }

        self.next()
    }

    fn map<B, F: FnMut(Self::Item) -> B>(self, fun: F) -> Map<Self, F> {
        Map {
            base: self,
            fun
        }
    }

    fn filter<P: FnMut(&Self::Item) -> bool>(self, pred: P) -> Filter<Self, P> {
        Filter {
            base: self,
            pred
        }
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { next: 0, base: self }
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

pub trait DoubleEndedIter8or : Iter8or {
    fn next_back(&mut self) -> Option<Self::Item>;
}

impl<T: Iter8or> IntoIter8or for T {
    type Item = T::Item;
    type IntoIter = T;

    fn into_iter(self) -> <Self as IntoIter8or>::IntoIter {
        self
    }
}

pub struct Map<I, F> {
    base: I,
    fun: F,
}

impl<I, F, B> Iter8or for Map<I, F>
    where I: Iter8or,
          F: FnMut(I::Item) -> B
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.base.next().map(&mut self.fun)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<I, F, B> ExactSizeIter8or for Map<I, F>
    where I: ExactSizeIter8or,
          F: FnMut(I::Item) -> B
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<I, F, B> DoubleEndedIter8or for Map<I, F>
    where I: DoubleEndedIter8or,
          F: FnMut(I::Item) -> B
{
    fn next_back(&mut self) -> Option<B> {
        self.base.next_back().map(&mut self.fun)
    }
}

pub struct Filter<I, P> {
    base: I,
    pred: P,
}

impl<I, P> Iter8or for Filter<I, P>
    where I: Iter8or,
          P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.base.next() {
            if (self.pred)(&result) {
                return Some(result);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_lower, upper) = self.base.size_hint();
        (0, upper)
    }
}

impl<I, P> DoubleEndedIter8or for Filter<I, P>
    where I: DoubleEndedIter8or,
          P: FnMut(&I::Item) -> bool
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.base.next_back() {
            if (self.pred)(&result) {
                return Some(result);
            }
        }

        None
    }
}

pub struct Enumerate<I> {
    next: usize,
    base: I,
}

impl<I> Iter8or for Enumerate<I>
    where I: Iter8or
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<<Self as Iter8or>::Item> {
        self.base.next().map(|item| {
            let next = self.next;
            self.next += 1;
            (next, item)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<I> ExactSizeIter8or for Enumerate<I>
    where I: ExactSizeIter8or
{
    fn len(&self) -> usize {
        self.base.len()
    }
}
