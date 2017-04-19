/// The number of letters in the alphabet.
pub const NLETTERS: usize = 26;

#[test]
fn test_char_code() {
    assert_eq!(char_code('a'), Some(0));
    assert_eq!(char_code('b'), Some(1));
    assert_eq!(char_code('z'), Some(25));
    assert_eq!(char_code('A'), Some(0));
    assert_eq!(char_code('B'), Some(1));
    assert_eq!(char_code('Z'), Some(25));
    assert_eq!(char_code(' '), None);
    assert_eq!(char_code(','), None);
    assert_eq!(char_code('\''), None);
}

/// Converts a letter to its position in the alphabet.
pub fn char_code(c: char) -> Option<usize> {
    if 'a' <= c && c <= 'z' {
        Some(c as usize - 'a' as usize)
    } else if 'A' <= c && c <= 'Z' {
        Some(c as usize - 'A' as usize)
    } else {
        None
    }
}

#[test]
fn test_code_char() {
    assert_eq!('a', code_char(0));
    assert_eq!('b', code_char(1));
    assert_eq!('z', code_char(25));
}

/// Converts a position in the alphabet to the letter.
pub fn code_char(c: usize) -> char {
    ('a' as usize + c) as u8 as char
}

#[test]
fn test_to_char_codes() {
    assert_eq!(vec![0, 1, 0, 25], to_char_codes("Abaz"));
}

#[test] #[should_panic]
fn test_to_char_codes_bad() {
    to_char_codes("Hello!");
}

/// Converts a word (made of letters) to a vector of its alphabet
/// positions.
pub fn to_char_codes(word: &str) -> Vec<usize> {
    word.chars()
        .map(|c| char_code(c).expect("got non-Roman character"))
        .collect()
}
