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

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Read, stdin, stdout, Write};

fn main() {
    let words  = Words::new(stdin());
    let counts = count_words(words);
    let sorted = sort_counts(counts);
    display_counts(stdout(), &sorted);
}

fn count_words<I>(input: I) -> HashMap<String, usize>
    where I: Iterator<Item=String>
{
    let mut counts  = HashMap::new();

    for word in input {
        *counts.entry(word.to_owned()).or_insert(0) += 1;
    }

    counts
}

fn sort_counts(counts: HashMap<String, usize>) -> Vec<WordFreq> {
    let mut result: Vec<_> =
        counts.into_iter().map(|(s, c)| WordFreq(s, c)).collect();
    result.sort();
    result
}

fn display_counts<W: Write>(mut output: W, counts: &[WordFreq]) {
    for each in counts {
        writeln!(output, "{}", each).unwrap();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WordFreq(String, usize);

impl fmt::Display for WordFreq {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}: {}", self.0, self.1)
    }
}

impl Ord for WordFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.1.cmp(&self.1) {
            Ordering::Equal => self.0.cmp(&other.0),
            otherwise => otherwise,
        }
    }
}

impl PartialOrd for WordFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An iterator over the words of a `Read`.
#[derive(Debug)]
struct Words<R> {
    lines: std::io::Lines<BufReader<R>>,
    words: std::vec::IntoIter<String>,
}

impl<R: Read> Words<R> {
    fn new(input: R) -> Self {
        Words {
            lines: BufReader::new(input).lines(),
            words: Vec::new().into_iter(),
        }
    }
}

impl<R: Read> Iterator for Words<R> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(word) = self.words.next() {
                return Some(word);
            } else if let Some(Ok(line)) = self.lines.next() {
                self.words = line.split(|c| !is_word_char(c))
                                 .filter(|s| !s.is_empty())
                                 .map(|s| s.to_owned())
                                 .collect::<Vec<_>>()
                                 .into_iter();
            } else {
                return None;
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\''
}

#[cfg(test)]
mod words_iterator_tests {
    use super::Words;
    use std::io::Cursor;

    #[test]
    fn hello_world_goodbye_world() {
        assert_split(&["hello", "world", "goodbye", "world"],
                     "hello world,\ngoodbye world\n");
    }

    #[test]
    fn abcaba() {
        assert_split(&["a", "b", "c", "a", "b", "a"],
                      "   a--b, c. a b a.");
    }

    #[test]
    fn apostrophes() {
        assert_split(&["can't", "won't"],
                     "can't won't");
    }

    fn assert_split(expected: &[&str], input: &str) {
        let act: Vec<_> = Words::new(Cursor::new(input)).collect();
        let exp: Vec<_> = expected.into_iter().map(|s| s.to_owned()).collect();
        assert_eq!(exp, act);
    }
}

#[cfg(test)]
mod count_word_tests {
    use super::*;

    #[test]
    fn hello_world_goodbye_world() {
        assert_counts(&[("hello", 1), ("goodbye", 1), ("world", 2)],
                      &["hello", "world", "goodbye", "world"]);
    }

    #[test]
    fn abcaba() {
        assert_counts(&[("a", 3), ("b", 2), ("c", 1)],
                      &["a", "b", "c", "a", "b", "a"]);
    }

    #[test]
    fn apostrophes() {
        assert_counts(&[("can't", 1), ("won't", 1)],
                      &["can't", "won't"]);
    }

    #[test]
    fn case_sensitive() {
        assert_counts(&[("Hello", 1), ("hello", 2)],
                      &["Hello", "hello", "hello"]);
    }

    fn assert_counts(expected: &[(&str, usize)], input: &[&str]) {
        let map    = make_hash_map(expected);
        let iter   = input.into_iter().map(|&s| s.to_owned());
        let actual = count_words(iter);
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
    use super::*;

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
        let sorted = sort_counts(map);
        assert_eq!(make_wf_vec(expected), sorted);
    }
}

#[cfg(test)]
mod display_counts_tests {
    use super::*;
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
        let counts = make_wf_vec(counts);
        display_counts(&mut cursor, &counts);
        assert_eq!(expected.as_bytes(), cursor.into_inner().as_slice());
    }
}

#[cfg(test)]
fn make_hash_map(slice: &[(&str, usize)]) -> HashMap<String, usize> {
    slice.into_iter().map(|&(s, c)| (s.to_owned(), c)).collect()
}

#[cfg(test)]
fn make_wf_vec(slice: &[(&str, usize)]) -> Vec<WordFreq> {
    slice.into_iter().map(|&(s, c)| WordFreq(s.to_owned(), c)).collect()
}
