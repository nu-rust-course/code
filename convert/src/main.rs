//! Reads a Fahrenheit temperature from stdin and converts it into
//! Celsius.

use std::io;
use std::io::Write;

fn main() {
    let f = read_input();
    let c = f_to_c(f);
    println!("{} °C", c);
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

/// Converts a Fahrenheit temperature into Celsius.
fn f_to_c(f: f64) -> f64 {
    5./9. * (f - 32.)
}

#[cfg(test)]
mod tests {
    use super::f_to_c;

    #[test]
    fn water_boiling() {
        assert_eq!(100., f_to_c(212.));
    }

    #[test]
    fn water_freezing() {
        assert_eq!(0., f_to_c(32.));
    }

    #[test]
    fn same_number() {
        assert_eq!(-40., f_to_c(-40.));
    }
}
