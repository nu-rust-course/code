//! freq -- word frequency counting
//!
//! Usage: freq < input.txt
//!
//! Assumptions:
//!
//!  - A word consists of alphanumeric characters and apostrophes,
//!    bounded by non-word characters.
//!
//!  - Case sensitive: "Hello" and "hello" are different words.
//!
//!  - Words with the same frequency will be printed in lexicographic
//!    order.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, stdin, stdout, Write};

fn main() {
    let counts = count_from_input(stdin());
    let sorted = sort_counts(&counts);
    display_counts(stdout(), &sorted);
}

fn count_from_input<R: Read>(input: R) -> HashMap<String, usize> {
    let mut counts  = HashMap::new();
    let mut lines   = BufReader::new(input).lines();

    while let Some(Ok(line)) = lines.next() {
        for word in line.split(|c| !is_word_char(c)) {
            if word.is_empty() {continue}
            *counts.entry(word.to_owned()).or_insert(0) += 1;
        }
    }

    counts
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\''
}

fn sort_counts(counts: &HashMap<String, usize>) -> Vec<(&str, usize)> {
    let mut result: Vec<_> =
        counts.into_iter().map(|(s, c)| (s.as_str(), *c)).collect();
    // First compare the counts in reverse order, and if those are the
    // same, compare the words in normal order:
    result.sort_by(|a, b| (b.1, a.0).cmp(&(a.1, b.0)));
    result
}

fn display_counts<W: Write>(mut output: W, counts: &[(&str, usize)]) {
    for &(word, count) in counts {
        writeln!(output, "{}: {}", word, count).unwrap();
    }
}

#[cfg(test)]
mod count_from_input_tests {
    use super::count_from_input;
    use super::test_helpers::*;
    use std::io::Cursor;

    #[test]
    fn hello_world_goodbye_world() {
        assert_counts(&[("hello", 1), ("goodbye", 1), ("world", 2)],
                      "hello world,\ngoodbye world\n");
    }

    #[test]
    fn abcaba() {
        assert_counts(&[("a", 3), ("b", 2), ("c", 1)],
                      "a--b, c. a b a.");
    }

    #[test]
    fn apostrophes() {
        assert_counts(&[("can't", 1), ("won't", 1)],
                      "can't won't");
    }

    #[test]
    fn case_sensitive() {
        assert_counts(&[("Hello", 1), ("hello", 2)],
                      "Hello hello   hello");
    }

    fn assert_counts(expected: &[(&str, usize)], input: &str) {
        let map    = make_hash_map(expected);
        let actual = count_from_input(Cursor::new(input));
        assert_eq!(map, actual);
    }
}

#[cfg(test)]
mod is_word_char_tests {
    use super::is_word_char;

    #[test]
    fn letters_are_word_chars() {
        assert!(is_word_char('a'));
        assert!(is_word_char('A'));
        assert!(is_word_char('z'));
        assert!(is_word_char('Z'));
    }

    #[test]
    fn digits_are_word_chars() {
        assert!(is_word_char('0'));
        assert!(is_word_char('9'));
    }

    #[test]
    fn apostrophe_is_word_char() {
        assert!(is_word_char('\''));
    }

    #[test]
    fn space_is_not_word_char() {
        assert!(!is_word_char(' '));
        assert!(!is_word_char('\t'));
    }

    #[test]
    fn punctuation_is_not_word_char() {
        assert!(!is_word_char(','));
        assert!(!is_word_char('.'));
        assert!(!is_word_char('"'));
        assert!(!is_word_char('”'));
    }
}

#[cfg(test)]
mod sort_counts_tests {
    use super::sort_counts;
    use super::test_helpers::*;

    #[test]
    fn hello_world_goodbye_world() {
        assert_sorted(&[("world", 2), ("goodbye", 1), ("hello", 1)],
                      &[("hello", 1), ("goodbye", 1), ("world", 2)]);
    }

    #[test]
    fn abcaba() {
        assert_sorted(&[("a", 3), ("b", 2), ("c", 1)],
                      &[("c", 1), ("b", 2), ("a", 3)]);
    }

    fn assert_sorted(expected: &[(&str, usize)], counts: &[(&str, usize)]) {
        let map    = make_hash_map(counts);
        let sorted = sort_counts(&map);
        assert_eq!(expected, sorted.as_slice());
    }
}

#[cfg(test)]
mod display_counts_tests {
    use super::display_counts;
    use std::io::Cursor;

    #[test]
    fn hello_world_goodbye_world() {
        assert_output("world: 2\ngoodbye: 1\nhello: 1\n",
                      &[("world", 2), ("goodbye", 1), ("hello", 1)]);
    }

    #[test]
    fn empty() {
        assert_output("", &[]);
    }

    fn assert_output(expected: &str, counts: &[(&str, usize)]) {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        display_counts(&mut cursor, counts);
        assert_eq!(expected.as_bytes(), cursor.into_inner().as_slice());
    }
}

#[cfg(test)]
mod test_helpers {
    use std::collections::HashMap;

    pub fn make_hash_map(slice: &[(&str, usize)]) -> HashMap<String, usize> {
        slice.into_iter().map(|&(s, c)| (s.to_owned(), c)).collect()
    }
}
