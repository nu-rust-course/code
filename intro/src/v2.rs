//! 2-vectors of `f64`s. Not to be confused with `std::vec::Vec`, these
//! are pairs representing 2-D vectors.

use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, Mul, Neg, Sub};

/// A 2-vector of `f64`s.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    /// Constructs a new `V2`.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::v2::*;
    /// let v = V2::new(2., 3.);
    /// assert_eq!(2., v.x);
    /// assert_eq!(3., v.y);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        V2 { x: x, y: y, }
    }

    /// Returns the origin.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::v2::*;
    /// let v = V2::origin();
    /// assert_eq!(0., v.x);
    /// assert_eq!(0., v.y);
    /// ```
    pub fn origin() -> Self {
        V2::new(0., 0.)
    }

    /// Multiplies the vector by a scalar.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::v2::*;
    /// let v = V2::new(3., 4.);
    /// let u = V2::new(6., 8.);
    /// assert_eq!(u, v.scale(2.));
    /// ```
    pub fn scale(&self, factor: f64) -> Self {
        V2 {
            x: factor * self.x,
            y: factor * self.y,
        }
    }

    /// Computes the inner produce (dot product) of two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::v2::*;
    /// let v = V2::new(1., 10.);
    /// let u = V2::new(2.,  4.);
    /// assert_eq!(42., v.inner_product(&u));
    /// ```
    pub fn inner_product(&self, other: &V2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Finds the magnitude of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::v2::*;
    /// let v = V2::new(3., 4.);
    /// assert_eq!(5., v.magnitude());
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.inner_product(self).sqrt()
    }
}

impl Display for V2 {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "⟨{}, {}⟩", self.x, self.y)
    }
}

#[test]
fn test_display() {
    let v = V2::new(3., 4.);
    assert_eq!("⟨3, 4⟩", format!("{}", &v));
}

impl Neg for V2 {
    /// The result of negating a vector is a vector.
    type Output = V2;

    /// Negates a vector.
    fn neg(self) -> V2 {
        V2::new(-self.x, -self.y)
    }
}

impl<'a> Neg for &'a V2 {
    /// The result of negating a vector is a vector.
    type Output = V2;

    /// Negates a vector.
    fn neg(self) -> V2 {
        -*self
    }
}

impl Add<V2> for V2 {
    /// The result of adding two vectors is a vector.
    type Output = V2;

    /// Adds two vectors.
    fn add(self, other: V2) -> V2 {
        V2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a> Add<&'a V2> for V2 {
    /// The result of adding two vectors is a vector.
    type Output = V2;

    /// Adds two vectors.
    fn add(self, other: &'a V2) -> V2 {
        self + *other
    }
}

impl<'a> Add<V2> for &'a V2 {
    /// The result of adding two vectors is a vector.
    type Output = V2;

    /// Adds two vectors.
    fn add(self, other: V2) -> V2 {
        *self + other
    }
}

impl<'a, 'b> Add<&'b V2> for &'a V2 {
    /// The result of adding two vectors is a vector.
    type Output = V2;

    /// Adds two vectors.
    fn add(self, other: &'b V2) -> V2 {
        *self + *other
    }
}

#[test]
fn add_test() {
    let u = V2::new(1., 2.);
    let v = V2::new(10., 20.);
    let w = V2::new(11., 22.);

    assert_eq!(w, u + v);
    assert_eq!(w, u + &v);
    assert_eq!(w, &u + v);
    assert_eq!(w, &u + &v);
}

impl Sub<V2> for V2 {
    /// The result of subtracting two vectors is a vector.
    type Output = V2;

    /// Subtracts two vectors.
    fn sub(self, other: V2) -> V2 {
        self + -other
    }
}

impl<'a> Sub<V2> for &'a V2 {
    /// The result of subtracting two vectors is a vector.
    type Output = V2;

    /// Subtracts two vectors.
    fn sub(self, other: V2) -> V2 {
        self + -other
    }
}

impl<'a, 'b> Sub<&'b V2> for &'a V2 {
    /// The result of subtracting two vectors is a vector.
    type Output = V2;

    /// Subtracts two vectors.
    fn sub(self, other: &'b V2) -> V2 {
        self + -other
    }
}

impl<'a> Sub<&'a V2> for V2 {
    /// The result of subtracting two vectors is a vector.
    type Output = V2;

    /// Subtracts two vectors.
    fn sub(self, other: &'a V2) -> V2 {
        self + -other
    }
}

impl Mul<V2> for f64 {
    /// The result of multiplying a vector by a scalar.
    type Output = V2;

    /// Multiplies a vector by a scalar.
    fn mul(self, other: V2) -> V2 {
        V2::new(self * other.x, self * other.y)
    }
}

impl<'a> Mul<V2> for &'a f64 {
    /// The result of multiplying a vector by a scalar.
    type Output = V2;

    /// Multiplies a vector by a scalar.
    fn mul(self, other: V2) -> V2 {
        *self * other
    }
}

impl<'a> Mul<&'a V2> for f64 {
    /// The result of multiplying a vector by a scalar.
    type Output = V2;

    /// Multiplies a vector by a scalar.
    fn mul(self, other: &'a V2) -> V2 {
        self * *other
    }
}

impl<'a, 'b> Mul<&'b V2> for &'a f64 {
    /// The result of multiplying a vector by a scalar.
    type Output = V2;

    /// Multiplies a vector by a scalar.
    fn mul(self, other: &'b V2) -> V2 {
        *self * *other
    }
}

#[test]
fn mul_test() {
    let v = V2::new(3., 4.);
    let u = V2::new(6., 8.);
    assert_eq!(u, 2. * v);
}
