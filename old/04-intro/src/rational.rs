//! Rational numbers as ratios of `isize`s.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Mul, Neg, Rem, Sub};

/// A rational number.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rational {
    num: isize,
    den: isize,
}
// Invariants:
//  - den > 0
//  - gcd(num, den) == 1

fn gcd(mut a: isize, mut b: isize) -> isize
{
    while a != 0 {
        let c = a;
        a = b % a;
        b = c;
    }

    b
}

#[allow(dead_code)]
fn gcd_generic<N>(mut a: N, mut b: N) -> N
    where N: Copy + Eq + Rem<Output = N> + Sub<Output = N>
{
    #[cfg_attr(feature = "cargo-clippy", allow(eq_op))]
    let zero = a - a;

    while a != zero {
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
    /// Panics if `den` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use intro::rational::*;
    /// let r = Rational::new(6, 8);
    /// assert_eq!(3, r.num());
    /// assert_eq!(4, r.den());
    /// ```
    pub fn new(mut num: isize, mut den: isize) -> Self {
        assert_ne!( den, 0, "Rational::new got 0 denominator" );

        let divisor = gcd(num, den);

        num /= divisor;
        den /= divisor;

        if den < 0 {
            num = -num;
            den = -den;
        }

        Rational { num, den }
    }

    /// Returns the numerator of the rational number in least terms.
    pub fn num(&self) -> isize { self.num }

    /// Returns the denominator of the rational number in least terms.
    pub fn den(&self) -> isize { self.den }

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
        Rational { num: -self.num(), den: self.den() }
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

        Rational { num, den }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn least_terms() {
        assert_eq!( Rational::new(2, 3), Rational::new(4, 6) );
        assert_eq!( Rational::new(2, 3), Rational::new(-4, -6) );
        assert_eq!( Rational::new(-2, 3), Rational::new(2, -3) );
    }

    #[test]
    #[should_panic]
    fn denominator_zero() {
        Rational::new(5, 0);
    }

    #[test]
    fn display() {
        assert_eq!( "5", Rational::new(5, 1).to_string() );
        assert_eq!( "5/2", Rational::new(5, 2).to_string() );
    }

    #[test]
    fn negation() {
        assert_eq!( Rational::new(-5, 6), -Rational::new(5, 6) );
        assert_eq!( Rational::new(5, 6), -Rational::new(-5, 6) );
        assert_eq!( Rational::new(5, 6), -&Rational::new(-5, 6) );
    }

    #[test]
    fn multiplication() {
        assert_eq!( Rational::new(1, 2),
                    Rational::new(2, 3) * Rational::new(3, 4) );
        assert_eq!( Rational::new(1, 2),
                    &Rational::new(2, 3) * Rational::new(3, 4) );
        assert_eq!( Rational::new(1, 2),
                    Rational::new(2, 3) * &Rational::new(3, 4) );
        assert_eq!( Rational::new(1, 2),
                    &Rational::new(2, 3) * &Rational::new(3, 4) );
    }
}
