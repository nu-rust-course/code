//! A futures represents an asynchronous computation that eventually
//! will complete and produce a value.

use std::mem;

/// Type to indicate readyness or not.
#[derive(Debug)]
pub enum Async<Item> {
    /// The value is ready.
    Ready(Item),
    /// The computation is ongoing.
    NotReady,
}

use Async::*;
use std::net::Shutdown::Read;

/// The result of asking a future about the state of its computation:
///
///  - `Ok(Ready(v))` means it's finished and `v` is the answer.
///  - `Ok(NotReady)` means it's still working.
///  - `Err(e)` means it errored out.
///
pub type Poll<I, E> = Result<Async<I>, E>;

pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

    fn map<F, I>(self, f: F) -> Map<Self, F>
    where F: FnOnce(Self::Item) -> I,
          Self: Sized {

        Map { future: self, mapper: Some(f) }
    }
}

pub struct Map<A, F> {
    future: A,
    mapper: Option<F>,
}

impl<A, F, I> Future for Map<A, F>
where A: Future,
      F: FnOnce(A::Item) -> I
{
    type Item = I;
    type Error = A::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.future.poll() {
            Err(e)       => Err(e),
            Ok(NotReady) => Ok(NotReady),
            Ok(Ready(v)) => {
                let f = self.mapper.take().expect("future already complete")
                Ok(Ready(f(v)))
            }
        }
    }
}



mod mio {
    pub struct File { }
    pub struct Error { }

    impl File {
        pub fn new() -> Self { File { } }
        pub fn start_read(&mut self, count: usize) { }
        pub fn try_finish_read(&mut self) -> Result<Option<String>, Error> { None }
    }
}

pub struct ReadOperation { file: Option<mio::File>, count: usize, sparked: bool, }

fn async_read(file: mio::File, count: usize) -> ReadOperation {
    ReadOperation { file: Some(file), count, sparked: false, }
}

impl Future for ReadOperation {
    type Item  = String;
    type Error = mio::Error;

    fn poll(&mut self) -> Result<Async<(Self::Item, mio::File)>, Self::Error> {
        let file = self.file.as_mut().expect("future is finished");

        if !self.sparked {
            self.file.start_read(self.count);
            self.sparked = true;
        }

        self.file.try_finish_read().map(|str_opt|
            match str_opt {
                None => NotReady,
                Some(s) => Ready((s, self.file.take()))
            }
        )
    }
}
