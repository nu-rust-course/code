use std::mem;

#[derive(Debug)]
pub enum Async<Item> {
    Ready(Item),
    NotReady,
}

use Async::*;

pub type Poll<I, E> = Result<Async<I>, E>;

pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

    fn map<F, J>(self, f: F) -> Map<Self, F>
        where F: FnOnce(Self::Item) -> J,
              Self: Sized
    {
        Map::new(self, f)
    }

    fn and_then<F, A>(self, f: F) -> AndThen<Self, A, F>
        where F: FnOnce(Self::Item) -> A,
              A: Future,
              Self: Sized
    {
        AndThen::new(self, f)
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Map<A: Future, F> {
    future: A,
    fun: Option<F>,
}

impl<A: Future, F> Map<A, F> {
    pub fn new(future: A, fun: F) -> Self {
        Map {
            future: future,
            fun: Some(fun),
        }
    }
}

impl<A, F, J> Future for Map<A, F> where
    A: Future,
    F: FnOnce(A::Item) -> J
{
    type Item = J;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.future.poll() {
            Err(e) => Err(e),
            Ok(NotReady) => Ok(NotReady),
            Ok(Ready(v)) => {
                let fun = self.fun.take().expect("cannot poll Map twice");
                Ok(Ready(fun(v)))
            }
        }
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub enum AndThen<A: Future, B: Future, F> {
    First(A, F),
    Second(B),
    Done,
}

impl<A: Future, B: Future, F> AndThen<A, B, F> {
    pub fn new(a: A, f: F) -> Self {
        AndThen::First(a, f)
    }
}

impl<A, B, F> Future for AndThen<A, B, F>
    where A: Future,
          B: Future<Error=A::Error>,
          F: FnOnce(A::Item) -> B
{
    type Item = B::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use AndThen::*;

        let va = match *self {
            First(ref mut a, _) => {
                match a.poll() {
                    Err(e) => return Err(e),
                    Ok(NotReady) => return Ok(NotReady),
                    Ok(Ready(va)) => va,
                }
            }
            Second(ref mut b) => return b.poll(),
            Done => panic!("cannot poll AndThen twice"),
        };

        match mem::replace(self, Done) {
            First(_, f) => {
                let mut b = f(va);
                let result = b.poll();
                *self = Second(b);
                result
            }
            _ => unreachable!(),
        }
    }
}
