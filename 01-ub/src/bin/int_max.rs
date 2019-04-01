#![allow(dead_code)]

fn is_int_max(x: i32) -> bool {
    return x > x + 1;
}

fn better_is_int_max(x: i32) -> bool {
    return x > x.wrapping_add(1);
}

fn test_int(x: i32) {
    println!("{} is{} INT_MAX", x,
             if is_int_max(x) {""} else {"n't"});
}

fn main() {
    test_int(5);
    test_int(std::i32::MAX);
}
