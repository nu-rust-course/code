#![allow(dead_code)]

use std::collections::HashMap;
use std::io::stdin;

type CountTable = HashMap<String, usize>;

pub fn count_words() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("line to read");
    let word_list_punc: Vec<_> = input.split_whitespace().collect();

    let mut word_list = Vec::new();
    for w in word_list_punc {
        word_list.push(w.to_string());
    }

    let mut freqs = CountTable::new();
    for mut w in word_list {
        strip_word(&mut w);
        increment_word(&mut freqs, w);
    }

    let mut freq_list: Vec<Pair> = Vec::new();
    for key in freqs.keys(){
        freq_list.push(Pair{word: key.clone(), freq: *(freqs.get(key).expect("Error"))})
    }

}

struct Pair {
    word: String,
    freq: usize,
}

fn strip_word(s: &mut String) {
    let non_chars = vec!["," , "." , "/" , "?" , "!" , "-" , ":" , ";" , ")" , "(" ];
    for c in non_chars {
        if s.ends_with(c) {
            s.pop();
        }
    }
}

fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn inserts_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "one".to_owned());

        assert_eq!(Some(&1), h.get("one"));
        assert_eq!(1, h.len());
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "three".to_owned());
        expected.insert("three".to_owned(), 4);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn insert_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());

        h
    }
}


