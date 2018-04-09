//! Generic 2-vectors. Not to be confused with `std::vec::Vec`, these
//! are pairs representing 2-D vectors.

use std::default::Default;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// A 2-vector of `f64`s.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct V2<Coord> {
    pub x: Coord,
    pub y: Coord,
}

impl<T> V2<T> {
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
    pub fn new(x: T, y: T) -> Self {
        V2 { x, y, }
    }
}

impl<T: Copy + Mul<Output=T>> V2<T> {
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
    pub fn scale(&self, factor: T) -> V2<T> {
        V2 {
            x: factor * self.x,
            y: factor * self.y,
        }
    }
}

impl<T: Copy + Mul<Output=T> + Add<Output=T>> V2<T> {
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
    pub fn inner_product(&self, other: &V2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Copy + Default> Default for V2<T> {
    fn default() -> Self {
        V2::new(T::default(), T::default())
    }
}

#[test]
fn test_default() {
    let v: V2<usize> = V2::default();
    assert_eq!(V2::new(0, 0), v);
}

impl<T: fmt::Display> fmt::Display for V2<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "⟨{}, {}⟩", self.x, self.y)
    }
}

#[test]
fn test_display() {
    let v = V2::new(3, 4);
    assert_eq!("⟨3, 4⟩", format!("{}", &v));
}

impl<T: Neg> Neg for V2<T> {
    /// The result of negating a vector is a vector.
    type Output = V2<T::Output>;

    /// Negates a vector.
    fn neg(self) -> V2<T::Output> {
        V2::new(-self.x, -self.y)
    }
}

impl<'a, T: Neg + Clone> Neg for &'a V2<T> {
    /// The result of negating a vector is a vector.
    type Output = V2<T::Output>;

    /// Negates a vector.
    fn neg(self) -> V2<T::Output> {
        -self.clone()
    }
}

impl<U, T: Add<U>> Add<V2<U>> for V2<T> {
    /// The result of adding two vectors is a vector.
    type Output = V2<T::Output>;

    /// Adds two vectors.
    fn add(self, other: V2<U>) -> V2<T::Output> {
        V2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, U: Clone, T: Add<U>> Add<&'a V2<U>> for V2<T> {
    /// The result of adding two vectors is a vector.
    type Output = V2<T::Output>;

    /// Adds two vectors.
    fn add(self, other: &'a V2<U>) -> V2<T::Output> {
        self + other.clone()
    }
}

impl<'a, U, T: Clone + Add<U>> Add<V2<U>> for &'a V2<T> {
    /// The result of adding two vectors is a vector.
    type Output = V2<T::Output>;

    /// Adds two vectors.
    fn add(self, other: V2<U>) -> V2<T::Output> {
        self.clone() + other
    }
}

impl<'a, 'b, U: Clone, T: Clone + Add<U>> Add<&'b V2<U>> for &'b V2<T> {
    /// The result of adding two vectors is a vector.
    type Output = V2<T::Output>;

    /// Adds two vectors.
    fn add(self, other: &'b V2<U>) -> V2<T::Output> {
        self.clone() + other.clone()
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

impl<T, U> Sub<V2<U>> for V2<T>
    where T: Sub<U>
{
    /// The result of subtracting two vectors is a vector.
    type Output = V2<T::Output>;

    /// Subtracts two vectors.
    fn sub(self, other: V2<U>) -> Self::Output {
        V2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a, T, U> Sub<&'a V2<U>> for V2<T>
    where T: Sub<&'a U>
{
    /// The result of subtracting two vectors is a vector.
    type Output = V2<T::Output>;

    /// Subtracts two vectors.
    fn sub(self, other: &'a V2<U>) -> Self::Output {
        V2::new(self.x - &other.x, self.y - &other.y)
    }
}

impl<'a, T, U> Sub<V2<U>> for &'a V2<T>
    where &'a T: Sub<U>
{
    /// The result of subtracting two vectors is a vector.
    type Output = V2<<&'a T as Sub<U>>::Output>;

    /// Subtracts two vectors.
    fn sub(self, other: V2<U>) -> Self::Output {
        V2::new(&self.x - other.x, &self.y - other.y)
    }
}

impl<'a, 'b, T, U> Sub<&'b V2<U>> for &'a V2<T>
    where &'a T: Sub<&'b U>
{
    /// The result of subtracting two vectors is a vector.
    type Output = V2<<&'a T as Sub<&'b U>>::Output>;

    /// Subtracts two vectors.
    fn sub(self, other: &'b V2<U>) -> Self::Output {
        V2::new(&self.x - &other.x, &self.y - &other.y)
    }
}


