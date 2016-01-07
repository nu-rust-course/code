use std::collections::HashMap;

#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Assumptions:
"]
fn main() {
    // let word_counts = read_and_count(...);
    // print_counts(word_counts);
}

// bool
// char
// usize (like size_t)
// ssize (like ssize_t)
// u8, u16, u32, u64
// s8, ...
// f32, f64
//
// String (like std::string)
// &str (like const char*)

// impl<K, V, S> HashMap<K, V, S> {
//     fn get(&self, k: &K) -> Option<&V> {
//         ...
//     }
// }

// let π = e;
// stms...
//
//   ===
//
// match e {
//   π => stms...
// }

fn increment_word(mut map: &mut HashMap<String, usize>, word: String) {
    let count_ref = map.entry(word).or_insert(0);
    *count_ref += 1;
}
