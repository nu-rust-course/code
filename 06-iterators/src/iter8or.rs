use std::cmp;
use std::mem;

pub trait Iter8or {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(mut self)-> usize
        where Self: Sized
    {
        let mut result = 0;

        while let Some(_) = self.next() {
            result += 1;
        }

        result
    }

    fn last(mut self) -> Option<Self::Item>
        where Self: Sized
    {
        let mut result = None;

        while let Some(item) = self.next() {
            result = Some(item);
        }

        result
    }

    fn nth(mut self, mut n: usize) -> Option<Self::Item>
        where Self: Sized
    {
        while n > 0 {
            if self.next().is_none() { return None; }
            n -= 1;
        }

        self.next()
    }

    fn by_ref(&mut self) -> &mut Self {
        self
    }

    fn collect<T: FromIter8or<Self::Item>>(self) -> T
        where Self: Sized
    {
        T::from_iter(self)
    }

    fn map<B, F: FnMut(Self::Item) -> B>(self, fun: F) -> Map<Self, F>
        where Self: Sized
    {
        Map {
            base: self,
            fun
        }
    }

    fn filter<P: FnMut(&Self::Item) -> bool>(self, pred: P) -> Filter<Self, P>
        where Self: Sized
    {
        Filter {
            base: self,
            pred
        }
    }

    fn any<P: FnMut(Self::Item) -> bool>(self, pred: P) -> bool
        where Self: Sized
    {
        self.map(pred).filter(|&b| b).next().is_some()
    }

    fn max_by_key<B, F>(mut self, mut get_key: F) -> Option<Self::Item>
        where B: Ord,
              F: FnMut(&Self::Item) -> B,
              Self: Sized
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

    fn enumerate(self) -> Enumerate<Self>
        where Self: Sized
    {
        Enumerate { next: 0, base: self }
    }

    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
        where U: IntoIter8or<Item = Self::Item>,
              U::IntoIter: Sized,
              Self: Sized
    {
        Chain(ChainImpl::Both(self, other.into_iter8or()))
    }

    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
        where U: IntoIter8or,
              U::IntoIter: Sized,
              Self: Sized
    {
        Zip { left: self, right: other.into_iter8or() }
    }

    fn filter_map<F, B>(self, fun: F) -> FilterMap<Self, F>
        where F: FnMut(Self::Item) -> Option<B>,
              Self: Sized
    {
        FilterMap { base: self, fun }
    }

    fn flat_map<F, U>(self, fun: F) -> FlatMap<Self, F, U>
        where F: FnMut(Self::Item) -> U,
              U: IntoIter8or,
              Self: Sized,
    {
        FlatMap {
            base: self,
            fun,
            buf: None,
        }
    }

    fn peekable(self) -> Peek<Self>
        where Self: Sized
    {
        Peek {
            base: self,
            next: None,
        }
    }
}

pub trait IntoIter8or {
    type Item;
    type IntoIter: Iter8or<Item = Self::Item>;

    fn into_iter8or(self) -> Self::IntoIter;
}

pub trait FromIter8or<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIter8or<Item = T>;
}

pub trait Xtend<T> {
    fn xtend<I: IntoIter8or<Item=T>>(&mut self, iter: I);
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

    fn into_iter8or(self) -> <Self as IntoIter8or>::IntoIter {
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
        (0, self.base.size_hint().1)
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

pub struct FilterMap<I, F> {
    base: I,
    fun: F,
}

impl<I, F, B> Iter8or for FilterMap<I, F>
    where I: Iter8or,
          F: FnMut(I::Item) -> Option<B>
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        while let Some(item) = self.base.next() {
            if let Some(item) = (self.fun)(item) {
                return Some(item);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.base.size_hint().1)
    }
}

pub struct FlatMap<I, F, U: IntoIter8or> {
    base: I,
    fun: F,
    buf: Option<U::IntoIter>
}

impl <I, F, U> Iter8or for FlatMap<I, F, U>
    where I: Iter8or,
          F: FnMut(I::Item) -> U,
          U: IntoIter8or
{
    type Item = U::Item;

    fn next(&mut self) -> Option<U::Item> {
        loop {
            match self.buf.take() {
                None => {
                    match self.base.next() {
                        None => {
                            return None;
                        }
                        Some(inner) => {
                            self.buf = Some((self.fun)(inner).into_iter8or());
                            continue;
                        }
                    }
                }

                Some(mut inner) => {
                    match inner.next() {
                        None => {
                            self.buf = Some(inner);
                            continue;
                        }
                        Some(item) => {
                            self.buf = Some(inner);
                            return Some(item);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Peek<I: Iter8or> {
    base: I,
    next: Option<I::Item>,
}

impl<I: Iter8or> Peek<I> {
    pub fn peek(&mut self) -> Option<&I::Item> {
        if let Some(ref item) = self.next {
            Some(item)
        } else {
            self.next = self.base.next();
            self.next.as_ref()
        }
    }
}

impl<I: Iter8or> Iter8or for Peek<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<<Self as Iter8or>::Item> {
        self.next.take().or_else(|| self.base.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let extra = if self.next.is_some() {1} else {0};
        let (low, option_high) = self.base.size_hint();
        (low + extra, option_high.map(|high| high + extra))
    }
}

impl<I: ExactSizeIter8or> ExactSizeIter8or for Peek<I> {
    fn len(&self) -> usize {
        self.base.len() + if self.next.is_some() {1} else {0}
    }
}

impl<'a, T: Iter8or> Iter8or for &'a mut T {
    type Item = T::Item;

    fn next(&mut self) -> Option<<Self as Iter8or>::Item> {
        (*self).next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (*self as &T).size_hint()
    }
}

impl<T, E, C> FromIter8or<Result<T, E>> for Result<C, E>
    where C: FromIter8or<T>
{
    fn from_iter<I: IntoIter8or<Item=Result<T, E>>>(iter: I) -> Self {
        struct Adapter<I, E> {
            iter: I,
            err:  Option<E>,
        }

        impl<T, E, I: Iter8or<Item=Result<T, E>>> Iter8or for Adapter<I, E> {
            type Item = T;

            fn next(&mut self) -> Option<T> {
                match self.iter.next() {
                    Some(Ok(value)) => Some(value),
                    Some(Err(err)) => {
                        self.err = Some(err);
                        None
                    }
                    None => None,
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, self.iter.size_hint().1)
            }
        }

        let mut adapter = Adapter { iter: iter.into_iter8or(), err: None };
        let container: C = FromIter8or::from_iter(adapter.by_ref());

        match adapter.err {
            Some(err) => Err(err),
            None      => Ok(container),
        }
    }
}
