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

use std::io;

fn main() -> io::Result<()> {
    Ok(())
}

