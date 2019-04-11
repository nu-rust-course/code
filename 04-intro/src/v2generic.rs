//! Generic 2-vectors. Not to be confused with `std::vec::Vec`, these
//! are pairs representing 2-D vectors.

/// A 2-vector of `f64`s.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct V2<Coord> {
    pub x: Coord,
    pub y: Coord,
}

