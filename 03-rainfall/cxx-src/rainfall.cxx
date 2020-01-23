#include <iostream>
#include <sstream>
#include <vector>

// x1. Read from stdin.
// x2. Split into lines.
// x3. Detect sentinel "999".
// x4. Convert strings to f64.
// x5. Cull invalid input.
// x6. Calculate mean
// x7. Count intervals (second pass; requires storage)

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

double mean(std::vector<double> const& readings)
{
    double sum = 0.0;
    for (auto d : readings) sum += d;
    return sum / readings.size();
}

std::pair<size_t, size_t>
count_ranges(double mean,
             std::vector<double> const& readings)
{
    size_t below = 0, above = 0;

    for (auto d : readings) {
        if (mean - 5 <= d && d < mean) ++below;
        if (mean < d && d <= mean + 5) ++above;
    }

    return {below, above};
}

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

