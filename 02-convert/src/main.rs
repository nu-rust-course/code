//! Reads a Fahrenheit temperature from stdin and converts it into
//! Celsius.

extern crate convert;

use std::io;
use std::io::Write;

use convert::f_to_c;

fn main() {
    let f = read_input();
    let c = f_to_c(f);
    println!("{} °F = {} °C", f, c);
}

/// Prompts the user and reads the input.
fn read_input() -> f64 {
    print!("Enter a temperature in °F:\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read temperature");

    input.trim().parse()
        .expect("Could not parse into number")
}

