//! 2-vectors of `f64`s. Not to be confused with `std::vec::Vec`, these
//! are pairs representing 2-D vectors.

use std::default;
use std::fmt;
use std::ops::{Add, Mul};

/// A 2-vector of `f64`s.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> Self {
        V2 { x, y, }
    }

    pub fn origin() -> Self {
        V2::new(0., 0.)
    }

    pub fn inner_product(&self, other: &V2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        self.inner_product(self).sqrt()
    }

    pub fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl Default for V2 {
    fn default() -> Self {
        V2::origin()
    }
}

impl fmt::Display for V2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<V2> for V2 {
    type Output = V2;

    fn add(self, rhs: V2) -> Self::Output {
        V2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&V2> for V2 {
    type Output = V2;

    fn add(self, rhs: &V2) -> Self::Output {
        self + *rhs
    }
}

impl Mul<f64> for V2 {
    type Output = V2;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.scale(rhs);
        self
    }
}

impl Mul<V2> for f64 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        rhs * self
    }
}

