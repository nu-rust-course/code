use super::trie;
use super::alphabet::{char_code, NLETTERS};

pub type Freqs = trie::TrieMap<usize>;

pub fn build_freqs<I>(mut chars: I) -> Freqs
    where I: Iterator<Item = char>
{
    let mut freqs  = trie::TrieMap::new(NLETTERS);

    while let Some(c) = chars.next() {
        if let Some(d) = char_code(c) {
            add_to_cursor(freqs.cursor_mut().into_child_add(d), &mut chars)
        }
    }

    freqs
}

fn add_to_cursor<I>(mut cursor: trie::CursorMut<usize>, chars: &mut I)
    where I: Iterator<Item = char>
{
    for c0 in chars {
        match char_code(c0) {
            None    => break,
            Some(c) => cursor = cursor.into_child_add(c),
        }
    }

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
