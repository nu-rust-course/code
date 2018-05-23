#include <benchmark/benchmark.h>
#include <string>

#define STRING_CONSTANT "hello, world"

void empty_string_creation(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(std::string());
    }
}
BENCHMARK(empty_string_creation);

void string_creation(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(std::string(STRING_CONSTANT));
    }
}
BENCHMARK(string_creation);

void string_copy(benchmark::State& state)
{
    const std::string s(STRING_CONSTANT);

    for (auto _ : state) {
        std::string copy = s;
        benchmark::DoNotOptimize(copy);
    }
}
BENCHMARK(string_copy);

int main(int argc, char** argv)
{
    benchmark::Initialize(&argc, argv);
    benchmark::RunSpecifiedBenchmarks();
}