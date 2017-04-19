use super::trie;
use super::alphabet::{char_code, NLETTERS};

/// A `TrieMap` mapping sequences of `usize`s (representing words) to
/// `usize`s.
pub type Freqs = trie::TrieMap<usize>;

/// Trains the spelling corrector by building a `TrieMap` from the
/// corpus. The corpus is supposed as an iterator over `char`s.
/// Consecutive sequences of Roman letters are considered to be words.
pub fn build_freqs<I>(mut chars: I) -> Freqs
    where I: Iterator<Item = char>
{
    let mut freqs  = trie::TrieMap::new(NLETTERS);

    // The algorithm here reads a character, and then checks if that
    // character is a word character by converting it to an alphabet
    // index. If the character is not a letter, it is ignored and we try
    // the next character. Otherwise, we create a mutable cursor and use
    // it to add a child for the given character, and then delegate to
    // `add_to_cursor` to handle the rest of the word. Note that
    // `add_to_cursor` merely borrows the iterator, so that we can
    // continue by adding the next word once it finishes.
    while let Some(c) = chars.next() {
        if let Some(d) = char_code(c) {
            add_to_cursor(freqs.cursor_mut().into_child_add(d), &mut chars)
        }
    }

    freqs
}

/// Given a mutable cursor and an iterator over `char`s, adds a path
/// corresponding to the sequence of chars, starting at the cursor.
fn add_to_cursor<I>(mut cursor: trie::CursorMut<usize>, chars: I)
    where I: Iterator<Item = char>
{
    // Iterate through characters, and check if each character is a word
    // character. If not, stop iterating; if so, add a child for that
    // character and update the cursor to point to the child.
    for c0 in chars {
        match char_code(c0) {
            None    => break,
            Some(c) => cursor = cursor.into_child_add(c),
        }
    }

    // Increment the value at the cursor, starting at 0 if absent.
    let count = cursor.value().unwrap_or(0);
    *cursor.value() = Some(count + 1);
}

#[cfg(test)]
mod test {
    use super::build_freqs;
    use super::super::alphabet::to_char_codes;

    #[test]
    fn test_build_simple() {
        test_build("hello, goodbye, hello",
                   &[
                   ("hello",   Some(2)),
                   ("goodbye", Some(1)),
                   ("nope",    None),
                   ("good",    None),
                   ]);
    }

    #[test]
    fn test_build_casefolding() {
        test_build("Hello, goodbye, hello.",
                   &[
                   ("hello",   Some(2)),
                   ("goodbye", Some(1)),
                   ("nope",    None),
                   ("good",    None),
                   ]);
    }

    #[test]
    fn test_build_overlap() {
        test_build("Hello, goodbye, hello, bye good.",
                   &[
                   ("hello",   Some(2)),
                   ("goodbye", Some(1)),
                   ("good",    Some(1)),
                   ("bye",     Some(1)),
                   ("nope",    None),
                   ("goo",     None),
                   ("goode",   None),
                   ]);
    }

    fn test_build(input: &str, results: &[(&str, Option<usize>)]) {
        let freqs = build_freqs(input.chars());

        for &(word, count) in results.iter() {
            assert_eq!(freqs[&*to_char_codes(word)], count);
        }
    }
}
