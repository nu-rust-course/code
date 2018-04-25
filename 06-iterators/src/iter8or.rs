use std::cmp;
use std::mem;

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

    fn max_by_key<B, F>(mut self, mut get_key: F) -> Option<Self::Item>
        where B: Ord,
              F: FnMut(&Self::Item) -> B
    {
        let mut best = None;

        while let Some(item) = self.next() {
            let key = get_key(&item);
            let replace = if let Some((_, ref best_key)) = best {
                key > *best_key
            } else {true};
            if replace {
                best = Some((item, key))
            }
        }

        best.map(|(item, _)| item)
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { next: 0, base: self }
    }

    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
        where U: IntoIter8or<Item = Self::Item>
    {
        Chain(ChainImpl::Both(self, other.into_iter()))
    }

    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
        where U: IntoIter8or
    {
        Zip { left: self, right: other.into_iter() }
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

pub struct Chain<A, B>(ChainImpl<A, B>);

enum ChainImpl<A, B> {
    Both(A, B),
    JustB(B),
    Done,
}

impl<A, B> Iter8or for Chain<A, B>
    where A: Iter8or,
          B: Iter8or<Item = A::Item>
{
    type Item = A::Item;

    fn next(&mut self) -> Option<A::Item> {
        match mem::replace(&mut self.0, ChainImpl::Done) {
            ChainImpl::Both(mut a, mut b) => {
                match a.next() {
                    Some(result) => {
                        self.0 = ChainImpl::Both(a, b);
                        Some(result)
                    }

                    None => {
                        match b.next() {
                            Some(result) => {
                                self.0 = ChainImpl::JustB(b);
                                Some(result)
                            }

                            None => {
                                self.0 = ChainImpl::Done;
                                None
                            }
                        }
                    }
                }
            }

            ChainImpl::JustB(mut b) => {
                match b.next() {
                    Some(result) => Some(result),

                    None => {
                        self.0 = ChainImpl::Done;
                        None
                    }
                }
            }

            ChainImpl::Done => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ChainImpl::Both(ref a, ref b) => {
                let (a_lower, a_upper_option) = a.size_hint();
                let (b_lower, b_upper_option) = b.size_hint();

                let lower = a_lower + b_lower;
                let upper_option = a_upper_option.and_then(|a_upper|
                    b_upper_option.map(|b_upper| a_upper + b_upper));

                (lower, upper_option)

            }

            ChainImpl::JustB(ref b) => b.size_hint(),

            ChainImpl::Done => (0, Some(0)),
        }
    }
}

impl<A, B> ExactSizeIter8or for Chain<A, B>
    where A: ExactSizeIter8or, B: ExactSizeIter8or<Item = A::Item>
{
    fn len(&self) -> usize {
        match self.0 {
            ChainImpl::Both(ref a, ref b) => a.len() + b.len(),
            ChainImpl::JustB(ref b) => b.len(),
            ChainImpl::Done => 0,
        }
    }
}

pub struct Zip<A, B> {
    left: A,
    right: B,
}

impl<A, B> Iter8or for Zip<A, B>
    where A: Iter8or, B: Iter8or
{
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<<Self as Iter8or>::Item> {
        match (self.left.next(), self.right.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper_option) = self.left.size_hint();
        let (b_lower, b_upper_option) = self.right.size_hint();

        let lower = cmp::min(a_lower, b_lower);
        let upper_option = a_upper_option.and_then(|a_upper|
            b_upper_option.map(|b_upper| cmp::min(a_upper, b_upper)));

        (lower, upper_option)
    }
}

impl<A, B> ExactSizeIter8or for Zip<A, B>
    where A: ExactSizeIter8or,
          B: ExactSizeIter8or
{
    fn len(&self) -> usize {
        cmp::min(self.left.len(), self.right.len())
    }
}
