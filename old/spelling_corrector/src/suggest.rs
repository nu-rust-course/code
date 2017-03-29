use super::util;
use super::trie;
use std::string::String;
use std::ops::Deref;
use std::ops::DerefMut;

const MAX_EDIT_DISTANCE: usize = 2;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Result {
    Correct,
    Incorrect,
    Suggestion(String)
}

// PRECONDITION: word only contains letters
pub fn suggest(trie: &trie::TrieMap<usize>, word: &str) -> Result {
    let word_vec   = util::to_char_codes(word);

    if trie.contains(&*word_vec) {
        return Result::Correct
    }

    let mut state = SearchState {
        buffer: String::with_capacity(word.len() + MAX_EDIT_DISTANCE),
        best:   String::with_capacity(word.len() + MAX_EDIT_DISTANCE),
        count:  0,
    };

    suggest_helper(trie.cursor(), &word_vec, 0, &mut state);

    if state.count > 0 {
        Result::Suggestion(state.best)
    } else {
        Result::Incorrect
    }
}

struct SearchState {
    buffer: String,     // Accumulates the current position in the trie
    best:   String,     // The most frequent word we've found so far
    count:  usize,      // The count of the best word we've found
}

struct NextState<'a>(&'a mut SearchState);

impl<'a> Drop for NextState<'a> {
    fn drop(&mut self) {
        self.0.buffer.pop();
    }
}

impl<'a> Deref for NextState<'a> {
    type Target = SearchState;
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'a> DerefMut for NextState<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
}

impl SearchState {
    fn push(&mut self, c: usize) -> NextState {
        self.buffer.push(util::code_char(c));
        NextState(self)
    }

    fn visit(&mut self, count: usize) {
        if count > self.count {
            self.count = count;
            self.best.clone_from(&self.buffer)
        }
    }
}

fn suggest_helper(cursor: trie::Cursor<usize>, word: &[usize],
                  dist: usize, state: &mut SearchState)
{
    match word.get(0) {
        // We've reached the end of the word, so let's record its count.
        None => {
            if let &Some(count) = cursor.value() {
                state.visit(count);
            }
        }

        // Try adding the next character to the buffer, and recur (so no
        // edits here.)
        Some(&c) => {
            if let Some(cursor) = cursor.child(c) {
                let mut state = state.push(c);
                suggest_helper(cursor, &word[1..], dist, &mut state);
            }
        }
    }

    // If we are allowed more edits, try them...
    if dist < MAX_EDIT_DISTANCE {
        // DELETION AND REPLACEMENT
        // If there's at least one character left in the word, we can try
        // deleting or replacing it. To delete, just skip a character and
        // recur; to replace, try every possible child in the tree while
        // skipping a character.
        if word.len() >= 1 {
            suggest_helper(cursor, &word[1..], dist + 1, state);

            for c in 0 .. util::NLETTERS {
                if let Some(cursor) = cursor.child(c) {
                    let mut state = state.push(c);
                    suggest_helper(cursor, &word[1..], dist + 1, &mut state);
                }
            }
        }

        // TRANSPOSITION
        // To transpose, get the first two letters in the word and try
        // descending the trie in their reverse order.
        if let (Some(&c0), Some(&c1)) = (word.get(0), word.get(1)) {
            if let Some(cursor) = cursor.child(c1) {
                if let Some(cursor) = cursor.child(c0) {
                    let mut state = state.push(c1);
                    let mut state = state.push(c0);
                    suggest_helper(cursor, &word[2..], dist + 1, &mut state);
                }
            }
        }

        // INSERTION
        // To insert, try descending to each child while not moving in
        // the word
        for c in 0 .. util::NLETTERS {
            if let Some(cursor) = cursor.child(c) {
                let mut state = state.push(c);
                suggest_helper(cursor, word, dist + 1, &mut state);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::train::build_freqs;
    use super::suggest;
    use super::Result;
    use super::Result::*;

    #[test]
    fn test_suggest_small() {
        test_suggest("hello world",
                     &[
                     ("hello",    Correct),
                     ("Hello",    Correct),
                     ("world",    Correct),
                     ("XXXXX",    Incorrect),
                     ("helloxyz", Incorrect),
                     ("hellox",   Suggestion("hello".to_string())),
                     ("helloxy",  Suggestion("hello".to_string())),
                     ("hxellox",  Suggestion("hello".to_string())),
                     ("ehlol",    Suggestion("hello".to_string())),
                     ("heo",      Suggestion("hello".to_string())),
                     ("herro",    Suggestion("hello".to_string())),
                     ("herrr",    Incorrect),
                     ]);
    }

    #[test]
    fn test_suggest_frequent() {
        test_suggest("abd abcd abcd",
                     &[
                     ("ab", Suggestion("abcd".to_string())),
                     ]);
    }

    fn test_suggest(corpus: &str, results: &[(&str, Result)]) {
        let freqs = build_freqs(corpus.chars());

        for &(word, ref result) in results.iter() {
            assert_eq!(suggest(&freqs, word), *result);
        }
    }
}
