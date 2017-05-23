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

    fn map<F, I>(self, f: F) -> Map<Self, F>
        where F: FnOnce(Self::Item) -> I,
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

    fn join<B>(self, other: B) -> Join<Self, B>
        where B: Future<Error = Self::Error>,
              Self: Sized
    {
        Join::new(self, other)
    }

    fn select<B>(self, other: B) -> Select<Self, B>
        where B: Future<Item = Self::Item, Error = Self::Error>,
              Self: Sized
    {
        Select::new(self, other)
    }

    fn boxed(self) -> BoxFuture<Self::Item, Self::Error>
        where Self: Sized + Send + 'static
    {
        Box::new(self)
    }

    fn fuse(self) -> Fuse<Self>
        where Self: Sized
    {
        Fuse::new(self)
    }
}

type BoxFuture<I, E> = Box<Future<Item = I, Error = E> + Send>;

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

impl<A, F, I> Future for Map<A, F> where
    A: Future,
    F: FnOnce(A::Item) -> I
{
    type Item = I;
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
          B: Future<Error = A::Error>,
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

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub enum Join<A: Future, B: Future> {
    Running(A, B),
    AReady(A::Item, B),
    BReady(A, B::Item),
    Done,
}

impl<A: Future, B: Future> Join<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Join::Running(a, b)
    }
}

impl<A: Future, B: Future<Error = A::Error>> Future for Join<A, B> {
    type Item = (A::Item, B::Item);
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use Join::*;

        match mem::replace(self, Done) {
            Running(mut a, mut b) => match (a.poll(), b.poll()) {
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
                (Ok(Ready(va)), Ok(Ready(vb))) => Ok(Ready((va, vb))),
                (Ok(NotReady), Ok(Ready(vb))) => {
                    *self = BReady(a, vb);
                    Ok(NotReady)
                }
                (Ok(Ready(va)), Ok(NotReady)) => {
                    *self = AReady(va, b);
                    Ok(NotReady)
                }
                (Ok(NotReady), Ok(NotReady)) => Ok(NotReady),
            },

            AReady(va, mut b) => match b.poll() {
                Err(e) => Err(e),
                Ok(Ready(vb)) => Ok(Ready((va, vb))),
                Ok(NotReady) => {
                   *self = AReady(va, b);
                    Ok(NotReady)
                }
            },

            BReady(mut a, vb) => match a.poll() {
                Err(e) => Err(e),
                Ok(Ready(va)) => Ok(Ready((va, vb))),
                Ok(NotReady) => {
                    *self = BReady(a, vb);
                    Ok(NotReady)
                }
            },

            Done => panic!("cannot poll Join twice"),
        }
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Select<A: Future, B: Future>(Option<(A, B)>);

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub enum SelectNext<A: Future, B: Future> {
    A(A),
    B(B),
}

impl<A: Future, B: Future> Select<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Select(Some((left, right)))
    }
}

impl<A, B> Future for Select<A, B>
    where A: Future,
          B: Future<Item = A::Item, Error = A::Error>
{
    type Item = (A::Item, SelectNext<A, B>);
    type Error = (A::Error, SelectNext<A, B>);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0.take() {
            Some((mut a, mut b)) => match a.poll() {
                Err(e) => Err((e, SelectNext::B(b))),
                Ok(Ready(va)) => Ok(Ready((va, SelectNext::B(b)))),
                Ok(NotReady) => match b.poll() {
                    Err(e) => Err((e, SelectNext::A(a))),
                    Ok(Ready(vb)) => Ok(Ready((vb, SelectNext::A(a)))),
                    Ok(NotReady) => {
                        self.0 = Some((a, b));
                        Ok(NotReady)
                    }
                },
            },

            None => panic!("cannot poll Select twice"),
        }
    }
}

impl<A, B> Future for SelectNext<A, B>
    where A: Future,
          B: Future<Item = A::Item, Error = A::Error>
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self {
            SelectNext::A(ref mut a) => a.poll(),
            SelectNext::B(ref mut b) => b.poll(),
        }
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Fuse<A>(Option<A>);

impl<A: Future> Fuse<A> {
    pub fn new(future: A) -> Self {
        Fuse(Some(future))
    }
}

impl<A: Future> Future for Fuse<A> {
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let result = match self.0 {
            None => return Ok(NotReady),
            Some(ref mut a) => match a.poll() {
                Err(e) => Err(e),
                Ok(NotReady) => return Ok(NotReady),
                Ok(Ready(va)) => Ok(Ready(va)),
            }
        };

        self.0 = None;
        result
    }
}
