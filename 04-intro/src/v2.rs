//! 2-vectors of `f64`s. Not to be confused with `std::vec::Vec`, these
//! are pairs representing 2-D vectors.

/// A 2-vector of `f64`s.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

