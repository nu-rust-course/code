#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate crossbeam;

pub mod sequential;
pub mod coarse;
pub mod lock_free;
