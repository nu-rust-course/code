use std::io::BufRead;

fn main() {
    let numbers = get_readings(std::io::stdin().lock());
    println!("{:?}", numbers);
}

fn get_readings(input: impl BufRead) -> Vec<f64> {
    input.lines()
        .map(Result::unwrap)
        .take_while(|s| s != "999")
        .filter_map(|s|
            match s.parse() {
                Ok(f) if f >= 0.0 => Some(f),
                _ => None,
            })
        .collect()
}

/*
std::vector<double>
get_readings(std::istream& input)
{
    std::vector<double> result;
    std::string line;

    while (std::getline(input, line) && line != "999") {
        std::istringstream iss(line);
        if (double reading; iss >> reading && reading >= 0) {
            result.push_back(reading);
        }
    }

    return result;
}
*/

fn mean(readings: &[f64]) -> f64 {
    let mut sum = 0.0;

    for d in readings {
        sum += *d;
    }

    sum / readings.len() as f64
}

#[test]
fn mean_works() {
    assert_eq!( mean(&[7.0, 8.0, 9.0]), 8.0 );
}

fn count_ranges(mean: f64, readings: &[f64]) -> (usize, usize) {
//    let mut below = 0;
//    let mut above = 0;
//
//    for &d in readings {
//        if mean - 5. <= d && d < mean { below += 1; }
//        if mean < d && d <= mean + 5. { above += 1; }
//    }
//
//    (below, above);

    (readings.iter().copied().filter(|d| mean - 5. <= *d && *d < mean).count(),
     readings.iter().copied().filter(|d| mean < *d && *d <= mean + 5.).count())
}

/*
int main()
{
    auto readings = get_readings(std::cin);
    auto m = mean(readings);
    auto counts = count_ranges(m, readings);

    std::cout
            << "Mean:  " << m << "\n"
            << "Below: " << counts.first << "\n"
            << "Above: " << counts.second << "\n";
}

*/