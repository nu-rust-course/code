use super::alphabet;
use super::trie;
use std::string::String;
use std::ops::Deref;
use std::ops::DerefMut;

const MAX_EDIT_DISTANCE: usize = 2;

/// A correction result: correct, incorrect and uncorrectable, or
/// correctable with a suggestion.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Result {
    Correct,
    Incorrect,
    Suggestion(String)
}

/// Given a trie representing the corpus and a word, produces a
/// correction `Result` for the word.
///
/// # Errors
///
/// Panics if any characters in the word are not Roman letters.
pub fn suggest(trie: &trie::TrieMap<usize>, word: &str) -> Result {
    let word_vec   = alphabet::to_char_codes(word);

    if trie.contains(&word_vec) {
        return Result::Correct
    }

    let mut state = SearchState::new(word.len() + MAX_EDIT_DISTANCE);
    suggest_helper(trie.cursor(), &word_vec, 0, &mut state);

    if state.count > 0 {
        Result::Suggestion(state.best)
    } else {
        Result::Incorrect
    }
}

/// Represents the state of the search.
struct SearchState {
    /// Accumulates the current position in the trie
    buffer: String,
    /// The most frequent word we've found so far
    best:   String,
    /// The count of the best word we've found
    count:  usize,
}

/// When we add a character to the search state buffer, we return a
/// `NextState` guard object, which removes the character when it goes
/// out of scope. This makes it easier to maintain the buffer.
struct NextState<'a>(&'a mut SearchState);

impl<'a> Drop for NextState<'a> {
    fn drop(&mut self) {
        self.0.buffer.pop();
    }
}

// The contents of a `NextState` is a `SearchState,` and this allows us to
// dereference and use the `SearchState` automatically.
impl<'a> Deref for NextState<'a> {
    type Target = SearchState;
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'a> DerefMut for NextState<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
}

impl SearchState {
    /// Creates a new `SearchState` with the given buffer capacity.
    /// This should be the maximum post-correction word length that we
    /// might encounter. Because each edit can increase the length of
    /// the word, that's the length of the original word plus the
    /// maximum edit distance.
    fn new(bufsize: usize) -> Self {
        SearchState {
            buffer: String::with_capacity(bufsize),
            best:   String::with_capacity(bufsize),
            count:  0,
        }
    }

    /// Adds a character to the end of the buffer. The resulting
    /// `NextState` object will remove the character when it gets
    /// dropped.
    fn push(&mut self, c: usize) -> NextState {
        self.buffer.push(alphabet::code_char(c));
        NextState(self)
    }

    /// Record in the search state that the current word in the buffer
    /// has the given count. If this is better than we've seen before,
    /// then we remember it.
    fn visit(&mut self, count: usize) {
        if count > self.count {
            self.count = count;
            self.best.clone_from(&self.buffer)
        }
    }
}

/// Discovers suggestions by traversing the trie while applying edits.
/// The idea is that we can descend the trie following the characters of
/// a word, while at each step trying edits to the word. That is, we can
///
///  - descend the tree by following a character of the word,
///
///  - descend to an arbitrary child to implement insertion,
///
///  - skip a letter of the word and not descend to implement deletion,
///
///  - skip a letter and descend to an arbitrary child to implement
///  changing a letter, or
///
///  - descend to the 1st letter, then the 0th letter, to implement
///  transposition.
///
/// # Parameters
///
/// `cursor` – the current position in the trie where we are searching
///
/// `word` – the rest of the word to try to correct, represented as a
/// sequence of alphabet indices
///
/// `dist` – the number of edits that have been applied so far
///
/// `state` – the search state, whose buffer should reflect the current
/// position of `cursor`
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

            for c in 0 .. alphabet::NLETTERS {
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
        for c in 0 .. alphabet::NLETTERS {
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
