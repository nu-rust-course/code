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

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let fake_samples = &[12.5, 18., 7., 0., 4.];
    let stats = calculate_results(fake_samples);
    Ok(())
}

#[derive(PartialEq, Debug)]
struct RainfallStats {
    mean:  f64,
    below: usize,
    above: usize,
}

fn calculate_results(fs: &[f64]) -> RainfallStats {
    let mean  = mean(fs);
    let below = fs.iter().filter(|f| mean - 5.0 <= **f && **f < mean).count();
    let above = fs.iter().filter(|f| mean < **f && **f <= mean + 5.0).count();

    RainfallStats {
        mean,
        below,
        above,
    }
}

#[cfg(test)]
mod calculate_results_tests {
    use super::{calculate_results, RainfallStats};

    #[test]
    fn given_example() {
        let samples = &[12.5, 18., 7., 0., 4.];
        let expected = RainfallStats { mean: 8.3, above: 1, below: 2, };
        assert_eq!( calculate_results(samples), expected );
    }
}

fn mean(fs: &[f64]) -> f64 {
    fs.iter().sum::<f64>() / fs.len() as f64
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
        assert_eq!(mean(&[2., 3., 4.]), 3.);
    }
}

fn write_output<W: Write>(mut writer: W, r: RainfallStats) -> io::Result<()> {
    if r.mean.is_nan() {
        writeln!(writer, "No measurements provided.")?;
    } else {
        writeln!(writer, "Mean rainfall: {} cm", r.mean)?;
        writeln!(writer, "Below count:   {}", r.below)?;
        writeln!(writer, "Above count:   {}", r.above)?;
    }

    Ok(())
}

#[cfg(test)]
mod write_output_tests {
    use super::{write_output, RainfallStats};

    #[test]
    fn no_measurements_output() {
        use std::f64::NAN;
        assert_write( RainfallStats { mean: NAN, above: 0, below: 0, },
                      "No measurements provided.\n" );
    }

    #[test]
    fn some_measurements_output() {
        assert_write( RainfallStats { mean: 5., above: 2, below: 3, },
                      "Mean rainfail: 5 cm\n"
    }

    fn assert_write(stats: RainfallStats, expected: &str) {
        let mut writer = Vec::new();
        write_output(&mut writer, stats).unwrap();
        assert_eq!( String::from_utf8(writer).unwrap(), expected );
    }
}
