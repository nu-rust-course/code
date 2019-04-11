//! Rational numbers as ratios of `isize`s.

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

