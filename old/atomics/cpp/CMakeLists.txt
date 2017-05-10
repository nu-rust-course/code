cmake_minimum_required(VERSION 3.3)
project(atomics CXX)
include(.eecs230/CMakeLists.txt)

add_program(data_race src/data_race.cpp)
add_program(mutex src/mutex.cpp)
add_program(publish src/publish.cpp)
