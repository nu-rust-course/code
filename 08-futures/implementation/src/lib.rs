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

/// The result of asking a future about the state of its computation:
///
///  - `Ok(Ready(v))` means it's finished and `v` is the answer.
///  - `Ok(NotReady)` means it's still working.
///  - `Err(e)` means it errored out.
///
pub type Poll<I, E> = Result<Async<I>, E>;

