extern crate spelling_corrector;

#[cfg(not(test))]
fn main() {
    use std::env;
    use helpers::*;

    let dict = train_from_file(&env::args().nth(1).expect("training file"));
    correct_from_stdin(&dict);
}

#[cfg(not(test))]
mod helpers {
    use spelling_corrector::{train, suggest};
    use std::io::{BufRead, BufReader, Bytes, Read};
    use std::fs::File;

    pub fn correct_from_stdin(dict: &train::Freqs) {
        use std::io;
        use spelling_corrector::suggest::Result::*;

        let stdin = BufReader::new(io::stdin());

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

    pub fn train_from_file(file_name: &str) -> train::Freqs {
        let chars = Chars::open(file_name)
                      .expect("couldn't open training file");
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

    impl Chars<BufReader<File>> {
        fn open(filename: &str) -> Option<Self> {
            File::open(filename).ok().map(|f| Self::new(BufReader::new(f)))
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
}
