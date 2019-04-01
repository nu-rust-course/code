#![feature(test)]

extern crate test;

use test::Bencher;

const STRING_CONST: &str = "hello, world";

#[bench]
fn empty_string_creation(b: &mut Bencher) {
    b.iter(String::new);
}

#[bench]
fn string_creation(b: &mut Bencher) {
    b.iter(|| STRING_CONST.to_owned());
}

#[bench]
fn string_cloning(b: &mut Bencher) {
    let s = STRING_CONST.to_owned();

    b.iter(|| s.clone());
}

#[bench]
fn string_clone_via_format(b: &mut Bencher) {
    let s = STRING_CONST.to_owned();

    b.iter(|| format!("{}", s))
}