use super::{Iter8or, IntoIter8or, vec::VecIntoIter};

use std::io;

pub struct Words<R, IsWordChar> {
    lines: io::Lines<R>,
    words: VecIntoIter<String>,
    pred:  IsWordChar,
}

impl<R: io::BufRead, IsWordChar: Fn(char) -> bool> Words<R, IsWordChar> {
    pub fn new(input: R, pred: IsWordChar) -> Self {
        Words {
            lines: input.lines(),
            words: Vec::new().into_iter8or(),
            pred
        }
    }
}

impl<R, IsWordChar> Iter8or for Words<R, IsWordChar>
    where R: io::BufRead,
          IsWordChar: Fn(char) -> bool
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        loop {
            if let Some(word) = self.words.next() {
                return Some(Ok(word));
            } else {
                match self.lines.next() {
                    Some(Ok(line)) =>
                        self.words = line.split(|c| !(self.pred)(c))
                            .filter(|s| !s.is_empty())
                            .map(ToOwned::to_owned)
                            .collect::<Vec<_>>()
                            .into_iter8or(),
                    Some(Err(e)) => return Some(Err(e)),
                    None => return None,
                }
            }
        }
    }
}

pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '’'
}

#[cfg(test)]
mod tests {
    use super::iter8or::Iter8or;

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
        use super::{Words, is_word_char};
        let actual_words: Vec<String> =
            Words::new(input.as_bytes(), is_word_char).map(Result::unwrap).collect();
        let expected_words: Vec<String> =
            expected_words.into_iter().map(ToOwned::to_owned).collect();
        assert_eq!( actual_words, expected_words );
    }
}

