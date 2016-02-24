cmake_minimum_required(VERSION 3.3)
project(seq_cst CXX)
include(.eecs230/CMakeLists.txt)

add_program(seq_cst src/seq_cst.cpp)
add_program(data_race src/data_race.cpp)
add_program(atomics src/atomics.cpp)
