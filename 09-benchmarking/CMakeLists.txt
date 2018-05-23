cmake_minimum_required(VERSION 3.0)
project(benchmarking LANGUAGES CXX)

find_package(benchmark REQUIRED)

add_executable(strings src/strings.cpp)
set_property(TARGET strings PROPERTY CXX_STANDARD 14)
target_link_libraries(strings benchmark::benchmark)
