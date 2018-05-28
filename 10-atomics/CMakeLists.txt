cmake_minimum_required(VERSION 3.3)
project(atomics CXX)
include(.eecs395/CMakeLists.txt)

add_program(example_broken
        src/Run_example.h src/example_broken.cpp)
add_program(mutex_broken
        src/Run_example.h src/mutex_broken.cpp)
add_program(publish_broken
        src/Run_example.h src/publish_broken.cpp)

add_program(example_atomic
        src/Run_example.h src/example_atomic.cpp)
add_program(mutex_atomic
        src/Run_example.h src/mutex_atomic.cpp)
add_program(publish_atomic
        src/Run_example.h src/publish_atomic.cpp)
