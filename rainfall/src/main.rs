/**!
 * rainfall
 *
 * Reads a sequence of rainfall measurements from the standard input and
 * writes a summary to the standard output.
 *
 * INPUT
 *
 * The input format is a sequence of measurements represented as
 * unitless, non-negative numbers, written in ASCII, one per line:
 *
 *     12.5
 *     18
 *     7
 *     0
 *     4
 *
 * Any noise in the input---blank lines, non-numeric text, negative
 * numbers---should be ignored:
 *
 *     seven
 *     -9
 *
 * The input terminates with either EOF or a line 999.
 *
 * OUTPUT
 *
 * The program computes three quantities: the mean (valid) measurement,
 * the count of measurements in the interval [mean - 5, mean), and the
 * count of measurements in the interval (mean, mean + 5]. It prints the
 * results in this format:
 *
 *     Mean rainfall: 8.3 cm
 *     Below count:   2
 *     Above count:   1
 *
 * ASSUMPTIONS
 *
 *  - Numbers are read according to the language’s number reading
 *  routines, in particular the trait FromStr for type f64. This means
 *  that scientific notation ("3.4E22") is accepted, but hex is not.
 *
 *  - A line containing more than one number is noise and should be
 *  ignored.
 *
 *  - The terminator is a line of text "999", not a line of text that
 *    when interpreted is merely the number 999.0.
 *
 *  - Input must be perfect---even leading or trailing spaces make a
 *    line considered garbage.
 *
 *  - If there are no measurements to read then there is no mean value
 *    to print, so
 */

use std::io::{BufRead,BufReader,Read,stdin};

fn main() {
    let measurements = read_measurements(stdin());
    produce_output(calculate_result(&measurements));
}

#[derive(Copy, Clone)]
struct Result {
    mean: f64,
    below: usize,
    above: usize,
}

/// Reads valid (non-negative, non-noise) measurements from `reader`.
fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
    let mut measurements = vec![];
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        if line == "999" { break }

        if let Ok(f) = line.parse() {
            if f >= 0.0 {
                measurements.push(f);
            }
        }
    }

    return measurements;
}

#[cfg(test)]
mod read_measurements_test {
    use super::read_measurements;
    use std::io::{Read, Result};

    #[test]
    fn reads_three_measurements() {
        assert_read(&[3., 4., 5.], "3\n4\n5\n");
    }

    #[test]
    fn discards_negative() {
        assert_read(&[3., 4., 5.], "3\n4\n-7\n5\n");
    }

    fn assert_read(expected: &[f64], input: &str) {
        let mock_reader  = StringReader::new(input.to_owned());
        let measurements = read_measurements(mock_reader);

        assert_eq!(expected, &measurements as &[f64]);
    }

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader { contents: s.into_bytes(), position: 0 }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }
}

/// Calculates the mean and counts given the samples.
fn calculate_result(fs: &[f64]) -> Result {
    let mean  = mean(fs);
    let below = count_vec(fs, |x| mean - 5.0 <= x && x < mean);
    let above = count_vec(fs, |x| mean < x && x <= mean + 5.0);

    Result {mean: mean, below: below, above: above}
}

/// Counts the number of elements of vector `v` satisfying predicate `f`
fn count_vec<F: Fn(f64) -> bool>(v: &[f64], f: F) -> usize {
    v.iter().filter(|&&x| f(x)).count()
}

/// Computes the mean of a slice, returning NaN if the slice is empty.
fn mean(samples: &[f64]) -> f64 {
    sum(samples) / samples.len() as f64
}

#[cfg(test)]
mod mean_test {
    use super::mean;

    #[test]
    fn mean_of_one() {
        assert_eq!(4.5, mean(&[4.5]));
    }

    #[test]
    fn mean_of_two() {
        assert_eq!(4.5, mean(&[3., 6.]));
    }

    #[test]
    fn mean_of_more() {
        assert_eq!(3.5, mean(&[3., 5., 8., -2.]));
    }

    #[test]
    fn mean_of_empty() {
        assert!(mean(&[]).is_nan());
    }
}

/// Sums the numbers in a slice.
fn sum(samples: &[f64]) -> f64 {
    samples.iter().fold(0.0, |x,&y| x + y)
}

#[cfg(test)]
mod sum_test {
    use super::sum;

    #[test]
    fn empty_sums_to_0() {
        assert_eq!(0., sum(&[]));
    }

    #[test]
    fn triangle_sums_to_10() {
        assert_eq!(10., sum(&[1., 2., 3., 4.]));
    }
}

/// Prints the results.
fn produce_output(r: Result) {
    if r.mean.is_nan() {
        println!("No measurements provided.");
    } else {
        println!("Mean rainfall: {} cm", r.mean);
        println!("Below count:   {}", r.below);
        println!("Above count:   {}", r.above);
    }
}

