//! Rational numbers as ratios of `isize`s.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Mul, Neg};

/// A rational number.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rational {
    num_: isize,
    den_: isize,
}

fn gcd(mut a: isize, mut b: isize) -> isize
{
    while a != 0 {
        let c = a;
        a = b % a;
        b = c;
    }

    b
}

impl Rational {
    /// Creates a new rational number with the given numerator and
    /// denominator.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::rational::*;
    /// let r = Rational::new(6, 8);
    /// assert_eq!(3, r.num());
    /// assert_eq!(4, r.den());
    /// ```
    pub fn new(mut n: isize, mut d: isize) -> Self {
        if d == 0 {
            panic!("Rational::new got 0 denominator");
        }

        let divisor = gcd(n, d);

        n /= divisor;
        d /= divisor;

        if d < 0 {
            n = -n;
            d = -d;
        }

        Rational { num_: n, den_: d }
    }

    /// Returns the numerator of the rational number in least terms.
    pub fn num(&self) -> isize { self.num_ }

    /// Returns the denominator of the rational number in least terms.
    pub fn den(&self) -> isize { self.den_ }

    /// Approximates the rational as an `f64`.
    pub fn as_f64(&self) -> f64 {
        self.num() as f64 / self.den() as f64
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.den() == 1 {
            write!(fmt, "{}", self.num())
        } else {
            write!(fmt, "{}/{}", self.num(), self.den())
        }
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational { num_: -self.num(), den_: self.den() }
    }
}

impl<'a> Neg for &'a Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        -*self
    }
}

impl Mul<Rational> for Rational {
    type Output = Rational;

    #[cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]
    fn mul(self, other: Rational) -> Rational {
        let ab_divisor = gcd(self.num(), other.den());
        let ba_divisor = gcd(other.num(), self.den());

        let a_num = self.num() / ab_divisor;
        let b_num = other.num() / ba_divisor;
        let a_den = self.den() / ba_divisor;
        let b_den = other.den() / ab_divisor;

        let num = a_num * b_num;
        let den = a_den * b_den;

        Rational { num_: num, den_: den }
    }
}

impl<'a> Mul<&'a Rational> for Rational {
    type Output = Rational;

    fn mul(self, other: &'a Rational) -> Rational {
        self * *other
    }
}

impl<'a> Mul<Rational> for &'a Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        *self * other
    }
}

impl<'a, 'b> Mul<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn mul(self, other: &'b Rational) -> Rational {
        *self * *other
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Rational) -> Ordering {
        (self.num() * other.den()).cmp(&(self.den() * other.num()))
    }
}
