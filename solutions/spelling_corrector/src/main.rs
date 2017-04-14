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
    use std::io::BufReader;
    use std::io::BufRead;
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

    pub fn train_from_file(file_name: &String) -> train::Freqs {
        let chars = CharIter::open(file_name)
                      .expect("couldn't open training file");
        train::build_freqs(chars)
    }

    struct CharIter<R>(R, Vec<u8>);

    impl<R: BufRead> CharIter<R> {
        fn new(buf_read: R) -> Self {
            CharIter(buf_read, Vec::with_capacity(1))
        }
    }

    impl CharIter<BufReader<File>> {
        fn open(filename: &str) -> Option<Self> {
            File::open(filename).ok().map(|f| Self::new(BufReader::new(f)))
        }
    }

    impl<R: BufRead> Iterator for CharIter<R> {
        type Item = char;

        fn next(&mut self) -> Option<char> {
            self.0.read(&mut self.1).ok().map(|_| self.1[0] as char)
        }
    }
}
