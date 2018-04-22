/// We can make an iterator over words, given an iterator over
/// characters. We also abstract it over the predicate on characters
/// that determines what characters should be included in a word.
pub struct Words<Chars, IsWordChar> {
    base: Chars,
    pred: IsWordChar,
}

impl<Chars, IsWordChar> Words<Chars, IsWordChar> {
    pub fn new(base: Chars, pred: IsWordChar) -> Self {
        Words { base, pred }
    }
}

impl<Chars, IsWordChar> Iterator for Words<Chars, IsWordChar>
    where Chars: Iterator<Item=char>,
          IsWordChar: Fn(char) -> bool
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buf;

        loop {
            let c = self.base.next()?;
            if (self.pred)(c) {
                buf = String::new();
                buf.push(c);
                break;
            }
        }

        while let Some(c) = self.base.next() {
            if (self.pred)(c) {
                buf.push(c)
            } else {
                break;
            }
        }

        Some(buf)
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

    #[test]
    fn hello_world_lower() {
        assert_words_lower("Hello, WORLD!", &["hello", "world"]);
    }

    fn assert_words(input: &str, expected_words: &[&str]) {
        use super::{Words, is_word_char};
        let actual_words: Vec<String> =
            Words::new(input.chars(), is_word_char).collect();
        let expected_words: Vec<String> =
            expected_words.into_iter().map(|&s| s.to_owned()) .collect();
        assert_eq!( actual_words, expected_words );
    }

    fn assert_words_lower(input: &str, expected_words: &[&str]) {
        use super::{Words, is_word_char};
        let actual_words: Vec<String> =
            Words::new(input.chars().flat_map(char::to_lowercase),
                       is_word_char)
                .collect();
        let expected_words: Vec<String> =
            expected_words.into_iter().map(|&s| s.to_owned()) .collect();
        assert_eq!( actual_words, expected_words );
    }
}

