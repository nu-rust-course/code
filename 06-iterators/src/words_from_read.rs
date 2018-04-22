use std::io;
use std::vec;

pub struct Words<R, IsWordChar> {
    lines: io::Lines<io::BufReader<R>>,
    words: vec::IntoIter<String>,
    pred:  IsWordChar,
}

impl<R: io::Read, IsWordChar> Words<R, IsWordChar> {
    pub fn new(input: R, pred: IsWordChar) -> Self {
        Words {
            lines: io::BufRead::lines(io::BufReader::new(input)),
            words: Vec::new().into_iter(),
            pred
        }
    }
}

impl<R, IsWordChar> Iterator for Words<R, IsWordChar>
    where R: io::Read,
          IsWordChar: Fn(char) -> bool
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(word) = self.words.next() {
                return Some(word);
            } else if let Some(Ok(line)) = self.lines.next() {
                self.words = line.split(|c| !(self.pred)(c))
                    .filter(|s| !s.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
                    .into_iter();
            } else {
                return None;
            }
        }
    }
}

pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '’'
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        assert_words("hello world", &["hello", "world"]);
        assert_words("hello, world", &["hello", "world"]);
        assert_words("   hello, world!   ", &["hello", "world"]);
    }

    #[test]
    fn empty() {
        assert_words("", &[]);
        assert_words("  ", &[]);
        assert_words(" - ", &[]);
    }

    fn assert_words(input: &str, expected_words: &[&str]) {
        use std::io::Cursor;
        use super::{Words, is_word_char};
        let actual_words: Vec<String> =
            Words::new(Cursor::new(input), is_word_char).collect();
        let expected_words: Vec<String> =
            expected_words.into_iter().map(|&s| s.to_owned()) .collect();
        assert_eq!( actual_words, expected_words );
    }

//    fn assert_words_lower(input: &str, expected_words: &[&str]) {
//        use super::{Words, is_word_char};
//        let actual_words: Vec<String> =
//            Words::new(input.chars().flat_map(char::to_lowercase),
//                       is_word_char)
//                .collect();
//        let expected_words: Vec<String> =
//            expected_words.into_iter().map(|&s| s.to_owned()) .collect();
//        assert_eq!( actual_words, expected_words );
//    }
}

