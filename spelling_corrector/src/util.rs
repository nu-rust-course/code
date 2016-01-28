pub const A_CODE:   usize = 'a' as usize;
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

pub fn char_code(c: char) -> Option<usize> {
    if 'a' <= c && c <= 'z' {
        Some(c as usize - A_CODE)
    } else if 'A' <= c && c <= 'Z' {
        Some(c.to_lowercase().next().unwrap() as usize - A_CODE)
    } else {
        None
    }
}

pub fn code_char(c: usize) -> char {
    ('a' as usize + c) as u8 as char
}

pub fn to_char_codes(word: &str) -> Vec<usize> {
    word.chars().map(|c| char_code(c).unwrap()).collect::<Vec<_>>()
}
