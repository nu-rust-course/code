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
        Map {
            future: self,
            fun:    Some(f),
        }
    }

    fn and_then<F, B>(self, f: F) -> AndThen<Self, B, F>
        where F: FnOnce(Self::Item) -> B,
              B: IntoFuture,
              Self: Sized
    {
        AndThen::First(self, f)
    }

    fn then<F, B>(self, f: F) -> Then<Self, B, F>
        where F: FnOnce(Result<Self::Item, Self::Error>) -> B,
              B: IntoFuture,
              Self: Sized
    {
        Then::First(self, f)
    }

    fn join<B>(self, other: B) -> Join<Self, B::Future>
        where B: IntoFuture<Error = Self::Error>,
              Self: Sized
    {
        Join::Running(self, other.into_future())
    }

    fn select<B>(self, other: B) -> Select<Self, B::Future>
        where B: IntoFuture<Item = Self::Item, Error = Self::Error>,
              Self: Sized
    {
        Select(Some((self, other.into_future())))
    }

    fn boxed(self) -> BoxFuture<Self::Item, Self::Error>
        where Self: Sized + Send + 'static
    {
        Box::new(self)
    }

    fn fuse(self) -> Fuse<Self>
        where Self: Sized
    {
        Fuse(Some(self))
    }
}

pub trait IntoFuture {
    type Item;
    type Error;
    type Future: Future<Item = Self::Item, Error = Self::Error>;

    fn into_future(self) -> Self::Future;
}

impl<F: Future> IntoFuture for F {
    type Item = F::Item;
    type Error = F::Error;
    type Future = F;

    fn into_future(self) -> Self::Future {
        self
    }
}

type BoxFuture<I, E> = Box<Future<Item = I, Error = E> + Send>;

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Map<A: Future, F> {
    future: A,
    fun: Option<F>,
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
pub enum AndThen<A: Future, B: IntoFuture, F> {
    First(A, F),
    Second(B::Future),
    Done,
}

impl<A, B, F> Future for AndThen<A, B, F>
    where A: Future,
          B: IntoFuture<Error = A::Error>,
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
                let mut b = f(va).into_future();
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
pub enum Then<A: Future, B: IntoFuture, F> {
    First(A, F),
    Second(B::Future),
    Done,
}

impl<A, B, F> Future for Then<A, B, F>
    where A: Future,
          B: IntoFuture,
          F: FnOnce(Result<A::Item, A::Error>) -> B
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use Then::*;

        let ra = match self {
            First(a, _) => {
                match a.poll() {
                    Err(e)        => Err(e),
                    Ok(NotReady)  => return Ok(NotReady),
                    Ok(Ready(va)) => Ok(va),
                }
            }
            Second(b)   => return b.poll(),
            Done        => panic!("cannot poll AndThen twice"),
        };

        match mem::replace(self, Done) {
            First(_, f) => {
                let mut b = f(ra).into_future();
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
