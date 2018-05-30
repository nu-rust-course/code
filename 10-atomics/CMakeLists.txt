cmake_minimum_required(VERSION 3.3)
project(atomics CXX)

include_directories(src)

macro (add_program name)
    add_executable(${name} ${ARGN})
    set_property(TARGET ${name} PROPERTY CXX_STANDARD 14)
    set_property(TARGET ${name} PROPERTY CXX_STANDARD_REQUIRED On)
endmacro (add_program)

add_program(example_example
        src/Run_example.h src/example_example.cpp)

add_program(race_broken
        src/Run_example.h src/race_broken.cpp)
add_program(mutex_broken
        src/Run_example.h src/mutex_broken.cpp)
add_program(publish_broken
        src/Run_example.h src/publish_broken.cpp)

add_program(race_atomic
        src/Run_example.h src/race_atomic.cpp)
add_program(mutex_atomic
        src/Run_example.h src/mutex_atomic.cpp)
add_program(publish_atomic
        src/Run_example.h src/publish_atomic.cpp)
