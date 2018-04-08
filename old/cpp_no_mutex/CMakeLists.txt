cmake_minimum_required(VERSION 3.3)
project(mutual_exclusion CXX)
include(.cmake/CMakeLists.txt)

add_test_program(no_mutex src/no_mutex.cpp)
