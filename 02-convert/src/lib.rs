//! Provides functions for converting between units.

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
        assert_eq!(f_to_c(212.), 100.);
    }

    #[test]
    fn water_freezing() {
        assert_eq!(f_to_c(32.), 0.);
    }

    #[test]
    fn same_number() {
        assert_eq!(f_to_c(-40.), -40.);
    }
}
