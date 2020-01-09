fn main() {
    println!("Hello, CS 396/496!");
}

fn add_f64_str(ss: &[&str]) -> Option<f64> {
    let mut sum = 0.0;

    for s in ss {
        match parse_f64(s) {
            Some(f) => {
                sum += f;
            }
            _ => {
                return None;
            }
        }
    }

    Some(sum)
}

#[test]
fn add_f64_test() {
    assert_eq!(add_f64_str(&[]), Some(0.0));
    assert_eq!(add_f64_str(&["5", "8.5", "2"]), Some(15.5));
    assert_eq!(add_f64_str(&["A", "8.5", "2"]), None);
    assert_eq!(add_f64_str(&["5", "8.j", "2"]), None);
}

fn parse_f64(s: &str) -> Option<f64> {
    s.parse().ok()
}

#[test]
fn can_parse_0() {
    assert_eq!(parse_f64("0.0"), Some(0.0));
}

#[test]
fn can_parse_1point3() {
    assert_eq!(parse_f64("1.3"), Some(1.3));
    assert_eq!(parse_f64("1.30"), Some(1.3));
}

#[test]
fn cannot_parse_other_stuff() {
    assert_eq!(parse_f64("1/300"), None);
}

#[test]
fn this_is_a_test() {
    assert_eq!(3 + 2, 5);
}
