extern crate spelling_corrector;

use spelling_corrector::{train, suggest};
use std::io::{BufRead, BufReader, Bytes, Read};
use std::fs::File;

fn main() {
    let filename = std::env::args().nth(1).expect("training file");
    let file     = File::open(filename).expect("couldn't open training file");
    let dict     = train(file);
    correct(std::io::stdin(), &dict);
}

pub fn correct<R: Read>(input: R, dict: &train::Freqs) {
    use spelling_corrector::suggest::Result::*;

    let stdin = BufReader::new(input);

    for mline in stdin.lines() {
        let untrimmed = mline.expect("a line of input");
        let trimmed   = untrimmed.trim();

        match suggest::suggest(dict, trimmed) {
            Correct          => println!("{}", trimmed),
            Incorrect        => println!("{}, -", trimmed),
            Suggestion(sugg) => println!("{}, {}", trimmed, sugg),
        }
    }
}

pub fn train<R: Read>(input: R) -> train::Freqs {
    let chars = Chars::new(BufReader::new(input));
    train::build_freqs(chars)
}

// This is a hack for getting an iterator over the chars of a Read.
// We need it because Read::chars() is not yet stable. It works
// provided the input file is ASCII, but for 8 bit encodings it may
// not do the right thing.
struct Chars<R>(Bytes<R>);

impl<R: Read> Chars<R> {
    fn new(read: R) -> Self {
        Chars(read.bytes())
    }
}

impl<R: Read> Iterator for Chars<R> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some(Ok(b)) = self.0.next() {
            Some(b as char)
        } else {
            None
        }
    }
}
