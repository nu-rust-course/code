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
 * The input terminates with either end-of-file or a line "999".
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
 *    routines, in particular the trait FromStr for type f64. This means
 *    that scientific notation ("3.4E22") is accepted, but hex is not.
 *
 *  - A line containing more than one number is noise and should be
 *    ignored.
 *
 *  - The terminator is a line of text "999", not a line of text that
 *    when interpreted is merely the number 999.0.
 *
 *  - Input must be perfect---even leading or trailing spaces make a
 *    line considered garbage.
 *
 *  - If there are no measurements to read then there is no mean value
 *    to print, so we will print an explanatory message instead.
 */

use std::io::{self, BufRead, BufReader, Read, stdin, Write, stdout};

fn main() {
    // Writing the whole program as a function that reads from a `Read`
    // and writes to `Write` is a bit weird, but when we can do it as
    // in this case, it lets us test the whole program from within
    // Rust's unit testing framework.
    transform(stdin(), stdout());
}

// Reads measurements from the given input stream and prints the summary to
// the given output stream. This function encapsulates the entire
// functionality of the rainfall program, which makes it possible to test
// the whole thing from simulated input to expected output. This isn't
// possible for every program, but when it is then it's pretty nice.
fn transform<R: Read, W: Write>(input: R, output: W) {
    let measurements = read_measurements(input);
    write_output(output, calculate_results(&measurements)).unwrap();
}

#[cfg(test)]
mod transform_tests {
    use super::transform;

    #[test]
    fn no_input() {
        assert_transform("", "No measurements provided.\n");
    }

    #[test]
    fn input_is_3_4_5() {
        assert_transform(
            "3\n4\n5\n",
            "Mean rainfall: 4 cm\nBelow count:   1\nAbove count:   1\n");
    }

    #[test]
    fn input_is_3_4_5_and_garbage() {
        assert_transform(
            "3\n4\ngarbage\n5\n",
            "Mean rainfall: 4 cm\nBelow count:   1\nAbove count:   1\n");
    }

    fn assert_transform(input: &str, expected_output: &str) {
        let mut output = Vec::new();
        transform(input.as_bytes(), &mut output);
        let output_string = String::from_utf8(output).unwrap();
        assert_eq!( output_string, expected_output );
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Results {
    mean:  f64,
    above: usize,
    below: usize,
}

// Reads the measurements from an input stream, cleaning and returning them.
fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
    BufReader::new(reader).lines()
        .map(|r| r.expect("Could not read measurement"))
        .take_while(|line| line != "999")
        .flat_map(|line| line.parse().ok())
        .filter(|&d| d >= 0.0)
        .collect()
}

#[cfg(test)]
mod read_measurements_tests {
    use super::read_measurements;

    #[test]
    fn reads_three_measurements() {
        assert_read(&[3., 4., 5.], "3\n4\n5\n");
    }

    #[test]
    fn discards_negative() {
        assert_read(&[3., 4., 5.], "3\n4\n-6\n5\n");
    }

    #[test]
    fn discards_noise() {
        assert_read(&[3., 4., 5.], "3\n4\nsix\n5\n");
    }

    #[test]
    fn stops_at_999() {
        assert_read(&[3., 4.], "3\n4\n999\n5\n");
    }

    // Asserts that reading from `input` yields `expected`.
    fn assert_read(expected: &[f64], input: &str) {
        let measurements = read_measurements(input.as_bytes());
        assert_eq!(expected, measurements.as_slice());
    }
}

// Calculates the results for the given dataset.
fn calculate_results(fs: &[f64]) -> Results {
    let mean = mean(fs);
    let below = fs.iter().filter(|x| mean - 5.0 <= **x && **x < mean).count();
    let above = fs.iter().filter(|x| mean < **x && **x <= mean + 5.0).count();

    Results {
        mean,
        above,
        below,
    }
}

#[cfg(test)]
mod calculate_results_tests {
    use super::{calculate_results, Results};

    #[test]
    fn given_example() {
        let samples  = &[12.5, 18., 7., 0., 4.];
        let expected = Results { mean: 8.3, above: 1, below: 2 };
        assert_eq!( expected, calculate_results(samples) );
    }
}

// Computes the mean of a slice of samples.
fn mean(samples: &[f64]) -> f64 {
    sum(samples) / samples.len() as f64
}

#[cfg(test)]
mod mean_tests {
    use super::mean;

    #[test]
    fn mean_empty_is_nan() {
        assert!(mean(&[]).is_nan());
    }

    #[test]
    fn mean_2_3_4_is_3() {
        assert_eq!(3.0, mean(&[2., 3., 4.]));
    }
}

// Computes the sum of a slice of samples.
fn sum(samples: &[f64]) -> f64 {
    samples.iter().fold(0.0, |a,b| a + *b)
}

#[cfg(test)]
mod sum_tests {
    use super::sum;

    #[test]
    fn sum_empty_is_0() {
        assert_eq!(0.0, sum(&[]));
    }

    #[test]
    fn sum_1_2_3_4_is_10() {
        assert_eq!(10.0, sum(&[1., 2., 3., 4.]));
    }
}

// Writes the results to the given output stream.
fn write_output<W: Write>(mut writer: W, r: Results) -> io::Result<()> {
  if r.mean.is_nan() {
      writeln!(writer, "No measurements provided.")
  } else {
      writeln!(writer, "Mean rainfall: {} cm", r.mean)?;
      writeln!(writer, "Below count:   {}", r.below)?;
      writeln!(writer, "Above count:   {}", r.above)
  }
}

#[cfg(test)]
mod write_output_tests {
    use super::{write_output, Results};

    #[test]
    fn no_measurements_output() {
        use std::f64::NAN;
        assert_write(
            "No measurements provided.\n",
            Results { mean: NAN, above: 0, below: 0 });
    }

    #[test]
    fn some_measurements_output() {
        assert_write(
            "Mean rainfall: 5 cm\nBelow count:   3\nAbove count:   2\n",
            Results { mean: 5., above: 2, below: 3 });
    }

    // Asserts that `results` when written produces `expected`.
    fn assert_write(expected: &str, results: Results) {
        let mut writer = Vec::new();
        write_output(&mut writer, results).unwrap();
        assert_eq!(expected, String::from_utf8(writer).unwrap());
        // consider the previous line versus:
        // assert_eq!(expected.as_bytes(), &*writer.into_inner());
    }
}
