//! Contains functions for converting between units.

/// Converts a Fahrenheit temperature into Celsius.
///
/// # Examples
///
/// ```
/// # use convert::*;
/// assert_eq!( f_to_c(212.), 100. );
/// ```
pub fn f_to_c(f: f64) -> f64 {
    5./9. * (f - 32.)
}

#[cfg(test)]
mod tests {
    use super::f_to_c;

    #[test]
    fn water_boiling() {
        assert_eq!(100., f_to_c(212.));
    }

    #[test]
    fn water_freezing() {
        assert_eq!(0., f_to_c(32.));
    }

    #[test]
    fn same_number() {
        assert_eq!(-40., f_to_c(-40.));
    }
}
